use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type XML_ParserStruct;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn XML_ExpatVersion() -> *const XML_LChar;
    static mut g_reparseDeferralEnabledDefault: XML_Bool;
    fn suite_create(name: *const ::core::ffi::c_char) -> *mut Suite;
    fn srunner_create(suite: *mut Suite) -> *mut SRunner;
    fn srunner_run_all(
        runner: *mut SRunner,
        context: *const ::core::ffi::c_char,
        verbosity: ::core::ffi::c_int,
    );
    fn srunner_summarize(runner: *mut SRunner, verbosity: ::core::ffi::c_int);
    fn srunner_ntests_failed(runner: *mut SRunner) -> ::core::ffi::c_int;
    fn srunner_free(runner: *mut SRunner);
    static mut g_chunkSize: ::core::ffi::c_int;
    fn make_basic_test_case(s: *mut Suite);
    fn make_namespace_test_case(s: *mut Suite);
    fn make_miscellaneous_test_case(s: *mut Suite);
    fn make_alloc_test_case(s: *mut Suite);
    fn make_nsalloc_test_case(s: *mut Suite);
    fn make_accounting_test_case(s: *mut Suite);
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type XML_LChar = ::core::ffi::c_char;
pub type XML_Parser = *mut XML_ParserStruct;
pub type XML_Bool = ::core::ffi::c_uchar;
pub type tcase_setup_function = Option<unsafe extern "C" fn() -> ()>;
pub type tcase_teardown_function = Option<unsafe extern "C" fn() -> ()>;
pub type tcase_test_function = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRunner {
    pub suite: *mut Suite,
    pub nchecks: ::core::ffi::c_int,
    pub nfailures: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Suite {
    pub name: *const ::core::ffi::c_char,
    pub tests: *mut TCase,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TCase {
    pub name: *const ::core::ffi::c_char,
    pub setup: tcase_setup_function,
    pub teardown: tcase_teardown_function,
    pub tests: *mut tcase_test_function,
    pub ntests: ::core::ffi::c_int,
    pub allocated: ::core::ffi::c_int,
    pub next_tcase: *mut TCase,
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const CK_SILENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CK_NORMAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CK_VERBOSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub static mut g_parser: XML_Parser =
    ::core::ptr::null::<XML_ParserStruct>() as *mut XML_ParserStruct;
unsafe extern "C" fn make_suite() -> *mut Suite {
    unsafe {
        let mut s: *mut Suite = suite_create(b"basic\0".as_ptr() as *const ::core::ffi::c_char);
        make_basic_test_case(s);
        make_namespace_test_case(s);
        make_miscellaneous_test_case(s);
        make_alloc_test_case(s);
        make_nsalloc_test_case(s);
        make_accounting_test_case(s);
        return s;
    }
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut nf: ::core::ffi::c_int = 0;
        let mut verbosity: ::core::ffi::c_int = CK_NORMAL;
        let mut s: *mut Suite = make_suite();
        let mut sr: *mut SRunner = srunner_create(s);
        i = 1 as ::core::ffi::c_int;
        while i < argc {
            let mut opt: *mut ::core::ffi::c_char = *argv.offset(i as isize);
            if strcmp(opt, b"-v\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcmp(opt, b"--verbose\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                verbosity = CK_VERBOSE;
            } else if strcmp(opt, b"-q\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
                || strcmp(opt, b"--quiet\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
            {
                verbosity = CK_SILENT;
            } else {
                fprintf(
                    stderr,
                    b"runtests: unknown option '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    opt,
                );
                return 2 as ::core::ffi::c_int;
            }
            i += 1;
        }
        if verbosity != CK_SILENT {
            printf(
                b"Expat version: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                XML_ExpatVersion(),
            );
        }
        g_chunkSize = 0 as ::core::ffi::c_int;
        while g_chunkSize <= 5 as ::core::ffi::c_int {
            let mut enabled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while enabled <= 1 as ::core::ffi::c_int {
                let mut context: [::core::ffi::c_char; 100] = [0; 100];
                g_reparseDeferralEnabledDefault = enabled as XML_Bool;
                snprintf(
                    &raw mut context as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 100]>() as size_t,
                    b"chunksize=%d deferral=%d\0".as_ptr() as *const ::core::ffi::c_char,
                    g_chunkSize,
                    enabled,
                );
                context[(::core::mem::size_of::<[::core::ffi::c_char; 100]>() as usize)
                    .wrapping_sub(1 as usize) as usize] = '\0' as i32 as ::core::ffi::c_char;
                srunner_run_all(sr, &raw mut context as *mut ::core::ffi::c_char, verbosity);
                enabled += 1;
            }
            g_chunkSize += 1;
        }
        srunner_summarize(sr, verbosity);
        nf = srunner_ntests_failed(sr);
        srunner_free(sr);
        return if nf == 0 as ::core::ffi::c_int {
            EXIT_SUCCESS
        } else {
            EXIT_FAILURE
        };
    }
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
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
