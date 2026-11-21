# charger_hal — MIUI 快充 HAL 移植（OPPO Pad）

将 OPPO Pad（OPD2101 / OP5223，SM8250）的充电能力，桥接到 MIUI/HyperOS 3.3 的
快充 UI 协议上，让系统能正确显示快充动画、电量小数与充电功率。

> **当前状态：Production（生产可用）**
> 现役实现是 `c_charge_service/` 下的纯 C 守护进程。
> 其余实现（C++ AIDL / Rust）为探索性分支，见「演进路线」。

---

## 一、为什么需要这个 HAL

### 机制真相

反编译 `MiuiSystemUI.apk` 后确认：**MIUI 的快充 UI 完全依靠广播，不查询任何 binder/HIDL 服务**。

| 广播 Action | Extras | 作用 |
|---|---|---|
| `miui.intent.action.ACTION_QUICK_CHARGE_TYPE` | `miui.intent.extra.quick_charge_type`（0普通/1快充/2超快充/3/4更强）<br>`miui.intent.extra.POWER_MAX`（功率 W）<br>`miui.intent.extra.CAR_CHARGE` | 决定充电动画的档位 |
| `miui.intent.action.ACTION_SOC_DECIMAL` | `miui.intent.extra.soc_decimal`（小数部分 ×100）<br>`miui.intent.extra.soc_decimal_rate`（10 秒内增速 ×100） | 驱动锁屏两位小数电量动画 |

关键类：
- `com.miui.charge.MiuiChargeManager` — 接收上述广播，计算 `chargeSpeed`
- `com.miui.charge.ChargeUtils.getChargeSpeed(wireState, chargeDeviceType)` — 档位映射
- `com.miui.charge.MiuiChargeController` — 锁屏充电动画与小数显示

`chargeSpeed` 映射关系（`wireState` 必须为 10/11 才有非零值）：
```
chargeDeviceType 1 → chargeSpeed 1（快速充电）
chargeDeviceType 2/3 → chargeSpeed 2（超级快充 / MiTurbo）
chargeDeviceType 4 → chargeSpeed 3（最强档）
```

### OPPO 侧缺什么

OPPO vendor 没有任何组件发送这些广播，因此 MIUI 的 `chargeSpeed` 恒为 0，
表现为「充电但不显示快充」。

### 本 HAL 的职责

1. 读取 OPPO 充电 sysfs（`/sys/class/oplus_chg/`、`/sys/class/power_supply/`）
2. 判断充电协议类型（VOOC/SVOOC/PD/QC/DCP）
3. 按 MIUI 协议广播状态变化

---

## 二、目录结构

```
charger_hal/
├── README.md                              本文档
├── Android.bp                             AIDL HAL 的编译定义（未来用）
├── aidl/
│   └── vendor/xiaomi/hardware/micharge/
│       └── IMiCharge.aidl                 接口契约（从 MIUI 逆向）
├── manifest/
│   └── vendor.xiaomi.hardware.micharge.xml  VINTF manifest（AIDL HAL 需注册）
├── sepolicy/
│   ├── hal_micharge.te                    SELinux 策略（.te 格式，未启用）
│   └── hal_micharge_fc                    file_contexts
├── michager_policy.cil                    SELinux 策略（CIL 格式，备用）
│
├── c_charge_service/                      ★ 现役实现（Production）
│   ├── micharge_service.c                 源码
│   ├── vendor.oplus.sandronedevteam.michager-service   编译产物（13KB）
│   └── micharge_hal                       早期构建产物
│
├── cpp_charge_service/                    ⚠️ 探索分支（未采用）
│   ├── MiChargeService.cpp                AIDL 服务主体
│   ├── MiChargeService_dlopen.cpp         dlopen libbinder.so 方案
│   ├── MiChargeService_simple.cpp         最简版本
│   ├── service_main.cpp
│   └── libbinder.so / libutils.so         从设备拉取的库
│
├── rust/                                  ⚠️ 探索分支（未编译通过）
│   ├── src/main.rs                        libbinder_ndk FFI 直连
│   ├── aidl/…/IMiCharge.aidl
│   ├── gen/rust/…/IMiCharge.rs            AIDL 生成的 Rust 绑定
│   ├── Cargo.toml                         依赖 binder-rust 0.1.2
│   └── build.sh                           构建脚本
│
├── src/MiChargeImpl.cpp                   AIDL 实现（未来用）
├── device_libs/                           从设备拉取的 binder 相关库（供分析）
├── deprecated/                            已废弃方案归档
└── rc/
    └── vendor.xiaomi.hardware.micharge-service.rc   AIDL HAL 的 init 配置
```

