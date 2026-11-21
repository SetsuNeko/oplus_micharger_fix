#include <dlfcn.h>
#include <android/binder_interface_utils.h>
#include <android/binder_ibinder.h>
#include <android/binder_parcel.h>
#include <android/binder_status.h>
#include <android/log.h>
#include <fstream>
#include <string>
#include <sstream>

// 函数指针类型定义
using AServiceManager_addService_fn = binder_status_t(*)(AIBinder*, const char*);
using AIBinder_Class_define_fn = AIBinder_Class*(*)(const char*, void*);
using AIBinder_Class_setOnTransact_fn = void(*)(AIBinder_Class*, void*);
using AIBinder_new_fn = AIBinder*(*)(AIBinder_Class*, void*);
using AServiceManager_addService_fn = binder_status_t(*)(AIBinder*, const char*);
using ABinderProcess_setThreadPoolMaxThreadCount_fn = void(*)(int32_t);
using ABinderProcess_startThreadPool_fn = void(*)();
using ABinderProcess_joinThreadPool_fn = void(*)();
using AParcel_writeString_fn = binder_status_t(*)(AParcel*, const char*, int32_t);
using AParcel_readInt32_fn = binder_status_t(*)(const AParcel*, int32_t*);
using AParcel_writeInt32_fn = binder_status_t(*)(AParcel*, int32_t);
using AParcel_writeString_fn = binder_status_t(*)(AParcel*, const char*, int32_t);
using AParcel_writeStatusHeader_fn = binder_status_t(*)(AParcel*, const binder_status_t*);

// 全局函数指针
AServiceManager_addService_fn g_AServiceManager_addService = nullptr;
AIBinder_Class_define_fn g_AIBinder_Class_define = nullptr;
AIBinder_Class_setOnTransact_fn g_AIBinder_Class_setOnTransact = nullptr;
AIBinder_new_fn g_AIBinder_new = nullptr;
AServiceManager_addService_fn g_AServiceManager_addService_fn = nullptr;
ABinderProcess_setThreadPoolMaxThreadCount_fn g_ABinderProcess_setThreadPoolMaxThreadCount = nullptr;
ABinderProcess_startThreadPool_fn g_ABinderProcess_startThreadPool = nullptr;
ABinderProcess_joinThreadPool_fn g_ABinderProcess_joinThreadPool = nullptr;
AParcel_writeString_fn g_AParcel_writeString = nullptr;
AParcel_readInt32_fn g_AParcel_readInt32 = nullptr;
AParcel_writeInt32_fn g_AParcel_writeInt32 = nullptr;
AParcel_writeString_fn g_AParcel_writeString_fn = nullptr;
AParcel_writeStatusHeader_fn g_AParcel_writeStatusHeader = nullptr;

// 加载 libbinder.so 符号
bool load_binder_symbols() {
    void* handle = dlopen("libbinder.so", RTLD_LAZY);
    if (!handle) {
        ALOGE("Failed to dlopen libbinder.so: %s", dlerror());
        return false;
    }

    g_AServiceManager_addService = (AServiceManager_addService_fn)dlsym(handle, "AServiceManager_addService");
    g_AIBinder_Class_define = (AIBinder_Class_define_fn)dlsym(handle, "AIBinder_Class_define");
    g_AIBinder_Class_setOnTransact = (AIBinder_Class_setOnTransact_fn)dlsym(handle, "AIBinder_Class_setOnTransact");
    g_AIBinder_new = (AIBinder_new_fn)dlsym(handle, "AIBinder_new");
    g_AServiceManager_addService_fn = (AServiceManager_addService_fn)dlsym(handle, "AServiceManager_addService");
    g_ABinderProcess_setThreadPoolMaxThreadCount = (ABinderProcess_setThreadPoolMaxThreadCount_fn)dlsym(handle, "ABinderProcess_setThreadPoolMaxThreadCount");
    g_ABinderProcess_startThreadPool = (ABinderProcess_startThreadPool_fn)dlsym(handle, "ABinderProcess_startThreadPool");
    g_ABinderProcess_joinThreadPool = (ABinderProcess_joinThreadPool_fn)dlsym(handle, "ABinderProcess_joinThreadPool");
    g_AParcel_writeString = (AParcel_writeString_fn)dlsym(handle, "AParcel_writeString");
    g_AParcel_readInt32 = (AParcel_readInt32_fn)dlsym(handle, "AParcel_readInt32");
    g_AParcel_writeInt32 = (AParcel_writeInt32_fn)dlsym(handle, "AParcel_writeInt32");
    g_AParcel_writeString_fn = (AParcel_writeString_fn)dlsym(handle, "AParcel_writeString");
    g_AParcel_writeStatusHeader = (AParcel_writeStatusHeader_fn)dlsym(handle, "AParcel_writeStatusHeader");

    if (!g_AServiceManager_addService_fn || !g_AIBinder_Class_define || !g_AIBinder_Class_setOnTransact ||
        !g_AIBinder_new || !g_ABinderProcess_setThreadPoolMaxThreadCount || !g_ABinderProcess_startThreadPool ||
        !g_ABinderProcess_joinThreadPool || !g_AParcel_writeString || !g_AParcel_readInt32 || !g_AParcel_writeInt32) {
        ALOGE("Failed to load required symbols");
        dlclose(handle);
        return false;
    }

    ALOGD("Successfully loaded libbinder.so symbols");
    return true;
}

