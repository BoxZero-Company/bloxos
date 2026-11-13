
use core::ffi::CStr;

unsafe extern "C" {
    fn blox_app_start() -> i32;
    fn blox_app_name() -> *const i8;
    fn blox_app_version() -> *const i8;
    fn blox_app_description() -> *const i8;
}

pub fn initialize() -> bool {
    unsafe {
        blox_app_start() == 0
    }
}

unsafe fn read_string(pointer: *const i8) -> &'static str {
    if pointer.is_null() {
        return "";
    }

    unsafe {
        CStr::from_ptr(pointer)
            .to_str()
            .unwrap_or("")
    }
}

pub fn name() -> &'static str {
    unsafe {
        read_string(blox_app_name())
    }
}

pub fn version() -> &'static str {
    unsafe {
        read_string(blox_app_version())
    }
}

pub fn description() -> &'static str {
    unsafe {
        read_string(blox_app_description())
    }
}
