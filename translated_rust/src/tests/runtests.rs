use ::c2rust_bitfields;
use std::cell::Cell;
use std::ffi::{CStr, CString};

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

#[repr(transparent)]
pub struct TestParserCell(Cell<XML_Parser>);

impl TestParserCell {
    const fn new(parser: XML_Parser) -> Self {
        Self(Cell::new(parser))
    }

    fn get(&self) -> XML_Parser {
        self.0.get()
    }

    fn set(&self, parser: XML_Parser) {
        self.0.set(parser);
    }

    fn take(&self) -> XML_Parser {
        self.0.replace(::core::ptr::null_mut())
    }
}

unsafe impl Sync for TestParserCell {}

#[no_mangle]
pub static g_parser: TestParserCell = TestParserCell::new(::core::ptr::null_mut());

pub(crate) fn current_test_parser() -> XML_Parser {
    g_parser.get()
}

pub(crate) fn set_current_test_parser(parser: XML_Parser) {
    g_parser.set(parser);
}

pub(crate) fn take_current_test_parser() -> XML_Parser {
    g_parser.take()
}

fn parse_verbosity(args: impl IntoIterator<Item = String>) -> Result<::core::ffi::c_int, String> {
    let mut verbosity = CK_NORMAL;

    for arg in args {
        match arg.as_str() {
            "-v" | "--verbose" => verbosity = CK_VERBOSE,
            "-q" | "--quiet" => verbosity = CK_SILENT,
            _ => return Err(arg),
        }
    }

    Ok(verbosity)
}

fn run_test_matrix(verbosity: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        let s: *mut Suite = suite_create(b"basic\0".as_ptr() as *const ::core::ffi::c_char);
        make_basic_test_case(s);
        make_namespace_test_case(s);
        make_miscellaneous_test_case(s);
        make_alloc_test_case(s);
        make_nsalloc_test_case(s);
        make_accounting_test_case(s);

        let sr: *mut SRunner = srunner_create(s);
        if verbosity != CK_SILENT {
            println!(
                "Expat version: {}",
                CStr::from_ptr(XML_ExpatVersion()).to_string_lossy()
            );
        }
        g_chunkSize = 0 as ::core::ffi::c_int;
        while g_chunkSize <= 5 as ::core::ffi::c_int {
            let mut enabled = 0 as ::core::ffi::c_int;
            while enabled <= 1 as ::core::ffi::c_int {
                g_reparseDeferralEnabledDefault = enabled as XML_Bool;
                let context =
                    CString::new(format!("chunksize={} deferral={}", g_chunkSize, enabled))
                        .expect("test context must not contain interior NUL bytes");
                srunner_run_all(sr, context.as_ptr(), verbosity);
                enabled += 1;
            }
            g_chunkSize += 1;
        }
        srunner_summarize(sr, verbosity);
        let nf = srunner_ntests_failed(sr);
        srunner_free(sr);
        if nf == 0 as ::core::ffi::c_int {
            EXIT_SUCCESS
        } else {
            EXIT_FAILURE
        }
    }
}

pub fn main() {
    let args = ::std::env::args().skip(1);
    let verbosity = match parse_verbosity(args) {
        Ok(verbosity) => verbosity,
        Err(arg) => {
            eprintln!("runtests: unknown option '{}'", arg);
            ::std::process::exit(2);
        }
    };

    ::std::process::exit(run_test_matrix(verbosity) as i32)
}
