use crate::src::tests::handlers::{set_triplet_flags, triplet_flags};
use crate::src::tests::runtests::{
    current_test_parser, set_current_test_parser, take_current_test_parser,
};

extern "C" {
    pub type XML_ParserStruct;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XML_ParserCreateNS(encoding: *const XML_Char, namespaceSeparator: XML_Char) -> XML_Parser;
    fn XML_ParserReset(parser: XML_Parser, encoding: *const XML_Char) -> XML_Bool;
    fn XML_SetElementHandler(
        parser: XML_Parser,
        start: XML_StartElementHandler,
        end: XML_EndElementHandler,
    );
    fn XML_SetStartElementHandler(parser: XML_Parser, handler: XML_StartElementHandler);
    fn XML_SetEndElementHandler(parser: XML_Parser, handler: XML_EndElementHandler);
    fn XML_SetNamespaceDeclHandler(
        parser: XML_Parser,
        start: XML_StartNamespaceDeclHandler,
        end: XML_EndNamespaceDeclHandler,
    );
    fn XML_SetStartNamespaceDeclHandler(parser: XML_Parser, start: XML_StartNamespaceDeclHandler);
    fn XML_SetEndNamespaceDeclHandler(parser: XML_Parser, end: XML_EndNamespaceDeclHandler);
    fn XML_SetExternalEntityRefHandler(parser: XML_Parser, handler: XML_ExternalEntityRefHandler);
    fn XML_SetUnknownEncodingHandler(
        parser: XML_Parser,
        handler: XML_UnknownEncodingHandler,
        encodingHandlerData: *mut ::core::ffi::c_void,
    );
    fn XML_SetReturnNSTriplet(parser: XML_Parser, do_nst: ::core::ffi::c_int);
    fn XML_SetUserData(parser: XML_Parser, userData: *mut ::core::ffi::c_void);
    fn XML_UseParserAsHandlerArg(parser: XML_Parser);
    fn XML_GetParsingStatus(parser: XML_Parser, status: *mut XML_ParsingStatus);
    fn XML_SetParamEntityParsing(
        parser: XML_Parser,
        parsing: XML_ParamEntityParsing,
    ) -> ::core::ffi::c_int;
    fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
    fn XML_ParserFree(parser: XML_Parser);
    fn set_subtest(fmt: *const ::core::ffi::c_char, ...);
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
    fn CharData_Init(storage: *mut CharData);
    fn CharData_CheckXMLChars(storage: *mut CharData, s: *const XML_Char) -> ::core::ffi::c_int;
    fn tcase_add_test__ifdef_xml_dtd(tc: *mut TCase, test: tcase_test_function);
    fn tcase_add_test__if_xml_ge(tc: *mut TCase, test: tcase_test_function);
    fn _xml_failure(parser: XML_Parser, file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);
    fn _XML_Parse_SINGLE_BYTES(
        parser: XML_Parser,
        s: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn _expect_failure(
        text: *const ::core::ffi::c_char,
        errorCode: XML_Error,
        errorMessage: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        lineno: ::core::ffi::c_int,
    );
    fn _run_character_check(
        text: *const ::core::ffi::c_char,
        expected: *const XML_Char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    );
    fn init_dummy_handlers();
    fn get_dummy_handler_flags() -> ::core::ffi::c_ulong;
    fn dummy_start_element(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn dummy_end_element(userData: *mut ::core::ffi::c_void, name: *const XML_Char);
    fn dummy_start_namespace_decl_handler(
        userData: *mut ::core::ffi::c_void,
        prefix: *const XML_Char,
        uri: *const XML_Char,
    );
    fn dummy_end_namespace_decl_handler(
        userData: *mut ::core::ffi::c_void,
        prefix: *const XML_Char,
    );
    fn start_element_event_handler(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn triplet_start_checker(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn triplet_end_checker(userData: *mut ::core::ffi::c_void, name: *const XML_Char);
    fn overwrite_start_checker(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn overwrite_end_checker(userData: *mut ::core::ffi::c_void, name: *const XML_Char);
    fn start_element_fail(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn start_ns_clearing_start_element(
        userData: *mut ::core::ffi::c_void,
        prefix: *const XML_Char,
        uri: *const XML_Char,
    );
    fn MiscEncodingHandler(
        data: *mut ::core::ffi::c_void,
        encoding: *const XML_Char,
        info: *mut XML_Encoding,
    ) -> ::core::ffi::c_int;
    fn external_entity_handler(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn accumulate_attribute(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
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
pub type XML_StartElementHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *mut *const XML_Char) -> (),
>;
pub type XML_EndElementHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_StartNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *const XML_Char) -> ()>;
pub type XML_EndNamespaceDeclHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_ExternalEntityRefHandler = Option<
    unsafe extern "C" fn(
        XML_Parser,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Encoding {
    pub map: [::core::ffi::c_int; 256],
    pub data: *mut ::core::ffi::c_void,
    pub convert: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub release: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
pub type XML_UnknownEncodingHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *mut XML_Encoding,
    ) -> ::core::ffi::c_int,
>;
pub type XML_Parsing = ::core::ffi::c_uint;
pub const XML_SUSPENDED: XML_Parsing = 3;
pub const XML_FINISHED: XML_Parsing = 2;
pub const XML_PARSING: XML_Parsing = 1;
pub const XML_INITIALIZED: XML_Parsing = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_ParsingStatus {
    pub parsing: XML_Parsing,
    pub finalBuffer: XML_Bool,
}
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
pub struct CharData {
    pub count: ::core::ffi::c_int,
    pub data: [XML_Char; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_case {
    pub expectedStatus: XML_Status,
    pub doc: *const ::core::ffi::c_char,
    pub namesep: XML_Char,
}
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const DUMMY_START_NS_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 12 as ::core::ffi::c_int;
pub const DUMMY_END_NS_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 13 as ::core::ffi::c_int;

const TEST_FILE: &[u8] = b"/root/work/expat/tests/ns_tests.c\0";

fn c_ptr(bytes: &[u8]) -> *const ::core::ffi::c_char {
    bytes.as_ptr() as *const ::core::ffi::c_char
}

fn bool_to_xml_bool(value: bool) -> XML_Bool {
    if value {
        XML_TRUE
    } else {
        XML_FALSE
    }
}

fn bool_to_c_int(value: bool) -> ::core::ffi::c_int {
    bool_to_xml_bool(value) as ::core::ffi::c_int
}

fn global_parser() -> XML_Parser {
    current_test_parser() as XML_Parser
}

fn set_global_parser(parser: XML_Parser) {
    set_current_test_parser(parser as crate::src::tests::runtests::XML_Parser);
}

fn take_global_parser() -> XML_Parser {
    take_current_test_parser() as XML_Parser
}

fn bytes_to_c_chars<const N: usize>(bytes: [u8; N]) -> [::core::ffi::c_char; N] {
    bytes.map(|byte| byte as ::core::ffi::c_char)
}

fn check_test_info(
    function: *const ::core::ffi::c_char,
    filename: *const ::core::ffi::c_char,
    line: ::core::ffi::c_int,
) {
    unsafe {
        _check_set_test_info(function, filename, line);
    }
}

fn fail_test(
    file: *const ::core::ffi::c_char,
    line: ::core::ffi::c_int,
    message: *const ::core::ffi::c_char,
) -> ! {
    unsafe { _fail(file, line, message) }
}

fn create_parser_ns(namespace_separator: XML_Char) -> XML_Parser {
    unsafe { XML_ParserCreateNS(::core::ptr::null::<XML_Char>(), namespace_separator) }
}

fn reset_parser(parser: XML_Parser) -> XML_Bool {
    unsafe { XML_ParserReset(parser, ::core::ptr::null::<XML_Char>()) }
}

fn free_parser(parser: XML_Parser) {
    unsafe {
        XML_ParserFree(parser);
    }
}

fn namespace_teardown_safe() {
    let parser = take_global_parser();
    if !parser.is_null() {
        free_parser(parser);
    }
}

fn c_strlen(text: *const ::core::ffi::c_char) -> size_t {
    unsafe { strlen(text) }
}

fn set_return_ns_triplet(parser: XML_Parser, enabled: bool) {
    unsafe {
        XML_SetReturnNSTriplet(parser, bool_to_c_int(enabled));
    }
}

fn set_user_data(parser: XML_Parser, user_data: *mut ::core::ffi::c_void) {
    unsafe {
        XML_SetUserData(parser, user_data);
    }
}

fn set_element_handler(
    parser: XML_Parser,
    start: XML_StartElementHandler,
    end: XML_EndElementHandler,
) {
    unsafe {
        XML_SetElementHandler(parser, start, end);
    }
}

fn set_start_element_handler(parser: XML_Parser, handler: XML_StartElementHandler) {
    unsafe {
        XML_SetStartElementHandler(parser, handler);
    }
}

fn set_end_element_handler(parser: XML_Parser, handler: XML_EndElementHandler) {
    unsafe {
        XML_SetEndElementHandler(parser, handler);
    }
}

fn set_namespace_decl_handler(
    parser: XML_Parser,
    start: XML_StartNamespaceDeclHandler,
    end: XML_EndNamespaceDeclHandler,
) {
    unsafe {
        XML_SetNamespaceDeclHandler(parser, start, end);
    }
}

fn set_start_namespace_decl_handler(parser: XML_Parser, start: XML_StartNamespaceDeclHandler) {
    unsafe {
        XML_SetStartNamespaceDeclHandler(parser, start);
    }
}

fn set_end_namespace_decl_handler(parser: XML_Parser, end: XML_EndNamespaceDeclHandler) {
    unsafe {
        XML_SetEndNamespaceDeclHandler(parser, end);
    }
}

fn set_external_entity_ref_handler(parser: XML_Parser, handler: XML_ExternalEntityRefHandler) {
    unsafe {
        XML_SetExternalEntityRefHandler(parser, handler);
    }
}

fn set_unknown_encoding_handler(
    parser: XML_Parser,
    handler: XML_UnknownEncodingHandler,
    encoding_handler_data: *mut ::core::ffi::c_void,
) {
    unsafe {
        XML_SetUnknownEncodingHandler(parser, handler, encoding_handler_data);
    }
}

fn use_parser_as_handler_arg(parser: XML_Parser) {
    unsafe {
        XML_UseParserAsHandlerArg(parser);
    }
}

fn parsing_status(parser: XML_Parser) -> XML_ParsingStatus {
    let mut status = ::core::mem::MaybeUninit::<XML_ParsingStatus>::uninit();
    unsafe {
        XML_GetParsingStatus(parser, status.as_mut_ptr());
        status.assume_init()
    }
}

fn set_param_entity_parsing(
    parser: XML_Parser,
    parsing: XML_ParamEntityParsing,
) -> ::core::ffi::c_int {
    unsafe { XML_SetParamEntityParsing(parser, parsing) }
}

fn parser_error_code(parser: XML_Parser) -> XML_Error {
    unsafe { XML_GetErrorCode(parser) }
}

fn xml_failure(parser: XML_Parser, file: *const ::core::ffi::c_char, line: ::core::ffi::c_int) {
    unsafe {
        _xml_failure(parser, file, line);
    }
}

fn parse_single_bytes_len(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    unsafe { _XML_Parse_SINGLE_BYTES(parser, text, len, is_final) }
}

fn parse_single_bytes(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    parse_single_bytes_len(parser, text, c_strlen(text) as ::core::ffi::c_int, is_final)
}

fn expect_parse_ok(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
    is_final: bool,
    line: ::core::ffi::c_int,
) {
    if parse_single_bytes(parser, text, bool_to_c_int(is_final)) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_uint
    {
        xml_failure(parser, c_ptr(TEST_FILE), line);
    }
}

fn expect_parse_ok_len(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    is_final: bool,
    line: ::core::ffi::c_int,
) {
    if parse_single_bytes_len(parser, text, len, bool_to_c_int(is_final)) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_uint
    {
        xml_failure(parser, c_ptr(TEST_FILE), line);
    }
}

fn expect_failure(
    text: *const ::core::ffi::c_char,
    error_code: XML_Error,
    error_message: *const ::core::ffi::c_char,
    file: *const ::core::ffi::c_char,
    line: ::core::ffi::c_int,
) {
    unsafe {
        _expect_failure(text, error_code, error_message, file, line);
    }
}

fn run_character_check(
    text: *const ::core::ffi::c_char,
    expected: *const XML_Char,
    file: *const ::core::ffi::c_char,
    line: ::core::ffi::c_int,
) {
    unsafe {
        _run_character_check(text, expected, file, line);
    }
}

fn init_dummy_handlers_safe() {
    unsafe {
        init_dummy_handlers();
    }
}

fn dummy_handler_flags() -> ::core::ffi::c_ulong {
    unsafe { get_dummy_handler_flags() }
}

fn char_data_init(storage: &mut CharData) {
    unsafe {
        CharData_Init(storage);
    }
}

fn char_data_check_xml_chars(storage: &mut CharData, text: *const XML_Char) -> ::core::ffi::c_int {
    unsafe { CharData_CheckXMLChars(storage, text) }
}

fn set_subtest_doc(doc: *const ::core::ffi::c_char) {
    unsafe {
        set_subtest(c_ptr(b"%s\0"), doc);
    }
}

fn create_tcase(name: &[u8]) -> *mut TCase {
    unsafe { tcase_create(c_ptr(name)) }
}

fn add_suite_tcase(suite: *mut Suite, test_case: *mut TCase) {
    unsafe {
        suite_add_tcase(suite, test_case);
    }
}

fn add_checked_fixture(test_case: *mut TCase, setup: extern "C" fn(), teardown: extern "C" fn()) {
    unsafe {
        tcase_add_checked_fixture(
            test_case,
            Some(setup as unsafe extern "C" fn() -> ()),
            Some(teardown as unsafe extern "C" fn() -> ()),
        );
    }
}

fn add_test(test_case: *mut TCase, test: extern "C" fn()) {
    unsafe {
        tcase_add_test(test_case, Some(test as unsafe extern "C" fn() -> ()));
    }
}

fn add_test_ifdef_xml_dtd(test_case: *mut TCase, test: extern "C" fn()) {
    unsafe {
        tcase_add_test__ifdef_xml_dtd(test_case, Some(test as unsafe extern "C" fn() -> ()));
    }
}

fn add_test_if_xml_ge(test_case: *mut TCase, test: extern "C" fn()) {
    unsafe {
        tcase_add_test__if_xml_ge(test_case, Some(test as unsafe extern "C" fn() -> ()));
    }
}
extern "C" fn namespace_setup() {
    set_global_parser(create_parser_ns(' ' as i32 as XML_Char));
    if global_parser().is_null() {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            60 as ::core::ffi::c_int,
            b"Parser not created.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
extern "C" fn namespace_teardown() {
    namespace_teardown_safe();
}
extern "C" fn test_return_ns_triplet() {
    check_test_info(
        b"test_return_ns_triplet\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        68 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/' bar:a='12'\n       xmlns:bar='http://example.org/'>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut epilog: *const ::core::ffi::c_char =
        b"</foo:e>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ e foo\0".as_ptr() as *const ::core::ffi::c_char,
        b"http://example.org/ a bar\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    set_return_ns_triplet(global_parser(), true);
    set_user_data(
        global_parser(),
        &raw mut elemstr as *mut *const XML_Char as *mut ::core::ffi::c_void,
    );
    set_element_handler(
        global_parser(),
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
    set_namespace_decl_handler(
        global_parser(),
        Some(
            dummy_start_namespace_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
        Some(
            dummy_end_namespace_decl_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    set_triplet_flags(
        XML_FALSE as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    );
    init_dummy_handlers_safe();
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            84 as ::core::ffi::c_int,
        );
    }
    set_return_ns_triplet(global_parser(), false);
    if parse_single_bytes_len(
        global_parser(),
        epilog,
        c_strlen(epilog) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            89 as ::core::ffi::c_int,
        );
    }
    let (triplet_start_flag, triplet_end_flag) = triplet_flags();
    if triplet_start_flag == 0 {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            91 as ::core::ffi::c_int,
            b"triplet_start_checker not invoked\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if triplet_end_flag == 0 {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            93 as ::core::ffi::c_int,
            b"triplet_end_checker not invoked\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if dummy_handler_flags() != DUMMY_START_NS_DECL_HANDLER_FLAG | DUMMY_END_NS_DECL_HANDLER_FLAG {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            96 as ::core::ffi::c_int,
            b"Namespace handlers not called\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
extern "C" fn test_ns_parser_reset() {
    check_test_info(
        b"test_ns_parser_reset\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        104 as ::core::ffi::c_int,
    );
    let mut status: XML_ParsingStatus = XML_ParsingStatus {
        parsing: XML_INITIALIZED,
        finalBuffer: 0,
    };
    status = parsing_status(global_parser());
    if status.parsing as ::core::ffi::c_uint
        != XML_INITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            109 as ::core::ffi::c_int,
            b"parsing status doesn't start INITIALIZED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    test_return_ns_triplet();
    status = parsing_status(global_parser());
    if status.parsing as ::core::ffi::c_uint
        != XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            113 as ::core::ffi::c_int,
            b"parsing status doesn't end FINISHED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    reset_parser(global_parser());
    status = parsing_status(global_parser());
    if status.parsing as ::core::ffi::c_uint
        != XML_INITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            117 as ::core::ffi::c_int,
            b"parsing status doesn't reset to INITIALIZED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
fn run_ns_tagname_overwrite_test(text: *const ::core::ffi::c_char, result: *const XML_Char) {
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    char_data_init(&mut storage);
    set_user_data(
        global_parser(),
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    set_element_handler(
        global_parser(),
        Some(
            overwrite_start_checker
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
        Some(
            overwrite_end_checker
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            130 as ::core::ffi::c_int,
        );
    }
    char_data_check_xml_chars(&mut storage, result);
}
extern "C" fn test_ns_tagname_overwrite() {
    check_test_info(
        b"test_ns_tagname_overwrite\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        135 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<n:e xmlns:n='http://example.org/'>\n  <n:f n:attr='foo'/>\n  <n:g n:attr2='bar'/>\n</n:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut result: *const XML_Char = b"start http://example.org/ e\nstart http://example.org/ f\nattribute http://example.org/ attr\nend http://example.org/ f\nstart http://example.org/ g\nattribute http://example.org/ attr2\nend http://example.org/ g\nend http://example.org/ e\n\0"
            .as_ptr() as *const XML_Char;
    run_ns_tagname_overwrite_test(text, result);
}
extern "C" fn test_ns_tagname_overwrite_triplet() {
    check_test_info(
        b"test_ns_tagname_overwrite_triplet\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        153 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<n:e xmlns:n='http://example.org/'>\n  <n:f n:attr='foo'/>\n  <n:g n:attr2='bar'/>\n</n:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut result: *const XML_Char = b"start http://example.org/ e n\nstart http://example.org/ f n\nattribute http://example.org/ attr n\nend http://example.org/ f n\nstart http://example.org/ g n\nattribute http://example.org/ attr2 n\nend http://example.org/ g n\nend http://example.org/ e n\n\0"
            .as_ptr() as *const XML_Char;
    set_return_ns_triplet(global_parser(), true);
    run_ns_tagname_overwrite_test(text, result);
}
extern "C" fn test_start_ns_clears_start_element() {
    check_test_info(
        b"test_start_ns_clears_start_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        172 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<e xmlns='http://example.org/'></e>\0".as_ptr() as *const ::core::ffi::c_char;
    set_start_element_handler(
        global_parser(),
        Some(
            start_element_fail
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    set_start_namespace_decl_handler(
        global_parser(),
        Some(
            start_ns_clearing_start_element
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    set_end_namespace_decl_handler(
        global_parser(),
        Some(
            dummy_end_namespace_decl_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    use_parser_as_handler_arg(global_parser());
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            185 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_default_ns_from_ext_subset_and_ext_ge() {
    check_test_info(
        b"test_default_ns_from_ext_subset_and_ext_ge\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        190 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0'?>\n<!DOCTYPE doc SYSTEM 'http://example.org/doc.dtd' [\n  <!ENTITY en SYSTEM 'http://example.org/entity.ent'>\n]>\n<doc xmlns='http://example.org/ns1'>\n&en;\n</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    set_param_entity_parsing(global_parser(), XML_PARAM_ENTITY_PARSING_ALWAYS);
    set_external_entity_ref_handler(
        global_parser(),
        Some(
            external_entity_handler
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    set_start_element_handler(
        global_parser(),
        Some(
            dummy_start_element
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    set_user_data(global_parser(), NULL);
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            206 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_prefix_with_empty_uri_1() {
    check_test_info(
        b"test_ns_prefix_with_empty_uri_1\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        211 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc xmlns:prefix='http://example.org/'>\n  <e xmlns:prefix=''/>\n</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_UNDECLARING_PREFIX,
        b"Did not report re-setting namespace URI with prefix to ''.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        218 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_prefix_with_empty_uri_2() {
    check_test_info(
        b"test_ns_prefix_with_empty_uri_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        223 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0'?>\n<docelem xmlns:pre=''/>\0".as_ptr() as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_UNDECLARING_PREFIX,
        b"Did not report setting namespace URI with prefix to ''.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        228 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_prefix_with_empty_uri_3() {
    check_test_info(
        b"test_ns_prefix_with_empty_uri_3\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        233 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ELEMENT doc EMPTY>\n  <!ATTLIST doc\n    xmlns:prefix CDATA ''>\n]>\n<doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_UNDECLARING_PREFIX,
        b"Didn't report attr default setting NS w/ prefix to ''.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        242 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_prefix_with_empty_uri_4() {
    check_test_info(
        b"test_ns_prefix_with_empty_uri_4\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        247 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ELEMENT prefix:doc EMPTY>\n  <!ATTLIST prefix:doc\n    xmlns:prefix CDATA 'http://example.org/'>\n]>\n<prefix:doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut elemstr: [*const XML_Char; 1] =
        [b"http://example.org/ doc prefix\0".as_ptr() as *const ::core::ffi::c_char];
    set_return_ns_triplet(global_parser(), true);
    set_user_data(
        global_parser(),
        &raw mut elemstr as *mut *const XML_Char as *mut ::core::ffi::c_void,
    );
    set_end_element_handler(
        global_parser(),
        Some(
            triplet_end_checker
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            263 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_unbound_prefix() {
    check_test_info(
        b"test_ns_unbound_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        268 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ELEMENT prefix:doc EMPTY>\n  <!ATTLIST prefix:doc\n    notxmlns:prefix CDATA 'http://example.org/'>\n]>\n<prefix:doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            278 as ::core::ffi::c_int,
            b"Unbound prefix incorrectly passed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if parser_error_code(global_parser()) as ::core::ffi::c_uint
        != XML_ERROR_UNBOUND_PREFIX as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            280 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_default_with_empty_uri() {
    check_test_info(
        b"test_ns_default_with_empty_uri\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        284 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc xmlns='http://example.org/'>\n  <e xmlns=''/>\n</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    set_start_namespace_decl_handler(
        global_parser(),
        Some(
            dummy_start_namespace_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    set_end_namespace_decl_handler(
        global_parser(),
        Some(
            dummy_end_namespace_decl_handler
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
        ),
    );
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            294 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_duplicate_attrs_diff_prefixes() {
    check_test_info(
        b"test_ns_duplicate_attrs_diff_prefixes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        299 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<doc xmlns:a='http://example.org/a'\n     xmlns:b='http://example.org/a'\n     a:a='v' b:a='v' />\0"
            .as_ptr() as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_DUPLICATE_ATTRIBUTE,
        b"did not report multiple attributes with same URI+name\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        304 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_duplicate_hashes() {
    check_test_info(
        b"test_ns_duplicate_hashes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        308 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc xmlns:a='http://example.org/a'\n     a:a='v' a:i='w' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            329 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_unbound_prefix_on_attribute() {
    check_test_info(
        b"test_ns_unbound_prefix_on_attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        334 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc a:attr=''/>\0".as_ptr() as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_UNBOUND_PREFIX,
        b"did not report unbound prefix on attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        337 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_unbound_prefix_on_element() {
    check_test_info(
        b"test_ns_unbound_prefix_on_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        342 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<a:doc/>\0".as_ptr() as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_UNBOUND_PREFIX,
        b"did not report unbound prefix on element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        345 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_long_element() {
    check_test_info(
        b"test_ns_long_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        350 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:thisisalongenoughelementnametotriggerareallocation\n xmlns:foo='http://example.org/' bar:a='12'\n xmlns:bar='http://example.org/'></foo:thisisalongenoughelementnametotriggerareallocation>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ thisisalongenoughelementnametotriggerareallocation foo\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"http://example.org/ a bar\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    set_return_ns_triplet(global_parser(), true);
    set_user_data(
        global_parser(),
        &raw mut elemstr as *mut *const XML_Char as *mut ::core::ffi::c_void,
    );
    set_element_handler(
        global_parser(),
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
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            366 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_mixed_prefix_atts() {
    check_test_info(
        b"test_ns_mixed_prefix_atts\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        371 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<e a='12' bar:b='13'\n xmlns:bar='http://example.org/'></e>\0".as_ptr()
            as *const ::core::ffi::c_char;
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            378 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_extend_uri_buffer() {
    check_test_info(
        b"test_ns_extend_uri_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        386 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/'> <foo:thisisalongenoughnametotriggerallocationaction   foo:a='12' /></foo:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    if parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            393 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_reserved_attributes() {
    check_test_info(
        b"test_ns_reserved_attributes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        400 as ::core::ffi::c_int,
    );
    let mut text1: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://example.org/' xmlns:xmlns='12' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://example.org/' foo:xmlns='12' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    expect_failure(
        text1,
        XML_ERROR_RESERVED_PREFIX_XMLNS,
        b"xmlns not rejected as an attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        406 as ::core::ffi::c_int,
    );
    reset_parser(global_parser());
    if parse_single_bytes_len(
        global_parser(),
        text2,
        c_strlen(text2) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            410 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_reserved_attributes_2() {
    check_test_info(
        b"test_ns_reserved_attributes_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        415 as ::core::ffi::c_int,
    );
    let mut text1: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://example.org/'  xmlns:xml='http://example.org/' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://www.w3.org/XML/1998/namespace' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut text3: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://www.w3.org/2000/xmlns/' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    expect_failure(
        text1,
        XML_ERROR_RESERVED_PREFIX_XML,
        b"xml not rejected as an attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        423 as ::core::ffi::c_int,
    );
    reset_parser(global_parser());
    expect_failure(
        text2,
        XML_ERROR_RESERVED_NAMESPACE_URI,
        b"Use of w3.org URL not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        426 as ::core::ffi::c_int,
    );
    reset_parser(global_parser());
    expect_failure(
        text3,
        XML_ERROR_RESERVED_NAMESPACE_URI,
        b"Use of w3.org xmlns URL not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        429 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_extremely_long_prefix() {
    check_test_info(
        b"test_ns_extremely_long_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        435 as ::core::ffi::c_int,
    );
    let mut text1: *const ::core::ffi::c_char = b"<doc ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:a='12'\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char = b" xmlns:ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP='foo'\n></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    if parse_single_bytes_len(
        global_parser(),
        text1,
        c_strlen(text1) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            516 as ::core::ffi::c_int,
        );
    }
    if parse_single_bytes_len(
        global_parser(),
        text2,
        c_strlen(text2) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            519 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_ns_unknown_encoding_success() {
    check_test_info(
        b"test_ns_unknown_encoding_success\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        524 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='prefix-conv'?>\n<foo:e xmlns:foo='http://example.org/'>Hi</foo:e>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    set_unknown_encoding_handler(
        global_parser(),
        Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
    );
    run_character_check(
        text,
        b"Hi\0".as_ptr() as *const XML_Char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        529 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_double_colon() {
    check_test_info(
        b"test_ns_double_colon\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        534 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://example.org/' foo:a:b='bar' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    let status: XML_Status = parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as XML_Status;
    if status as ::core::ffi::c_uint == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        || parser_error_code(global_parser()) as ::core::ffi::c_uint
            != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            542 as ::core::ffi::c_int,
            b"Double colon in attribute name not faulted (despite active namespace support)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
extern "C" fn test_ns_double_colon_element() {
    check_test_info(
        b"test_ns_double_colon_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        553 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<foo:bar:e xmlns:foo='http://example.org/' />\0".as_ptr() as *const ::core::ffi::c_char;
    let status: XML_Status = parse_single_bytes_len(
        global_parser(),
        text,
        c_strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as XML_Status;
    if status as ::core::ffi::c_uint == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        || parser_error_code(global_parser()) as ::core::ffi::c_uint
            != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            561 as ::core::ffi::c_int,
            b"Double colon in element name not faulted (despite active namespace support)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
extern "C" fn test_ns_bad_attr_leafname() {
    check_test_info(
        b"test_ns_bad_attr_leafname\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        573 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://example.org/' foo:?ar='baz' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character in leafname not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        577 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_bad_element_leafname() {
    check_test_info(
        b"test_ns_bad_element_leafname\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        581 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<foo:?oc xmlns:foo='http://example.org/' />\0".as_ptr() as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character in element leafname not faulted\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        585 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_utf16_leafname() {
    check_test_info(
        b"test_ns_utf16_leafname\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        590 as ::core::ffi::c_int,
    );
    let text: [::core::ffi::c_char; 59] = bytes_to_c_chars(*b"<\0n\0:\0e\0 \0x\0m\0l\0n\0s\0:\0n\0=\0'\0U\0R\0I\0'\0 \0n\0:\0\x04\x0E=\0'\0a\0'\0 \0/\0>\0\0");
    let mut expected: *const XML_Char = b"a\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    char_data_init(&mut storage);
    set_start_element_handler(
        global_parser(),
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    set_user_data(
        global_parser(),
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    if parse_single_bytes_len(
        global_parser(),
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 59]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            605 as ::core::ffi::c_int,
        );
    }
    char_data_check_xml_chars(&mut storage, expected);
}
extern "C" fn test_ns_utf16_element_leafname() {
    check_test_info(
        b"test_ns_utf16_element_leafname\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        610 as ::core::ffi::c_int,
    );
    let text: [::core::ffi::c_char; 41] =
        bytes_to_c_chars(*b"\0<\0n\0:\x0E\x04\0 \0x\0m\0l\0n\0s\0:\0n\0=\0'\0U\0R\0I\0'\0/\0>\0");
    let mut expected: *const XML_Char = b"URI \xE0\xB8\x84\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    char_data_init(&mut storage);
    set_start_element_handler(
        global_parser(),
        Some(
            start_element_event_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    set_user_data(
        global_parser(),
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    if parse_single_bytes_len(
        global_parser(),
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 41]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            628 as ::core::ffi::c_int,
        );
    }
    char_data_check_xml_chars(&mut storage, expected);
}
extern "C" fn test_ns_utf16_doctype() {
    check_test_info(
        b"test_ns_utf16_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        633 as ::core::ffi::c_int,
    );
    let text: [::core::ffi::c_char; 155] = bytes_to_c_chars(*b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0f\0o\0o\0:\x0E\x04\0 \0[\0 \0<\0!\0E\0N\0T\0I\0T\0Y\0 \0b\0a\0r\0 \0'\0b\0a\0z\0'\0>\0 \0]\0>\0\n\0<\0f\0o\0o\0:\x0E\x04\0 \0x\0m\0l\0n\0s\0:\0f\0o\0o\0=\0'\0U\0R\0I\0'\0>\0&\0b\0a\0r\0;\0<\0/\0f\0o\0o\0:\x0E\x04\0>\0");
    let mut expected: *const XML_Char = b"URI \xE0\xB8\x84\0".as_ptr() as *const XML_Char;
    let mut storage: CharData = CharData {
        count: 0,
        data: [0; 2048],
    };
    char_data_init(&mut storage);
    set_user_data(
        global_parser(),
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    set_start_element_handler(
        global_parser(),
        Some(
            start_element_event_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    set_unknown_encoding_handler(
        global_parser(),
        Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
    );
    if parse_single_bytes_len(
        global_parser(),
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 155]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(
            global_parser(),
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            659 as ::core::ffi::c_int,
        );
    }
    char_data_check_xml_chars(&mut storage, expected);
}
extern "C" fn test_ns_invalid_doctype() {
    check_test_info(
        b"test_ns_invalid_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        664 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE foo:!bad [ <!ENTITY bar 'baz' ]>\n<foo:!bad>&bar;</foo:!bad>\0".as_ptr()
            as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character in document local name not faulted\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        669 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_double_colon_doctype() {
    check_test_info(
        b"test_ns_double_colon_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        673 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE foo:a:doc [ <!ENTITY bar 'baz' ]>\n<foo:a:doc>&bar;</foo:a:doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Double colon in document name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        678 as ::core::ffi::c_int,
    );
}
extern "C" fn test_ns_separator_in_uri() {
    check_test_info(
        b"test_ns_separator_in_uri\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        682 as ::core::ffi::c_int,
    );
    let mut cases: [test_case; 3] = [
        test_case {
            expectedStatus: XML_STATUS_OK,
            doc: b"<doc xmlns='one_two' />\0".as_ptr() as *const ::core::ffi::c_char,
            namesep: '\n' as i32 as XML_Char,
        },
        test_case {
            expectedStatus: XML_STATUS_ERROR,
            doc: b"<doc xmlns='one&#x0A;two' />\0".as_ptr() as *const ::core::ffi::c_char,
            namesep: '\n' as i32 as XML_Char,
        },
        test_case {
            expectedStatus: XML_STATUS_OK,
            doc: b"<doc xmlns='one:two' />\0".as_ptr() as *const ::core::ffi::c_char,
            namesep: ':' as i32 as XML_Char,
        },
    ];
    let mut i: size_t = 0 as size_t;
    let mut failCount: size_t = 0 as size_t;
    while i
        < (::core::mem::size_of::<[test_case; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<test_case>() as usize)
    {
        set_subtest_doc(cases[i as usize].doc);
        let mut parser: XML_Parser = create_parser_ns(cases[i as usize].namesep);
        set_element_handler(
            parser,
            Some(
                dummy_start_element
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
            Some(
                dummy_end_element
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        if parse_single_bytes_len(
            parser,
            cases[i as usize].doc,
            c_strlen(cases[i as usize].doc) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != cases[i as usize].expectedStatus as ::core::ffi::c_uint
        {
            failCount = failCount.wrapping_add(1);
        }
        free_parser(parser);
        i = i.wrapping_add(1);
    }
    if failCount != 0 {
        fail_test(
            b"/root/work/expat/tests/ns_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            709 as ::core::ffi::c_int,
            b"Namespace separator handling is broken\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_namespace_test_case(mut s: *mut Suite) {
    let mut tc_namespace: *mut TCase = create_tcase(b"XML namespaces\0");
    add_suite_tcase(s, tc_namespace);
    add_checked_fixture(tc_namespace, namespace_setup, namespace_teardown);
    add_test(tc_namespace, test_return_ns_triplet);
    add_test(tc_namespace, test_ns_parser_reset);
    add_test(tc_namespace, test_ns_tagname_overwrite);
    add_test(tc_namespace, test_ns_tagname_overwrite_triplet);
    add_test(tc_namespace, test_start_ns_clears_start_element);
    add_test_ifdef_xml_dtd(tc_namespace, test_default_ns_from_ext_subset_and_ext_ge);
    add_test(tc_namespace, test_ns_prefix_with_empty_uri_1);
    add_test(tc_namespace, test_ns_prefix_with_empty_uri_2);
    add_test(tc_namespace, test_ns_prefix_with_empty_uri_3);
    add_test(tc_namespace, test_ns_prefix_with_empty_uri_4);
    add_test(tc_namespace, test_ns_unbound_prefix);
    add_test(tc_namespace, test_ns_default_with_empty_uri);
    add_test(tc_namespace, test_ns_duplicate_attrs_diff_prefixes);
    add_test(tc_namespace, test_ns_duplicate_hashes);
    add_test(tc_namespace, test_ns_unbound_prefix_on_attribute);
    add_test(tc_namespace, test_ns_unbound_prefix_on_element);
    add_test(tc_namespace, test_ns_long_element);
    add_test(tc_namespace, test_ns_mixed_prefix_atts);
    add_test(tc_namespace, test_ns_extend_uri_buffer);
    add_test(tc_namespace, test_ns_reserved_attributes);
    add_test(tc_namespace, test_ns_reserved_attributes_2);
    add_test(tc_namespace, test_ns_extremely_long_prefix);
    add_test(tc_namespace, test_ns_unknown_encoding_success);
    add_test(tc_namespace, test_ns_double_colon);
    add_test(tc_namespace, test_ns_double_colon_element);
    add_test(tc_namespace, test_ns_bad_attr_leafname);
    add_test(tc_namespace, test_ns_bad_element_leafname);
    add_test(tc_namespace, test_ns_utf16_leafname);
    add_test(tc_namespace, test_ns_utf16_element_leafname);
    add_test_if_xml_ge(tc_namespace, test_ns_utf16_doctype);
    add_test(tc_namespace, test_ns_invalid_doctype);
    add_test(tc_namespace, test_ns_double_colon_doctype);
    add_test(tc_namespace, test_ns_separator_in_uri);
}
