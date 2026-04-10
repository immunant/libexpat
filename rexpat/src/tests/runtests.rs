#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(extern_types, raw_ref_op, register_tool)]
pub mod expat_h {
    pub type XML_Parser = *mut ::rexpat::expat_h::XML_ParserStruct;

    pub type XML_Bool = ::core::ffi::c_uchar;
}
pub mod expat_external_h {
    pub type XML_LChar = ::core::ffi::c_char;
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod common_h {
    extern "C" {
        pub static mut g_chunkSize: ::core::ffi::c_int;
    }
}
pub mod internal_h {
    extern "C" {
        pub static mut g_reparseDeferralEnabledDefault: crate::expat_h::XML_Bool;
    }
}
pub mod minicheck_h {
    pub const CK_NORMAL: ::core::ffi::c_int = 1;

    pub type tcase_setup_function = Option<unsafe extern "C" fn() -> ()>;

    pub type tcase_teardown_function = Option<unsafe extern "C" fn() -> ()>;

    pub type tcase_test_function = Option<unsafe extern "C" fn() -> ()>;
}
pub mod stdlib {
    extern "C" {
        pub static mut stderr: *mut crate::stdlib::FILE;
        pub type _IO_marker;

        pub type _IO_codecvt;

        pub type _IO_wide_data;
    }
    pub type FILE = ::rexpat::stdlib::_IO_FILE;
    pub const EXIT_FAILURE: ::core::ffi::c_int = 1;

    pub const EXIT_SUCCESS: ::core::ffi::c_int = 0;
    pub type _IO_lock_t = ();
    pub type __uint64_t = u64;

    pub type __off_t = ::core::ffi::c_long;

    pub type __off64_t = ::core::ffi::c_long;
}
#[macro_use]
extern crate c2rust_bitfields;
#[allow(unused_imports)]
use ::rexpat;

pub use crate::__stddef_size_t_h::size_t;
pub use crate::expat_external_h::XML_LChar;
pub use crate::expat_h::XML_Bool;
pub use crate::expat_h::XML_Parser;
use crate::internal_h::g_reparseDeferralEnabledDefault;
pub use crate::minicheck_h::tcase_setup_function;
pub use crate::minicheck_h::tcase_teardown_function;
pub use crate::minicheck_h::tcase_test_function;
pub use crate::minicheck_h::CK_NORMAL;
pub use ::rexpat::expat_h::XML_ParserStruct;
pub use ::rexpat::src::lib::xmlparse::XML_ExpatVersion;
use ::rexpat::src::tests::acc_tests::make_accounting_test_case;
pub use ::rexpat::src::tests::acc_tests::Suite;
pub use ::rexpat::src::tests::acc_tests::TCase;
use ::rexpat::src::tests::alloc_tests::make_alloc_test_case;
use ::rexpat::src::tests::basic_tests::make_basic_test_case;
pub use ::rexpat::src::tests::minicheck::srunner_create;
pub use ::rexpat::src::tests::minicheck::srunner_free;
pub use ::rexpat::src::tests::minicheck::srunner_ntests_failed;
pub use ::rexpat::src::tests::minicheck::srunner_run_all;
pub use ::rexpat::src::tests::minicheck::srunner_summarize;
pub use ::rexpat::src::tests::minicheck::suite_create;
pub use ::rexpat::src::tests::minicheck::SRunner;
pub use ::rexpat::src::tests::minicheck::CK_SILENT;
pub use ::rexpat::src::tests::minicheck::CK_VERBOSE;
use ::rexpat::src::tests::misc_tests::make_miscellaneous_test_case;
use ::rexpat::src::tests::ns_tests::make_namespace_test_case;
use ::rexpat::src::tests::nsalloc_tests::make_nsalloc_test_case;

use crate::stdlib::stderr;
pub use crate::stdlib::EXIT_FAILURE;
pub use crate::stdlib::EXIT_SUCCESS;
use ::rexpat::stdlib::fprintf;
use ::rexpat::stdlib::printf;
use ::rexpat::stdlib::snprintf;
use ::rexpat::stdlib::strcmp;

use crate::common_h::g_chunkSize;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::FILE;
pub use ::rexpat::__stddef_null_h::NULL;
pub use ::rexpat::stdlib::_IO_FILE;
#[no_mangle]

pub static mut g_parser: XML_Parser =
    ::core::ptr::null::<XML_ParserStruct>() as *mut XML_ParserStruct;

unsafe extern "C" fn make_suite() -> *mut Suite {
    let mut s: *mut Suite = suite_create(b"basic\0".as_ptr() as *const ::core::ffi::c_char);
    make_basic_test_case(s);
    make_namespace_test_case(s);
    make_miscellaneous_test_case(s);
    make_alloc_test_case(s);
    make_nsalloc_test_case(s);
    make_accounting_test_case(s);
    return s;
}

unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut nf: ::core::ffi::c_int = 0;
    let mut verbosity: ::core::ffi::c_int = CK_NORMAL;
    let mut s: *mut Suite = make_suite();
    let mut sr: *mut SRunner = srunner_create(s);
    i = 1;
    while i < argc {
        let mut opt: *mut ::core::ffi::c_char = *argv.offset(i as isize);
        if strcmp(opt, b"-v\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(opt, b"--verbose\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            verbosity = CK_VERBOSE;
        } else if strcmp(opt, b"-q\0".as_ptr() as *const ::core::ffi::c_char) == 0
            || strcmp(opt, b"--quiet\0".as_ptr() as *const ::core::ffi::c_char) == 0
        {
            verbosity = CK_SILENT;
        } else {
            fprintf(
                stderr,
                b"runtests: unknown option '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                opt,
            );
            return 2i32;
        }
        i += 1;
    }
    if verbosity != CK_SILENT {
        printf(
            b"Expat version: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            XML_ExpatVersion(),
        );
    }
    g_chunkSize = 0;
    while g_chunkSize <= 5 {
        let mut enabled: ::core::ffi::c_int = 0;
        while enabled <= 1 {
            let mut context: [::core::ffi::c_char; 100] = [0; 100];
            g_reparseDeferralEnabledDefault = enabled as XML_Bool;
            snprintf(
                &raw mut context as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
                b"chunksize=%d deferral=%d\0".as_ptr() as *const ::core::ffi::c_char,
                g_chunkSize,
                enabled,
            );
            context[(::core::mem::size_of::<[::core::ffi::c_char; 100]>()).wrapping_sub(1usize)] =
                '\0' as ::core::ffi::c_char;
            srunner_run_all(sr, &raw mut context as *mut ::core::ffi::c_char, verbosity);
            enabled += 1;
        }
        g_chunkSize += 1;
    }
    srunner_summarize(sr, verbosity);
    nf = srunner_ntests_failed(sr);
    srunner_free(sr);
    return if nf == 0 { EXIT_SUCCESS } else { EXIT_FAILURE };
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr(),
        ))
    }
}
