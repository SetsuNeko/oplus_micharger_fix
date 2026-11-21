#include <android/binder_interface_utils.h>
#include <android/binder_manager.h>
#include <android/binder_process.h>
#include <aidl/vendor/xiaomi/hardware/micharge/IMiCharge.h>
#include <log/log.h>
#include <fstream>
#include <string>
#include <sstream>

using namespace aidl::vendor::xiaomi::hardware::micharge;

class MiChargeService : public BnMiCharge {
public:
    MiChargeService() = default;

    // 读取 sysfs 整数
    int readSysfsInt(const std::string& path) {
        std::ifstream file(path);
        if (!file.is_open()) return -1;
        int val = -1;
        file >> val;
        return val;
    }

    // 读取 sysfs 字符串
    std::string readSysfsStr(const std::string& path) {
        std::ifstream file(path);
        if (!file.is_open()) return "";
        std::string val;
        std::getline(file, val);
        return val;
    }

    // 关键方法：获取快充类型
    ndk::ScopedAStatus getQuickChargeType(std::string* _aidl_return) override {
        int vooc = readSysfsInt("/sys/class/oplus_chg/battery/voocchg_ing");
        int fast = readSysfsInt("/sys/class/oplus_chg/battery/fast_charge");
        
        if (vooc == 1 || fast == 1) {
            ALOGD("[MiChargeCpp] getQuickChargeType: vooc=%d fast=%d -> 2", vooc, fast);
            *_aidl_return = "2"; // SuperRapidCharge
        } else {
            *_aidl_return = "0";
        }
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getTxAdapt(std::string* _aidl_return) override {
        *_aidl_return = readSysfsStr("/sys/class/power_supply/usb/REAL_TYPE");
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getChargingPowerMax(std::string* _aidl_return) override {
        int voltage = readSysfsInt("/sys/class/power_supply/battery/voltage_now");
        int current = readSysfsInt("/sys/class/power_supply/battery/current_now");
        if (voltage > 0 && current > 0) {
            long power_mw = (long)voltage * (long)current / 1000;
            *_aidl_return = std::to_string(power_mw);
        } else {
            *_aidl_return = "0";
        }
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryChargeType(std::string* _aidl_return) override {
        *_aidl_return = std::to_string(readSysfsInt("/sys/class/oplus_chg/battery/charge_technology"));
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryVbat(std::string* _aidl_return) override {
        *_aidl_return = std::to_string(readSysfsInt("/sys/class/power_supply/battery/voltage_now"));
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryIbat(std::string* _aidl_return) override {
        *_aidl_return = std::to_string(readSysfsInt("/sys/class/power_supply/battery/current_now"));
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryCapacity(std::string* _aidl_return) override {
        *_aidl_return = std::to_string(readSysfsInt("/sys/class/power_supply/battery/capacity"));
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatterySoh(std::string* _aidl_return) override {
        *_aidl_return = std::to_string(readSysfsInt("/sys/class/oplus_chg/battery/soh"));
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryTbat(std::string* _aidl_return) override {
        *_aidl_return = std::to_string(readSysfsInt("/sys/class/power_supply/battery/temp"));
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus isFunctionSupported(const std::string& funcName, bool* _aidl_return) override {
        *_aidl_return = false;
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus isWirelessChargingSupported(bool* _aidl_return) override {
        *_aidl_return = false;
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getMiChargePath(const std::string& path, std::string* _aidl_return) override {
        *_aidl_return = "";
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryCommonInfo(const std::string& info, std::string* _aidl_return) override {
        *_aidl_return = "";
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryAuthentic(std::string* _aidl_return) override {
        *_aidl_return = "1";
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryChargeFull(std::string* _aidl_return) override {
        *_aidl_return = std::to_string(readSysfsInt("/sys/class/power_supply/battery/charge_full"));
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryCycleCount(std::string* _aidl_return) override {
        *_aidl_return = std::to_string(readSysfsInt("/sys/class/oplus_chg/battery/cycle_count"));
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryResistance(std::string* _aidl_return) override {
        *_aidl_return = std::to_string(readSysfsInt("/sys/class/oplus_chg/battery/resistance_id"));
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryThermaLevel(std::string* _aidl_return) override {
        *_aidl_return = "0";
        return ndk::ScopedAStatus::ok();
    }
};

int main() {
    android::base::InitLogging(nullptr, android::base::LogdLogger(android::base::DEBUG));
    ALOGD("[MiChargeCpp] Starting MIUI Charge HAL Bridge for OPPO");

    // 检查 OPPO 充电节点
    std::ifstream test("/sys/class/oplus_chg/battery/voocchg_ing");
    if (test.good()) {
        int vooc;
        test >> vooc;
        ALOGD("[MiChargeCpp] OPPO oplus_chg node found: voocchg_ing=%d", vooc);
    } else {
        ALOGE("[MiChargeCpp] OPPO oplus_chg node NOT found");
    }

    // 启动 binder 线程池
    ABinderProcess_setThreadPoolMaxThreadCount(4);
    ABinderProcess_startThreadPool();

    // 创建服务实例
    std::shared_ptr<MiChargeService> service = ndk::SharedRefBase::make<MiChargeService>();

    // 注册服务
    const char* instance = "vendor.xiaomi.hardware.micharge.IMiCharge/default";
    binder_status_t status = AServiceManager_addService(service->asBinder().get(), instance);
    if (status != STATUS_OK) {
        ALOGE("[MiChargeCpp] Failed to register service: %d", status);
        return 1;
    }
    ALOGD("[MiChargeCpp] Service registered: %s", instance);

    // 加入线程池，阻塞等待 binder 调用
    ABinderProcess_joinThreadPool();
    return 0;
}
