// =============== BEGIN chardata_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharData {
    pub count: ::core::ffi::c_int,
    pub data: [crate::expat_external_h::XML_Char; 2048],
}
pub use crate::__stddef_size_t_h::size_t;
use crate::stdlib::__assert_fail;

pub use crate::expat_external_h::XML_Char;
use crate::src::tests::minicheck::_fail;
use crate::stdlib::memcmp;
use crate::stdlib::memcpy;
use crate::stdlib::snprintf;

unsafe extern "C" fn xmlstrlen(
    mut s: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !s.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/chardata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            54 as ::core::ffi::c_uint,
            b"int xmlstrlen(const XML_Char *)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    while *s.offset(len as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        len += 1;
    }
    return len;
}
#[no_mangle]

pub unsafe extern "C" fn CharData_Init(mut storage: *mut crate::src::tests::chardata::CharData) {
    if !storage.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/chardata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            62 as ::core::ffi::c_uint,
            b"void CharData_Init(CharData *)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    (*storage).count = -1 as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn CharData_AppendXMLChars(
    mut storage: *mut crate::src::tests::chardata::CharData,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut maxchars: ::core::ffi::c_int = 0;
    if !storage.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/chardata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            70 as ::core::ffi::c_uint,
            b"void CharData_AppendXMLChars(CharData *, const XML_Char *, int)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if !s.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/chardata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            71 as ::core::ffi::c_uint,
            b"void CharData_AppendXMLChars(CharData *, const XML_Char *, int)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    maxchars = (::core::mem::size_of::<[crate::expat_external_h::XML_Char; 2048]>() as usize)
        .wrapping_div(::core::mem::size_of::<crate::expat_external_h::XML_Char>() as usize)
        as ::core::ffi::c_int;
    if (*storage).count < 0 as ::core::ffi::c_int {
        (*storage).count = 0 as ::core::ffi::c_int;
    }
    if len < 0 as ::core::ffi::c_int {
        len = xmlstrlen(s);
    }
    if len + (*storage).count > maxchars {
        len = maxchars - (*storage).count;
    }
    if len + (*storage).count
        < ::core::mem::size_of::<[crate::expat_external_h::XML_Char; 2048]>() as ::core::ffi::c_int
    {
        crate::stdlib::memcpy(
            (&raw mut (*storage).data as *mut crate::expat_external_h::XML_Char)
                .offset((*storage).count as isize) as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            (len as crate::__stddef_size_t_h::size_t)
                .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                    as crate::__stddef_size_t_h::size_t),
        );
        (*storage).count += len;
    }
}
#[no_mangle]

pub unsafe extern "C" fn CharData_CheckXMLChars(
    mut storage: *mut crate::src::tests::chardata::CharData,
    mut expected: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = xmlstrlen(expected);
    let mut count: ::core::ffi::c_int = 0;
    if !storage.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/chardata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            91 as ::core::ffi::c_uint,
            b"int CharData_CheckXMLChars(CharData *, const XML_Char *)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    count = if (*storage).count < 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        (*storage).count
    };
    if len != count {
        let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
        crate::stdlib::snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                as crate::__stddef_size_t_h::size_t,
            b"wrong number of data characters: got %d, expected %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            count,
            len,
        );
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/chardata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            98 as ::core::ffi::c_int,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
    if crate::stdlib::memcmp(
        expected as *const ::core::ffi::c_void,
        &raw mut (*storage).data as *mut crate::expat_external_h::XML_Char
            as *const ::core::ffi::c_void,
        (len as crate::__stddef_size_t_h::size_t)
            .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                as crate::__stddef_size_t_h::size_t),
    ) != 0 as ::core::ffi::c_int
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/chardata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            102 as ::core::ffi::c_int,
            b"got bad data bytes\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return 1 as ::core::ffi::c_int;
}