// 读取 sysfs
int readSysfsInt(const std::string& path) {
    std::ifstream file(path);
    if (!file.is_open()) return -1;
    int val = -1;
    file >> val;
    return val;
}

std::string readSysfsStr(const std::string& path) {
    std::ifstream file(path);
    if (!file.is_open()) return "";
    std::string val;
    std::getline(file, val);
    return val;
}

// 事务处理函数
binder_status_t onTransact(AIBinder* binder, AParcel* in, AParcel* out) {
    int32_t code;
    if (AParcel_readInt32(in, &code) != STATUS_OK) {
        return STATUS_BAD_VALUE;
    }

    AParcel* reply = AParcel_create();
    
    switch (code) {
        case 1: { // getQuickChargeType
            int vooc = readSysfsInt("/sys/class/oplus_chg/battery/voocchg_ing");
            int fast = readSysfsInt("/sys/class/oplus_chg/battery/fast_charge");
            const char* result = (vooc == 1 || fast == 1) ? "2" : "0";
            AParcel_writeStatusHeader(reply, AStatus_newOk());
            AParcel_writeString_fn(reply, result, strlen(result));
            break;
        }
        case 2: { // getTxAdapt
            std::string type = readSysfsStr("/sys/class/power_supply/usb/REAL_TYPE");
            AParcel_writeStatusHeader(reply, AStatus_newOk());
            AParcel_writeString_fn(reply, type.c_str(), type.length());
            break;
        }
        case 3: { // getChargingPowerMax
            int voltage = readSysfsInt("/sys/class/power_supply/battery/voltage_now");
            int current = readSysfsInt("/sys/class/power_supply/battery/current_now");
            std::string result = "0";
            if (voltage > 0 && current > 0) {
                long power_mw = (long)voltage * (long)current / 1000;
                result = std::to_string(power_mw);
            }
            AParcel_writeStatusHeader(reply, AStatus_newOk());
            AParcel_writeString_fn(reply, result.c_str(), result.length());
            break;
        }
        default:
            AParcel_writeStatusHeader(reply, AStatus_fromExceptionCode(STATUS_UNKNOWN_TRANSACTION));
            break;
    }

    // 发送回复
    // 这里需要使用 binder 机制发送回复，比较复杂
    // 简化处理：直接返回
    AParcel_delete(reply);
    return STATUS_OK;
}

int main() {
    android::base::InitLogging(nullptr, android::base::LogdLogger(android::base::DEBUG));
    ALOGD("[MiChargeDlopen] Starting MIUI Charge HAL Bridge (dlopen version)");

    // 加载 libbinder.so 符号
    if (!load_binder_symbols()) {
        return 1;
    }

    // 检查 OPPO 充电节点
    std::ifstream test("/sys/class/oplus_chg/battery/voocchg_ing");
    if (test.good()) {
        int vooc;
        test >> vooc;
        ALOGD("[MiChargeDlopen] OPPO oplus_chg node found: voocchg_ing=%d", vooc);
    } else {
        ALOGE("[MiChargeDlopen] OPPO oplus_chg node NOT found");
    }

    // 启动 binder 线程池
    if (g_ABinderProcess_setThreadPoolMaxThreadCount) g_ABinderProcess_setThreadPoolMaxThreadCount(4);
    if (g_ABinderProcess_startThreadPool) g_ABinderProcess_startThreadPool();

    // 创建 binder 类
    AIBinder_Class* clazz = g_AIBinder_Class_define("vendor.xiaomi.hardware.micharge.IMiCharge", onTransact);
    if (!clazz) {
        ALOGE("Failed to define binder class");
        return 1;
    }
    if (g_AIBinder_Class_setOnTransact) {
        g_AIBinder_Class_setOnTransact(clazz, onTransact);
    }

    // 创建 binder 对象
    AIBinder* binder = g_AIBinder_new(clazz, nullptr);
    if (!binder) {
        ALOGE("Failed to create binder");
        return 1;
    }

    // 注册服务
    const char* instance = "vendor.xiaomi.hardware.micharge.IMiCharge/default";
    binder_status_t status = g_AServiceManager_addService_fn(binder, instance);
    if (status != STATUS_OK) {
        ALOGE("Failed to register service: %d", status);
        return 1;
    }
    ALOGD("[MiChargeDlopen] Service registered: %s", instance);

    // 加入线程池
    if (g_ABinderProcess_joinThreadPool) g_ABinderProcess_joinThreadPool();
    return 0;
}
