#[no_mangle]

pub unsafe extern "C" fn codepageMap(
    mut _cp: ::core::ffi::c_int,
    mut _map: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn codepageConvert(
    mut _cp: ::core::ffi::c_int,
    mut _p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return -(1i32);
}
