extern "C" {
    fn _fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type XML_Char = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharData {
    pub count: ::core::ffi::c_int,
    pub data: [XML_Char; 2048],
}
unsafe extern "C" fn xmlstrlen(mut s: *const XML_Char) -> ::core::ffi::c_int {
    unsafe {
        let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !s.is_null() {
        } else {
            __assert_fail(
                b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/chardata.c\0".as_ptr() as *const ::core::ffi::c_char,
                54 as ::core::ffi::c_uint,
                b"int xmlstrlen(const XML_Char *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        while *s.offset(len as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            len += 1;
        }
        return len;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CharData_Init(mut storage: *mut CharData) {
    unsafe {
        if !storage.is_null() {
        } else {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/chardata.c\0".as_ptr() as *const ::core::ffi::c_char,
                62 as ::core::ffi::c_uint,
                b"void CharData_Init(CharData *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        (*storage).count = -(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CharData_AppendXMLChars(
    mut storage: *mut CharData,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    unsafe {
        let mut maxchars: ::core::ffi::c_int = 0;
        if !storage.is_null() {
        } else {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/chardata.c\0".as_ptr() as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_uint,
                b"void CharData_AppendXMLChars(CharData *, const XML_Char *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if !s.is_null() {
        } else {
            __assert_fail(
                b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/chardata.c\0".as_ptr() as *const ::core::ffi::c_char,
                71 as ::core::ffi::c_uint,
                b"void CharData_AppendXMLChars(CharData *, const XML_Char *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        maxchars = (::core::mem::size_of::<[XML_Char; 2048]>() as usize)
            .wrapping_div(::core::mem::size_of::<XML_Char>() as usize)
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
        if len + (*storage).count < ::core::mem::size_of::<[XML_Char; 2048]>() as ::core::ffi::c_int
        {
            memcpy(
                (&raw mut (*storage).data as *mut XML_Char).offset((*storage).count as isize)
                    as *mut ::core::ffi::c_void,
                s as *const ::core::ffi::c_void,
                (len as size_t).wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
            );
            (*storage).count += len;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn CharData_CheckXMLChars(
    mut storage: *mut CharData,
    mut expected: *const XML_Char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut len: ::core::ffi::c_int = xmlstrlen(expected);
        let mut count: ::core::ffi::c_int = 0;
        if !storage.is_null() {
        } else {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/chardata.c\0".as_ptr() as *const ::core::ffi::c_char,
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
            snprintf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                b"wrong number of data characters: got %d, expected %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                count,
                len,
            );
            _fail(
                b"/root/work/expat/tests/chardata.c\0".as_ptr() as *const ::core::ffi::c_char,
                98 as ::core::ffi::c_int,
                &raw mut buffer as *mut ::core::ffi::c_char,
            );
        }
        if memcmp(
            expected as *const ::core::ffi::c_void,
            &raw mut (*storage).data as *mut XML_Char as *const ::core::ffi::c_void,
            (len as size_t).wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t),
        ) != 0 as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/chardata.c\0".as_ptr() as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_int,
                b"got bad data bytes\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return 1 as ::core::ffi::c_int;
    }
}
