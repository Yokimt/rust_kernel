use rust_kernel::kernel::KernelDriver;
fn main() {
    let mut driver = KernelDriver::new();
    let ret =driver.cmd_ctl();
    driver.get_pid("bin.mt.plus");
    println!("Hello, world! {ret}");
}
