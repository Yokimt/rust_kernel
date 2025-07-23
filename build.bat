@echo off

set NDK_PATH=C:\Users\TokiHanako\AppData\Local\Android\Sdk\ndk\29.0.13599879
set PATH=%NDK_PATH%\toolchains\llvm\prebuilt\windows-x86_64\bin;%PATH%

call cargo build --target aarch64-linux-android --release

adb push target\aarch64-linux-android\release\rust_kernel /data/local/tmp
adb shell "chmod 777 /data/local/tmp/rust_kernel"
