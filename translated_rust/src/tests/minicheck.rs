use ::c2rust_bitfields;
use std::ptr::NonNull;
use std::sync::{Mutex, MutexGuard};
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn _setjmp(__env: *mut __jmp_buf_tag) -> ::core::ffi::c_int;
    fn longjmp(__env: *mut __jmp_buf_tag, __val: ::core::ffi::c_int) -> !;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type va_list = __builtin_va_list;
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
pub type __jmp_buf = [::core::ffi::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::core::ffi::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const CK_SILENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CK_VERBOSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SUBTEST_LEN: ::core::ffi::c_int = 50 as ::core::ffi::c_int;

#[derive(Copy, Clone)]
struct CheckState {
    current_function: usize,
    current_subtest: [::core::ffi::c_char; SUBTEST_LEN as usize],
    current_lineno: ::core::ffi::c_int,
    current_filename: usize,
}

static CHECK_STATE: Mutex<CheckState> = Mutex::new(CheckState {
    current_function: 0,
    current_subtest: [0; SUBTEST_LEN as usize],
    current_lineno: -(1 as ::core::ffi::c_int),
    current_filename: 0,
});

fn check_state() -> MutexGuard<'static, CheckState> {
    CHECK_STATE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

macro_rules! assert_not_null {
    ($ptr:expr, $assertion:expr, $file:expr, $line:expr, $function:expr $(,)?) => {{
        match NonNull::new($ptr) {
            Some(ptr) => ptr,
            None => unsafe { __assert_fail($assertion, $file, $line, $function) },
        }
    }};
}

macro_rules! assert_is_null {
    ($ptr:expr, $assertion:expr, $file:expr, $line:expr, $function:expr $(,)?) => {{
        if !$ptr.is_null() {
            unsafe { __assert_fail($assertion, $file, $line, $function) }
        }
    }};
}

struct OwnedSuite {
    header: Suite,
}
unsafe impl Send for OwnedSuite {}

struct OwnedTCase {
    header: TCase,
    tests: Vec<tcase_test_function>,
}
unsafe impl Send for OwnedTCase {}

struct OwnedRunner {
    header: SRunner,
}
unsafe impl Send for OwnedRunner {}

static SUITES: Mutex<Vec<Box<OwnedSuite>>> = Mutex::new(Vec::new());
static TCASES: Mutex<Vec<Box<OwnedTCase>>> = Mutex::new(Vec::new());
static RUNNERS: Mutex<Vec<Box<OwnedRunner>>> = Mutex::new(Vec::new());

fn suites() -> MutexGuard<'static, Vec<Box<OwnedSuite>>> {
    SUITES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn tcases() -> MutexGuard<'static, Vec<Box<OwnedTCase>>> {
    TCASES
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn runners() -> MutexGuard<'static, Vec<Box<OwnedRunner>>> {
    RUNNERS
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn suite_ptr(owned: &OwnedSuite) -> *mut Suite {
    (&owned.header as *const Suite).cast_mut()
}

fn tcase_ptr(owned: &OwnedTCase) -> *mut TCase {
    (&owned.header as *const TCase).cast_mut()
}

fn runner_ptr(owned: &OwnedRunner) -> *mut SRunner {
    (&owned.header as *const SRunner).cast_mut()
}

fn with_suite<R>(ptr: *mut Suite, f: impl FnOnce(&OwnedSuite) -> R) -> Option<R> {
    let key = ptr as usize;
    let suites = suites();
    suites
        .iter()
        .find(|owned| suite_ptr(owned.as_ref()) as usize == key)
        .map(|owned| f(owned.as_ref()))
}

fn with_suite_mut<R>(ptr: *mut Suite, f: impl FnOnce(&mut OwnedSuite) -> R) -> Option<R> {
    let key = ptr as usize;
    let mut suites = suites();
    suites
        .iter_mut()
        .find(|owned| suite_ptr(owned.as_ref()) as usize == key)
        .map(|owned| f(owned.as_mut()))
}

fn remove_suite(ptr: *mut Suite) -> Option<Box<OwnedSuite>> {
    let key = ptr as usize;
    let mut suites = suites();
    let index = suites
        .iter()
        .position(|owned| suite_ptr(owned.as_ref()) as usize == key)?;
    Some(suites.remove(index))
}

fn with_tcase<R>(ptr: *mut TCase, f: impl FnOnce(&OwnedTCase) -> R) -> Option<R> {
    let key = ptr as usize;
    let tcases = tcases();
    tcases
        .iter()
        .find(|owned| tcase_ptr(owned.as_ref()) as usize == key)
        .map(|owned| f(owned.as_ref()))
}

fn with_tcase_mut<R>(ptr: *mut TCase, f: impl FnOnce(&mut OwnedTCase) -> R) -> Option<R> {
    let key = ptr as usize;
    let mut tcases = tcases();
    tcases
        .iter_mut()
        .find(|owned| tcase_ptr(owned.as_ref()) as usize == key)
        .map(|owned| f(owned.as_mut()))
}

fn remove_tcase(ptr: *mut TCase) -> Option<Box<OwnedTCase>> {
    let key = ptr as usize;
    let mut tcases = tcases();
    let index = tcases
        .iter()
        .position(|owned| tcase_ptr(owned.as_ref()) as usize == key)?;
    Some(tcases.remove(index))
}

fn with_runner<R>(ptr: *mut SRunner, f: impl FnOnce(&OwnedRunner) -> R) -> Option<R> {
    let key = ptr as usize;
    let runners = runners();
    runners
        .iter()
        .find(|owned| runner_ptr(owned.as_ref()) as usize == key)
        .map(|owned| f(owned.as_ref()))
}

fn with_runner_mut<R>(ptr: *mut SRunner, f: impl FnOnce(&mut OwnedRunner) -> R) -> Option<R> {
    let key = ptr as usize;
    let mut runners = runners();
    runners
        .iter_mut()
        .find(|owned| runner_ptr(owned.as_ref()) as usize == key)
        .map(|owned| f(owned.as_mut()))
}

fn remove_runner(ptr: *mut SRunner) -> Option<Box<OwnedRunner>> {
    let key = ptr as usize;
    let mut runners = runners();
    let index = runners
        .iter()
        .position(|owned| runner_ptr(owned.as_ref()) as usize == key)?;
    Some(runners.remove(index))
}

enum PrintMessage {
    Pass(*const ::core::ffi::c_char),
    Fail {
        context: *const ::core::ffi::c_char,
        function: *const ::core::ffi::c_char,
        phase_info: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    },
    Summary {
        display: ::core::ffi::c_int,
        nchecks: ::core::ffi::c_int,
        nfailures: ::core::ffi::c_int,
    },
}

fn print_message(message: PrintMessage) {
    unsafe {
        match message {
            PrintMessage::Pass(function) => {
                printf(
                    b"PASS: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    function,
                );
            }
            PrintMessage::Fail {
                context,
                function,
                phase_info,
                filename,
                line,
            } => {
                printf(
                    b"FAIL [%s]: %s (%s at %s:%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                    context,
                    function,
                    phase_info,
                    filename,
                    line,
                );
            }
            PrintMessage::Summary {
                display,
                nchecks,
                nfailures,
            } => {
                printf(
                    b"%d%%: Checks: %d, Failed: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                    display,
                    nchecks,
                    nfailures,
                );
            }
        }
    }
}

fn check_state_function(state: &CheckState) -> *const ::core::ffi::c_char {
    state.current_function as *const ::core::ffi::c_char
}

fn check_state_filename(state: &CheckState) -> *const ::core::ffi::c_char {
    state.current_filename as *const ::core::ffi::c_char
}
#[no_mangle]
pub unsafe extern "C" fn suite_create(mut name: *const ::core::ffi::c_char) -> *mut Suite {
    let mut owned = Box::new(OwnedSuite {
        header: Suite {
            name,
            tests: ::core::ptr::null_mut(),
        },
    });
    let suite = suite_ptr(owned.as_ref());
    suites().push(owned);
    suite
}
#[no_mangle]
pub unsafe extern "C" fn tcase_create(mut name: *const ::core::ffi::c_char) -> *mut TCase {
    let mut owned = Box::new(OwnedTCase {
        header: TCase {
            name,
            setup: None,
            teardown: None,
            tests: ::core::ptr::null_mut(),
            ntests: 0,
            allocated: 0,
            next_tcase: ::core::ptr::null_mut(),
        },
        tests: Vec::new(),
    });
    let tc = tcase_ptr(owned.as_ref());
    tcases().push(owned);
    tc
}
#[no_mangle]
pub unsafe extern "C" fn suite_add_tcase(mut suite: *mut Suite, mut tc: *mut TCase) {
    let suite = assert_not_null!(
        suite,
        b"suite != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/minicheck.c\0".as_ptr() as *const ::core::ffi::c_char,
        75 as ::core::ffi::c_uint,
        b"void suite_add_tcase(Suite *, TCase *)\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .as_ptr();
    let tc = assert_not_null!(
        tc,
        b"tc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/minicheck.c\0".as_ptr() as *const ::core::ffi::c_char,
        76 as ::core::ffi::c_uint,
        b"void suite_add_tcase(Suite *, TCase *)\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .as_ptr();

    assert_is_null!(
        with_tcase(tc, |tc| tc.header.next_tcase).unwrap_or(::core::ptr::null_mut()),
        b"tc->next_tcase == NULL\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/minicheck.c\0".as_ptr() as *const ::core::ffi::c_char,
        77 as ::core::ffi::c_uint,
        b"void suite_add_tcase(Suite *, TCase *)\0".as_ptr() as *const ::core::ffi::c_char,
    );

    let tests = with_suite(suite, |suite| suite.header.tests)
        .expect("suite_add_tcase should receive a suite created by suite_create");
    with_tcase_mut(tc, |tc| {
        tc.header.next_tcase = tests;
    })
    .expect("suite_add_tcase should receive a test case created by tcase_create");
    with_suite_mut(suite, |suite| {
        suite.header.tests = tc;
    })
    .expect("suite_add_tcase should receive a suite created by suite_create");
}
#[no_mangle]
pub unsafe extern "C" fn tcase_add_checked_fixture(
    mut tc: *mut TCase,
    mut setup: tcase_setup_function,
    mut teardown: tcase_teardown_function,
) {
    let tc = assert_not_null!(
        tc,
        b"tc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/minicheck.c\0".as_ptr() as *const ::core::ffi::c_char,
        86 as ::core::ffi::c_uint,
        b"void tcase_add_checked_fixture(TCase *, tcase_setup_function, tcase_teardown_function)\0"
            .as_ptr() as *const ::core::ffi::c_char,
    )
    .as_ptr();
    with_tcase_mut(tc, |tc| {
        tc.header.setup = setup;
        tc.header.teardown = teardown;
    })
    .expect("tcase_add_checked_fixture should receive a test case created by tcase_create");
}
#[no_mangle]
pub unsafe extern "C" fn tcase_add_test(mut tc: *mut TCase, mut test: tcase_test_function) {
    let tc = assert_not_null!(
        tc,
        b"tc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/minicheck.c\0".as_ptr() as *const ::core::ffi::c_char,
        93 as ::core::ffi::c_uint,
        b"void tcase_add_test(TCase *, tcase_test_function)\0".as_ptr()
            as *const ::core::ffi::c_char,
    )
    .as_ptr();
    with_tcase_mut(tc, |tc| {
        if tc.header.allocated == tc.header.ntests {
            let nalloc = tc.header.allocated + 100 as ::core::ffi::c_int;
            let required_capacity =
                usize::try_from(nalloc).expect("test allocation count should fit into usize");
            if tc.tests.capacity() < required_capacity {
                tc.tests
                    .reserve_exact(required_capacity - tc.tests.capacity());
            }
            tc.header.allocated = nalloc;
        }

        tc.tests.push(test);
        tc.header.ntests =
            ::core::ffi::c_int::try_from(tc.tests.len()).expect("test count should fit into c_int");
        tc.header.tests = if tc.tests.is_empty() {
            ::core::ptr::null_mut()
        } else {
            tc.tests.as_mut_ptr()
        };
    })
    .expect("tcase_add_test should receive a test case created by tcase_create");
}
fn tcase_free(tc: *mut TCase) {
    let _ = remove_tcase(tc);
}
fn suite_free(suite: *mut Suite) {
    let Some(owned_suite) = remove_suite(suite) else {
        return;
    };
    let mut next_tcase = owned_suite.header.tests;

    while !next_tcase.is_null() {
        let current = next_tcase;
        next_tcase =
            with_tcase(current, |tc| tc.header.next_tcase).unwrap_or(::core::ptr::null_mut());
        tcase_free(current);
    }
}
#[no_mangle]
pub unsafe extern "C" fn srunner_create(mut suite: *mut Suite) -> *mut SRunner {
    let mut owned = Box::new(OwnedRunner {
        header: SRunner {
            suite,
            nchecks: 0,
            nfailures: 0,
        },
    });
    let runner = runner_ptr(owned.as_ref());
    runners().push(owned);
    runner
}
static mut env: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub unsafe extern "C" fn _check_set_test_info(
    mut function: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut lineno: ::core::ffi::c_int,
) {
    let mut state = check_state();
    state.current_function = function as usize;
    state.current_lineno = lineno;
    state.current_filename = filename as usize;
    state.current_subtest = [0; SUBTEST_LEN as usize];
}
#[no_mangle]
pub unsafe extern "C" fn set_subtest(mut fmt: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut ap: ::core::ffi::VaListImpl = c2rust_args.clone();
    let mut state = check_state();
    unsafe {
        vsnprintf(
            state.current_subtest.as_mut_ptr(),
            SUBTEST_LEN as size_t,
            fmt,
            ap.as_va_list(),
        );
    }
    for ch in state.current_subtest.iter_mut() {
        if *ch as ::core::ffi::c_int == '\n' as i32 {
            *ch = ' ' as i32 as ::core::ffi::c_char;
        }
    }
    state.current_subtest[(SUBTEST_LEN - 1 as ::core::ffi::c_int) as usize] =
        '\0' as i32 as ::core::ffi::c_char;
}
fn handle_success(verbosity: ::core::ffi::c_int) {
    if verbosity >= CK_VERBOSE {
        let state = check_state();
        print_message(PrintMessage::Pass(check_state_function(&state)));
    }
}
fn handle_failure(
    runner: *mut SRunner,
    verbosity: ::core::ffi::c_int,
    context: *const ::core::ffi::c_char,
    phase_info: *const ::core::ffi::c_char,
) {
    let _ = with_runner_mut(runner, |runner| {
        runner.header.nfailures += 1;
    });
    if verbosity != CK_SILENT {
        let state = check_state();
        let phase_info = if state.current_subtest[0] != 0 {
            state.current_subtest.as_ptr()
        } else {
            phase_info
        };
        print_message(PrintMessage::Fail {
            context,
            function: check_state_function(&state),
            phase_info,
            filename: check_state_filename(&state),
            line: state.current_lineno,
        });
    }
}
#[no_mangle]
pub unsafe extern "C" fn srunner_run_all(
    mut runner: *mut SRunner,
    mut context: *const ::core::ffi::c_char,
    mut verbosity: ::core::ffi::c_int,
) {
    unsafe {
        let mut suite: *mut Suite = ::core::ptr::null_mut::<Suite>();
        let mut tc: *mut TCase = ::core::ptr::null_mut::<TCase>();
        if !runner.is_null() {
        } else {
            __assert_fail(
                b"runner != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/minicheck.c\0".as_ptr() as *const ::core::ffi::c_char,
                195 as ::core::ffi::c_uint,
                b"void srunner_run_all(SRunner *, const char *, int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        suite = (*runner).suite;
        ::core::ptr::write_volatile(&mut tc as *mut *mut TCase, (*suite).tests);
        while !tc.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            let mut c2rust_current_block_13: u64;
            ::core::ptr::write_volatile(&mut i as *mut ::core::ffi::c_int, 0 as ::core::ffi::c_int);
            while i < (*tc).ntests {
                (*runner).nchecks += 1;
                set_subtest(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if (*tc).setup.is_some() {
                    if _setjmp(&raw mut env as *mut __jmp_buf_tag) != 0 {
                        handle_failure(
                            runner,
                            verbosity,
                            context,
                            b"during setup\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        c2rust_current_block_13 = 715039052867723359;
                    } else {
                        (*tc).setup.expect("non-null function pointer")();
                        c2rust_current_block_13 = 2868539653012386629;
                    }
                } else {
                    c2rust_current_block_13 = 2868539653012386629;
                }
                match c2rust_current_block_13 {
                    2868539653012386629 => {
                        if _setjmp(&raw mut env as *mut __jmp_buf_tag) != 0 {
                            handle_failure(
                                runner,
                                verbosity,
                                context,
                                b"during actual test\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else {
                            (*(*tc).tests.offset(i as isize)).expect("non-null function pointer")();
                            set_subtest(
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                b"\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            if (*tc).teardown.is_some() {
                                if _setjmp(&raw mut env as *mut __jmp_buf_tag) != 0 {
                                    handle_failure(
                                        runner,
                                        verbosity,
                                        context,
                                        b"during teardown\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    c2rust_current_block_13 = 715039052867723359;
                                } else {
                                    (*tc).teardown.expect("non-null function pointer")();
                                    c2rust_current_block_13 = 8457315219000651999;
                                }
                            } else {
                                c2rust_current_block_13 = 8457315219000651999;
                            }
                            match c2rust_current_block_13 {
                                715039052867723359 => {}
                                _ => {
                                    handle_success(verbosity);
                                }
                            }
                        }
                    }
                    _ => {}
                }
                ::core::ptr::write_volatile(
                    &mut i as *mut ::core::ffi::c_int,
                    ::core::ptr::read_volatile::<::core::ffi::c_int>(
                        &i as *const ::core::ffi::c_int,
                    ) + 1,
                );
            }
            ::core::ptr::write_volatile(&mut tc as *mut *mut TCase, (*tc).next_tcase);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn srunner_summarize(
    mut runner: *mut SRunner,
    mut verbosity: ::core::ffi::c_int,
) {
    if verbosity != CK_SILENT {
        let (nchecks, nfailures) = with_runner(runner, |runner| {
            (runner.header.nchecks, runner.header.nfailures)
        })
        .unwrap_or((0, 0));
        let passed: ::core::ffi::c_int = nchecks - nfailures;
        let percentage: ::core::ffi::c_double =
            passed as ::core::ffi::c_double / nchecks as ::core::ffi::c_double;
        let display: ::core::ffi::c_int =
            (percentage * 100 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int;
        print_message(PrintMessage::Summary {
            display,
            nchecks,
            nfailures,
        });
    }
}
#[no_mangle]
pub unsafe extern "C" fn _fail(
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) -> ! {
    {
        let mut state = check_state();
        state.current_filename = file as usize;
        state.current_lineno = line;
    }
    if !msg.is_null() {
        let has_newline: ::core::ffi::c_int = unsafe {
            (*msg.offset(strlen(msg).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '\n' as i32) as ::core::ffi::c_int
        };
        unsafe {
            fprintf(
                stderr,
                b"ERROR: %s%s\0".as_ptr() as *const ::core::ffi::c_char,
                msg,
                if has_newline != 0 {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\n\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
    }
    unsafe {
        longjmp(&raw mut env as *mut __jmp_buf_tag, 1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn srunner_ntests_failed(mut runner: *mut SRunner) -> ::core::ffi::c_int {
    let runner = assert_not_null!(
        runner,
        b"runner != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/minicheck.c\0".as_ptr() as *const ::core::ffi::c_char,
        263 as ::core::ffi::c_uint,
        b"int srunner_ntests_failed(SRunner *)\0".as_ptr() as *const ::core::ffi::c_char,
    )
    .as_ptr();
    with_runner(runner, |runner| runner.header.nfailures)
        .expect("srunner_ntests_failed should receive a runner created by srunner_create")
}
#[no_mangle]
pub unsafe extern "C" fn srunner_free(mut runner: *mut SRunner) {
    if let Some(suite) = with_runner(runner, |runner| runner.header.suite) {
        suite_free(suite);
    }
    let _ = remove_runner(runner);
}
