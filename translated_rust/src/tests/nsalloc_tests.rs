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

fn copy_memory(dest: *mut ::core::ffi::c_void, src: *const ::core::ffi::c_void, len: size_t) {
    ffi_call3(memcpy, dest, src, len);
}

fn parse_single_bytes(parser: XML_Parser, text: *const ::core::ffi::c_char) -> XML_Status {
    ffi_call4(
        _XML_Parse_SINGLE_BYTES,
        parser,
        text,
        c_string_len(text),
        XML_TRUE as ::core::ffi::c_int,
    )
}

fn parse_single_bytes_with_final(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    ffi_call4(
        _XML_Parse_SINGLE_BYTES,
        parser,
        text,
        c_string_len(text),
        is_final,
    )
}

fn parse_buffer(
    parser: XML_Parser,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    ffi_call3(XML_ParseBuffer, parser, len, is_final)
}

fn parser_resume(parser: XML_Parser) -> XML_Status {
    ffi_call1(XML_ResumeParser, parser)
}

fn parser_error_code(parser: XML_Parser) -> XML_Error {
    ffi_call1(XML_GetErrorCode, parser)
}

fn parser_reset(parser: XML_Parser, encoding: *const XML_Char) -> XML_Bool {
    ffi_call2(XML_ParserReset, parser, encoding)
}

fn parser_get_buffer(parser: XML_Parser, len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    ffi_call2(XML_GetBuffer, parser, len)
}

fn parser_set_default_handler(parser: XML_Parser, handler: XML_DefaultHandler) {
    ffi_call2(XML_SetDefaultHandler, parser, handler);
}

fn parser_set_character_data_handler(parser: XML_Parser, handler: XML_CharacterDataHandler) {
    ffi_call2(XML_SetCharacterDataHandler, parser, handler);
}

fn parser_set_return_ns_triplet(parser: XML_Parser, enabled: ::core::ffi::c_int) {
    ffi_call2(XML_SetReturnNSTriplet, parser, enabled);
}

fn parser_set_user_data(parser: XML_Parser, user_data: *mut ::core::ffi::c_void) {
    ffi_call2(XML_SetUserData, parser, user_data);
}

fn parser_set_param_entity_parsing(
    parser: XML_Parser,
    parsing: XML_ParamEntityParsing,
) -> ::core::ffi::c_int {
    ffi_call2(XML_SetParamEntityParsing, parser, parsing)
}

fn parser_set_external_entity_ref_handler(
    parser: XML_Parser,
    handler: XML_ExternalEntityRefHandler,
) {
    ffi_call2(XML_SetExternalEntityRefHandler, parser, handler);
}

fn parser_configure_external_entity_loader(parser: XML_Parser, options: &mut [ExtOption]) {
    parser_set_user_data(parser, options.as_mut_ptr().cast());
    parser_set_param_entity_parsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
    parser_set_external_entity_ref_handler(parser, Some(external_entity_optioner));
}

fn parser_set_element_handler(
    parser: XML_Parser,
    start: XML_StartElementHandler,
    end: XML_EndElementHandler,
) {
    ffi_call3(XML_SetElementHandler, parser, start, end);
}

fn assert_buffer_not_null(buffer: *mut ::core::ffi::c_void, line: ::core::ffi::c_uint) {
    if buffer.is_null() {
        ffi_call4(
            __assert_fail,
            bytes_as_c_char_ptr(b"buffer != NULL\0"),
            bytes_as_c_char_ptr(NSALLOC_TESTS_FILE),
            line,
            bytes_as_c_char_ptr(b"void test_nsalloc_parse_buffer(void)\0"),
        );
    }
}

fn xml_failure(line: ::core::ffi::c_int) {
    ffi_call3(
        _xml_failure,
        current_parser(),
        bytes_as_c_char_ptr(NSALLOC_TESTS_FILE),
        line,
    );
}