---

## 三、现役实现（c_charge_service）

### 3.1 架构

```
┌─────────────────────────────────────────────┐
│  micharger-service（native daemon, root）    │
│                                             │
│  每 500ms（快充中 150ms）轮询：               │
│    /sys/class/power_supply/usb/real_type    │
│    /sys/class/oplus_chg/battery/voocchg_ing │
│    /sys/class/oplus_chg/battery/fast_charge │
│    /sys/class/power_supply/battery/*        │
│                                             │
│  状态变化时 → fork + execve                 │
│    /system/bin/app_process64                 │
│    com.android.commands.am.Am broadcast ... │
└─────────────────────────────────────────────┘
                    ↓ 广播
        MIUI SystemUI（快充动画 / 小数）
```

### 3.2 三段式状态机（仿原厂行为）

```
插上充电器
   │
   ├─ 立即广播 type=1（"快速充电"，乐观显示）
   │
   ├─ 检测到 SVOOC/VOOC 握手 → 广播 type=2（MiTurbo）
   ├─ 检测到 PD/QC        → 保持 type=1
   └─ 3 秒内无任何快充协议 → 广播 type=0（降级为普通充电）

快充进行中
   └─ 每 10 秒广播 SOC_DECIMAL
        rate = 实时功率(W) × 8.62（按 8360mAh@3.85V 折算）
```

**为什么是"乐观显示"**：小米原厂在协议确认前就先显示快速充电，
确认后再升降档，避免插上瞬间的"普通充电"闪烁。

### 3.3 三个关键实现细节（踩过的坑）

#### ① 必须用 `/system/bin/app_process64`，不是 APEX 路径

```c
// ✗ 错误：本机 ART 不走 APEX 挂载
execl("/apex/com.android.art/bin/app_process64", ...);

// ✓ 正确
execv("/system/bin/app_process64", argv);
```

本机 `/apex/com.android.art/bin/app_process64` **不存在**，exec 会静默失败。

#### ② 必须用 `execv` 数组，不能用 `execl` 变参

```c
// ✗ 错误：变参在条件分支里会丢参数
execl(path, "app_process64", "/system/bin", "com.android.commands.am.Am",
      "broadcast", "-a", action,
      "--ei", k1, v1, "--ei", k2, v2,
      (k3 ? "--ei" : NULL), (k3 ? v3 : NULL),   // ← NULL 提前终止参数列表
      "--receiver-foreground", NULL);
// 结果：am 收到 "--ei 0"（key 丢值）→ 报 java.lang.IllegalArgumentException

// ✓ 正确：显式构建 argv 数组
char *argv[20]; int i = 0;
argv[i++] = "app_process64";
argv[i++] = "/system/bin";
argv[i++] = "com.android.commands.am.Am";
argv[i++] = "broadcast";
argv[i++] = "-a"; argv[i++] = (char *)action;
if (k1) { argv[i++] = "--ei"; argv[i++] = (char *)k1; argv[i++] = (char *)v1; }
/* ... */
argv[i] = NULL;
execv("/system/bin/app_process64", argv);
```

#### ③ init rc 必须用 `seclabel` + `boot_completed` 触发

```rc
service vendor.oplus.sandronedevteam.michager-service /system/bin/vendor.oplus.sandronedevteam.michager-service
    class late_start
    user root
    group root
    seclabel u:r:init:s0        # ← 关键：显式指定域，绕过文件标签转换
    disabled                     # ← 关键：不自动启动

on property:sys.boot_completed=1
    start vendor.oplus.sandronedevteam.michager-service
```

**为什么需要 `seclabel`**：
本机 `vendor_file` 标签的 rc 文件不会被 init 解析，
用 `seclabel u:r:init:s0` 可直接以 init 域运行，避开所有 SELinux 域转换问题。

### 3.4 部署

| 文件 | 目标路径 |
|---|---|
| `vendor.oplus.sandronedevteam.michager-service` | `/system/bin/`（0755）|
| `vendor.oplus.sandronedevteam.michager-service.rc` | `/system/etc/init/`（0644）|

