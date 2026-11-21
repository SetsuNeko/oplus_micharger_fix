package vendor.xiaomi.hardware.micharge;

interface IMiCharge {
    string getQuickChargeType();
    string getTxAdapt();
    string getChargingPowerMax();
    string getBatteryChargeType();
    string getBatteryVbat();
    string getBatteryIbat();
    string getBatteryCapacity();
    string getBatterySoh();
    string getBatteryTbat();
    boolean isFunctionSupported(in string funcName, out boolean result);
    boolean isWirelessChargingSupported(out boolean result);
    string getMiChargePath(in string path, out string result);
    string getBatteryCommonInfo(in string info, out string result);
    string getBatteryAuthentic();
    string getBatteryChargeFull();
    string getBatteryCycleCount();
    string getBatteryResistance();
    string getBatteryThermaLevel();
};
