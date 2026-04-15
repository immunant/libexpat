use crate::src::tests::runtests::{current_test_parser, set_current_test_parser};

extern "C" {
    pub type XML_ParserStruct;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn XML_SetElementDeclHandler(parser: XML_Parser, eldecl: XML_ElementDeclHandler);
    fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser;
    fn XML_ParserCreateNS(encoding: *const XML_Char, namespaceSeparator: XML_Char) -> XML_Parser;
    fn XML_ParserCreate_MM(
        encoding: *const XML_Char,
        memsuite: *const XML_Memory_Handling_Suite,
        namespaceSeparator: *const XML_Char,
    ) -> XML_Parser;
    fn XML_ParserReset(parser: XML_Parser, encoding: *const XML_Char) -> XML_Bool;
    fn XML_SetEntityDeclHandler(parser: XML_Parser, handler: XML_EntityDeclHandler);
    fn XML_SetElementHandler(
        parser: XML_Parser,
        start: XML_StartElementHandler,
        end: XML_EndElementHandler,
    );
    fn XML_SetStartElementHandler(parser: XML_Parser, handler: XML_StartElementHandler);
    fn XML_SetCharacterDataHandler(parser: XML_Parser, handler: XML_CharacterDataHandler);
    fn XML_SetExternalEntityRefHandler(parser: XML_Parser, handler: XML_ExternalEntityRefHandler);
    fn XML_SetUserData(parser: XML_Parser, userData: *mut ::core::ffi::c_void);
    fn XML_Parse(
        parser: XML_Parser,
        s: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn XML_GetBuffer(parser: XML_Parser, len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn XML_StopParser(parser: XML_Parser, resumable: XML_Bool) -> XML_Status;
    fn XML_ResumeParser(parser: XML_Parser) -> XML_Status;
    fn XML_ExternalEntityParserCreate(
        parser: XML_Parser,
        context: *const XML_Char,
        encoding: *const XML_Char,
    ) -> XML_Parser;
    fn XML_SetParamEntityParsing(
        parser: XML_Parser,
        parsing: XML_ParamEntityParsing,
    ) -> ::core::ffi::c_int;
    fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
    fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
    fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size;
    fn XML_GetCurrentByteCount(parser: XML_Parser) -> ::core::ffi::c_int;
    fn XML_GetInputContext(
        parser: XML_Parser,
        offset: *mut ::core::ffi::c_int,
        size: *mut ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
    fn XML_ParserFree(parser: XML_Parser);
    fn XML_ErrorString(code: XML_Error) -> *const XML_LChar;
    fn XML_ExpatVersion() -> *const XML_LChar;
    fn XML_ExpatVersionInfo() -> XML_Expat_Version;
    fn XML_GetFeatureList() -> *const XML_Feature;
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
    fn tracking_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn tracking_free(ptr: *mut ::core::ffi::c_void);
    fn tracking_realloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn tracking_report() -> ::core::ffi::c_int;
    fn CharData_Init(storage: *mut CharData);
    fn CharData_AppendXMLChars(storage: *mut CharData, s: *const XML_Char, len: ::core::ffi::c_int);
    fn CharData_CheckXMLChars(storage: *mut CharData, s: *const XML_Char) -> ::core::ffi::c_int;
    static mut g_chunkSize: ::core::ffi::c_int;
    fn tcase_add_test__ifdef_xml_dtd(tc: *mut TCase, test: tcase_test_function);
    fn tcase_add_test__if_xml_ge(tc: *mut TCase, test: tcase_test_function);
    fn basic_teardown();
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
    static mut g_allocation_count: ::core::ffi::c_int;
    fn duff_allocator(size: size_t) -> *mut ::core::ffi::c_void;
    fn portable_strndup(s: *const ::core::ffi::c_char, n: size_t) -> *mut ::core::ffi::c_char;
    fn start_element_issue_240(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn end_element_issue_240(userData: *mut ::core::ffi::c_void, name: *const XML_Char);
    fn external_entity_failer__if_not_xml_ge(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_null_loader(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_oneshot_loader(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn suspend_after_element_declaration(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        model: *mut XML_Content,
    );
    fn accumulate_entity_decl(
        userData: *mut ::core::ffi::c_void,
        entityName: *const XML_Char,
        is_parameter_entity: ::core::ffi::c_int,
        value: *const XML_Char,
        value_length: ::core::ffi::c_int,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
        notationName: *const XML_Char,
    );
    fn accumulate_start_element(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn accumulate_characters(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
}
pub type size_t = usize;
pub type XML_Char = ::core::ffi::c_char;
pub type XML_LChar = ::core::ffi::c_char;
pub type XML_Size = ::core::ffi::c_ulong;
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
pub type XML_Content_Type = ::core::ffi::c_uint;
pub const XML_CTYPE_SEQ: XML_Content_Type = 6;
pub const XML_CTYPE_CHOICE: XML_Content_Type = 5;
pub const XML_CTYPE_NAME: XML_Content_Type = 4;
pub const XML_CTYPE_MIXED: XML_Content_Type = 3;
pub const XML_CTYPE_ANY: XML_Content_Type = 2;
pub const XML_CTYPE_EMPTY: XML_Content_Type = 1;
pub type XML_Content_Quant = ::core::ffi::c_uint;
pub const XML_CQUANT_PLUS: XML_Content_Quant = 3;
pub const XML_CQUANT_REP: XML_Content_Quant = 2;
pub const XML_CQUANT_OPT: XML_Content_Quant = 1;
pub const XML_CQUANT_NONE: XML_Content_Quant = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_cp {
    pub type_0: XML_Content_Type,
    pub quant: XML_Content_Quant,
    pub name: *mut XML_Char,
    pub numchildren: ::core::ffi::c_uint,
    pub children: *mut XML_Content,
}
pub type XML_Content = XML_cp;
pub type XML_ElementDeclHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *mut XML_Content) -> ()>;
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
pub type XML_EntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        ::core::ffi::c_int,
        *const XML_Char,
        ::core::ffi::c_int,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> (),
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Expat_Version {
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
    pub micro: ::core::ffi::c_int,
}
pub type XML_FeatureEnum = ::core::ffi::c_uint;
pub const XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT: XML_FeatureEnum = 15;
pub const XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT: XML_FeatureEnum = 14;
pub const XML_FEATURE_GE: XML_FeatureEnum = 13;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT:
    XML_FeatureEnum = 12;
pub const XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT:
    XML_FeatureEnum = 11;
pub const XML_FEATURE_ATTR_INFO: XML_FeatureEnum = 10;
pub const XML_FEATURE_LARGE_SIZE: XML_FeatureEnum = 9;
pub const XML_FEATURE_NS: XML_FeatureEnum = 8;
pub const XML_FEATURE_SIZEOF_XML_LCHAR: XML_FeatureEnum = 7;
pub const XML_FEATURE_SIZEOF_XML_CHAR: XML_FeatureEnum = 6;
pub const XML_FEATURE_MIN_SIZE: XML_FeatureEnum = 5;
pub const XML_FEATURE_CONTEXT_BYTES: XML_FeatureEnum = 4;
pub const XML_FEATURE_DTD: XML_FeatureEnum = 3;
pub const XML_FEATURE_UNICODE_WCHAR_T: XML_FeatureEnum = 2;
pub const XML_FEATURE_UNICODE: XML_FeatureEnum = 1;
pub const XML_FEATURE_END: XML_FeatureEnum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct XML_Feature {
    pub feature: XML_FeatureEnum,
    pub name: *const XML_LChar,
    pub value: ::core::ffi::c_long,
}
pub type tcase_setup_function = Option<extern "C" fn() -> ()>;
pub type tcase_teardown_function = Option<extern "C" fn() -> ()>;
pub type tcase_test_function = Option<extern "C" fn() -> ()>;
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
pub struct DataIssue240 {
    pub parser: XML_Parser,
    pub deep: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext_hdlr_data {
    pub parse_text: *const ::core::ffi::c_char,
    pub handler: XML_ExternalEntityRefHandler,
    pub storage: *mut CharData,
}
pub type ExtHdlrData = ext_hdlr_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct test_case {
    pub doc: *const ::core::ffi::c_char,
    pub expectedStatusNoGE: XML_Status,
    pub expectedErrorNoGE: XML_Error,
    pub expectedErrorLine: XML_Size,
    pub expectedErrorColumn: XML_Size,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub xml_error: XML_Error,
    pub integer: ::core::ffi::c_int,
}
const fn bytes_to_c_chars<const N: usize>(bytes: &[u8; N]) -> [::core::ffi::c_char; N] {
    let mut chars = [0; N];
    let mut index = 0;
    while index < N {
        chars[index] = bytes[index] as ::core::ffi::c_char;
        index += 1;
    }
    chars
}

pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 29] =
    bytes_to_c_chars(b"void test_misc_version(void)\0");
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const ASCII_0: ::core::ffi::c_int = 0x30 as ::core::ffi::c_int;
pub const ASCII_9: ::core::ffi::c_int = 0x39 as ::core::ffi::c_int;
pub const ASCII_PERIOD: ::core::ffi::c_int = 0x2e as ::core::ffi::c_int;
const MISC_TESTS_FILE: &[u8] = b"/root/work/expat/tests/misc_tests.c\0";

fn bytes_as_c_char_ptr(bytes: &[u8]) -> *const ::core::ffi::c_char {
    bytes.as_ptr().cast()
}

macro_rules! ffi_call0 {
    ($function:expr $(,)?) => {{
        unsafe { $function() }
    }};
}

macro_rules! ffi_call1 {
    ($function:expr, $a:expr $(,)?) => {{
        unsafe { $function($a) }
    }};
}

macro_rules! ffi_call2 {
    ($function:expr, $a:expr, $b:expr $(,)?) => {{
        unsafe { $function($a, $b) }
    }};
}

macro_rules! ffi_call3 {
    ($function:expr, $a:expr, $b:expr, $c:expr $(,)?) => {{
        unsafe { $function($a, $b, $c) }
    }};
}

macro_rules! ffi_call4 {
    ($function:expr, $a:expr, $b:expr, $c:expr, $d:expr $(,)?) => {{
        unsafe { $function($a, $b, $c, $d) }
    }};
}

macro_rules! ffi_call5 {
    ($function:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr $(,)?) => {{
        unsafe { $function($a, $b, $c, $d, $e) }
    }};
}

macro_rules! set_allocation_count {
    ($count:expr $(,)?) => {{
        unsafe {
            g_allocation_count = $count;
        }
    }};
}

macro_rules! current_chunk_size {
    () => {{
        unsafe { g_chunkSize }
    }};
}

macro_rules! next_feature {
    ($feature:expr $(,)?) => {{
        let feature = $feature;
        unsafe {
            feature
                .as_ref()
                .map(|feature_ref| (feature_ref, feature.wrapping_add(1)))
        }
    }};
}

macro_rules! set_case_subtest {
    ($index:expr $(,)?) => {{
        unsafe {
            set_subtest(bytes_as_c_char_ptr(b"cases[%d]\0"), $index);
        }
    }};
}

macro_rules! set_issue_317_subtest {
    ($input_index:expr, $suspend:expr, $input:expr $(,)?) => {{
        let suspend = $suspend;
        unsafe {
            set_subtest(
                bytes_as_c_char_ptr(b"[input=%d suspend=%s] %s\0"),
                $input_index,
                if suspend as ::core::ffi::c_int != 0 {
                    bytes_as_c_char_ptr(b"true\0")
                } else {
                    bytes_as_c_char_ptr(b"false\0")
                },
                $input,
            );
        }
    }};
}

macro_rules! c_string {
    ($text:expr $(,)?) => {{
        unsafe { std::ffi::CStr::from_ptr($text) }
    }};
}

fn current_parser() -> XML_Parser {
    current_test_parser() as XML_Parser
}

fn set_current_parser(parser: XML_Parser) {
    set_current_test_parser(parser as crate::src::tests::runtests::XML_Parser);
}

fn parser_create() -> XML_Parser {
    ffi_call1!(XML_ParserCreate, ::core::ptr::null::<XML_Char>())
}

fn parser_create_with_encoding(encoding: *const XML_Char) -> XML_Parser {
    ffi_call1!(XML_ParserCreate, encoding)
}

fn parser_create_ns(namespace_separator: XML_Char) -> XML_Parser {
    ffi_call2!(
        XML_ParserCreateNS,
        ::core::ptr::null::<XML_Char>(),
        namespace_separator,
    )
}

fn parser_create_mm(
    encoding: *const XML_Char,
    memsuite: &XML_Memory_Handling_Suite,
    namespace_separator: *const XML_Char,
) -> XML_Parser {
    ffi_call3!(
        XML_ParserCreate_MM,
        encoding,
        memsuite as *const XML_Memory_Handling_Suite,
        namespace_separator,
    )
}

fn parser_free(parser: XML_Parser) {
    ffi_call1!(XML_ParserFree, parser);
}

fn parser_reset(parser: XML_Parser) -> XML_Bool {
    ffi_call2!(XML_ParserReset, parser, ::core::ptr::null::<XML_Char>())
}

fn parser_create_external_entity(parser: XML_Parser) -> XML_Parser {
    ffi_call3!(
        XML_ExternalEntityParserCreate,
        parser,
        ::core::ptr::null::<XML_Char>(),
        ::core::ptr::null::<XML_Char>(),
    )
}

fn parse_single_bytes(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    ffi_call4!(_XML_Parse_SINGLE_BYTES, parser, text, len, is_final)
}

fn parser_get_buffer(parser: XML_Parser, len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
    ffi_call2!(XML_GetBuffer, parser, len)
}

fn stop_parser(parser: XML_Parser, resumable: XML_Bool) -> XML_Status {
    ffi_call2!(XML_StopParser, parser, resumable)
}

fn resume_parser(parser: XML_Parser) -> XML_Status {
    ffi_call1!(XML_ResumeParser, parser)
}

fn parser_error_code(parser: XML_Parser) -> XML_Error {
    ffi_call1!(XML_GetErrorCode, parser)
}

fn parser_current_line(parser: XML_Parser) -> XML_Size {
    ffi_call1!(XML_GetCurrentLineNumber, parser)
}

fn parser_current_column(parser: XML_Parser) -> XML_Size {
    ffi_call1!(XML_GetCurrentColumnNumber, parser)
}

fn parser_current_byte_count(parser: XML_Parser) -> ::core::ffi::c_int {
    ffi_call1!(XML_GetCurrentByteCount, parser) as ::core::ffi::c_int
}

fn parser_input_context(
    parser: XML_Parser,
    offset: &mut ::core::ffi::c_int,
    size: &mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    ffi_call3!(
        XML_GetInputContext,
        parser,
        offset as *mut _,
        size as *mut _,
    )
    .cast()
}

fn parser_set_user_data(parser: XML_Parser, user_data: *mut ::core::ffi::c_void) {
    ffi_call2!(XML_SetUserData, parser, user_data);
}

fn parser_set_entity_decl_handler(parser: XML_Parser, handler: XML_EntityDeclHandler) {
    ffi_call2!(XML_SetEntityDeclHandler, parser, handler);
}

fn parser_set_element_handler(
    parser: XML_Parser,
    start: XML_StartElementHandler,
    end: XML_EndElementHandler,
) {
    ffi_call3!(XML_SetElementHandler, parser, start, end);
}

fn parser_set_start_element_handler(parser: XML_Parser, handler: XML_StartElementHandler) {
    ffi_call2!(XML_SetStartElementHandler, parser, handler);
}

fn parser_set_character_data_handler(parser: XML_Parser, handler: XML_CharacterDataHandler) {
    ffi_call2!(XML_SetCharacterDataHandler, parser, handler);
}

fn parser_set_external_entity_ref_handler(
    parser: XML_Parser,
    handler: XML_ExternalEntityRefHandler,
) {
    ffi_call2!(XML_SetExternalEntityRefHandler, parser, handler);
}

fn parser_set_element_decl_handler(parser: XML_Parser, handler: XML_ElementDeclHandler) {
    ffi_call2!(XML_SetElementDeclHandler, parser, handler);
}

fn parser_set_param_entity_parsing(
    parser: XML_Parser,
    parsing: XML_ParamEntityParsing,
) -> ::core::ffi::c_int {
    ffi_call2!(XML_SetParamEntityParsing, parser, parsing)
}

fn parser_parse(
    parser: XML_Parser,
    text: *const ::core::ffi::c_char,
    len: ::core::ffi::c_int,
    is_final: ::core::ffi::c_int,
) -> XML_Status {
    ffi_call4!(XML_Parse, parser, text, len, is_final)
}

fn xml_error_string(code: XML_Error) -> *const XML_LChar {
    ffi_call1!(XML_ErrorString, code)
}

fn expat_version_text() -> *const XML_LChar {
    ffi_call0!(XML_ExpatVersion)
}

fn expat_version_info() -> XML_Expat_Version {
    ffi_call0!(XML_ExpatVersionInfo)
}

fn feature_list() -> *const XML_Feature {
    ffi_call0!(XML_GetFeatureList)
}

fn char_data_init(storage: &mut CharData) {
    ffi_call1!(CharData_Init, storage as *mut CharData);
}

fn char_data_check_xml_chars(
    storage: &mut CharData,
    expected: *const XML_Char,
) -> ::core::ffi::c_int {
    ffi_call2!(CharData_CheckXMLChars, storage as *mut CharData, expected)
}

fn set_test_info_name(name: &[u8], line: ::core::ffi::c_int) {
    ffi_call3!(
        _check_set_test_info,
        bytes_as_c_char_ptr(name),
        bytes_as_c_char_ptr(MISC_TESTS_FILE),
        line,
    );
}

fn fail_test(line: ::core::ffi::c_int, message: &[u8]) -> ! {
    ffi_call3!(
        _fail,
        bytes_as_c_char_ptr(MISC_TESTS_FILE),
        line,
        bytes_as_c_char_ptr(message),
    )
}

fn xml_failure(parser: XML_Parser, line: ::core::ffi::c_int) {
    ffi_call3!(
        _xml_failure,
        parser,
        bytes_as_c_char_ptr(MISC_TESTS_FILE),
        line,
    );
}

fn c_string_len(text: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    ffi_call1!(strlen, text) as ::core::ffi::c_int
}

fn expect_failure(
    text: *const ::core::ffi::c_char,
    error_code: XML_Error,
    error_message: &[u8],
    line: ::core::ffi::c_int,
) {
    ffi_call5!(
        _expect_failure,
        text,
        error_code,
        bytes_as_c_char_ptr(error_message),
        bytes_as_c_char_ptr(MISC_TESTS_FILE),
        line,
    );
}

fn compare_strings(
    left: *const ::core::ffi::c_char,
    right: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    ffi_call2!(strcmp, left, right)
}

fn duplicate_c_string(text: *const ::core::ffi::c_char, len: size_t) -> *mut ::core::ffi::c_char {
    ffi_call2!(portable_strndup, text, len)
}

fn free_ptr(ptr: *mut ::core::ffi::c_void) {
    ffi_call1!(free, ptr);
}

fn tracking_report_value() -> ::core::ffi::c_int {
    ffi_call0!(tracking_report)
}

extern "C" fn basic_teardown_shim() {
    ffi_call0!(basic_teardown);
}

extern "C" fn test_misc_deny_internal_entity_closing_doctype_issue_317_shim() {
    test_misc_deny_internal_entity_closing_doctype_issue_317();
}

extern "C" fn test_misc_alloc_create_parser() {
    set_test_info_name(b"test_misc_alloc_create_parser\0", 67 as ::core::ffi::c_int);
    let memsuite = XML_Memory_Handling_Suite {
        malloc_fcn: Some(duff_allocator),
        realloc_fcn: Some(realloc),
        free_fcn: Some(free),
    };
    let max_alloc_count: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint = 0;
    while i < max_alloc_count {
        set_allocation_count!(i as ::core::ffi::c_int);
        set_current_parser(parser_create_mm(
            ::core::ptr::null::<XML_Char>(),
            &memsuite,
            ::core::ptr::null::<XML_Char>(),
        ));
        if !current_parser().is_null() {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i == 0 {
        fail_test(
            80 as ::core::ffi::c_int,
            b"Parser unexpectedly ignored failing allocator\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            82 as ::core::ffi::c_int,
            b"Parser not created with max allocation count\0",
        );
    }
}
extern "C" fn test_misc_alloc_create_parser_with_encoding() {
    set_test_info_name(
        b"test_misc_alloc_create_parser_with_encoding\0",
        87 as ::core::ffi::c_int,
    );
    let memsuite = XML_Memory_Handling_Suite {
        malloc_fcn: Some(duff_allocator),
        realloc_fcn: Some(realloc),
        free_fcn: Some(free),
    };
    let max_alloc_count: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_uint = 0;
    while i < max_alloc_count {
        set_allocation_count!(i as ::core::ffi::c_int);
        set_current_parser(parser_create_mm(
            b"us-ascii\0".as_ptr() as *const XML_Char,
            &memsuite,
            ::core::ptr::null::<XML_Char>(),
        ));
        if !current_parser().is_null() {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i == 0 {
        fail_test(
            100 as ::core::ffi::c_int,
            b"Parser ignored failing allocator\0",
        );
    } else if i == max_alloc_count {
        fail_test(
            102 as ::core::ffi::c_int,
            b"Parser not created with max allocation count\0",
        );
    }
}
extern "C" fn test_misc_null_parser() {
    set_test_info_name(b"test_misc_null_parser\0", 109 as ::core::ffi::c_int);
    parser_free(::core::ptr::null_mut::<XML_ParserStruct>());
}
extern "C" fn test_misc_error_string() {
    set_test_info_name(b"test_misc_error_string\0", 125 as ::core::ffi::c_int);
    if ::core::mem::size_of::<XML_Error>() != ::core::mem::size_of::<::core::ffi::c_int>() {
        fail_test(
            132 as ::core::ffi::c_int,
            b"check failed: sizeof(enum XML_Error) == sizeof(int)\0",
        );
    }
    if !xml_error_string((-(1 as ::core::ffi::c_int)) as XML_Error).is_null() {
        fail_test(
            136 as ::core::ffi::c_int,
            b"Negative error code not rejected\0",
        );
    }
    if !xml_error_string(100 as XML_Error).is_null() {
        fail_test(
            140 as ::core::ffi::c_int,
            b"Large error code not rejected\0",
        );
    }
}
fn expat_version() -> Option<(XML_Expat_Version, &'static std::ffi::CStr)> {
    let version_text = expat_version_text();
    if version_text.is_null() {
        None
    } else {
        Some((expat_version_info(), c_string!(version_text)))
    }
}

fn parse_version(version_text: &std::ffi::CStr) -> Option<XML_Expat_Version> {
    fn parse_component(bytes: &[u8], index: &mut usize) -> Option<::core::ffi::c_int> {
        let start = *index;
        let mut value = 0 as ::core::ffi::c_int;

        while let Some(byte) = bytes.get(*index) {
            if !byte.is_ascii_digit() {
                break;
            }

            value = 10 as ::core::ffi::c_int * value + (::core::ffi::c_int::from(*byte) - ASCII_0);
            *index += 1;
        }

        (*index != start).then_some(value)
    }

    let bytes = version_text.to_bytes();
    let mut index = bytes.iter().position(|byte| byte.is_ascii_digit())?;
    let major = parse_component(bytes, &mut index)?;

    if bytes.get(index).copied() != Some(ASCII_PERIOD as u8) {
        return None;
    }
    index += 1;

    let minor = parse_component(bytes, &mut index)?;
    if bytes.get(index).copied() != Some(ASCII_PERIOD as u8) {
        return None;
    }
    index += 1;

    let micro = parse_component(bytes, &mut index)?;
    (index == bytes.len()).then_some(XML_Expat_Version {
        major,
        minor,
        micro,
    })
}

fn versions_equal(first: &XML_Expat_Version, second: &XML_Expat_Version) -> bool {
    first.major == second.major && first.minor == second.minor && first.micro == second.micro
}

extern "C" fn test_misc_version() {
    set_test_info_name(b"test_misc_version\0", 201 as ::core::ffi::c_int);

    let (read_version, version_text) = match expat_version() {
        Some(version) => version,
        None => fail_test(
            208 as ::core::ffi::c_int,
            b"Could not obtain version text\0",
        ),
    };

    let parsed_version = match parse_version(version_text) {
        Some(version) => version,
        None => fail_test(211 as ::core::ffi::c_int, b"Unable to parse version text\0"),
    };

    if !versions_equal(&read_version, &parsed_version) {
        fail_test(213 as ::core::ffi::c_int, b"Version mismatch\0");
    }

    if version_text.to_bytes() != b"expat_2.7.4" {
        fail_test(
            217 as ::core::ffi::c_int,
            b"XML_*_VERSION in expat.h out of sync?\n\0",
        );
    }
}
extern "C" fn test_misc_features() {
    set_test_info_name(b"test_misc_features\0", 222 as ::core::ffi::c_int);
    let mut features = feature_list();
    set_current_parser(::core::ptr::null_mut::<XML_ParserStruct>());
    if features.is_null() {
        fail_test(
            228 as ::core::ffi::c_int,
            b"Failed to get feature information\0",
        );
    }
    while let Some((feature, next)) = next_feature!(features) {
        if feature.feature == XML_FEATURE_END {
            break;
        }
        match feature.feature {
            XML_FEATURE_SIZEOF_XML_CHAR => {
                if feature.value as usize != ::core::mem::size_of::<XML_Char>() {
                    fail_test(235 as ::core::ffi::c_int, b"Incorrect size of XML_Char\0");
                }
            }
            XML_FEATURE_SIZEOF_XML_LCHAR => {
                if feature.value as usize != ::core::mem::size_of::<XML_LChar>() {
                    fail_test(239 as ::core::ffi::c_int, b"Incorrect size of XML_LChar\0");
                }
            }
            _ => {}
        }
        features = next;
    }
}
extern "C" fn test_misc_attribute_leak() {
    set_test_info_name(b"test_misc_attribute_leak\0", 253 as ::core::ffi::c_int);
    let text = b"<D xmlns:L=\"D\" l:a='' L:a=''/>\0".as_ptr() as *const ::core::ffi::c_char;
    let memsuite = XML_Memory_Handling_Suite {
        malloc_fcn: Some(tracking_malloc),
        realloc_fcn: Some(tracking_realloc),
        free_fcn: Some(tracking_free),
    };
    set_current_parser(parser_create_mm(
        b"UTF-8\0".as_ptr() as *const XML_Char,
        &memsuite,
        b"\n\0".as_ptr() as *const XML_Char,
    ));
    expect_failure(
        text,
        XML_ERROR_UNBOUND_PREFIX,
        b"Unbound prefixes not found\0",
        259 as ::core::ffi::c_int,
    );
    parser_free(current_parser());
    set_current_parser(::core::ptr::null_mut::<XML_ParserStruct>());
    if tracking_report_value() == 0 {
        fail_test(265 as ::core::ffi::c_int, b"Memory leak found\0");
    }
}
extern "C" fn test_misc_utf16le() {
    set_test_info_name(b"test_misc_utf16le\0", 270 as ::core::ffi::c_int);
    let text: [u8; 61] =
        *b"<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0?\0>\0<\0q\0>\0H\0i\0<\0/\0q\0>\0\0";
    let expected = b"Hi\0".as_ptr() as *const XML_Char;
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };
    set_current_parser(parser_create_with_encoding(
        b"UTF-16LE\0".as_ptr() as *const XML_Char
    ));
    if current_parser().is_null() {
        fail_test(281 as ::core::ffi::c_int, b"Parser not created\0");
    }
    char_data_init(&mut storage);
    parser_set_user_data(current_parser(), (&mut storage as *mut CharData).cast());
    parser_set_character_data_handler(current_parser(), Some(accumulate_characters));
    if parse_single_bytes(
        current_parser(),
        text.as_ptr().cast(),
        text.len() as ::core::ffi::c_int - 1,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(current_parser(), 288 as ::core::ffi::c_int);
    }
    char_data_check_xml_chars(&mut storage, expected);
}
extern "C" fn test_misc_stop_during_end_handler_issue_240_1() {
    set_test_info_name(
        b"test_misc_stop_during_end_handler_issue_240_1\0",
        293 as ::core::ffi::c_int,
    );
    let doc1 = b"<doc><e1/><e><foo/></e></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let parser = parser_create();
    parser_set_element_handler(
        parser,
        Some(start_element_issue_240),
        Some(end_element_issue_240),
    );
    let mut mydata = DataIssue240 { parser, deep: 0 };
    parser_set_user_data(parser, (&mut mydata as *mut DataIssue240).cast());
    let result = parse_single_bytes(parser, doc1, c_string_len(doc1), 1 as ::core::ffi::c_int);
    parser_free(parser);
    if result as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            311 as ::core::ffi::c_int,
            b"Stopping the parser did not work as expected\0",
        );
    }
}
extern "C" fn test_misc_stop_during_end_handler_issue_240_2() {
    set_test_info_name(
        b"test_misc_stop_during_end_handler_issue_240_2\0",
        315 as ::core::ffi::c_int,
    );
    let doc2 = b"<doc><elem/></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let parser = parser_create();
    parser_set_element_handler(
        parser,
        Some(start_element_issue_240),
        Some(end_element_issue_240),
    );
    let mut mydata = DataIssue240 { parser, deep: 0 };
    parser_set_user_data(parser, (&mut mydata as *mut DataIssue240).cast());
    let result = parse_single_bytes(parser, doc2, c_string_len(doc2), 1 as ::core::ffi::c_int);
    parser_free(parser);
    if result as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            333 as ::core::ffi::c_int,
            b"Stopping the parser did not work as expected\0",
        );
    }
}
fn test_misc_deny_internal_entity_closing_doctype_issue_317() {
    set_test_info_name(
        b"test_misc_deny_internal_entity_closing_doctype_issue_317\0",
        337 as ::core::ffi::c_int,
    );
    let input_one = b"<!DOCTYPE d [\n<!ENTITY % element_d '<!ELEMENT d (#PCDATA)*>'>\n%element_d;\n<!ENTITY % e ']><d/>'>\n\n%e;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let input_two = b"<!DOCTYPE d [\n<!ENTITY % element_d '<!ELEMENT d (#PCDATA)*>'>\n%element_d;\n<!ENTITY % e1 ']><d/>'><!ENTITY % e2 '&#37;e1;'>\n\n%e2;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let input_three = b"<!DOCTYPE d [\n<!ENTITY % element_d '<!ELEMENT d (#PCDATA)*>'>\n%element_d;\n<!ENTITY % e ']><d'>\n\n%e;/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let input_issue_317 = b"<!DOCTYPE doc [\n<!ENTITY % element_doc '<!ELEMENT doc (#PCDATA)*>'>\n%element_doc;\n<!ENTITY % foo ']>\n<doc>Hell<oc (#PCDATA)*>'>\n%foo;\n]>\n<doc>Hello, world</dVc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let inputs = [input_one, input_two, input_three, input_issue_317];
    let suspend_or_not = [XML_FALSE, XML_TRUE];

    for (input_index, input) in inputs.iter().copied().enumerate() {
        for suspend in suspend_or_not {
            if suspend as ::core::ffi::c_int != 0 && current_chunk_size!() > 0 as ::core::ffi::c_int
            {
                return;
            }

            set_issue_317_subtest!(input_index as ::core::ffi::c_int, suspend, input);

            let parser = parser_create();
            let mut parse_result = XML_STATUS_ERROR;
            let set_param_entity_result =
                parser_set_param_entity_parsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            let line_number;
            let column_number;

            if set_param_entity_result != 1 as ::core::ffi::c_int {
                fail_test(
                    398 as ::core::ffi::c_int,
                    b"Failed to set XML_PARAM_ENTITY_PARSING_ALWAYS.\0",
                );
            }

            if suspend != 0 {
                parser_set_user_data(parser, parser as *mut ::core::ffi::c_void);
                parser_set_element_decl_handler(parser, Some(suspend_after_element_declaration));
            }

            if suspend != 0 {
                parse_result =
                    parser_parse(parser, input, c_string_len(input), 0 as ::core::ffi::c_int);
                while parse_result as ::core::ffi::c_uint
                    == XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    parse_result = resume_parser(parser);
                }
                if parse_result as ::core::ffi::c_uint
                    != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    parse_result = parser_parse(
                        parser,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                }
                while parse_result as ::core::ffi::c_uint
                    == XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    parse_result = resume_parser(parser);
                }
            } else {
                parse_result =
                    parse_single_bytes(parser, input, c_string_len(input), 0 as ::core::ffi::c_int);
                if parse_result as ::core::ffi::c_uint
                    != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    parse_result = parse_single_bytes(
                        parser,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                }
            }

            if parse_result as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                fail_test(
                    435 as ::core::ffi::c_int,
                    b"Parsing was expected to fail but succeeded.\0",
                );
            }

            if parser_error_code(parser) as ::core::ffi::c_uint
                != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                fail_test(
                    439 as ::core::ffi::c_int,
                    b"Error code does not match XML_ERROR_INVALID_TOKEN\0",
                );
            }

            line_number = parser_current_line(parser);
            if line_number != 6 as XML_Size {
                fail_test(
                    443 as ::core::ffi::c_int,
                    b"XML_GetCurrentLineNumber does not work as expected.\0",
                );
            }

            column_number = parser_current_column(parser);
            if column_number != 0 as XML_Size {
                fail_test(
                    447 as ::core::ffi::c_int,
                    b"XML_GetCurrentColumnNumber does not work as expected.\0",
                );
            }

            parser_free(parser);
        }
    }
}
extern "C" fn test_misc_tag_mismatch_reset_leak() {
    set_test_info_name(
        b"test_misc_tag_mismatch_reset_leak\0",
        455 as ::core::ffi::c_int,
    );
    let text =
        b"<open xmlns='https://namespace1.test'></close>\0".as_ptr() as *const ::core::ffi::c_char;
    let parser = parser_create_ns('\n' as i32 as XML_Char);
    if parse_single_bytes(
        parser,
        text,
        c_string_len(text),
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            462 as ::core::ffi::c_int,
            b"Call to parse was expected to fail\0",
        );
    }
    if parser_error_code(parser) as ::core::ffi::c_uint
        != XML_ERROR_TAG_MISMATCH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            464 as ::core::ffi::c_int,
            b"Call to parse was expected to fail from a closing tag mismatch\0",
        );
    }
    parser_reset(parser);
    if parse_single_bytes(
        parser,
        text,
        c_string_len(text),
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            470 as ::core::ffi::c_int,
            b"Call to parse was expected to fail\0",
        );
    }
    if parser_error_code(parser) as ::core::ffi::c_uint
        != XML_ERROR_TAG_MISMATCH as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            472 as ::core::ffi::c_int,
            b"Call to parse was expected to fail from a closing tag mismatch\0",
        );
    }
    parser_free(parser);
}
extern "C" fn test_misc_create_external_entity_parser_with_null_context() {
    set_test_info_name(
        b"test_misc_create_external_entity_parser_with_null_context\0",
        479 as ::core::ffi::c_int,
    );
    let parser = parser_create();
    let ext_parser = parser_create_external_entity(parser);
    if ext_parser.is_null() {
        fail_test(
            487 as ::core::ffi::c_int,
            b"check failed: ext_parser != NULL\0",
        );
    }
    parser_free(ext_parser);
    parser_free(parser);
}
extern "C" fn test_misc_general_entities_support() {
    set_test_info_name(
        b"test_misc_general_entities_support\0",
        496 as ::core::ffi::c_int,
    );
    let doc = b"<!DOCTYPE r [\n<!ENTITY e1 'v1'>\n<!ENTITY e2 SYSTEM 'v2'>\n]>\n<r a1='[&e1;]'>[&e1;][&e2;][&amp;&apos;&gt;&lt;&quot;]</r>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };
    char_data_init(&mut storage);
    let parser = parser_create();
    parser_set_user_data(parser, (&mut storage as *mut CharData).cast());
    parser_set_start_element_handler(parser, Some(accumulate_start_element));
    parser_set_external_entity_ref_handler(parser, Some(external_entity_failer__if_not_xml_ge));
    parser_set_entity_decl_handler(parser, Some(accumulate_entity_decl));
    parser_set_character_data_handler(parser, Some(accumulate_characters));
    if parse_single_bytes(
        parser,
        doc,
        c_string_len(doc),
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(parser, 517 as ::core::ffi::c_int);
    }
    parser_free(parser);
    char_data_check_xml_chars(
        &mut storage,
        b"e1=v1\ne2=(null)\n(r(a1=[v1]))\n[v1][][&'><\"]\0".as_ptr() as *const XML_Char,
    );
}
extern "C" fn resumable_stopping_character_handler(
    user_data: *mut ::core::ffi::c_void,
    _s: *const XML_Char,
    _len: ::core::ffi::c_int,
) {
    stop_parser(user_data as XML_Parser, XML_TRUE);
}
extern "C" fn test_misc_char_handler_stop_without_leak() {
    set_test_info_name(
        b"test_misc_char_handler_stop_without_leak\0",
        550 as ::core::ffi::c_int,
    );
    let data = b"<!DOCTYPE t1[<!ENTITY e1 'angle<'><!ENTITY e2 '&e1;'>]><t1>&e2;\0".as_ptr()
        as *const ::core::ffi::c_char;
    let parser = parser_create();
    if parser.is_null() {
        fail_test(554 as ::core::ffi::c_int, b"check failed: parser != NULL\0");
    }
    parser_set_user_data(parser, parser.cast());
    parser_set_character_data_handler(parser, Some(resumable_stopping_character_handler));
    parse_single_bytes(
        parser,
        data,
        c_string_len(data),
        XML_FALSE as ::core::ffi::c_int,
    );
    parser_free(parser);
}
extern "C" fn test_misc_resumeparser_not_crashing() {
    set_test_info_name(
        b"test_misc_resumeparser_not_crashing\0",
        562 as ::core::ffi::c_int,
    );
    let parser = parser_create();
    parser_get_buffer(parser, 1 as ::core::ffi::c_int);
    stop_parser(parser, XML_TRUE);
    resume_parser(parser);
    parser_free(parser);
}
extern "C" fn test_misc_stopparser_rejects_unstarted_parser() {
    set_test_info_name(
        b"test_misc_stopparser_rejects_unstarted_parser\0",
        571 as ::core::ffi::c_int,
    );
    let cases: [XML_Bool; 2] = [XML_TRUE, XML_FALSE];
    let mut i: size_t = 0;
    while i < cases.len() {
        let resumable = cases[i];
        let parser = parser_create();
        if parser_error_code(parser) as ::core::ffi::c_uint
            != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            fail_test(
                576 as ::core::ffi::c_int,
                b"check failed: XML_GetErrorCode(parser) == XML_ERROR_NONE\0",
            );
        }
        if stop_parser(parser, resumable) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            fail_test(
                577 as ::core::ffi::c_int,
                b"check failed: XML_StopParser(parser, resumable) == XML_STATUS_ERROR\0",
            );
        }
        if parser_error_code(parser) as ::core::ffi::c_uint
            != XML_ERROR_NOT_STARTED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            fail_test(
                578 as ::core::ffi::c_int,
                b"check failed: XML_GetErrorCode(parser) == XML_ERROR_NOT_STARTED\0",
            );
        }
        parser_free(parser);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn accumulate_characters_ext_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    unsafe {
        let test_data: *mut ExtHdlrData = userData as *mut ExtHdlrData;
        CharData_AppendXMLChars((*test_data).storage, s, len);
    }
}
extern "C" fn test_renter_loop_finite_content() {
    set_test_info_name(
        b"test_renter_loop_finite_content\0",
        594 as ::core::ffi::c_int,
    );
    let mut storage = CharData {
        count: 0,
        data: [0; 2048],
    };
    char_data_init(&mut storage);
    let text = b"<!DOCTYPE doc [\n<!ENTITY e1 '&e2;'>\n<!ENTITY e2 '&e3;'>\n<!ENTITY e3 SYSTEM '012.ent'>\n<!ENTITY e4 '&e5;'>\n<!ENTITY e5 '(e5)'>\n<!ELEMENT doc (#PCDATA)>\n]>\n<doc>&e1;</doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut test_data = ext_hdlr_data {
        parse_text: b"&e4;\n\0".as_ptr() as *const ::core::ffi::c_char,
        handler: Some(external_entity_null_loader),
        storage: &mut storage,
    };
    let expected = b"(e5)\n\0".as_ptr() as *const XML_Char;
    let parser = parser_create();
    if parser.is_null() {
        fail_test(610 as ::core::ffi::c_int, b"check failed: parser != NULL\0");
    }
    parser_set_user_data(parser, (&mut test_data as *mut ExtHdlrData).cast());
    parser_set_external_entity_ref_handler(parser, Some(external_entity_oneshot_loader));
    parser_set_character_data_handler(parser, Some(accumulate_characters_ext_handler));
    if parse_single_bytes(
        parser,
        text,
        c_string_len(text),
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure(parser, 616 as ::core::ffi::c_int);
    }
    char_data_check_xml_chars(&mut storage, expected);
    parser_free(parser);
}
fn dup_original_string(parser: XML_Parser) -> *mut ::core::ffi::c_char {
    let byte_count = parser_current_byte_count(parser);
    if byte_count < 0 {
        fail_test(
            628 as ::core::ffi::c_int,
            b"check failed: byte_count >= 0\0",
        );
    }
    let mut offset = -(1 as ::core::ffi::c_int);
    let mut size = -(1 as ::core::ffi::c_int);
    let context = parser_input_context(parser, &mut offset, &mut size);
    if context.is_null() {
        fail_test(
            636 as ::core::ffi::c_int,
            b"check failed: context != NULL\0",
        );
    }
    if offset < 0 {
        fail_test(637 as ::core::ffi::c_int, b"check failed: offset >= 0\0");
    }
    if size < 0 {
        fail_test(638 as ::core::ffi::c_int, b"check failed: size >= 0\0");
    }
    duplicate_c_string(context.wrapping_add(offset as usize), byte_count as size_t)
}
extern "C" fn on_characters_issue_980(
    user_data: *mut ::core::ffi::c_void,
    _s: *const XML_Char,
    _len: ::core::ffi::c_int,
) {
    let parser = user_data as XML_Parser;
    let original_string = dup_original_string(parser);
    if original_string.is_null() {
        fail_test(
            655 as ::core::ffi::c_int,
            b"check failed: original_string != NULL\0",
        );
    }
    if compare_strings(
        original_string,
        b"&draft.day;\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
    {
        fail_test(
            656 as ::core::ffi::c_int,
            b"check failed: strcmp(original_string, \"&draft.day;\") == 0\0",
        );
    }
    free_ptr(original_string.cast());
}
extern "C" fn test_misc_expected_event_ptr_issue_980() {
    set_test_info_name(
        b"test_misc_expected_event_ptr_issue_980\0",
        663 as ::core::ffi::c_int,
    );
    let doc = b"<!DOCTYPE day [\n  <!ENTITY draft.day '10'>\n]>\n<day>&draft.day;</day>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let parser = parser_create();
    parser_set_user_data(parser, parser.cast());
    parser_set_character_data_handler(parser, Some(on_characters_issue_980));
    if parse_single_bytes(
        parser,
        doc,
        c_string_len(doc),
        1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            677 as ::core::ffi::c_int,
            b"check failed: _XML_Parse_SINGLE_BYTES(parser, doc, (int)strlen(doc), XML_TRUE) == XML_STATUS_OK\0",
        );
    }
    parser_free(parser);
}
extern "C" fn test_misc_sync_entity_tolerated() {
    set_test_info_name(
        b"test_misc_sync_entity_tolerated\0",
        683 as ::core::ffi::c_int,
    );
    let doc = b"<!DOCTYPE t0 [\n   <!ENTITY a '<t1></t1>'>\n   <!ENTITY b '<t2>two</t2>'>\n   <!ENTITY c '<t3>three<t4>four</t4>three</t3>'>\n   <!ENTITY d '<t5>&b;</t5>'>\n]>\n<t0>&a;&b;&c;&d;</t0>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let parser = parser_create();
    if parse_single_bytes(
        parser,
        doc,
        c_string_len(doc),
        1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_test(
            695 as ::core::ffi::c_int,
            b"check failed: _XML_Parse_SINGLE_BYTES(parser, doc, (int)strlen(doc), XML_TRUE) == XML_STATUS_OK\0",
        );
    }
    parser_free(parser);
}
extern "C" fn test_misc_async_entity_rejected() {
    set_test_info_name(
        b"test_misc_async_entity_rejected\0",
        701 as ::core::ffi::c_int,
    );
    let cases: [test_case; 5] = [
            test_case {
                doc: b"<!DOCTYPE t0 [\n   <!ENTITY open '<t1>'>\n   <!ENTITY close '</t1>'>\n]>\n<t0>&open;&close;</t0>\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expectedStatusNoGE: XML_STATUS_OK,
                expectedErrorNoGE: XML_ERROR_NONE,
                expectedErrorLine: 5 as XML_Size,
                expectedErrorColumn: 4 as XML_Size,
            },
            test_case {
                doc: b"<!DOCTYPE t0 [\n  <!ENTITY g0 ''>\n  <!ENTITY g1 '&g0;</t1>'>\n]>\n<t0><t1>&g1;</t0>\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expectedStatusNoGE: XML_STATUS_ERROR,
                expectedErrorNoGE: XML_ERROR_TAG_MISMATCH,
                expectedErrorLine: 5 as XML_Size,
                expectedErrorColumn: 8 as XML_Size,
            },
            test_case {
                doc: b"<!DOCTYPE t0 [\n  <!ENTITY g0 ''>\n  <!ENTITY g1 '&g0;</t0>'>\n]>\n<t0>&g1;\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expectedStatusNoGE: XML_STATUS_ERROR,
                expectedErrorNoGE: XML_ERROR_NO_ELEMENTS,
                expectedErrorLine: 5 as XML_Size,
                expectedErrorColumn: 4 as XML_Size,
            },
            test_case {
                doc: b"<!DOCTYPE t0 [\n  <!ENTITY g0 ''>\n  <!ENTITY g1 '<t1>&g0;'>\n]>\n<t0>&g1;</t1></t0>\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expectedStatusNoGE: XML_STATUS_ERROR,
                expectedErrorNoGE: XML_ERROR_TAG_MISMATCH,
                expectedErrorLine: 5 as XML_Size,
                expectedErrorColumn: 4 as XML_Size,
            },
            test_case {
                doc: b"<!DOCTYPE t0 [\n  <!ENTITY open '<t1>'>\n  <!ENTITY close '</t1>'>\n]>\n<t0><t1>&close;&open;</t1></t0>\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expectedStatusNoGE: XML_STATUS_OK,
                expectedErrorNoGE: XML_ERROR_NONE,
                expectedErrorLine: 5 as XML_Size,
                expectedErrorColumn: 8 as XML_Size,
            },
        ];
    let mut i: size_t = 0;
    while i < cases.len() {
        let testCase: test_case = cases[i as usize];
        set_case_subtest!(i as ::core::ffi::c_int);
        let doc: *const ::core::ffi::c_char = testCase.doc;
        let expectedStatus: XML_Status = XML_STATUS_ERROR;
        let expectedError: XML_Error = XML_ERROR_ASYNC_ENTITY;
        let parser = parser_create();
        if parse_single_bytes(
            parser,
            doc,
            c_string_len(doc),
            1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != expectedStatus as ::core::ffi::c_uint
        {
            fail_test(
                    763 as ::core::ffi::c_int,
                    b"check failed: _XML_Parse_SINGLE_BYTES(parser, doc, (int)strlen(doc), XML_TRUE) == expectedStatus\0",
                );
        }
        if parser_error_code(parser) as ::core::ffi::c_uint != expectedError as ::core::ffi::c_uint
        {
            fail_test(
                764 as ::core::ffi::c_int,
                b"check failed: XML_GetErrorCode(parser) == expectedError\0",
            );
        }
        if parser_current_line(parser) != testCase.expectedErrorLine {
            fail_test(
                766 as ::core::ffi::c_int,
                b"check failed: XML_GetCurrentLineNumber(parser) == testCase.expectedErrorLine\0",
            );
        }
        if parser_current_column(parser) != testCase.expectedErrorColumn {
            fail_test(
                    768 as ::core::ffi::c_int,
                    b"check failed: XML_GetCurrentColumnNumber(parser) == testCase.expectedErrorColumn\0",
                );
        }
        parser_free(parser);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_miscellaneous_test_case(mut s: *mut Suite) {
    unsafe {
        let mut tc_misc: *mut TCase =
            tcase_create(b"miscellaneous tests\0".as_ptr() as *const ::core::ffi::c_char);
        suite_add_tcase(s, tc_misc);
        tcase_add_checked_fixture(tc_misc, None, Some(basic_teardown_shim));
        tcase_add_test(tc_misc, Some(test_misc_alloc_create_parser));
        tcase_add_test(tc_misc, Some(test_misc_alloc_create_parser_with_encoding));
        tcase_add_test(tc_misc, Some(test_misc_null_parser));
        tcase_add_test(tc_misc, Some(test_misc_error_string));
        tcase_add_test(tc_misc, Some(test_misc_version));
        tcase_add_test(tc_misc, Some(test_misc_features));
        tcase_add_test(tc_misc, Some(test_misc_attribute_leak));
        tcase_add_test(tc_misc, Some(test_misc_utf16le));
        tcase_add_test(tc_misc, Some(test_misc_stop_during_end_handler_issue_240_1));
        tcase_add_test(tc_misc, Some(test_misc_stop_during_end_handler_issue_240_2));
        tcase_add_test__ifdef_xml_dtd(
            tc_misc,
            Some(test_misc_deny_internal_entity_closing_doctype_issue_317_shim),
        );
        tcase_add_test(tc_misc, Some(test_misc_tag_mismatch_reset_leak));
        tcase_add_test(
            tc_misc,
            Some(test_misc_create_external_entity_parser_with_null_context),
        );
        tcase_add_test(tc_misc, Some(test_misc_general_entities_support));
        tcase_add_test(tc_misc, Some(test_misc_char_handler_stop_without_leak));
        tcase_add_test(tc_misc, Some(test_misc_resumeparser_not_crashing));
        tcase_add_test(tc_misc, Some(test_misc_stopparser_rejects_unstarted_parser));
        tcase_add_test__if_xml_ge(tc_misc, Some(test_renter_loop_finite_content));
        tcase_add_test(tc_misc, Some(test_misc_expected_event_ptr_issue_980));
        tcase_add_test(tc_misc, Some(test_misc_sync_entity_tolerated));
        tcase_add_test(tc_misc, Some(test_misc_async_entity_rejected));
    }
}
