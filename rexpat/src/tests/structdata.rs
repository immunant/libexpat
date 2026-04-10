// =============== BEGIN structdata_h ================
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
    pub entries: *mut crate::src::tests::structdata::StructDataEntry,
}
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::expat_external_h::XML_Char;
use crate::src::tests::minicheck::_fail;
use crate::stdlib::__assert_fail;
use crate::stdlib::snprintf;

use crate::stdlib::free;
use crate::stdlib::malloc;
use crate::stdlib::memcpy;
use crate::stdlib::realloc;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;

pub const STRUCT_EXTENSION_COUNT: ::core::ffi::c_int = 8i32;

unsafe extern "C" fn xmlstrdup(
    mut s: *const XML_Char,
) -> *mut XML_Char {
    let mut byte_count: size_t =
        strlen(s)
            .wrapping_add(1usize)
            .wrapping_mul(::core::mem::size_of::<XML_Char>());
    let dup: *mut XML_Char =
        malloc(byte_count) as *mut XML_Char;
    if !dup.is_null() {
    } else {
        __assert_fail(
            b"dup != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            66u32,
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
#[no_mangle]

pub unsafe extern "C" fn StructData_Init(
    mut storage: *mut crate::src::tests::structdata::StructData,
) {
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            73u32,
            b"void StructData_Init(StructData *)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    (*storage).count = 0i32;
    (*storage).max_count = 0i32;
    (*storage).entries = ::core::ptr::null_mut::<crate::src::tests::structdata::StructDataEntry>();
}
#[no_mangle]

pub unsafe extern "C" fn StructData_AddItem(
    mut storage: *mut crate::src::tests::structdata::StructData,
    mut s: *const XML_Char,
    mut data0: ::core::ffi::c_int,
    mut data1: ::core::ffi::c_int,
    mut data2: ::core::ffi::c_int,
) {
    let mut entry: *mut crate::src::tests::structdata::StructDataEntry =
        ::core::ptr::null_mut::<crate::src::tests::structdata::StructDataEntry>();
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            84u32,
            b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if !s.is_null() {
    } else {
        __assert_fail(
            b"s != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            85u32,
            b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if (*storage).count == (*storage).max_count {
        let mut new_entries: *mut crate::src::tests::structdata::StructDataEntry =
            ::core::ptr::null_mut::<crate::src::tests::structdata::StructDataEntry>();
        (*storage).max_count += STRUCT_EXTENSION_COUNT;
        new_entries = realloc(
            (*storage).entries as *mut ::core::ffi::c_void,
            ((*storage).max_count as size_t).wrapping_mul(
                
                ::core::mem::size_of::<crate::src::tests::structdata::StructDataEntry>(),
            ),
        ) as *mut crate::src::tests::structdata::StructDataEntry;
        if !new_entries.is_null() {
        } else {
            __assert_fail(
                b"new_entries != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                92u32,
                b"void StructData_AddItem(StructData *, const XML_Char *, int, int, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*storage).entries = new_entries;
    }
    entry =  (*storage).entries.offset((*storage).count as isize);
    (*entry).str = xmlstrdup(s);
    (*entry).data0 = data0;
    (*entry).data1 = data1;
    (*entry).data2 = data2;
    (*storage).count += 1;
}
#[no_mangle]

pub unsafe extern "C" fn StructData_CheckItems(
    mut storage: *mut crate::src::tests::structdata::StructData,
    mut expected: *const crate::src::tests::structdata::StructDataEntry,
    mut count: ::core::ffi::c_int,
) {
    let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            112u32,
            b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if !expected.is_null() {
    } else {
        __assert_fail(
            b"expected != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            113u32,
            b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if count != (*storage).count {
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            b"wrong number of entries: got %d, expected %d\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*storage).count,
            count,
        );
        StructData_Dispose(storage);
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            119i32,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0i32;
        while i < count {
            let mut got: *const crate::src::tests::structdata::StructDataEntry =
                
                (*storage).entries.offset(i as isize);
            let mut want: *const crate::src::tests::structdata::StructDataEntry =  expected
                .offset(i as isize);
            if !got.is_null() {
            } else {
                __assert_fail(
                    b"got != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    125u32,
                    b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if !want.is_null() {
            } else {
                __assert_fail(
                    b"want != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    126u32,
                    b"void StructData_CheckItems(StructData *, const StructDataEntry *, int)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            if strcmp(
                
                (*got).str,
                
                (*want).str,
            ) != 0i32
            {
                StructData_Dispose(storage);
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    130i32,
                    b"structure got bad string\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else if (*got).data0 != (*want).data0
                || (*got).data1 != (*want).data1
                || (*got).data2 != (*want).data2
            {
                snprintf(
                    &raw mut buffer as *mut ::core::ffi::c_char,
                    
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
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
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    140i32,
                    &raw mut buffer as *mut ::core::ffi::c_char,
                );
            }
            i += 1;
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn StructData_Dispose(
    mut storage: *mut crate::src::tests::structdata::StructData,
) {
    let mut i: ::core::ffi::c_int = 0;
    if !storage.is_null() {
    } else {
        __assert_fail(
            b"storage != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/structdata.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            151u32,
            b"void StructData_Dispose(StructData *)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    i = 0i32;
    while i < (*storage).count {
        free(
            (*(*storage).entries.offset(i as isize)).str as *mut ::core::ffi::c_void,
        );
        i += 1;
    }
    free((*storage).entries as *mut ::core::ffi::c_void);
    (*storage).count = 0i32;
    (*storage).entries = ::core::ptr::null_mut::<crate::src::tests::structdata::StructDataEntry>();
}
