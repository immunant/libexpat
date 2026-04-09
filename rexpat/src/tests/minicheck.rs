// =============== BEGIN minicheck_h ================
pub const CK_SILENT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const CK_VERBOSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub type tcase_setup_function = Option<unsafe extern "C" fn() -> ()>;

pub type tcase_teardown_function = Option<unsafe extern "C" fn() -> ()>;

pub type tcase_test_function = Option<unsafe extern "C" fn() -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct SRunner {
    pub suite: *mut crate::src::tests::minicheck::Suite,
    pub nchecks: ::core::ffi::c_int,
    pub nfailures: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct Suite {
    pub name: *const ::core::ffi::c_char,
    pub tests: *mut crate::src::tests::minicheck::TCase,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct TCase {
    pub name: *const ::core::ffi::c_char,
    pub setup: crate::src::tests::minicheck::tcase_setup_function,
    pub teardown: crate::src::tests::minicheck::tcase_teardown_function,
    pub tests: *mut crate::src::tests::minicheck::tcase_test_function,
    pub ntests: ::core::ffi::c_int,
    pub allocated: ::core::ffi::c_int,
    pub next_tcase: *mut crate::src::tests::minicheck::TCase,
}
use ::c2rust_bitfields;

pub use crate::__stdarg_va_list_h::va_list;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdlib::__jmp_buf;

pub use crate::__stddef_null_h::NULL;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
use crate::stdlib::__assert_fail;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__sigset_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;

pub use crate::stdlib::__jmp_buf_tag;
pub use crate::stdlib::_setjmp;
use crate::stdlib::calloc;
use crate::stdlib::fprintf;
use crate::stdlib::free;
pub use crate::stdlib::jmp_buf;
pub use crate::stdlib::longjmp;
use crate::stdlib::printf;
use crate::stdlib::realloc;
use crate::stdlib::stderr;
use crate::stdlib::strlen;
use crate::stdlib::vsnprintf;
#[no_mangle]

pub unsafe extern "C" fn suite_create(
    mut name: *const ::core::ffi::c_char,
) -> *mut crate::src::tests::minicheck::Suite {
    let mut suite: *mut crate::src::tests::minicheck::Suite = calloc(
        1 as size_t,
        ::core::mem::size_of::<crate::src::tests::minicheck::Suite>()
            as size_t,
    )
        as *mut crate::src::tests::minicheck::Suite;
    if !suite.is_null() {
        (*suite).name = name;
    }
    return suite;
}
#[no_mangle]

pub unsafe extern "C" fn tcase_create(
    mut name: *const ::core::ffi::c_char,
) -> *mut crate::src::tests::minicheck::TCase {
    let mut tc: *mut crate::src::tests::minicheck::TCase = calloc(
        1 as size_t,
        ::core::mem::size_of::<crate::src::tests::minicheck::TCase>()
            as size_t,
    )
        as *mut crate::src::tests::minicheck::TCase;
    if !tc.is_null() {
        (*tc).name = name;
    }
    return tc;
}
#[no_mangle]

pub unsafe extern "C" fn suite_add_tcase(
    mut suite: *mut crate::src::tests::minicheck::Suite,
    mut tc: *mut crate::src::tests::minicheck::TCase,
) {
    if !suite.is_null() {
    } else {
        __assert_fail(
            b"suite != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/minicheck.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            75 as ::core::ffi::c_uint,
            b"void suite_add_tcase(Suite *, TCase *)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    if !tc.is_null() {
    } else {
        __assert_fail(
            b"tc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/minicheck.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            76 as ::core::ffi::c_uint,
            b"void suite_add_tcase(Suite *, TCase *)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    if (*tc).next_tcase.is_null() {
    } else {
        __assert_fail(
            b"tc->next_tcase == NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/minicheck.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            77 as ::core::ffi::c_uint,
            b"void suite_add_tcase(Suite *, TCase *)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    (*tc).next_tcase = (*suite).tests;
    (*suite).tests = tc;
}
#[no_mangle]

pub unsafe extern "C" fn tcase_add_checked_fixture(
    mut tc: *mut crate::src::tests::minicheck::TCase,
    mut setup: crate::src::tests::minicheck::tcase_setup_function,
    mut teardown: crate::src::tests::minicheck::tcase_teardown_function,
) {
    if !tc.is_null() {
    } else {
        __assert_fail(
            b"tc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/minicheck.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            86 as ::core::ffi::c_uint,
            b"void tcase_add_checked_fixture(TCase *, tcase_setup_function, tcase_teardown_function)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    (*tc).setup = setup;
    (*tc).teardown = teardown;
}
#[no_mangle]

pub unsafe extern "C" fn tcase_add_test(
    mut tc: *mut crate::src::tests::minicheck::TCase,
    mut test: crate::src::tests::minicheck::tcase_test_function,
) {
    if !tc.is_null() {
    } else {
        __assert_fail(
            b"tc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/minicheck.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            93 as ::core::ffi::c_uint,
            b"void tcase_add_test(TCase *, tcase_test_function)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    if (*tc).allocated == (*tc).ntests {
        let mut nalloc: ::core::ffi::c_int = (*tc).allocated + 100 as ::core::ffi::c_int;
        let mut new_size: size_t =
            (::core::mem::size_of::<crate::src::tests::minicheck::tcase_test_function>()
                as size_t)
                .wrapping_mul(nalloc as size_t);
        let new_tests: *mut crate::src::tests::minicheck::tcase_test_function =
            realloc((*tc).tests as *mut ::core::ffi::c_void, new_size)
                as *mut crate::src::tests::minicheck::tcase_test_function;
        if !new_tests.is_null() {
        } else {
            __assert_fail(
                b"new_tests != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/minicheck.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                99 as ::core::ffi::c_uint,
                b"void tcase_add_test(TCase *, tcase_test_function)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        };
        (*tc).tests = new_tests;
        (*tc).allocated = nalloc;
    }
    let ref mut c2rust_fresh0 = *(*tc).tests.offset((*tc).ntests as isize);
    *c2rust_fresh0 = test;
    (*tc).ntests += 1;
}

unsafe extern "C" fn tcase_free(mut tc: *mut crate::src::tests::minicheck::TCase) {
    if tc.is_null() {
        return;
    }
    free((*tc).tests as *mut ::core::ffi::c_void);
    free(tc as *mut ::core::ffi::c_void);
}

unsafe extern "C" fn suite_free(mut suite: *mut crate::src::tests::minicheck::Suite) {
    if suite.is_null() {
        return;
    }
    while !(*suite).tests.is_null() {
        let mut next: *mut crate::src::tests::minicheck::TCase = (*(*suite).tests).next_tcase;
        tcase_free((*suite).tests);
        (*suite).tests = next;
    }
    free(suite as *mut ::core::ffi::c_void);
}
#[no_mangle]

pub unsafe extern "C" fn srunner_create(
    mut suite: *mut crate::src::tests::minicheck::Suite,
) -> *mut crate::src::tests::minicheck::SRunner {
    let runner: *mut crate::src::tests::minicheck::SRunner = calloc(
        1 as size_t,
        ::core::mem::size_of::<crate::src::tests::minicheck::SRunner>()
            as size_t,
    )
        as *mut crate::src::tests::minicheck::SRunner;
    if !runner.is_null() {
        (*runner).suite = suite;
    }
    return runner;
}

static mut env: jmp_buf = [__jmp_buf_tag {
    __jmpbuf:  [0; 8],
    __mask_was_saved:  0,
    __saved_mask:  __sigset_t { __val:  [0; 16] },
}; 1];

pub const SUBTEST_LEN: ::core::ffi::c_int = 50 as ::core::ffi::c_int;

static mut _check_current_function: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();

static mut _check_current_subtest: [::core::ffi::c_char; 50] = [0; 50];

static mut _check_current_lineno: ::core::ffi::c_int = -1 as ::core::ffi::c_int;

static mut _check_current_filename: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]

pub unsafe extern "C" fn _check_set_test_info(
    mut function: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut lineno: ::core::ffi::c_int,
) {
    _check_current_function = function;
    set_subtest(
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
    );
    _check_current_lineno = lineno;
    _check_current_filename = filename;
}
#[no_mangle]

pub unsafe extern "C" fn set_subtest(mut fmt: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = c2rust_args.clone();
    vsnprintf(
        &raw mut _check_current_subtest as *mut ::core::ffi::c_char,
        SUBTEST_LEN as size_t,
        fmt,
        ap.as_va_list(),
    );
    let mut i: size_t = 0 as size_t;
    while i < SUBTEST_LEN as size_t {
        if _check_current_subtest[i as usize] as ::core::ffi::c_int == '\n' as i32 {
            _check_current_subtest[i as usize] = ' ' as i32 as ::core::ffi::c_char;
        }
        i = i.wrapping_add(1);
    }
    _check_current_subtest[(SUBTEST_LEN - 1 as ::core::ffi::c_int) as usize] =
        '\0' as i32 as ::core::ffi::c_char;
}

unsafe extern "C" fn handle_success(mut verbosity: ::core::ffi::c_int) {
    if verbosity >= crate::src::tests::minicheck::CK_VERBOSE {
        printf(
            b"PASS: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            _check_current_function,
        );
    }
}

unsafe extern "C" fn handle_failure(
    mut runner: *mut crate::src::tests::minicheck::SRunner,
    mut verbosity: ::core::ffi::c_int,
    mut context: *const ::core::ffi::c_char,
    mut phase_info: *const ::core::ffi::c_char,
) {
    (*runner).nfailures += 1;
    if verbosity != crate::src::tests::minicheck::CK_SILENT {
        if strlen(&raw mut _check_current_subtest as *mut ::core::ffi::c_char)
            != 0 as size_t
        {
            phase_info = &raw mut _check_current_subtest as *mut ::core::ffi::c_char;
        }
        printf(
            b"FAIL [%s]: %s (%s at %s:%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
            context,
            _check_current_function,
            phase_info,
            _check_current_filename,
            _check_current_lineno,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn srunner_run_all(
    mut runner: *mut crate::src::tests::minicheck::SRunner,
    mut context: *const ::core::ffi::c_char,
    mut verbosity: ::core::ffi::c_int,
) {
    let mut suite: *mut crate::src::tests::minicheck::Suite =
        ::core::ptr::null_mut::<crate::src::tests::minicheck::Suite>();
    let mut tc: *mut crate::src::tests::minicheck::TCase =
        ::core::ptr::null_mut::<crate::src::tests::minicheck::TCase>();
    if !runner.is_null() {
    } else {
        __assert_fail(
            b"runner != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/minicheck.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            195 as ::core::ffi::c_uint,
            b"void srunner_run_all(SRunner *, const char *, int)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    };
    suite = (*runner).suite;
    ::core::ptr::write_volatile(&raw mut tc, (*suite).tests);
    while !::core::ptr::read_volatile::<*mut crate::src::tests::minicheck::TCase>(&raw const tc)
        .is_null()
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut c2rust_current_block_13: u64;
        ::core::ptr::write_volatile(&raw mut i, 0 as ::core::ffi::c_int);
        while ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const i)
            < (*::core::ptr::read_volatile::<*mut crate::src::tests::minicheck::TCase>(
                &raw const tc,
            ))
            .ntests
        {
            (*runner).nchecks += 1;
            set_subtest(
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if (*::core::ptr::read_volatile::<*mut crate::src::tests::minicheck::TCase>(
                &raw const tc,
            ))
            .setup
            .is_some()
            {
                if _setjmp(&raw mut env as *mut __jmp_buf_tag) != 0 {
                    handle_failure(
                        runner,
                        verbosity,
                        context,
                        b"during setup\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    c2rust_current_block_13 = 715039052867723359;
                } else {
                    (*::core::ptr::read_volatile::<*mut crate::src::tests::minicheck::TCase>(
                        &raw const tc,
                    ))
                    .setup
                    .expect("non-null function pointer")();
                    c2rust_current_block_13 = 2868539653012386629;
                }
            } else {
                c2rust_current_block_13 = 2868539653012386629;
            }
            match c2rust_current_block_13 {
                2868539653012386629 => {
                    if _setjmp(&raw mut env as *mut __jmp_buf_tag)
                        != 0
                    {
                        handle_failure(
                            runner,
                            verbosity,
                            context,
                            b"during actual test\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    } else {
                        (*(*::core::ptr::read_volatile::<*mut crate::src::tests::minicheck::TCase>(&raw const tc))
                            .tests
                            .offset(
                                ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const i)
                                    as isize,
                            ))
                        .expect("non-null function pointer")();
                        set_subtest(
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            b"\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if (*::core::ptr::read_volatile::<*mut crate::src::tests::minicheck::TCase>(
                            &raw const tc,
                        ))
                        .teardown
                        .is_some()
                        {
                            if _setjmp(
                                &raw mut env as *mut __jmp_buf_tag,
                            ) != 0
                            {
                                handle_failure(
                                    runner,
                                    verbosity,
                                    context,
                                    b"during teardown\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                c2rust_current_block_13 = 715039052867723359;
                            } else {
                                (*::core::ptr::read_volatile::<
                                    *mut crate::src::tests::minicheck::TCase,
                                >(&raw const tc))
                                .teardown
                                .expect("non-null function pointer")(
                                );
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
                &raw mut i,
                ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const i) + 1,
            );
        }
        ::core::ptr::write_volatile(
            &raw mut tc,
            (*::core::ptr::read_volatile::<*mut crate::src::tests::minicheck::TCase>(
                &raw const tc,
            ))
            .next_tcase,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn srunner_summarize(
    mut runner: *mut crate::src::tests::minicheck::SRunner,
    mut verbosity: ::core::ffi::c_int,
) {
    if verbosity != crate::src::tests::minicheck::CK_SILENT {
        let mut passed: ::core::ffi::c_int = (*runner).nchecks - (*runner).nfailures;
        let mut percentage: ::core::ffi::c_double =
            passed as ::core::ffi::c_double / (*runner).nchecks as ::core::ffi::c_double;
        let mut display: ::core::ffi::c_int =
            (percentage * 100 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_int;
        printf(
            b"%d%%: Checks: %d, Failed: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            display,
            (*runner).nchecks,
            (*runner).nfailures,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn _fail(
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) -> ! {
    _check_current_filename = file;
    _check_current_lineno = line;
    if !msg.is_null() {
        let has_newline: ::core::ffi::c_int = (*msg.offset(
            strlen(msg).wrapping_sub(1 as size_t) as isize,
        ) as ::core::ffi::c_int
            == '\n' as i32) as ::core::ffi::c_int;
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
    longjmp(
        &raw mut env as *mut __jmp_buf_tag,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn srunner_ntests_failed(
    mut runner: *mut crate::src::tests::minicheck::SRunner,
) -> ::core::ffi::c_int {
    if !runner.is_null() {
    } else {
        __assert_fail(
            b"runner != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/minicheck.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            263 as ::core::ffi::c_uint,
            b"int srunner_ntests_failed(SRunner *)\0".as_ptr() as *const ::core::ffi::c_char,
        );
    };
    return (*runner).nfailures;
}
#[no_mangle]

pub unsafe extern "C" fn srunner_free(mut runner: *mut crate::src::tests::minicheck::SRunner) {
    if runner.is_null() {
        return;
    }
    suite_free((*runner).suite);
    free(runner as *mut ::core::ffi::c_void);
}