fn set_resumable(resumable: XML_Bool) {
    unsafe {
        g_resumable = resumable;
    }
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
extern "C" fn test_nsalloc_xmlns() {
    set_test_info(b"test_nsalloc_xmlns\0", 78 as ::core::ffi::c_int);
    let text = b"<doc xmlns='http://example.org/'>\n  <e xmlns=''/>\n</doc>\0".as_ptr()
        as *const ::core::ffi::c_char;
    let max_alloc_count: ::core::ffi::c_uint = 30 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint = 0;

    while i < max_alloc_count {
        set_allocation_count(i as ::core::ffi::c_int);
        parser_set_default_handler(current_parser(), Some(dummy_default_handler));
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i = i.wrapping_add(1);
    }

    if i == 0 {
        fail_test(
            101 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            103 as ::core::ffi::c_int,
            b"Parsing failed even at maximum allocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_parse_buffer() {
    set_test_info(b"test_nsalloc_parse_buffer\0", 108 as ::core::ffi::c_int);
    let text = b"<doc>Hello</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut buffer = ::core::ptr::null_mut::<::core::ffi::c_void>();

    if parse_buffer(
        current_parser(),
        0 as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            115 as ::core::ffi::c_int,
            b"Pre-init XML_ParseBuffer not faulted\0",
        );
    }
    if parser_error_code(current_parser()) as ::core::ffi::c_uint
        != XML_ERROR_NO_BUFFER as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            117 as ::core::ffi::c_int,
            b"Pre-init XML_ParseBuffer faulted for wrong reason\0",
        );
    }

    buffer = parser_get_buffer(current_parser(), 1 as ::core::ffi::c_int);
    if buffer.is_null() {
        fail_test(
            121 as ::core::ffi::c_int,
            b"Could not acquire parse buffer\0",
        );
    }

    set_allocation_count(0 as ::core::ffi::c_int);
    if parse_buffer(
        current_parser(),
        0 as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            125 as ::core::ffi::c_int,
            b"Pre-init XML_ParseBuffer not faulted\0",
        );
    }
    if parser_error_code(current_parser()) as ::core::ffi::c_uint
        != XML_ERROR_NO_MEMORY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            127 as ::core::ffi::c_int,
            b"Pre-init XML_ParseBuffer faulted for wrong reason\0",
        );
    }

    set_allocation_count(ALLOC_ALWAYS_SUCCEED);
    if parse_buffer(
        current_parser(),
        0 as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(132 as ::core::ffi::c_int);
    }
    if parser_resume(current_parser()) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            136 as ::core::ffi::c_int,
            b"Resuming unsuspended parser not faulted\0",
        );
    }
    if parser_error_code(current_parser()) as ::core::ffi::c_uint
        != XML_ERROR_NOT_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(138 as ::core::ffi::c_int);
    }

    parser_set_character_data_handler(current_parser(), Some(clearing_aborting_character_handler));
    set_resumable(XML_TRUE);
    buffer = parser_get_buffer(current_parser(), c_string_len(text));
    if buffer.is_null() {
        fail_test(
            145 as ::core::ffi::c_int,
            b"Could not acquire parse buffer\0",
        );
    }
    assert_buffer_not_null(buffer, 146 as ::core::ffi::c_uint);
    copy_memory(
        buffer,
        text as *const ::core::ffi::c_void,
        c_string_len(text) as size_t,
    );

    if parse_buffer(
        current_parser(),
        c_string_len(text),
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(150 as ::core::ffi::c_int);
    }
    if parser_error_code(current_parser()) as ::core::ffi::c_uint
        != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(152 as ::core::ffi::c_int);
    }
    if parse_buffer(
        current_parser(),
        c_string_len(text),
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            155 as ::core::ffi::c_int,
            b"Suspended XML_ParseBuffer not faulted\0",
        );
    }
    if parser_error_code(current_parser()) as ::core::ffi::c_uint
        != XML_ERROR_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(157 as ::core::ffi::c_int);
    }
    if !parser_get_buffer(current_parser(), c_string_len(text)).is_null() {
        fail_test(
            159 as ::core::ffi::c_int,
            b"Suspended XML_GetBuffer not faulted\0",
        );
    }

    parser_set_character_data_handler(current_parser(), None);
    if parser_resume(current_parser()) as ::core::ffi::c_uint
        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(164 as ::core::ffi::c_int);
    }
    if parse_buffer(
        current_parser(),
        c_string_len(text),
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            167 as ::core::ffi::c_int,
            b"Post-finishing XML_ParseBuffer not faulted\0",
        );
    }
    if parser_error_code(current_parser()) as ::core::ffi::c_uint
        != XML_ERROR_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(169 as ::core::ffi::c_int);
    }
    if !parser_get_buffer(current_parser(), c_string_len(text)).is_null() {
        fail_test(
            171 as ::core::ffi::c_int,
            b"Post-finishing XML_GetBuffer not faulted\0",
        );
    }
}
extern "C" fn test_nsalloc_long_prefix() {
    set_test_info(b"test_nsalloc_long_prefix\0", 176 as ::core::ffi::c_int);
    let text = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;

    while i < max_alloc_count {
        set_allocation_count(i);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }

    if i == 0 {
        fail_test(
            245 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            247 as ::core::ffi::c_int,
            b"Parsing failed even at max allocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_long_uri() {
    set_test_info(b"test_nsalloc_long_uri\0", 252 as ::core::ffi::c_int);
    let text = b"<foo:e xmlns:foo='http://example.org/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/' bar:a='12'\nxmlns:bar='http://example.org/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/'></foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;

    while i < max_alloc_count {
        set_allocation_count(i);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }

    if i == 0 {
        fail_test(
            305 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            307 as ::core::ffi::c_int,
            b"Parsing failed even at max allocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_long_attr() {
    set_test_info(b"test_nsalloc_long_attr\0", 312 as ::core::ffi::c_int);
    let text = b"<foo:e xmlns:foo='http://example.org/' bar:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='12'\nxmlns:bar='http://example.org/'></foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;

    while i < max_alloc_count {
        set_allocation_count(i);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }

    if i == 0 {
        fail_test(
            348 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            350 as ::core::ffi::c_int,
            b"Parsing failed even at max allocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_long_attr_prefix() {
    set_test_info(
        b"test_nsalloc_long_attr_prefix\0",
        355 as ::core::ffi::c_int,
    );
    let text = b"<foo:e xmlns:foo='http://example.org/' ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:a='12'\nxmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'></foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ e foo\0".as_ptr() as *const ::core::ffi::c_char,
        b"http://example.org/ a ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ];
    let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;

    while i < max_alloc_count {
        set_allocation_count(i);
        parser_set_return_ns_triplet(current_parser(), XML_TRUE as ::core::ffi::c_int);
        parser_set_user_data(
            current_parser(),
            &raw mut elemstr as *mut *const XML_Char as *mut ::core::ffi::c_void,
        );
        parser_set_element_handler(
            current_parser(),
            Some(triplet_start_checker),
            Some(triplet_end_checker),
        );
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }

    if i == 0 {
        fail_test(
            434 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            436 as ::core::ffi::c_int,
            b"Parsing failed even at max allocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_realloc_attributes() {
    set_test_info(
        b"test_nsalloc_realloc_attributes\0",
        441 as ::core::ffi::c_int,
    );
    let text = b"<foo:e xmlns:foo='http://example.org/' bar:a='12'\n       xmlns:bar='http://example.org/'></foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;

    while i < max_realloc_count {
        set_reallocation_count(i);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }

    if i != 0 {
        fail_test(459 as ::core::ffi::c_int, b"check failed: i == 0\0");
    }
}
extern "C" fn test_nsalloc_long_element() {
    set_test_info(b"test_nsalloc_long_element\0", 470 as ::core::ffi::c_int);
    let text = b"<foo:thisisalongenoughelementnametotriggerareallocation\n xmlns:foo='http://example.org/' bar:a='12'\n xmlns:bar='http://example.org/'></foo:thisisalongenoughelementnametotriggerareallocation>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ thisisalongenoughelementnametotriggerareallocation foo\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"http://example.org/ a bar\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let max_alloc_count: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;

    while i < max_alloc_count {
        set_allocation_count(i);
        parser_set_return_ns_triplet(current_parser(), XML_TRUE as ::core::ffi::c_int);
        parser_set_user_data(
            current_parser(),
            &raw mut elemstr as *mut *const XML_Char as *mut ::core::ffi::c_void,
        );
        parser_set_element_handler(
            current_parser(),
            Some(triplet_start_checker),
            Some(triplet_end_checker),
        );
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }

    if i == 0 {
        fail_test(
            496 as ::core::ffi::c_int,
            b"Parsing worked despite failing reallocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            498 as ::core::ffi::c_int,
            b"Parsing failed at max reallocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_realloc_binding_uri() {
    set_test_info(
        b"test_nsalloc_realloc_binding_uri\0",
        512 as ::core::ffi::c_int,
    );
    let mut first: *const ::core::ffi::c_char =
        b"<doc xmlns='http://example.org/'>\n  <e xmlns='' />\n</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut second: *const ::core::ffi::c_char = b"<doc xmlns='http://example.org/long/enough/URI/to/reallocate/'>\n  <e xmlns='' />\n</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_uint = 0;
    let max_realloc_count: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
    if parse_single_bytes(current_parser(), first) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(526 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_uint;
    while i < max_realloc_count {
        parser_reset(current_parser(), ::core::ptr::null::<XML_Char>());
        set_reallocation_count(i as ::core::ffi::c_int);
        if parse_single_bytes(current_parser(), second) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i == 0 as ::core::ffi::c_uint {
        fail_test(
            537 as ::core::ffi::c_int,
            b"Parsing worked despite failing reallocation\0",
        );
    } else if i == max_realloc_count {
        fail_test(
            539 as ::core::ffi::c_int,
            b"Parsing failed at max reallocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_realloc_long_prefix() {
    set_test_info(
        b"test_nsalloc_realloc_long_prefix\0",
        544 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < max_realloc_count {
        set_reallocation_count(i);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        fail_test(
            613 as ::core::ffi::c_int,
            b"Parsing worked despite failing reallocations\0",
        );
    } else if i == max_realloc_count {
        fail_test(
            615 as ::core::ffi::c_int,
            b"Parsing failed even at max reallocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_realloc_longer_prefix() {
    set_test_info(
        b"test_nsalloc_realloc_longer_prefix\0",
        620 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ='http://example.org/'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ:foo>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < max_realloc_count {
        set_reallocation_count(i);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        fail_test(
            689 as ::core::ffi::c_int,
            b"Parsing worked despite failing reallocations\0",
        );
    } else if i == max_realloc_count {
        fail_test(
            691 as ::core::ffi::c_int,
            b"Parsing failed even at max reallocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_long_namespace() {
    set_test_info(b"test_nsalloc_long_namespace\0", 695 as ::core::ffi::c_int);
    let mut text1: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:e xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:f ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:attr='foo'/>\n</ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < max_alloc_count {
        set_allocation_count(i);
        if parse_single_bytes_with_final(current_parser(), text1, XML_FALSE as ::core::ffi::c_int)
            as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            && parse_single_bytes_with_final(
                current_parser(),
                text2,
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
            803 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            805 as ::core::ffi::c_int,
            b"Parsing failed even at max allocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_less_long_namespace() {
    set_test_info(
        b"test_nsalloc_less_long_namespace\0",
        812 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:e xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678='http://example.org/'>\n<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:f ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:att='foo'/>\n</ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < max_alloc_count {
        set_allocation_count(i);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        fail_test(
            876 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            878 as ::core::ffi::c_int,
            b"Parsing failed even at max allocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_long_context() {
    set_test_info(b"test_nsalloc_long_context\0", 882 as ::core::ffi::c_int);
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
        set_allocation_count(i);
        parser_configure_external_entity_loader(current_parser(), &mut options);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        fail_test(
            928 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            930 as ::core::ffi::c_int,
            b"Parsing failed even at max allocation count\0",
        );
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
extern "C" fn test_nsalloc_realloc_long_ge_name() {
    set_test_info(
        b"test_nsalloc_realloc_long_ge_name\0",
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
        set_reallocation_count(i);
        parser_configure_external_entity_loader(current_parser(), &mut options);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        fail_test(
            1240 as ::core::ffi::c_int,
            b"Parsing worked despite failing reallocations\0",
        );
    } else if i == max_realloc_count {
        fail_test(
            1242 as ::core::ffi::c_int,
            b"Parsing failed even at max reallocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_realloc_long_context_in_dtd() {
    set_test_info(
        b"test_nsalloc_realloc_long_context_in_dtd\0",
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
        set_reallocation_count(i);
        parser_configure_external_entity_loader(current_parser(), &mut options);
        if parse_single_bytes_with_final(current_parser(), text1, XML_FALSE as ::core::ffi::c_int)
            as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            && parse_single_bytes_with_final(
                current_parser(),
                text2,
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
            1348 as ::core::ffi::c_int,
            b"Parsing worked despite failing reallocations\0",
        );
    } else if i == max_realloc_count {
        fail_test(
            1350 as ::core::ffi::c_int,
            b"Parsing failed even at max reallocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_long_default_in_ext() {
    set_test_info(
        b"test_nsalloc_long_default_in_ext\0",
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
        set_allocation_count(i);
        parser_configure_external_entity_loader(current_parser(), &mut options);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        fail_test(
            1397 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            1399 as ::core::ffi::c_int,
            b"Parsing failed even at max allocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_long_systemid_in_ext() {
    set_test_info(
        b"test_nsalloc_long_systemid_in_ext\0",
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
        set_allocation_count(i);
        parser_configure_external_entity_loader(current_parser(), &mut options);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        fail_test(
            1466 as ::core::ffi::c_int,
            b"Parsing worked despite failing allocations\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            1468 as ::core::ffi::c_int,
            b"Parsing failed even at max allocation count\0",
        );
    }
}
extern "C" fn test_nsalloc_prefixed_element() {
    set_test_info(
        b"test_nsalloc_prefixed_element\0",
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
        set_allocation_count(i);
        parser_configure_external_entity_loader(current_parser(), &mut options);
        if parse_single_bytes(current_parser(), text) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        reset_nsalloc_fixture();
        i += 1;
    }
    if i == 0 as ::core::ffi::c_int {
        fail_test(
            1502 as ::core::ffi::c_int,
            b"Success despite failing allocator\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            1504 as ::core::ffi::c_int,
            b"Failed even at full allocation count\0",
        );
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
