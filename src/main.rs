use rust_kernel::kernel::KernelDriver;
use rust_kernel::menu;

fn main() {
    let mut driver = KernelDriver::new();
    let ret = driver.cmd_ctl();
    driver.get_pid("bin.mt.plus");
    let base = driver.get_mod_base("libmt1.so");
    let mut tmp = driver.read::<i32>(base);
    tmp = driver.read::<i32>(base+4);
    tmp = driver.read::<i32>(base+8);
    tmp = driver.read::<i32>(base+12);
    print!("tmp {}\n", tmp);
    driver.write::<i32>(base+12, 1999);
    tmp = driver.read::<i32>(base+12);
    print!("tmp {}\n", tmp);
    if let Err(e) = menu::init_menu() {
        eprintln!("Error initializing menu: {}", e);
    }
}
