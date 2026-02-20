#[no_mangle]
#[c2rust::src_loc = "47:1"]
pub unsafe extern "C" fn codepageMap(
    mut _cp: ::core::ffi::c_int,
    mut _map: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "84:1"]
pub unsafe extern "C" fn codepageConvert(
    mut _cp: ::core::ffi::c_int,
    mut _p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return -(1 as ::core::ffi::c_int);
}
