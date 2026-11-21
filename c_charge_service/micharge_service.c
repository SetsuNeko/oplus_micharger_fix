/*
 * MiCharge HAL Service v3 - 事件驱动快充状态守护进程（纯 C，无 shell）
 *
 * 仿原生小米 vendor 充电服务协议：
 *  1. miui.intent.action.ACTION_QUICK_CHARGE_TYPE（real_type 协议变化时）
 *     - VOOC/SVOOC (私有)  -> type=2 超级快充
 *     - PD/QC     (公有)  -> type=1 快充
 *     - 其他/拔线           -> type=0
 *     extras: quick_charge_type / POWER_MAX(实时W) / CAR_CHARGE=0
 *  2. miui.intent.action.ACTION_SOC_DECIMAL（快充中，插上立即第一帧，之后每10s）
 *     extras: soc_decimal / soc_decimal_rate —— 驱动锁屏两位小数电量
 *
 * 反编译依据：MiuiChargeManager.java / MiuiChargeController.java (HyperOS 3.3)
 */

#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <time.h>
#include <android/log.h>

#define LOG_TAG "MiChargerService"
#define LOGI(...) __android_log_print(ANDROID_LOG_INFO,  LOG_TAG, __VA_ARGS__)
#define LOGE(...) __android_log_print(ANDROID_LOG_ERROR, LOG_TAG, __VA_ARGS__)

#define POLL_INTERVAL_MS   500  /* 常规轮询 */
#define FAST_POLL_MS       150  /* 插上充电后加速轮询：尽快捕捉协议握手 */
#define FAST_POLL_WINDOW_S 6    /* 加速窗口时长（覆盖 VOOC/SVOOC 握手） */
#define SOC_EVERY_MS     10000  /* SOC 小数广播周期 = 原厂动画 10s 节奏 */

/* OPPO real_type 电池参数估算：8360mAh * 3.85V = 32.2Wh = 115920 J
 * rate(x100, per 10s) = power_W * 10 * 100 / 115920 = power_W * 0.0862 * 100 */
#define WATT_TO_RATE(w) ((int)((w) * 8.62f))
#define RATE_MIN 10
#define RATE_MAX 500

/* ---------- sysfs helpers ---------- */
static int sysfs_int(const char *path, int def) {
    FILE *f = fopen(path, "r");
    if (!f) return def;
    int v = def;
    if (fscanf(f, "%d", &v) != 1) v = def;
    fclose(f);
    return v;
}

static void sysfs_str(const char *path, char *buf, size_t len) {
    FILE *f = fopen(path, "r");
    if (!f) { if (len) buf[0] = 0; return; }
    if (!fgets(buf, (int)len, f)) buf[0] = 0;
    fclose(f);
    char *nl = strchr(buf, '\n');
    if (nl) *nl = 0;
}

static long now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000L + ts.tv_nsec / 1000000L;
}

/* ---------- 充电状态 ---------- */
typedef struct {
    int  charging;      /* AC/USB online */
    int  qct;           /* quick_charge_type: 0/1/2 */
    int  level;         /* 电量 0-100 */
    int  watts;         /* 实时功率 W */
    char real_type[32]; /* USB_DCP / PD / SVOOC / ... */
} charge_state_t;

static void read_state(charge_state_t *s) {
    int usb_online = sysfs_int("/sys/class/power_supply/usb/online", 0);
    int ac_online  = sysfs_int("/sys/class/power_supply/ac/online", 0);
    s->charging = (usb_online == 1 || ac_online == 1);

    sysfs_str("/sys/class/power_supply/usb/real_type", s->real_type, sizeof(s->real_type));
    if (s->real_type[0] == 0)
        sysfs_str("/sys/class/oplus_chg/usb/real_type", s->real_type, sizeof(s->real_type));

    /* 快充类型判定 */
    s->qct = 0;
    if (s->charging) {
        int vooc = sysfs_int("/sys/class/oplus_chg/battery/voocchg_ing", 0);
        int fast = sysfs_int("/sys/class/oplus_chg/battery/fast_charge", 0);
        int fchg = sysfs_int("/sys/class/oplus_chg/usb/fast_chg_type", 0);
        if (vooc == 1 || fast == 1 ||
            strstr(s->real_type, "VOOC") != NULL ||     /* SVOOC/SUPER_VOOC 均含 VOOC */
            fchg == 1) {
            s->qct = 2;                                  /* 私有协议 → 超级快充 */
        } else if (strstr(s->real_type, "PD") != NULL ||
                   strstr(s->real_type, "QC") != NULL ||
                   fchg == 2) {
            s->qct = 1;                                  /* 公有快充 → 快充 */
        }
    }

    s->level = sysfs_int("/sys/class/power_supply/battery/capacity", 0);
    int uv = sysfs_int("/sys/class/power_supply/battery/voltage_now", 0);
    int ua = sysfs_int("/sys/class/power_supply/battery/current_now", 0);
    if (ua < 0) ua = -ua;
    s->watts = (uv > 0 && ua > 0) ? (int)((long)uv * ua / 1000000000L) : 0;
}

