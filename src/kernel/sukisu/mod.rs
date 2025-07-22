use syscalls::syscall;
use syscalls::aarch64::Sysno;
use std::ffi::{c_uint, CString,c_int,c_long};



const KSU_OPTIONS: c_long = 0xDEADBEEF;
// KPM控制代码
const CMD_KPM_CONTROL:c_uint  =  28;

const CMD_KPM_CONTROL_MAX:c_uint   = 7;

const SUKISU_KPM_LOAD : c_uint  = 28;

const SUKISU_KPM_UNLOAD : c_uint  = 29;

const SUKISU_KPM_NUM :c_uint  = 30;

const SUKISU_KPM_LIST :c_uint  = 31;

const SUKISU_KPM_INFO :c_uint  = 32;

const SUKISU_KPM_CONTROL :c_uint  = 33;

const SUKISU_KPM_VERSION :c_uint  = 34;

pub fn load_kpm(path:&str) -> i32
{
    let c_path = CString::new(path).expect("Invalid load_path string (contains null byte)");
    let mut out:c_int = -1;
    let result =unsafe {
        syscall!(
            Sysno::prctl, // 系统调用号
            KSU_OPTIONS,  // 选项
            SUKISU_KPM_LOAD , // 子命令
            c_path.as_ptr() , // 路径指针
            0, // 第四个参数 (NULL)
            &mut out as *mut c_int // 输出参数指针
        )
    };

    match result {
        Ok(ret) => {
            // 检查操作结果
            if out > 0 {
                println!("load_kpm Success");
            }
            ret as i32
        }
        Err(err) => {
            // 系统调用失败
            eprintln!("load_kpm failed: {}", err);
            -22
        }
    }
}

pub fn unload_kpm(path:&str) -> i32
{
    let c_path = CString::new(path).expect("Invalid unload_path string (contains null byte)");
    let mut out:c_int = -1;
    let result =unsafe {
        syscall!(
            Sysno::prctl, // 系统调用号
            KSU_OPTIONS,  // 选项
            SUKISU_KPM_UNLOAD, // 子命令
            c_path.as_ptr(), // 路径指针
            0, // 第四个参数 (NULL)
            &mut out as *mut c_int // 输出参数指针
        )
    };

    match result {
        Ok(ret) => {
            // 检查操作结果
            if out > 0 {
                println!("unload_kpm Success");
            }
            ret as i32
        }
        Err(err) => {
            // 系统调用失败
            eprintln!("unload_kpm failed: {}", err);
            -23
        }
    }
}
pub fn kpm_control(name :&str, arg:&str) -> i32 {
    let c_name = CString::new(name).expect("Invalid name string (contains null byte)");
    let c_arg =  CString::new(arg).expect("Invalid arg string (contains null byte)");
    let mut out: c_int = -1;

    
    let result = unsafe {
        syscall!(
            Sysno::prctl, // 系统调用号
            KSU_OPTIONS,  // 选项
            SUKISU_KPM_CONTROL , // 控制命令
            c_name.as_ptr(), // 命令参数
            c_arg.as_ptr(), // 附加参数
            &mut out as *mut c_int // 输出参数指针
        )
    };

    match result {
        Ok(ret) => {
            if out > 0 {
                println!("kpm_control Success");
            }
            ret as i32
        }
        Err(err) => {
            eprintln!("kpm_control Success");
            out
        }
    }
}