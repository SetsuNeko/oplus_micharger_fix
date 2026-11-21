package vendor.xiaomi.hardware.micharge;

interface IMiCharge {
    String getQuickChargeType();
    String getTxAdapt();
    String getChargingPowerMax();
    String getBatteryChargeType();
    String getBatteryVbat();
    String getBatteryIbat();
    String getBatteryCapacity();
    String getBatterySoh();
    String getBatteryTbat();
    boolean isFunctionSupported(String funcName);
    boolean isWirelessChargingSupported();
    String getMiChargePath(String path);
    String getBatteryCommonInfo(String info);
    String getBatteryAuthentic();
    String getBatteryChargeFull();
    String getBatteryCycleCount();
    String getBatteryResistance();
    String getBatteryThermaLevel();
}
