#include <aidl/vendor/xiaomi/hardware/micharge/BnMiCharge.h>
#include <android/binder_process.h>
#include <android/binder_manager.h>
#include <utils/Log.h>
#include <fstream>
#include <string>
#include <sstream>

using ::aidl::vendor::xiaomi::hardware::micharge::IMiCharge;
using ::android::OK;
using ::android::status_t;
using ::android::sp;
using ::android::String16;
using ::android::ProcessState;
using ::android::IServiceManager;
using ::android::defaultServiceManager;

namespace aidl {
namespace vendor {
namespace xiaomi {
namespace hardware {
namespace micharge {
namespace implementation {

class MiChargeImpl : public BnMiCharge {
public:
    MiChargeImpl() = default;

    // 读取 sysfs 节点为整数
    static int readSysfsInt(const std::string& path) {
        std::ifstream file(path);
        if (!file.is_open()) return -1;
        int val = -1;
        file >> val;
        return val;
    }

    // 读取 sysfs 节点为字符串
    static std::string readSysfsString(const std::string& path) {
        std::ifstream file(path);
        if (!file.is_open()) return "";
        std::string val;
        std::getline(file, val);
        return val;
    }

    // 关键方法：返回快充类型
    // MIUI 映射：4=Strong Super Rapid, 2/3=Super Rapid, 1=Rapid, 0=None
    ndk::ScopedAStatus getQuickChargeType(std::string* _aidl_return) override {
        int vooc = readSysfsInt("/sys/class/oplus_chg/battery/voocchg_ing");
        int fast = readSysfsInt("/sys/class/oplus_chg/battery/fast_charge");
        
        if (vooc == 1 || fast == 1) {
            // OPPO Pad 支持 SVOOC 33W，映射为 SuperRapidCharge (2)
            *_aidl_return = "2";
            ALOGD("[MiCharge] getQuickChargeType: vooc=%d fast=%d -> %s", 
                  vooc, fast, _aidl_return->c_str());
        } else {
            *_aidl_return = "0";
        }
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getTxAdapt(std::string* _aidl_return) override {
        // 读取适配器类型
        std::string type = readSysfsString("/sys/class/power_supply/usb/REAL_TYPE");
        if (type.empty()) type = "USB";
        *_aidl_return = type;
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getChargingPowerMax(std::string* _aidl_return) override {
        // 计算最大充电功率：电压 * 电流 / 1000000 (微伏*微安 -> 微瓦 -> 毫瓦)
        // 或者直接读取 OPPO 节点
        int voltage = readSysfsInt("/sys/class/power_supply/battery/voltage_now");
        int current = readSysfsInt("/sys/class/power_supply/battery/current_now");
        
        if (voltage > 0 && current > 0) {
            // 微伏 * 微安 = 微瓦，除以 1000 得到毫瓦
            long power_mw = (long)voltage * (long)current / 1000;
            *_aidl_return = std::to_string(power_mw);
        } else {
            *_aidl_return = "0";
        }
        ALOGD("[MiCharge] getChargingPowerMax: volt=%d curr=%d -> %s", 
              voltage, current, _aidl_return->c_str());
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryChargeType(std::string* _aidl_return) override {
        int charge_type = readSysfsInt("/sys/class/oplus_chg/battery/charge_technology");
        *_aidl_return = std::to_string(charge_type);
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryVbat(std::string* _aidl_return) override {
        int vbat = readSysfsInt("/sys/class/power_supply/battery/voltage_now");
        *_aidl_return = std::to_string(vbat);
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryIbat(std::string* _aidl_return) override {
        int ibat = readSysfsInt("/sys/class/power_supply/battery/current_now");
        *_aidl_return = std::to_string(ibat);
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryCapacity(std::string* _aidl_return) override {
        int cap = readSysfsInt("/sys/class/power_supply/battery/capacity");
        *_aidl_return = std::to_string(cap);
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatterySoh(std::string* _aidl_return) override {
        int soh = readSysfsInt("/sys/class/oplus_chg/battery/soh");
        *_aidl_return = std::to_string(soh);
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryTbat(std::string* _aidl_return) override {
        int temp = readSysfsInt("/sys/class/power_supply/battery/temp");
        *_aidl_return = std::to_string(temp);
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus isFunctionSupported(
            const std::string& funcName, bool* _aidl_return) override {
        *_aidl_return = false;
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus isWirelessChargingSupported(bool* _aidl_return) override {
        *_aidl_return = false;
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getMiChargePath(
            const std::string& path, std::string* _aidl_return) override {
        *_aidl_return = "";
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryCommonInfo(
            const std::string& info, std::string* _aidl_return) override {
        *_aidl_return = "";
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryAuthentic(std::string* _aidl_return) override {
        *_aidl_return = "1";
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryChargeFull(std::string* _aidl_return) override {
        int full = readSysfsInt("/sys/class/power_supply/battery/charge_full");
        *_aidl_return = std::to_string(full);
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryCycleCount(std::string* _aidl_return) override {
        int cycle = readSysfsInt("/sys/class/oplus_chg/battery/cycle_count");
        *_aidl_return = std::to_string(cycle);
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryResistance(std::string* _aidl_return) override {
        int res = readSysfsInt("/sys/class/oplus_chg/battery/resistance_id");
        *_aidl_return = std::to_string(res);
        return ndk::ScopedAStatus::ok();
    }

    ndk::ScopedAStatus getBatteryThermaLevel(std::string* _aidl_return) override {
        *_aidl_return = "0";
        return ndk::ScopedAStatus::ok();
    }
};

}  // namespace implementation
}  // namespace micharge
}  // namespace hardware
}  // namespace xiaomi
}  // namespace vendor

void run() {
    using ::aidl::vendor::xiaomi::hardware::micharge::implementation::MiChargeImpl;

    std::string instance = IMiCharge::descriptor + "/default";
    auto binder = ndk::SharedRefBase::make<MiChargeImpl>();
    binder_status_t status = AServiceManager_addService(binder->asBinder().get(), instance.c_str());
    if (status != STATUS_OK) {
        ALOGE("[MiCharge] Failed to add service: %d", status);
        return;
    }
    ALOGD("[MiCharge] Service published: %s", instance.c_str());
    ABinderProcess_joinThreadPool();
}

int main() {
    ABinderProcess_setThreadPoolMaxThreadCount(4);
    ABinderProcess_startThreadPool();
    run();
    return 0;
}
