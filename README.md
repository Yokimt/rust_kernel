## usage
 - 1.自己编译android_native_window库下的android_native_control_support项目，将生成的libnative-window-control.a复制到\target\cxxbuild目录下
 - 2.修改android_native_window 库的pub struct Window 结构体的 new函数，增加一个 bool值参数 用来传输是否开启防录制，然后将修改下面的这个地方。
    ```rust
    let ptr = core::ptr::NonNull::new_unchecke(safe_create_native_window(title, res, res, screenshot,));//screenshot为加入的bool变量
 - 3.配置好 cargo的ndk 环境变量，然后运行 ./build.bat 编译。