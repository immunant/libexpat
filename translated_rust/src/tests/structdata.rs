extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn _fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type XML_Char = ::core::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructDataEntry {
    pub str: *const XML_Char,
    pub data0: ::core::ffi::c_int,
    pub data1: ::core::ffi::c_int,
    pub data2: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructData {
    pub count: ::core::ffi::c_int,
    pub max_count: ::core::ffi::c_int,
    pub entries: *mut StructDataEntry,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const STRUCT_EXTENSION_COUNT: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn xmlstrdup(mut s: *const XML_Char) -> *mut XML_Char {
    unsafe {
        let mut byte_count: size_t = strlen(s as *const ::core::ffi::c_char)
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<XML_Char>() as size_t);
        let dup: *mut XML_Char = malloc(byte_count) as *mut XML_Char;
        if !dup.is_null() {
        } else {
            __assert_fail(
                b"dup != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/structdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                66 as ::core::ffi::c_uint,
                b"XML_Char *xmlstrdup(const XML_Char *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        memcpy(
            dup as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            byte_count,
        );
        return dup;
    }
}
#[no_mangle]
pub unsafe extern "C" fn StructData_Init(mut storage: *mut StructData) {
    unsafe {
        if !storage.is_null() {
        } else {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/structdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                73 as ::core::ffi::c_uint,
                b"void StructData_Init(StructData *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        (*storage).count = 0 as ::core::ffi::c_int;
        (*storage).max_count = 0 as ::core::ffi::c_int;
        (*storage).entries = ::core::ptr::null_mut::<StructDataEntry>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn StructData_AddItem(
    mut storage: *mut StructData,
    mut s: *const XML_Char,
    mut data0: ::core::ffi::c_int,
    mut data1: ::core::ffi::c_int,
    mut data2: ::core::ffi::c_int,
) {
    unsafe {
        let mut entry: *mut StructDataEntry = ::core::ptr::null_mut::<StructDataEntry>();
        if !storage.is_null() {
        } else {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/structdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_uint,
                b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if !s.is_null() {
        } else {
            __assert_fail(
                b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/structdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_uint,
                b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if (*storage).count == (*storage).max_count {
            let mut new_entries: *mut StructDataEntry = ::core::ptr::null_mut::<StructDataEntry>();
            (*storage).max_count += STRUCT_EXTENSION_COUNT;
            new_entries = realloc(
                (*storage).entries as *mut ::core::ffi::c_void,
                ((*storage).max_count as size_t)
                    .wrapping_mul(::core::mem::size_of::<StructDataEntry>() as size_t),
            ) as *mut StructDataEntry;
            if !new_entries.is_null() {
            } else {
                __assert_fail(
                    b"new_entries != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/root/work/expat/tests/structdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                    92 as ::core::ffi::c_uint,
                    b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            (*storage).entries = new_entries;
        }
        entry = (*storage).entries.offset((*storage).count as isize) as *mut StructDataEntry;
        (*entry).str = xmlstrdup(s);
        (*entry).data0 = data0;
        (*entry).data1 = data1;
        (*entry).data2 = data2;
        (*storage).count += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn StructData_CheckItems(
    mut storage: *mut StructData,
    mut expected: *const StructDataEntry,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
        if !storage.is_null() {
        } else {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/structdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                112 as ::core::ffi::c_uint,
                b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if !expected.is_null() {
        } else {
            __assert_fail(
                b"expected != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/structdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                113 as ::core::ffi::c_uint,
                b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        if count != (*storage).count {
            snprintf(
                &raw mut buffer as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                b"wrong number of entries: got %d, expected %d\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*storage).count,
                count,
            );
            StructData_Dispose(storage);
            _fail(
                b"/root/work/expat/tests/structdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                119 as ::core::ffi::c_int,
                &raw mut buffer as *mut ::core::ffi::c_char,
            );
        } else {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < count {
                let mut got: *const StructDataEntry =
                    (*storage).entries.offset(i as isize) as *mut StructDataEntry;
                let mut want: *const StructDataEntry =
                    expected.offset(i as isize) as *const StructDataEntry;
                if !got.is_null() {
                } else {
                    __assert_fail(
                        b"got != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/root/work/expat/tests/structdata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        125 as ::core::ffi::c_uint,
                        b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if !want.is_null() {
                } else {
                    __assert_fail(
                        b"want != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/root/work/expat/tests/structdata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        126 as ::core::ffi::c_uint,
                        b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                if strcmp(
                    (*got).str as *const ::core::ffi::c_char,
                    (*want).str as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    StructData_Dispose(storage);
                    _fail(
                        b"/root/work/expat/tests/structdata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        130 as ::core::ffi::c_int,
                        b"structure got bad string\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                } else if (*got).data0 != (*want).data0
                    || (*got).data1 != (*want).data1
                    || (*got).data2 != (*want).data2
                {
                    snprintf(
                        &raw mut buffer as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                        b"struct '%s' expected (%d,%d,%d), got (%d,%d,%d)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*got).str,
                        (*want).data0,
                        (*want).data1,
                        (*want).data2,
                        (*got).data0,
                        (*got).data1,
                        (*got).data2,
                    );
                    StructData_Dispose(storage);
                    _fail(
                        b"/root/work/expat/tests/structdata.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        140 as ::core::ffi::c_int,
                        &raw mut buffer as *mut ::core::ffi::c_char,
                    );
                }
                i += 1;
            }
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn StructData_Dispose(mut storage: *mut StructData) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        if !storage.is_null() {
        } else {
            __assert_fail(
                b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/structdata.c\0".as_ptr() as *const ::core::ffi::c_char,
                151 as ::core::ffi::c_uint,
                b"void StructData_Dispose(StructData *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        };
        i = 0 as ::core::ffi::c_int;
        while i < (*storage).count {
            free((*(*storage).entries.offset(i as isize)).str as *mut ::core::ffi::c_void);
            i += 1;
        }
        free((*storage).entries as *mut ::core::ffi::c_void);
        (*storage).count = 0 as ::core::ffi::c_int;
        (*storage).entries = ::core::ptr::null_mut::<StructDataEntry>();
    }
}
