#include <dlfcn.h>
#include <android/log.h>
#include <unistd.h>
#include <cstdlib>

int main() {
    __android_log_print(ANDROID_LOG_DEBUG, "MiCharge", "Starting...");
    
    const char* path = "/data/local/tmp/libmicharge_simple.so";
    __android_log_print(ANDROID_LOG_DEBUG, "MiCharge", "Trying to dlopen: %s", path);
    
    void* handle = dlopen("/data/local/tmp/libmicharge_simple.so", RTLD_LAZY);
    if (!handle) {
        __android_log_print(ANDROID_LOG_ERROR, "MiCharge", "dlopen failed: %s", dlerror());
        return 1;
    }
    
    __android_log_print(ANDROID_LOG_DEBUG, "MiCharge", "Library loaded successfully");
    
    // 保持运行
    while (true) {
        sleep(3600);
    }
    
    dlclose(handle);
    return 0;
}
