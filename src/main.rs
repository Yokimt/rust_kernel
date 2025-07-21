use rust_kernel::kernel::KernelDriver;
fn main() {
    let mut driver = KernelDriver::new();
    println!("Hello, world!");
}
