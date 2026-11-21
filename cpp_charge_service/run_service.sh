#!/system/bin/sh
export LD_LIBRARY_PATH=/data/local/tmp:$LD_LIBRARY_PATH
export LD_PRELOAD=/data/local/tmp/libbinder.so:/data/local/tmp/libutils.so
exec /data/local/tmp/micharge_cpp