/* ---------- 广播发送（直接 execve app_process64，不经 shell） ---------- */
#include <sys/wait.h>
#include <signal.h>

static void run_am(const char *action, const char *k1, const char *v1,
                   const char *k2, const char *v2,
                   const char *k3, const char *v3) {
    char *argv[20];
    int i = 0;
    argv[i++] = (char *)"app_process64";
    argv[i++] = (char *)"/system/bin";
    argv[i++] = (char *)"com.android.commands.am.Am";
    argv[i++] = (char *)"broadcast";
    argv[i++] = (char *)"-a";
    argv[i++] = (char *)action;
    if (k1) { argv[i++] = (char *)"--ei"; argv[i++] = (char *)k1; argv[i++] = (char *)v1; }
    if (k2) { argv[i++] = (char *)"--ei"; argv[i++] = (char *)k2; argv[i++] = (char *)v2; }
    if (k3) { argv[i++] = (char *)"--ei"; argv[i++] = (char *)k3; argv[i++] = (char *)v3; }
    argv[i++] = (char *)"--receiver-foreground";
    argv[i] = NULL;

    pid_t pid = fork();
    if (pid < 0) return;
    if (pid == 0) {
        /* 子进程：stdout/stderr 记录到日志文件，execve ELF（无 shell） */
        int out = open("/data/local/tmp/am_out.log", O_WRONLY | O_CREAT | O_APPEND, 0666);
        if (out >= 0) {
            dup2(out, 1);
            dup2(out, 2);
        }
        setenv("CLASSPATH", "/system/framework/am.jar", 1);
        execv("/system/bin/app_process64", argv);
        _exit(127);
    }
    int status = 0;
    waitpid(pid, &status, 0);
    if (WIFEXITED(status) && WEXITSTATUS(status) == 127) {
        LOGE("am exec failed (exit 127)");
    }
}

static void send_quick_charge_type(int type, int watts) {
    static char t[16], w[16], a1[16];
    snprintf(t, sizeof(t), "%d", type);
    snprintf(w, sizeof(w), "%d", watts);
    snprintf(a1, sizeof(a1), "%d", 0);
    run_am("miui.intent.action.ACTION_QUICK_CHARGE_TYPE",
           "miui.intent.extra.quick_charge_type", t,
           "miui.intent.extra.POWER_MAX", w,
           "miui.intent.extra.CAR_CHARGE", a1);
    LOGI("QUICK_CHARGE_TYPE: type=%d power=%dW", type, watts);
}

static void send_soc_decimal(int level, int decimal, int rate) {
    static char d[16], r[16];
    snprintf(d, sizeof(d), "%d", decimal);
    snprintf(r, sizeof(r), "%d", rate);
    run_am("miui.intent.action.ACTION_SOC_DECIMAL",
           "miui.intent.extra.soc_decimal", d,
           "miui.intent.extra.soc_decimal_rate", r,
           NULL, NULL);
    LOGI("SOC_DECIMAL: level=%d decimal=%d rate=%d", level, decimal, rate);
}

