#!/bin/bash
# charger_hal/rust/build.sh
# 使用 cargo-ndk 编译 Rust MIUI Charge HAL 服务
#
# 前置条件：
#   1. 安装 Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#   2. 添加 Android target: rustup target add aarch64-linux-android
#   3. 安装 cargo-ndk: cargo install cargo-ndk
#   4. 设置 ANDROID_NDK_HOME 环境变量

set -e

cd "$(dirname "$0")"

TARGET="aarch64-linux-android"
RELEASE_DIR="target/${TARGET}/release"
OUTPUT_BIN="vendor.xiaomi.hardware.micharge-service"

echo "=== 检查前置条件 ==="

# 检查 rustup
if ! command -v rustup &> /dev/null; then
    echo "错误: 未找到 rustup，请先安装 Rust"
    exit 1
fi

# 检查 Android target
if ! rustup target list --installed 2>/dev/null | grep -q "${TARGET}"; then
    echo "正在添加 Android target: ${TARGET}"
    rustup target add "${TARGET}"
fi

# 检查 NDK
if [ -z "${ANDROID_NDK_HOME}" ]; then
    echo "警告: ANDROID_NDK_HOME 未设置，尝试自动检测..."
    if [ -d "$HOME/Android/Sdk/ndk" ]; then
        export ANDROID_NDK_HOME="$HOME/Android/Sdk/ndk/$(ls $HOME/Android/Sdk/ndk | head -1)"
    elif [ -d "/opt/android-ndk" ]; then
        export ANDROID_NDK_HOME="/opt/android-ndk"
    else
        echo "错误: 未找到 Android NDK，请设置 ANDROID_NDK_HOME"
        exit 1
    fi
fi

echo "ANDROID_NDK_HOME: ${ANDROID_NDK_HOME}"

echo
echo "=== 编译 Rust 代码 ==="
cargo ndk -t "${TARGET}" -o "${RELEASE_DIR}" build --release 2>&1

if [ $? -ne 0 ]; then
    echo "错误: 编译失败"
    exit 1
fi

echo
echo "=== 检查输出 ==="
if [ -f "${RELEASE_DIR}/${OUTPUT_BIN}" ]; then
    echo "编译成功: ${RELEASE_DIR}/${OUTPUT_BIN}"
    ls -la "${RELEASE_DIR}/${OUTPUT_BIN}"
    file "${RELEASE_DIR}/${OUTPUT_BIN}"
else
    echo "错误: 未找到输出二进制文件"
    exit 1
fi

echo
echo "=== 部署到设备 ==="
read -p "是否推送到连接的设备？(y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    adb root
    adb remount
    adb push "${RELEASE_DIR}/${OUTPUT_BIN}" /vendor/bin/hw/vendor.xiaomi.hardware.micharge-service
    adb shell chmod 755 /vendor/bin/hw/vendor.xiaomi.hardware.micharge-service
    adb shell chcon u:object_r:hal_micharge_default_exec:s0 /vendor/bin/hw/vendor.xiaomi.hardware.micharge-service
    adb shell stop
    adb shell start
    echo "部署完成，设备已重启"
fi
