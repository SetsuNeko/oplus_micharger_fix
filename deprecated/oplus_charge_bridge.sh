#!/system/bin/sh
# oplus_charge_bridge.sh - OPPO 充电节点 → MIUI 快充广播桥接
# 读取 /sys/class/oplus_chg/battery/voocchg_ing 和 fast_charge
# 向 MIUI 广播 ACTION_QUICK_CHARGE_TYPE，触发快充动画显示

LOG_TAG="oplus_charge_bridge"
LOG=/data/local/tmp/oplus_charge_bridge.log

log() {
    echo "$(date '+%H:%M:%S') $*" >> "$LOG" 2>/dev/null
}

: > "$LOG" 2>/dev/null
log "=== oplus_charge_bridge start ==="

while true; do
    # 读取 OPPO 快充节点
    vooc=$(cat /sys/class/oplus_chg/battery/voocchg_ing 2>/dev/null)
    fast=$(cat /sys/class/oplus_chg/battery/fast_charge 2>/dev/null)
    
    if [ "$vooc" = "1" ] || [ "$fast" = "1" ]; then
        # 2 = SuperRapidCharge，MIUI 会显示快充动画
        am broadcast -a miui.intent.action.ACTION_QUICK_CHARGE_TYPE \
            --ei miui.intent.extra.quick_charge_type 2 \
            --ei miui.intent.extra.POWER_MAX 0 \
            --ei miui.intent.extra.CAR_CHARGE 0 2>/dev/null
        log "broadcast quick_charge_type=2 (vooc=$vooc fast=$fast)"
    fi
    
    sleep 5
done
