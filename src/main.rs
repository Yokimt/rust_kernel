use rust_kernel::kernel::KernelDriver;
fn main() {
    let mut driver = KernelDriver::new();
    let ret =driver.cmd_ctl();
    println!("Hello, world! {ret}");
}
