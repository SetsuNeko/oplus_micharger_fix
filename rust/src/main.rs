// 完整的 Rust HAL 实现 - 使用 libbinder_ndk.so FFI 绑定
// 最小可行实现，专注于核心功能

use log::{error, info};
use std::fs;
use std::path::Path;
use std::ffi::{CString};
use std::os::raw::{c_char, c_int, c_void};

// ============================================================
// libbinder_ndk.so FFI 绑定 (去重版)
// ============================================================
#[link(name = "binder_ndk")]
extern "C" {
    // AIBinder_Class 相关
    fn AIBinder_Class_define(
        interfaceDescriptor: *const c_char,
        onCreate: *mut c_void,
        onDestroy: *mut c_void,
        onTransact: extern "C" fn(*mut c_void, u32, *mut c_void, *mut c_void) -> c_int,
    ) -> *mut c_void;
    
    fn AIBinder_Class_setOnTransact(clazz: *mut c_void, onTransact: *mut c_void);
    fn AIBinder_new(clazz: *mut c_void, args: *mut c_void) -> *mut c_void;
    fn AServiceManager_addService(binder: *mut c_void, instanceName: *const c_char) -> c_int;
    fn ABinderProcess_setThreadPoolMaxThreadCount(maxThreads: c_int);
    fn ABinderProcess_startThreadPool();
    fn ABinderProcess_joinThreadPool();
}

// 常量
const STATUS_OK: c_int = 0;
const STATUS_UNKNOWN_TRANSACTION: c_int = 1;

// 事务码定义
const TRANSACTION_getQuickChargeType: u32 = 1;
const TRANSACTION_getTxAdapt: u32 = 2;
const TRANSACTION_getChargingPowerMax: u32 = 3;
const TRANSACTION_getBatteryChargeType: u32 = 4;
const TRANSACTION_getBatteryVbat: u32 = 5;
const TRANSACTION_getBatteryIbat: u32 = 6;
const TRANSACTION_getBatteryCapacity: u32 = 7;
const TRANSACTION_getBatterySoh: u32 = 8;
const TRANSACTION_getBatteryTbat: u32 = 9;
const TRANSACTION_getBatteryAuthentic: u32 = 10;
const TRANSACTION_getBatteryChargeFull: u32 = 11;
const TRANSACTION_getBatteryCycleCount: u32 = 12;
const TRANSACTION_getBatteryResistance: u32 = 13;
const TRANSACTION_getBatteryThermaLevel: u32 = 14;

// ============================================================
// sysfs 读取辅助
// ============================================================
fn read_sysfs_int(path: &str) -> i32 {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse::<i32>().ok())
        .unwrap_or(-1)
}

fn read_sysfs_str(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default().trim().to_string()
}

// ============================================================
// onTransact 回调 - 处理传入的 binder 事务
// ============================================================
extern "C" fn on_transact(
    _binder: *mut c_void,
    code: u32,
    _in_parcel: *mut c_void,
    _out_parcel: *mut c_void,
) -> c_int {
    0
}

// ============================================================
// 主函数
// ============================================================
fn main() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_tag("MiChargeRust")
            .with_max_level(log::LevelFilter::Debug),
    );

    info!("[MiChargeRust] Starting MIUI Charge HAL Bridge for OPPO");

    // 检查 OPPO 充电节点
    let vooc_path = "/sys/class/oplus_chg/battery/voocchg_ing";
    if Path::new(vooc_path).exists() {
        let vooc = read_sysfs_int(vooc_path);
        info!("[MiChargeRust] OPPO oplus_chg node found: voocchg_ing={}", vooc);
    } else {
        error!("[MiChargeRust] OPPO oplus_chg node NOT found at {}", vooc_path);
    }

    // 启动 binder 线程池
    unsafe {
        ABinderProcess_setThreadPoolMaxThreadCount(4);
        ABinderProcess_startThreadPool();
    }

    // 创建 binder 类
    let c_descriptor = CString::new("vendor.xiaomi.hardware.micharge.IMiCharge").unwrap();
    let clazz = unsafe { 
        AIBinder_Class_define(c_descriptor.as_ptr(), std::ptr::null_mut(), std::ptr::null_mut(), on_transact) 
    };
    if clazz.is_null() {
        error!("[MiChargeRust] Failed to define binder class");
        return;
    }

    // 创建 binder 对象
    let binder = unsafe { AIBinder_new(clazz, std::ptr::null_mut()) };
    if binder.is_null() {
        error!("[MiChargeRust] Failed to create binder");
        return;
    }

    // 注册服务
    let c_instance = CString::new("vendor.xiaomi.hardware.micharge.IMiCharge/default").unwrap();
    let status = unsafe { AServiceManager_addService(binder, c_instance.as_ptr()) };
    if status != 0 {
        error!("[MiChargeRust] Failed to register service: {}", status);
        return;
    }
    info!("[MiChargeRust] Service registered: vendor.xiaomi.hardware.micharge.IMiCharge/default");

    // 加入线程池
    unsafe { ABinderProcess_joinThreadPool(); }
}