> 部署在 **system 分区**（ext4），不是 vendor。
> 原因：vendor/product 是 erofs 只读，且 vendor 的 file_contexts 机制会踩坑。

### 3.5 验证

```bash
# 服务是否拉起
getprop init.svc.vendor.oplus.sandronedevteam.michager-service   # running

# 广播日志
logcat -s MiChargerService
# 应看到：
#   QUICK_CHARGE_TYPE: type=1 power=5W
#   QUICK_CHARGE_TYPE: type=2 power=7W
#   SOC_DECIMAL: level=92 decimal=53 rate=34

# 系统侧确认
logcat -s MiuiChargeManager | grep chargeSpeed
# 应看到 chargeSpeed: 2
```

---

## 四、演进路线

### 4.1 为什么现在是 daemon 而不是 AIDL HAL

MIUI 的快充 UI **不消费 binder 服务**（已用反编译证实），
所以 AIDL HAL 在当前协议下**对 UI 无直接价值**。

但改成 AIDL HAL 仍有意义：
- 符合 Android HAL 规范（便于长期维护、SELinux 策略、VINTF 注册）
- 未来若有 MIUI 组件查询 `vendor.xiaomi.hardware.micharge.IMiCharge` 可直接对接
- 广播逻辑可作为 HAL 内部实现保留

### 4.2 目标架构（Rust AIDL HAL）

```
┌──────────────────────────────────────────────────┐
│  vendor.oplus.sandronedevteam.michager-service   │
│  (Rust, AIDL HAL)                                │
│                                                  │
│  ┌────────────────────────────────────────────┐  │
│  │ IMiCharge AIDL 服务（binder）               │  │
│  │  getQuickChargeType() → "2"                │  │
│  │  getChargingPowerMax() → "7"               │  │
│  │  getBatteryCapacity() → "92"               │  │
│  │  ...（19 个方法，见 IMiCharge.aidl）        │  │
│  └────────────────────────────────────────────┘  │
│                                                  │
│  ┌────────────────────────────────────────────┐  │
│  │ 广播桥接线程（保留现有逻辑）                 │  │
│  │  轮询 sysfs → 状态机 → am broadcast        │  │
│  └────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────┘
```

### 4.3 Rust 化的已探索结论

**已尝试的方案与阻塞点**（都在 `rust/` 目录留有代码）：

| 方案 | 状态 | 阻塞原因 |
|---|---|---|
| `binder-rust` crate 0.1.2 | ❌ 不可用 | `register_service` 有设计缺陷：传给 `addService` 的是 ServiceManager 指针而非 service binder 对象。改源码后可编译但运行时仍不通 |
| `libbinder_ndk.so` + FFI | ❌ 链接失败 | NDK 的 `libbinder_ndk.so` **不导出** `AServiceManager_addService`、`ABinderProcess_*` 等符号（只导出 `AIBinder_*` 基础 API） |
| `libbinder.so` + FFI | ❌ 链接失败 | 设备上的 `/system/lib64/libbinder.so` 被 **strip**，无动态符号表 |
| `dlopen` + `dlsym` | ⚠️ 部分可用 | 运行时能找到 `AIBinder_Class_define` 等，但**本机 `AIBinder_Class_define` 要求 onCreate/onDestroy 全部非 NULL**（反汇编确认），否则返回 NULL。补上空回调后可注册成功 |

**关键发现（本机特性）**：
```
AIBinder_Class_define 反汇编（/system/lib64/libbinder_ndk.so）：
  0x1ad88: cbz x19, 0x1adf0   ← descriptor 为 NULL → 返回 NULL
  0x1ad90: cbz x1,  0x1adf0   ← onCreate 为 NULL → 返回 NULL  ★
  0x1ad98: cbz x2,  0x1adf0   ← onDestroy 为 NULL → 返回 NULL ★
  0x1ada0: cbz x3,  0x1adf0   ← onTransact 为 NULL → 返回 NULL
```
必须提供 4 个非 NULL 参数（AOSP 老版本允许 onCreate/onDestroy 为 NULL）。

### 4.4 推荐的 Rust 改造路线

