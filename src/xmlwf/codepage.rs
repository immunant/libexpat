use core::ffi::c_int;

pub fn codepageMap(mut _cp: c_int, mut _map: *mut c_int) -> c_int {
    return 0;
}
pub fn codepageConvert(mut _cp: c_int, mut _p: *const ::core::ffi::c_char) -> c_int {
    return -(1);
}
