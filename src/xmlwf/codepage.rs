use ::core::ffi::{c_char, c_int};

pub unsafe fn codepageMap(mut _cp: c_int, mut _map: *mut c_int) -> c_int {
    return 0;
}
pub unsafe fn codepageConvert(mut _cp: c_int, mut _p: *const c_char) -> c_int {
    return -(1);
}