**路线 A（推荐）：在 AOSP 源码树内编译**
```
在 ACK/BakaBuildPlafrom 里创建 rust_ffi 或 aidl_hal 模块
使用 Soong 的 rust_binder 支持（Android 12+ 原生支持 Rust AIDL HAL）
优点：链接、符号、SELinux、VINTF 全部由构建系统处理
```

**路线 B：用 C 写 binder 协议，Rust 写业务逻辑**
```
C 侧：直接操作 /dev/binder ioctl（零库依赖，最稳）
Rust 侧：通过 FFI 调用 C 的注册/回调框架
优点：绕开所有 binder crate 的兼容性问题
```

**路线 C：Rust 静态链接 libbinder_ndk（需自己补符号）**
```
手动实现 AServiceManager_addService 的 binder 事务（约 200 行 C）
通过 extern "C" 暴露给 Rust
```

### 4.5 迁移时的注意事项

1. **AIDL 版本**：`IMiCharge.aidl` 是 V2（`version: "2", stable: true`），
   改接口需要升 version 并更新 `manifest/`
2. **VINTF 注册**：AIDL HAL 需要在 `/vendor/etc/vintf/manifest/` 注册
   （模板见 `manifest/vendor.xiaomi.hardware.micharge.xml`）
3. **SELinux**：`sepolicy/hal_micharge.te` 是 .te 格式，需要编译进 vendor sepolicy；
   当前已有一个可用的 CIL 版本 `michager_policy.cil`（但未启用，因为 daemon 跑在 init 域）
4. **广播逻辑不要丢**：即使做了 AIDL HAL，MIUI UI 仍靠广播，
   广播桥接必须保留（可作为 HAL 内部线程）
5. **execve 路径与 execv 传参**：这两个坑在 Rust 里同样存在
   （`Command::new("/system/bin/app_process64").args([...])` 即可，比 C 简单）

---

## 五、硬件适配信息

### OPPO 充电节点

| 节点 | 含义 | 取值 |
|---|---|---|
| `/sys/class/oplus_chg/battery/voocchg_ing` | VOOC 快充中 | 0/1 |
| `/sys/class/oplus_chg/battery/fast_charge` | 快充中 | 0/1 |
| `/sys/class/power_supply/usb/real_type` | USB 协议类型 | `USB_DCP` / `USB_PD` / `SVOOC` / `Unknown` |
| `/sys/class/power_supply/usb/online` | USB 供电 | 0/1 |
| `/sys/class/power_supply/ac/online` | AC 供电 | 0/1 |
| `/sys/class/power_supply/battery/capacity` | 电量 | 0-100 |
| `/sys/class/power_supply/battery/voltage_now` | 电压 | µV |
| `/sys/class/power_supply/battery/current_now` | 电流 | µA |

### 电池参数（用于电量小数推算）

```
容量：8360 mAh
标称电压：3.85 V
总能量：8360 × 3.85 = 32.2 Wh = 115.9 kJ

rate（10 秒增速 ×100）= 功率(W) × 10s × 100 / 115920 J
                      = 功率 × 8.62
```

### 协议判定逻辑

```c
if (voocchg_ing == 1 || fast_charge == 1 ||
    real_type 含 "VOOC" || fast_chg_type == 1)
    qct = 2;   // 私有协议 → 超级快充（MiTurbo）
else if (real_type 含 "PD" || real_type 含 "QC" || fast_chg_type == 2)
    qct = 1;   // 公有快充
else
    qct = 0;   // 普通充电
```

---

## 六、已知限制

| 限制 | 说明 |
|---|---|
| 依赖 `am` 命令启动开销 | 每次广播要 fork+exec 一个 ART 进程（约 100-200ms）。优化方向：常驻一个 ART 进程或用 binder 直连 ActivityManager |
| 无法获取真实电量小数 | OPPO 只提供整数 SOC（`capacity`/`chip_soc`），两位小数是按功率**推算**的 |
| 快充协议识别靠字符串匹配 | `real_type` 的取值依赖 OPPO 内核实现，不同机型可能不同 |
| SELinux 域名 | 当前跑在 `init` 域（权限最大），生产环境应切到专用域（`michager_policy.cil` 已备好，需验证） |

---

## 七、参考

- 反编译依据：`MiuiSystemUI.apk` → `com.miui.charge.*`
- MIUI 广播协议来源：`MiuiChargeManager.java` / `MiuiChargeController.java`
- 相关修复文档：`../fixs/README.md`（01-charging-hal）