/* ---------- main ---------- */
int main(void) {
    LOGI("MiChargeHAL v3 (event-driven) starting");

    charge_state_t st, last;
    memset(&st, 0, sizeof(st));
    memset(&last, 0, sizeof(last));
    int last_sent_type = -1;

    /* 电量小数估算 */
    double soc_f = 0.0;        /* 估算精确电量 */
    long soc_timer = 0;        /* SOC 广播计时 */
    long fast_poll_until = 0;  /* 插上充电后的加速轮询截止时刻 */

    /* 三段序列状态机（仿原厂）：
     * 0=未插, 1=已插(乐观显示快速充电), 2=SVOOC确认(MiTurbo), 3=普通确认 */
    int plug_stage = 0;
    long plug_time = 0;
    int peak_watts = 0;        /* 近期最大功率（握手瞬间电流未爬升时用） */

    read_state(&st);
    last = st;
    soc_f = st.level;

    /* 启动时若在充电，直接按检测结果同步 */
    if (st.charging) {
        plug_stage = st.qct > 0 ? 2 : 1;
        plug_time = now_ms();
        send_quick_charge_type(st.qct > 0 ? st.qct : 1, st.watts);
        last_sent_type = st.qct > 0 ? st.qct : 1;
        fast_poll_until = now_ms() + FAST_POLL_WINDOW_S * 1000L;
    } else {
        last_sent_type = 0;
    }

    while (1) {
        long now = now_ms();
        long interval = (st.charging && now < fast_poll_until) ? FAST_POLL_MS : POLL_INTERVAL_MS;
        usleep(interval * 1000);
        read_state(&st);
        now = now_ms();

        if (st.watts > peak_watts) peak_watts = st.watts;

        /* === 1. 插拔状态变化 === */
        if (st.charging != last.charging) {
            if (st.charging) {
                /* 插上：立即发 type=1 乐观显示"快速充电"（原厂三段序列第一段） */
                plug_stage = 1;
                plug_time = now;
                peak_watts = st.watts;
                send_quick_charge_type(1, st.watts);
                last_sent_type = 1;
                soc_f = st.level;
                soc_timer = now_ms();
                fast_poll_until = now_ms() + FAST_POLL_WINDOW_S * 1000L;
            } else {
                /* 拔线：不发广播，保留残留（下次插上 SystemUI 直接按该类型显示） */
                plug_stage = 0;
                LOGI("unplugged, keep last type=%d as residual", last_sent_type);
            }
        }

        /* === 2. 插上后的序列推进 === */
        if (st.charging && plug_stage == 1) {
            if (st.qct == 2) {
                /* SVOOC 握手确认 → 升级 MiTurbo（第二段） */
                plug_stage = 2;
                int pw = peak_watts > st.watts ? peak_watts : st.watts;
                send_quick_charge_type(2, pw);
                last_sent_type = 2;
            } else if (st.qct == 1) {
                /* PD 公有快充确认 → 停留在快速充电 */
                plug_stage = 2;
                int pw = peak_watts > st.watts ? peak_watts : st.watts;
                send_quick_charge_type(1, pw);
                last_sent_type = 1;
            } else if (now - plug_time > 3000) {
                /* 3s 无任何快充协议 → 降级普通充电（真 DCP） */
                plug_stage = 3;
                send_quick_charge_type(0, st.watts);
                last_sent_type = 0;
            }
        } else if (st.charging && plug_stage >= 2 && st.qct != last_sent_type && st.qct > 0) {
            /* 已确认后协议变化（如 PD → 换成 SVOOC 不拔线，罕见） */
            send_quick_charge_type(st.qct, st.watts);
            last_sent_type = st.qct;
            plug_stage = st.qct == 2 ? 2 : 2;
        }

        /* === 2. 快充中：SOC_DECIMAL 节奏 === */
        if (st.charging && st.qct > 0 && st.level < 100) {
            long now = now_ms();
            /* 每轮按功率累积估算电量 */
            double per_ms = (double)st.watts * 8.62 / 100.0 / (double)SOC_EVERY_MS;
            soc_f += per_ms * POLL_INTERVAL_MS;
            if (soc_f < st.level) soc_f = st.level;   /* 对齐系统电量 */

            if (soc_timer == 0) {
                soc_timer = now;
                /* 立即发第一帧 */
                int rate = WATT_TO_RATE(peak_watts > 0 ? peak_watts : st.watts);
                if (rate < RATE_MIN) rate = RATE_MIN;
                if (rate > RATE_MAX) rate = RATE_MAX;
                send_soc_decimal(st.level, 0, rate);
            } else if (now - soc_timer >= SOC_EVERY_MS) {
                int rate = WATT_TO_RATE(peak_watts > st.watts ? peak_watts : st.watts);
                if (rate < RATE_MIN) rate = RATE_MIN;
                if (rate > RATE_MAX) rate = RATE_MAX;
                int decimal = (int)((soc_f - st.level) * 100.0);
                if (decimal < 0) decimal = 0;
                if (decimal > 99) decimal = 99;
                send_soc_decimal(st.level, decimal, rate);
                soc_timer = now;
            }
        } else {
            soc_timer = 0;
        }

        last = st;
    }
    return 0;
}
