// exports from gphoto2-port-log.h

// this file is incomplete

use libc::{c_int, c_char, c_void};

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
#[repr(C)]
pub enum GPLogLevel {
	GP_LOG_ERROR = 0,
	GP_LOG_VERBOSE = 1,
	GP_LOG_DEBUG = 2,
	GP_LOG_DATA = 3
}

pub type GPLogFunc = extern "C" fn(GPLogLevel, *const c_char, *const c_char, *mut c_void);

extern "C" {
    pub fn gp_log_add_func(level: GPLogLevel, func: GPLogFunc, data: *mut c_void) -> c_int;
    pub fn gp_log_remove_func(id: c_int) -> c_int;
}
