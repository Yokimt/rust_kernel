#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod sukisu;

use std::{
    env::consts,
    os::raw::{c_char, c_int, c_long, c_void},
};
use syscalls::{syscall, Sysno};
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

        if ret < 0 {
            return ret;
        }
        self.kread.key = (ret & 0xFFFF) as u16;
        self.cmd_read = ((ret >> 16) & 0xFFFF) as u16;
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
    pub fn get_mod_base() {
        
    }
    pub fn get_pid() {}
}
