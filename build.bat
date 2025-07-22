@echo off
call cargo build --target aarch64-linux-android --release

adb push target\aarch64-linux-android\release\rust_kernel /data/local/tmp
adb shell "chmod 777 /data/local/tmp/rust_kernel"
