extern "C" {
    pub type XML_ParserStruct;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn XML_ParserCreate_MM(
        encoding: *const XML_Char,
        memsuite: *const XML_Memory_Handling_Suite,
        namespaceSeparator: *const XML_Char,
    ) -> XML_Parser;
    fn XML_ParserReset(parser: XML_Parser, encoding: *const XML_Char) -> XML_Bool;
    fn XML_SetElementHandler(
        parser: XML_Parser,
        start: XML_StartElementHandler,
        end: XML_EndElementHandler,
    );
    fn XML_SetCharacterDataHandler(parser: XML_Parser, handler: XML_CharacterDataHandler);
    fn XML_SetDefaultHandler(parser: XML_Parser, handler: XML_DefaultHandler);
    fn XML_SetExternalEntityRefHandler(parser: XML_Parser, handler: XML_ExternalEntityRefHandler);
    fn XML_SetReturnNSTriplet(parser: XML_Parser, do_nst: ::core::ffi::c_int);
    fn XML_SetUserData(parser: XML_Parser, userData: *mut ::core::ffi::c_void);
    fn XML_GetBuffer(parser: XML_Parser, len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn XML_ParseBuffer(
        parser: XML_Parser,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn XML_ResumeParser(parser: XML_Parser) -> XML_Status;
    fn XML_SetParamEntityParsing(
        parser: XML_Parser,
        parsing: XML_ParamEntityParsing,
    ) -> ::core::ffi::c_int;
    fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
    fn _check_set_test_info(
        function: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
        lineno: ::core::ffi::c_int,
    );
    fn _fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
    ) -> !;
    fn tcase_create(name: *const ::core::ffi::c_char) -> *mut TCase;
    fn suite_add_tcase(suite: *mut Suite, tc: *mut TCase);
    fn tcase_add_checked_fixture(
        tc: *mut TCase,
        setup: tcase_setup_function,
        teardown: tcase_teardown_function,
    );
    fn tcase_add_test(tc: *mut TCase, test: tcase_test_function);
    static mut g_parser: XML_Parser;
    static mut g_resumable: XML_Bool;
    fn tcase_add_test__if_xml_ge(tc: *mut TCase, test: tcase_test_function);
    fn basic_teardown();
    fn _xml_failure(parser: XML_Parser, file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);
    fn _XML_Parse_SINGLE_BYTES(
        parser: XML_Parser,
        s: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    static mut g_allocation_count: ::core::ffi::c_int;
    static mut g_reallocation_count: ::core::ffi::c_int;
    fn duff_allocator(size: size_t) -> *mut ::core::ffi::c_void;
    fn duff_reallocator(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn dummy_default_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn triplet_start_checker(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn triplet_end_checker(userData: *mut ::core::ffi::c_void, name: *const XML_Char);
    fn external_entity_optioner(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn clearing_aborting_character_handler(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
}
pub type size_t = usize;
pub type XML_Char = ::core::ffi::c_char;
pub type XML_Parser = *mut XML_ParserStruct;
pub type XML_Bool = ::core::ffi::c_uchar;
pub type XML_Status = ::core::ffi::c_uint;
pub const XML_STATUS_SUSPENDED: XML_Status = 2;
pub const XML_STATUS_OK: XML_Status = 1;
pub const XML_STATUS_ERROR: XML_Status = 0;
pub type XML_Error = ::core::ffi::c_uint;
pub const XML_ERROR_NOT_STARTED: XML_Error = 44;
pub const XML_ERROR_AMPLIFICATION_LIMIT_BREACH: XML_Error = 43;
pub const XML_ERROR_NO_BUFFER: XML_Error = 42;
pub const XML_ERROR_INVALID_ARGUMENT: XML_Error = 41;
pub const XML_ERROR_RESERVED_NAMESPACE_URI: XML_Error = 40;
pub const XML_ERROR_RESERVED_PREFIX_XMLNS: XML_Error = 39;
pub const XML_ERROR_RESERVED_PREFIX_XML: XML_Error = 38;
pub const XML_ERROR_SUSPEND_PE: XML_Error = 37;
pub const XML_ERROR_FINISHED: XML_Error = 36;
pub const XML_ERROR_ABORTED: XML_Error = 35;
pub const XML_ERROR_NOT_SUSPENDED: XML_Error = 34;
pub const XML_ERROR_SUSPENDED: XML_Error = 33;
pub const XML_ERROR_PUBLICID: XML_Error = 32;
pub const XML_ERROR_TEXT_DECL: XML_Error = 31;
pub const XML_ERROR_XML_DECL: XML_Error = 30;
pub const XML_ERROR_INCOMPLETE_PE: XML_Error = 29;
pub const XML_ERROR_UNDECLARING_PREFIX: XML_Error = 28;
pub const XML_ERROR_UNBOUND_PREFIX: XML_Error = 27;
pub const XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING: XML_Error = 26;
pub const XML_ERROR_FEATURE_REQUIRES_XML_DTD: XML_Error = 25;
pub const XML_ERROR_ENTITY_DECLARED_IN_PE: XML_Error = 24;
pub const XML_ERROR_UNEXPECTED_STATE: XML_Error = 23;
pub const XML_ERROR_NOT_STANDALONE: XML_Error = 22;
pub const XML_ERROR_EXTERNAL_ENTITY_HANDLING: XML_Error = 21;
pub const XML_ERROR_UNCLOSED_CDATA_SECTION: XML_Error = 20;
pub const XML_ERROR_INCORRECT_ENCODING: XML_Error = 19;
pub const XML_ERROR_UNKNOWN_ENCODING: XML_Error = 18;
pub const XML_ERROR_MISPLACED_XML_PI: XML_Error = 17;
pub const XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF: XML_Error = 16;
pub const XML_ERROR_BINARY_ENTITY_REF: XML_Error = 15;
pub const XML_ERROR_BAD_CHAR_REF: XML_Error = 14;
pub const XML_ERROR_ASYNC_ENTITY: XML_Error = 13;
pub const XML_ERROR_RECURSIVE_ENTITY_REF: XML_Error = 12;
pub const XML_ERROR_UNDEFINED_ENTITY: XML_Error = 11;
pub const XML_ERROR_PARAM_ENTITY_REF: XML_Error = 10;
pub const XML_ERROR_JUNK_AFTER_DOC_ELEMENT: XML_Error = 9;
pub const XML_ERROR_DUPLICATE_ATTRIBUTE: XML_Error = 8;
pub const XML_ERROR_TAG_MISMATCH: XML_Error = 7;
pub const XML_ERROR_PARTIAL_CHAR: XML_Error = 6;
pub const XML_ERROR_UNCLOSED_TOKEN: XML_Error = 5;
pub const XML_ERROR_INVALID_TOKEN: XML_Error = 4;
pub const XML_ERROR_NO_ELEMENTS: XML_Error = 3;
pub const XML_ERROR_SYNTAX: XML_Error = 2;
pub const XML_ERROR_NO_MEMORY: XML_Error = 1;
pub const XML_ERROR_NONE: XML_Error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Memory_Handling_Suite {
    pub malloc_fcn: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub realloc_fcn:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
    pub free_fcn: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub type XML_StartElementHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *mut *const XML_Char) -> (),
>;
pub type XML_EndElementHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_CharacterDataHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_DefaultHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> ::core::ffi::c_int,
>;
pub type XML_ParamEntityParsing = ::core::ffi::c_uint;
pub const XML_PARAM_ENTITY_PARSING_ALWAYS: XML_ParamEntityParsing = 2;
pub const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: XML_ParamEntityParsing = 1;
pub const XML_PARAM_ENTITY_PARSING_NEVER: XML_ParamEntityParsing = 0;
pub type tcase_setup_function = Option<unsafe extern "C" fn() -> ()>;
pub type tcase_teardown_function = Option<unsafe extern "C" fn() -> ()>;
pub type tcase_test_function = Option<unsafe extern "C" fn() -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtOption {
    pub system_id: *const XML_Char,
    pub parse_text: *const ::core::ffi::c_char,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 37] = unsafe {
    ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
        *b"void test_nsalloc_parse_buffer(void)\0",
    )
};
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
pub const ALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const REALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
const NSALLOC_TESTS_FILE: &[u8] = b"/root/work/expat/tests/nsalloc_tests.c\0";

fn bytes_as_c_char_ptr(bytes: &[u8]) -> *const ::core::ffi::c_char {
    bytes.as_ptr().cast::<::core::ffi::c_char>()
}

fn ffi_call0<R>(function: unsafe extern "C" fn() -> R) -> R {
    unsafe { function() }
}

fn ffi_call1<A, R>(function: unsafe extern "C" fn(A) -> R, a: A) -> R {
    unsafe { function(a) }
}

fn ffi_call2<A, B, R>(function: unsafe extern "C" fn(A, B) -> R, a: A, b: B) -> R {
    unsafe { function(a, b) }
}

fn ffi_call3<A, B, C, R>(function: unsafe extern "C" fn(A, B, C) -> R, a: A, b: B, c: C) -> R {
    unsafe { function(a, b, c) }
}

fn ffi_call4<A, B, C, D, R>(
    function: unsafe extern "C" fn(A, B, C, D) -> R,
    a: A,
    b: B,
    c: C,
    d: D,
) -> R {
    unsafe { function(a, b, c, d) }
}

fn current_parser() -> XML_Parser {
    unsafe { g_parser }
}

fn set_current_parser(parser: XML_Parser) {
    unsafe {
        g_parser = parser;
    }
}

fn set_allocation_count(count: ::core::ffi::c_int) {
    unsafe {
        g_allocation_count = count;
    }
}

fn set_reallocation_count(count: ::core::ffi::c_int) {
    unsafe {
        g_reallocation_count = count;
    }
}

fn set_test_info(name: &[u8], line: ::core::ffi::c_int) {
    ffi_call3(
        _check_set_test_info,
        bytes_as_c_char_ptr(name),
        bytes_as_c_char_ptr(NSALLOC_TESTS_FILE),
        line,
    );
}

fn fail_test(line: ::core::ffi::c_int, msg: &[u8]) -> ! {
    ffi_call3(
        _fail,
        bytes_as_c_char_ptr(NSALLOC_TESTS_FILE),
        line,
        bytes_as_c_char_ptr(msg),
    )
}

fn c_string_len(text: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    ffi_call1(strlen, text) as ::core::ffi::c_int
}

fn reset_nsalloc_fixture() {
    nsalloc_teardown();
    nsalloc_setup();
}

extern "C" fn nsalloc_setup() {
    let mut memsuite: XML_Memory_Handling_Suite = XML_Memory_Handling_Suite {
        malloc_fcn: Some(duff_allocator),
        realloc_fcn: Some(duff_reallocator),
        free_fcn: Some(free),
    };
    let mut ns_sep: [XML_Char; 2] = [' ' as i32 as XML_Char, '\0' as i32 as XML_Char];
    set_allocation_count(ALLOC_ALWAYS_SUCCEED);
    set_reallocation_count(REALLOC_ALWAYS_SUCCEED);
    set_current_parser(ffi_call3(
        XML_ParserCreate_MM,
        ::core::ptr::null::<XML_Char>(),
        &raw mut memsuite as *mut XML_Memory_Handling_Suite as *const XML_Memory_Handling_Suite,
        &raw mut ns_sep as *mut XML_Char as *const XML_Char,
    ));
    if current_parser().is_null() {
        fail_test(67 as ::core::ffi::c_int, b"Parser not created\0");
    }
}

extern "C" fn nsalloc_teardown() {
    ffi_call0(basic_teardown);
}
unsafe extern "C" fn test_nsalloc_xmlns() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_xmlns\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            78 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc xmlns='http://example.org/'>\n  <e xmlns=''/>\n</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_uint = 0;
        let max_alloc_count: ::core::ffi::c_uint = 30 as ::core::ffi::c_uint;
        i = 0 as ::core::ffi::c_uint;
        while i < max_alloc_count {
            g_allocation_count = i as ::core::ffi::c_int;
            XML_SetDefaultHandler(
                g_parser,
                Some(
                    dummy_default_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i = i.wrapping_add(1);
        }
        if i == 0 as ::core::ffi::c_uint {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                101 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_int,
                b"Parsing failed even at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_parse_buffer() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_parse_buffer\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            108 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc>Hello</doc>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        if XML_ParseBuffer(
            g_parser,
            0 as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                115 as ::core::ffi::c_int,
                b"Pre-init XML_ParseBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_NO_BUFFER as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_int,
                b"Pre-init XML_ParseBuffer faulted for wrong reason\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        buffer = XML_GetBuffer(g_parser, 1 as ::core::ffi::c_int);
        if buffer.is_null() {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                121 as ::core::ffi::c_int,
                b"Could not acquire parse buffer\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        g_allocation_count = 0 as ::core::ffi::c_int;
        if XML_ParseBuffer(
            g_parser,
            0 as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                125 as ::core::ffi::c_int,
                b"Pre-init XML_ParseBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_NO_MEMORY as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                127 as ::core::ffi::c_int,
                b"Pre-init XML_ParseBuffer faulted for wrong reason\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        g_allocation_count = ALLOC_ALWAYS_SUCCEED;
        if XML_ParseBuffer(
            g_parser,
            0 as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_int,
            );
        }
        if XML_ResumeParser(g_parser) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                136 as ::core::ffi::c_int,
                b"Resuming unsuspended parser not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_NOT_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                138 as ::core::ffi::c_int,
            );
        }
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                clearing_aborting_character_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        g_resumable = XML_TRUE;
        buffer = XML_GetBuffer(g_parser, strlen(text) as ::core::ffi::c_int);
        if buffer.is_null() {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                145 as ::core::ffi::c_int,
                b"Could not acquire parse buffer\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !buffer.is_null() {
        } else {
            __assert_fail(
                b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                146 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        };
        memcpy(buffer, text as *const ::core::ffi::c_void, strlen(text));
        if XML_ParseBuffer(
            g_parser,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                150 as ::core::ffi::c_int,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                152 as ::core::ffi::c_int,
            );
        }
        if XML_ParseBuffer(
            g_parser,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                155 as ::core::ffi::c_int,
                b"Suspended XML_ParseBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                157 as ::core::ffi::c_int,
            );
        }
        if !XML_GetBuffer(g_parser, strlen(text) as ::core::ffi::c_int).is_null() {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                159 as ::core::ffi::c_int,
                b"Suspended XML_GetBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_SetCharacterDataHandler(g_parser, None);
        if XML_ResumeParser(g_parser) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                164 as ::core::ffi::c_int,
            );
        }
        if XML_ParseBuffer(
            g_parser,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                167 as ::core::ffi::c_int,
                b"Post-finishing XML_ParseBuffer not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                169 as ::core::ffi::c_int,
            );
        }
        if !XML_GetBuffer(g_parser, strlen(text) as ::core::ffi::c_int).is_null() {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_int,
                b"Post-finishing XML_GetBuffer not faulted\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_long_prefix() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_long_prefix\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            176 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                247 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_long_uri() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_long_uri\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            252 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/' bar:a='12'\nxmlns:bar='http://example.org/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/'></foo:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                307 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_long_attr() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_long_attr\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            312 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/' bar:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='12'\nxmlns:bar='http://example.org/'></foo:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                348 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                350 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_long_attr_prefix() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_long_attr_prefix\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            355 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/' ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:a='12'\nxmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'></foo:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut elemstr: [*const XML_Char; 2] = [
            b"http://example.org/ e foo\0".as_ptr() as *const ::core::ffi::c_char,
            b"http://example.org/ a ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ\0"
                .as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetReturnNSTriplet(g_parser, XML_TRUE as ::core::ffi::c_int);
            XML_SetUserData(
                g_parser,
                &raw mut elemstr as *mut *const XML_Char as *mut ::core::ffi::c_void,
            );
            XML_SetElementHandler(
                g_parser,
                Some(
                    triplet_start_checker
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut *const XML_Char,
                        ) -> (),
                ),
                Some(
                    triplet_end_checker
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
                ),
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                434 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                436 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_realloc_attributes() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_realloc_attributes\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            441 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/' bar:a='12'\n       xmlns:bar='http://example.org/'></foo:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if !(i == 0 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                459 as ::core::ffi::c_int,
                b"check failed: i == 0\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_long_element() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_long_element\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            470 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<foo:thisisalongenoughelementnametotriggerareallocation\n xmlns:foo='http://example.org/' bar:a='12'\n xmlns:bar='http://example.org/'></foo:thisisalongenoughelementnametotriggerareallocation>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut elemstr: [*const XML_Char; 2] = [
            b"http://example.org/ thisisalongenoughelementnametotriggerareallocation foo\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"http://example.org/ a bar\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetReturnNSTriplet(g_parser, XML_TRUE as ::core::ffi::c_int);
            XML_SetUserData(
                g_parser,
                &raw mut elemstr as *mut *const XML_Char as *mut ::core::ffi::c_void,
            );
            XML_SetElementHandler(
                g_parser,
                Some(
                    triplet_start_checker
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut *const XML_Char,
                        ) -> (),
                ),
                Some(
                    triplet_end_checker
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
                ),
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                496 as ::core::ffi::c_int,
                b"Parsing worked despite failing reallocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                498 as ::core::ffi::c_int,
                b"Parsing failed at max reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_realloc_binding_uri() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_realloc_binding_uri\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            512 as ::core::ffi::c_int,
        );
        let mut first: *const ::core::ffi::c_char =
            b"<doc xmlns='http://example.org/'>\n  <e xmlns='' />\n</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut second: *const ::core::ffi::c_char = b"<doc xmlns='http://example.org/long/enough/URI/to/reallocate/'>\n  <e xmlns='' />\n</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_uint = 0;
        let max_realloc_count: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            first,
            strlen(first) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                526 as ::core::ffi::c_int,
            );
        }
        i = 0 as ::core::ffi::c_uint;
        while i < max_realloc_count {
            XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
            g_reallocation_count = i as ::core::ffi::c_int;
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                second,
                strlen(second) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == 0 as ::core::ffi::c_uint {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                537 as ::core::ffi::c_int,
                b"Parsing worked despite failing reallocation\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                539 as ::core::ffi::c_int,
                b"Parsing failed at max reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_realloc_long_prefix() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_realloc_long_prefix\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            544 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                613 as ::core::ffi::c_int,
                b"Parsing worked despite failing reallocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                615 as ::core::ffi::c_int,
                b"Parsing failed even at max reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_realloc_longer_prefix() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_realloc_longer_prefix\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            620 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ='http://example.org/'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ:foo>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                689 as ::core::ffi::c_int,
                b"Parsing worked despite failing reallocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                691 as ::core::ffi::c_int,
                b"Parsing failed even at max reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_long_namespace() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_long_namespace\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            695 as ::core::ffi::c_int,
        );
        let mut text1: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:e xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut text2: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:f ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:attr='foo'/>\n</ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text1,
                strlen(text1) as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                && _XML_Parse_SINGLE_BYTES(
                    g_parser,
                    text2,
                    strlen(text2) as ::core::ffi::c_int,
                    XML_TRUE as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint
                    != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                803 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                805 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_less_long_namespace() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_less_long_namespace\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            812 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:e xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678='http://example.org/'>\n<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:f ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:att='foo'/>\n</ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                876 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                878 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_long_context() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_long_context\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            882 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ATTLIST doc baz ID #REQUIRED>\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKL' baz='2'>\n&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut options: [ExtOption; 3] = [
            ExtOption {
                system_id: b"foo\0".as_ptr() as *const XML_Char,
                parse_text: b"<!ELEMENT e EMPTY>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: b"bar\0".as_ptr() as *const XML_Char,
                parse_text: b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: ::core::ptr::null::<XML_Char>(),
                parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut options as *mut ExtOption as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_optioner
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                928 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                930 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
fn context_realloc_test(text: *const ::core::ffi::c_char) {
    let mut options: [ExtOption; 3] = [
        ExtOption {
            system_id: b"foo\0".as_ptr() as *const XML_Char,
            parse_text: b"<!ELEMENT e EMPTY>\0".as_ptr() as *const ::core::ffi::c_char,
        },
        ExtOption {
            system_id: b"bar\0".as_ptr() as *const XML_Char,
            parse_text: b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
        },
        ExtOption {
            system_id: ::core::ptr::null::<XML_Char>(),
            parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
        },
    ];
    let max_realloc_count: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < max_realloc_count {
        set_reallocation_count(i);
        ffi_call2(
            XML_SetUserData,
            current_parser(),
            &raw mut options as *mut ExtOption as *mut ::core::ffi::c_void,
        );
        ffi_call2(
            XML_SetParamEntityParsing,
            current_parser(),
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
        ffi_call2(
            XML_SetExternalEntityRefHandler,
            current_parser(),
            Some(external_entity_optioner),
        );
        if ffi_call4(
            _XML_Parse_SINGLE_BYTES,
            current_parser(),
            text,
            c_string_len(text),
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        fail_test(
            957 as ::core::ffi::c_int,
            b"Parsing worked despite failing reallocations\0",
        );
    } else if i == max_realloc_count {
        fail_test(
            959 as ::core::ffi::c_int,
            b"Parsing failed even at max reallocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_realloc_long_context() {
    set_test_info(
        b"test_nsalloc_realloc_long_context\0",
        962 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKL'>\n&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}
extern "C" fn test_nsalloc_realloc_long_context_2() {
    set_test_info(
        b"test_nsalloc_realloc_long_context_2\0",
        993 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJK'>\n&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}
extern "C" fn test_nsalloc_realloc_long_context_3() {
    set_test_info(
        b"test_nsalloc_realloc_long_context_3\0",
        1024 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGH'>\n&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}
extern "C" fn test_nsalloc_realloc_long_context_4() {
    set_test_info(
        b"test_nsalloc_realloc_long_context_4\0",
        1055 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO'>\n&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}
extern "C" fn test_nsalloc_realloc_long_context_5() {
    set_test_info(
        b"test_nsalloc_realloc_long_context_5\0",
        1086 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABC'>\n&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}
extern "C" fn test_nsalloc_realloc_long_context_6() {
    set_test_info(
        b"test_nsalloc_realloc_long_context_6\0",
        1117 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOP'>\n&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}
extern "C" fn test_nsalloc_realloc_long_context_7() {
    set_test_info(
        b"test_nsalloc_realloc_long_context_7\0",
        1147 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLM'>\n&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}
unsafe extern "C" fn test_nsalloc_realloc_long_ge_name() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_realloc_long_ge_name\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1178 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/baz'>\n&ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut options: [ExtOption; 3] = [
            ExtOption {
                system_id: b"foo\0".as_ptr() as *const XML_Char,
                parse_text: b"<!ELEMENT el EMPTY>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: b"bar\0".as_ptr() as *const XML_Char,
                parse_text: b"<el/>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: ::core::ptr::null::<XML_Char>(),
                parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut options as *mut ExtOption as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_optioner
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1240 as ::core::ffi::c_int,
                b"Parsing worked despite failing reallocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1242 as ::core::ffi::c_int,
                b"Parsing failed even at max reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_realloc_long_context_in_dtd() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_realloc_long_context_in_dtd\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1251 as ::core::ffi::c_int,
        );
        let mut text1: *const ::core::ffi::c_char = b"<!DOCTYPE ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:doc [\n  <!ENTITY First SYSTEM 'foo/First'>\n]>\n<ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:doc xmlns:ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP='foo/Second'>&First;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut text2: *const ::core::ffi::c_char = b"</ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut options: [ExtOption; 2] = [
            ExtOption {
                system_id: b"foo/First\0".as_ptr() as *const XML_Char,
                parse_text: b"Hello world\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: ::core::ptr::null::<XML_Char>(),
                parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut options as *mut ExtOption as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_optioner
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text1,
                strlen(text1) as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                && _XML_Parse_SINGLE_BYTES(
                    g_parser,
                    text2,
                    strlen(text2) as ::core::ffi::c_int,
                    XML_TRUE as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint
                    != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1348 as ::core::ffi::c_int,
                b"Parsing worked despite failing reallocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1350 as ::core::ffi::c_int,
                b"Parsing failed even at max reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_long_default_in_ext() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_long_default_in_ext\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1354 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ATTLIST e a1 CDATA 'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP'>\n  <!ENTITY x SYSTEM 'foo'>\n]>\n<doc>&x;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut options: [ExtOption; 2] = [
            ExtOption {
                system_id: b"foo\0".as_ptr() as *const XML_Char,
                parse_text: b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: ::core::ptr::null::<XML_Char>(),
                parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut options as *mut ExtOption as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_optioner
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1397 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1399 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_long_systemid_in_ext() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_long_systemid_in_ext\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1403 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut options: [ExtOption; 3] = [
            ExtOption {
                system_id: b"foo\0".as_ptr() as *const XML_Char,
                parse_text: b"<!ELEMENT e EMPTY>\0".as_ptr()
                    as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: b"ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/\0"
                    .as_ptr() as *const XML_Char,
                parse_text: b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: ::core::ptr::null::<XML_Char>(),
                parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 55 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut options as *mut ExtOption as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_optioner
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1466 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1468 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_nsalloc_prefixed_element() {
    unsafe {
        _check_set_test_info(
            b"test_nsalloc_prefixed_element\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1475 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE pfx:element SYSTEM 'foo' [\n  <!ATTLIST pfx:element baz ID #REQUIRED>\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<pfx:element xmlns:pfx='http://example.org/' baz='2'>\n&en;</pfx:element>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut options: [ExtOption; 3] = [
            ExtOption {
                system_id: b"foo\0".as_ptr() as *const XML_Char,
                parse_text: b"<!ELEMENT e EMPTY>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: b"bar\0".as_ptr() as *const XML_Char,
                parse_text: b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: ::core::ptr::null::<XML_Char>(),
                parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 70 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut options as *mut ExtOption as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_optioner
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            nsalloc_teardown();
            nsalloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1502 as ::core::ffi::c_int,
                b"Success despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/nsalloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1504 as ::core::ffi::c_int,
                b"Failed even at full allocation count\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_nsalloc_test_case(mut s: *mut Suite) {
    unsafe {
        let mut tc_nsalloc: *mut TCase =
            tcase_create(b"namespace allocation tests\0".as_ptr() as *const ::core::ffi::c_char);
        suite_add_tcase(s, tc_nsalloc);
        tcase_add_checked_fixture(
            tc_nsalloc,
            Some(nsalloc_setup as unsafe extern "C" fn() -> ()),
            Some(nsalloc_teardown as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_xmlns as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_parse_buffer as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_long_prefix as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_long_uri as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_long_attr as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_long_attr_prefix as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_realloc_attributes as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_long_element as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_realloc_binding_uri as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_prefix as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_realloc_longer_prefix as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_long_namespace as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_less_long_namespace as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_long_context as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_context as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_context_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_context_3 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_context_4 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_context_5 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_context_6 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_context_7 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_ge_name as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_nsalloc,
            Some(test_nsalloc_realloc_long_context_in_dtd as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_nsalloc,
            Some(test_nsalloc_long_default_in_ext as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_long_systemid_in_ext as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_nsalloc,
            Some(test_nsalloc_prefixed_element as unsafe extern "C" fn() -> ()),
        );
    }
}
