#include <dlfcn.h>
#include <android/binder_interface_utils.h>
#include <android/binder_ibinder.h>
#include <android/binder_parcel.h>
#include <android/binder_status.h>
#include <android/log.h>
#include <fstream>
#include <string>
#include <cstring>

// 简化日志宏
#define LOGD(fmt, ...) __android_log_print(ANDROID_LOG_DEBUG, "MiCharge", fmt, ##__VA_ARGS__)
#define LOGE(fmt, ...) __android_log_print(ANDROID_LOG_ERROR, "MiCharge", fmt, ##__VA_ARGS__)

// 函数指针类型定义 - 使用正确的签名
using AIBinder_Class_define_fn = AIBinder_Class*(*)(const char*, void*(*)(void*), void(*)(void*), binder_status_t(*)(AIBinder*, uint32_t, AParcel*, AParcel*));
using AIBinder_Class_setOnTransact_fn = void(*)(AIBinder_Class*, binder_status_t(*)(AIBinder*, uint32_t, AParcel*, AParcel*));
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
AIBinder_Class_define_fn g_AIBinder_Class_define = nullptr;
AIBinder_Class_setOnTransact_fn g_AIBinder_Class_setOnTransact = nullptr;
AIBinder_new_fn g_AIBinder_new = nullptr;
AServiceManager_addService_fn g_AServiceManager_addService = nullptr;
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
        __android_log_print(ANDROID_LOG_ERROR, "MiCharge", "Failed to dlopen libbinder.so: %s", dlerror());
        return false;
    }

    auto load_fn = [&](void** fn, const char* name) -> bool {
        *fn = dlsym(handle, name);
        return *fn != nullptr;
    };

    bool ok = true;
    ok &= load_fn((void**)&g_AIBinder_Class_define, "AIBinder_Class_define");
    ok &= load_fn((void**)&g_AIBinder_Class_setOnTransact, "AIBinder_Class_setOnTransact");
    ok &= load_fn((void**)&g_AIBinder_new, "AIBinder_new");
    ok &= load_fn((void**)&g_AServiceManager_addService, "AServiceManager_addService");
    ok &= load_fn((void**)&g_ABinderProcess_setThreadPoolMaxThreadCount, "ABinderProcess_setThreadPoolMaxThreadCount");
    ok &= load_fn((void**)&g_ABinderProcess_startThreadPool, "ABinderProcess_startThreadPool");
    ok &= load_fn((void**)&g_ABinderProcess_joinThreadPool, "ABinderProcess_joinThreadPool");
    ok &= load_fn((void**)&g_AParcel_writeString, "AParcel_writeString");
    ok &= load_fn((void**)&g_AParcel_readInt32, "AParcel_readInt32");
    ok &= load_fn((void**)&g_AParcel_writeInt32, "AParcel_writeInt32");
    ok &= load_fn((void**)&g_AParcel_writeString, "AParcel_writeString");
    ok &= load_fn((void**)&g_AParcel_writeStatusHeader, "AParcel_writeStatusHeader");

    if (!ok) {
        __android_log_print(ANDROID_LOG_ERROR, "MiCharge", "Failed to load required symbols");
        dlclose(handle);
        return false;
    }

    __android_log_print(ANDROID_LOG_DEBUG, "MiCharge", "Successfully loaded libbinder.so symbols");
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

// 事务处理函数 - 签名必须匹配 AIBinder_Class_onTransact
binder_status_t onTransact(AIBinder* binder, uint32_t code, AParcel* in, AParcel* out) {
    int32_t code32 = static_cast<int32_t>(code);
    if (AParcel_readInt32(in, &code32) != STATUS_OK) {
        return STATUS_BAD_VALUE;
    }

    switch (code32) {
        case 1: { // getQuickChargeType
            int vooc = 0, fast = 0;
            std::ifstream("/sys/class/oplus_chg/battery/voocchg_ing") >> vooc;
            std::ifstream("/sys/class/oplus_chg/battery/fast_charge") >> fast;
            
            const char* result = (vooc == 1 || fast == 1) ? "2" : "0";
            
            AParcel_writeStatusHeader(out, AStatus_newOk());
            g_AParcel_writeString(out, result, strlen(result));
            break;
        }
        case 2: { // getTxAdapt
            std::string type;
            std::ifstream("/sys/class/power_supply/usb/REAL_TYPE") >> type;
            
            AParcel_writeStatusHeader(out, AStatus_newOk());
            g_AParcel_writeString(out, type.c_str(), type.length());
            break;
        }
        case 3: { // getChargingPowerMax
            int voltage = 0, current = 0;
            std::ifstream("/sys/class/power_supply/battery/voltage_now") >> voltage;
            std::ifstream("/sys/class/power_supply/battery/current_now") >> current;
            
            std::string result = "0";
            if (voltage > 0 && current > 0) {
                long power_mw = (long)voltage * (long)current / 1000;
                result = std::to_string(power_mw);
            }
            
            AParcel_writeStatusHeader(out, AStatus_newOk());
            g_AParcel_writeString(out, result.c_str(), result.length());
            break;
        }
        default:
            AParcel_writeStatusHeader(out, AStatus_fromExceptionCode(STATUS_UNKNOWN_TRANSACTION));
            break;
    }
    
    return STATUS_OK;
}

int main() {
    __android_log_print(ANDROID_LOG_DEBUG, "MiCharge", "Starting MIUI Charge HAL Bridge (dlopen version)");

    // 加载 libbinder.so 符号
    if (!load_binder_symbols()) {
        return 1;
    }

    // 检查 OPPO 充电节点
    int vooc = 0;
    std::ifstream("/sys/class/oplus_chg/battery/voocchg_ing") >> vooc;
    if (vooc >= 0) {
        __android_log_print(ANDROID_LOG_DEBUG, "MiCharge", "OPPO oplus_chg node found: voocchg_ing=%d", vooc);
    } else {
        __android_log_print(ANDROID_LOG_ERROR, "MiCharge", "OPPO oplus_chg node NOT found");
    }

    // 启动 binder 线程池
    if (g_ABinderProcess_setThreadPoolMaxThreadCount) g_ABinderProcess_setThreadPoolMaxThreadCount(4);
    if (g_ABinderProcess_startThreadPool) g_ABinderProcess_startThreadPool();

    // 创建 binder 类
    AIBinder_Class* clazz = g_AIBinder_Class_define("vendor.xiaomi.hardware.micharge.IMiCharge", nullptr, nullptr, onTransact);
    if (!clazz) {
        __android_log_print(ANDROID_LOG_ERROR, "MiCharge", "Failed to define binder class");
        return 1;
    }
    g_AIBinder_Class_setOnTransact(clazz, onTransact);

    // 创建 binder 对象
    AIBinder* binder = g_AIBinder_new(clazz, nullptr);
    if (!binder) {
        __android_log_print(ANDROID_LOG_ERROR, "MiCharge", "Failed to create binder");
        return 1;
    }

    // 注册服务
    const char* instance = "vendor.xiaomi.hardware.micharge.IMiCharge/default";
    binder_status_t status = g_AServiceManager_addService(binder, instance);
    if (status != STATUS_OK) {
        __android_log_print(ANDROID_LOG_ERROR, "MiCharge", "Failed to register service: %d", status);
        return 1;
    }
    __android_log_print(ANDROID_LOG_DEBUG, "MiCharge", "Service registered: %s", instance);

    // 加入线程池
    __android_log_print(ANDROID_LOG_DEBUG, "MiCharge", "Service ready, joining thread pool");
    g_ABinderProcess_joinThreadPool();
    return 0;
}
