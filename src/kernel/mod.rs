#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod sukisu;
use std::ffi::{c_char, c_int, c_long, c_uint, c_ulonglong, c_void, CString};
use syscalls::syscall;
use syscalls::aarch64::Sysno;
#[repr(C)]
pub struct kpm_read {
    pub key: u16,
    pub pid: c_int,
    pub size: c_int,
    pub addr: u64,
    pub buffer: *mut c_void,
}
#[repr(C)]
pub struct kpm_mod {
    pub key: u16,
    pub pid: c_int,
    pub SoName: [c_char; 256],
    pub base: u64,
    pub pkg_name: [c_char; 256],
}

pub struct KernelDriver {
    kread: kpm_read,
    kmod: kpm_mod,
    cmd_read: u16,
    cmd_write: u16,
    cmd_mod: u16,
    cmd_pid: u16,
}
impl KernelDriver {
    pub fn new() -> Self {
        KernelDriver {
            kread: kpm_read {
                key: 0,
                pid: 0,
                size: 0,
                addr: 0,
                buffer: std::ptr::null_mut(),
            },
            kmod: kpm_mod {
                key: 0,
                pid: 0,
                SoName: [0; 256],
                base: 0,
                pkg_name: [0; 256],
            },
            cmd_read: 0,
            cmd_write: 0,
            cmd_mod: 0,
            cmd_pid: 0,
        }
    }
    // 初始化命令控制
    pub fn cmd_ctl(&mut self) -> i32 {
        let kpm_name = "kernel-mem";
        let ret = sukisu::kpm_control(kpm_name, "get_key");
        match ret {
            -1 =>{
                print!("初始化失败\n");
                return -1;
            }
            -2 =>{
                print!("设备到期\n");
                return -2;
            }
            -3 =>{
                print!("未注册设备\n");
                return -3;
            }
            _ =>{
                print!("初始化成功\n");
            }
        }
        self.kread.key = (ret & 0xFFFF) as u16;
        self.cmd_read = ((ret >> 16) & 0xFFFF) as u16;
        println!("key:{},cmd:{}\n",self.kread.key,self.cmd_read);
        self.init(self.cmd_read, self.kread.key);
        0
    }
    pub fn init(&mut self, cmd: u16, key: u16) {
        self.cmd_read = cmd;
        self.cmd_write = cmd + 1;
        self.cmd_mod = cmd + 2;
        self.cmd_pid = cmd + 3;
        self.kread.key = key;
        self.kmod.key = key;
    }
    pub fn read_mem(&mut self, addr: u64, buffer: *mut c_void, size: c_int) -> i32 {
        self.kread.addr = addr;
        self.kread.buffer = buffer;
        self.kread.size = size;
        let result = unsafe {
            syscall!(
                Sysno::ioctl,
                -114 as c_int,
                self.cmd_read as c_int,
                &self.kread as *const kpm_read
            )
        };
        match result {
            Ok(ret) => {
                if ret > 0 {
                    println!("read Success");
                }
                ret as i32
            }
            Err(err) => {
                eprintln!("read failed: {}", err);
                -25
            }
        }
    }
    pub fn read<T>(&mut self, addr: u64) -> T {
        self.kread.addr = addr;
        self.kread.buffer = vec![0u8; std::mem::size_of::<T>()].as_mut_ptr() as *mut c_void;
        self.kread.size = std::mem::size_of::<T>() as c_int;
        self.read_mem(self.kread.addr, self.kread.buffer, self.kread.size);
        unsafe { std::ptr::read(self.kread.buffer as *const T) }
    }
    pub fn write_mem(&mut self, addr: u64, buffer: *mut c_void, size: c_int) {
        self.kread.addr = addr;
        self.kread.buffer = buffer;
        self.kread.size = size;
        let result = unsafe {
            syscall!(
                Sysno::ioctl,
                -114 as c_int,
                self.cmd_write as c_int,
                &self.kread as *const kpm_read
            )
        };
        match result {
            Ok(ret) => {
                if ret > 0 {
                    println!("write_mem Success");
                }
            }
            Err(err) => {
                eprintln!("write_mem failed: {}", err);
            }
        }
    }
    pub fn write<T>(&mut self,addr:u64,value:T) {
        self.kread.addr = addr;
        self.kread.buffer = &value as *const T as *mut c_void;
        self.kread.size = std::mem::size_of::<T>() as c_int;
        self.write_mem(self.kread.addr, self.kread.buffer, self.kread.size);
    }
    pub fn get_mod_base(&mut self,so_name:&str) ->u64{
        let c_so_name = CString::new(so_name).expect("Invalid so_name string (contains null byte)");
        let bytes = c_so_name.as_bytes_with_nul();
        let len = bytes.len().min(255);
        self.kmod.SoName[0..len].iter_mut().zip(bytes.iter())
        .for_each(|(dest,&src)|*dest = src as c_char);
        let result = unsafe{
            syscall!(
                Sysno::ioctl,
                -114 as c_int,
                self.cmd_mod as c_int,
                &self.kmod as *const kpm_mod
            )
        };
        match result {
            Ok(ret) => {
                if ret > 0 {
                    println!("get_module_base Success {:#x}",self.kmod.base);
                }
                self.kmod.base
            }
            Err(err) => {
                // 系统调用失败
                eprintln!("load_kpm failed: {}", err);
                0
            }
        }
    }
    pub fn set_pid (&mut self,pid:c_int) {
        print!("set_pid:{}\n",pid);
        self.kmod.pid = pid;
        self.kread.pid = pid;
    }
    pub fn get_pid(&mut self,pkg:&str)->i32 {
        let c_pkg = CString::new(pkg).expect("Invalid pkg string (contains null byte)");
        let bytes = c_pkg.as_bytes_with_nul();
        let len = bytes.len().min(255);
        self.kmod.pkg_name[..len].iter_mut().zip(bytes.iter())
            .for_each(|(dest, &src)| *dest = src as c_char);
        let result = unsafe {
            syscall!(
                Sysno::ioctl,
                -114 as c_int,
                self.cmd_pid as c_int,
                &self.kmod as *const kpm_mod
            )
        };
        match result {
            Ok(ret) => {
                if ret > 0 {
                    println!("get_pid Success");
                    self.set_pid(self.kmod.pid);
                }
                ret as i32
            }
            Err(err) => {
                eprintln!("get_pid Success");
                -26
            }
        }
    }
}
