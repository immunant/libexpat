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
    static mut g_parser: XML_Parser;
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
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 29] = unsafe {
    ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
        *b"void test_misc_version(void)\0",
    )
};
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const ASCII_0: ::core::ffi::c_int = 0x30 as ::core::ffi::c_int;
pub const ASCII_9: ::core::ffi::c_int = 0x39 as ::core::ffi::c_int;
pub const ASCII_PERIOD: ::core::ffi::c_int = 0x2e as ::core::ffi::c_int;
unsafe extern "C" fn test_misc_alloc_create_parser() {
    unsafe {
        _check_set_test_info(
            b"test_misc_alloc_create_parser\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            67 as ::core::ffi::c_int,
        );
        let mut memsuite: XML_Memory_Handling_Suite = XML_Memory_Handling_Suite {
            malloc_fcn: Some(
                duff_allocator as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
            ),
            realloc_fcn: Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
            free_fcn: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        };
        let mut i: ::core::ffi::c_uint = 0;
        let max_alloc_count: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
        i = 0 as ::core::ffi::c_uint;
        while i < max_alloc_count {
            g_allocation_count = i as ::core::ffi::c_int;
            g_parser = XML_ParserCreate_MM(
                ::core::ptr::null::<XML_Char>(),
                &raw mut memsuite,
                ::core::ptr::null::<XML_Char>(),
            );
            if !g_parser.is_null() {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == 0 as ::core::ffi::c_uint {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                80 as ::core::ffi::c_int,
                b"Parser unexpectedly ignored failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_int,
                b"Parser not created with max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_misc_alloc_create_parser_with_encoding() {
    unsafe {
        _check_set_test_info(
            b"test_misc_alloc_create_parser_with_encoding\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            87 as ::core::ffi::c_int,
        );
        let mut memsuite: XML_Memory_Handling_Suite = XML_Memory_Handling_Suite {
            malloc_fcn: Some(
                duff_allocator as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
            ),
            realloc_fcn: Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
            free_fcn: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        };
        let mut i: ::core::ffi::c_uint = 0;
        let max_alloc_count: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
        i = 0 as ::core::ffi::c_uint;
        while i < max_alloc_count {
            g_allocation_count = i as ::core::ffi::c_int;
            g_parser = XML_ParserCreate_MM(
                b"us-ascii\0".as_ptr() as *const XML_Char,
                &raw mut memsuite,
                ::core::ptr::null::<XML_Char>(),
            );
            if !g_parser.is_null() {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i == 0 as ::core::ffi::c_uint {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                100 as ::core::ffi::c_int,
                b"Parser ignored failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_int,
                b"Parser not created with max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_misc_null_parser() {
    unsafe {
        _check_set_test_info(
            b"test_misc_null_parser\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            109 as ::core::ffi::c_int,
        );
        XML_ParserFree(::core::ptr::null_mut::<XML_ParserStruct>());
    }
}
unsafe extern "C" fn test_misc_error_string() {
    unsafe {
        _check_set_test_info(
            b"test_misc_error_string\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            125 as ::core::ffi::c_int,
        );
        let mut trickery: C2Rust_Unnamed = C2Rust_Unnamed {
            xml_error: XML_ERROR_NONE,
        };
        if !(::core::mem::size_of::<XML_Error>() as usize
            == ::core::mem::size_of::<::core::ffi::c_int>() as usize)
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                132 as ::core::ffi::c_int,
                b"check failed: sizeof(enum XML_Error) == sizeof(int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        trickery.integer = -(1 as ::core::ffi::c_int);
        if !XML_ErrorString(trickery.xml_error).is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                136 as ::core::ffi::c_int,
                b"Negative error code not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        trickery.integer = 100 as ::core::ffi::c_int;
        if !XML_ErrorString(trickery.xml_error).is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_int,
                b"Large error code not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
fn set_test_info(
    function: *const ::core::ffi::c_char,
    filename: *const ::core::ffi::c_char,
    lineno: ::core::ffi::c_int,
) {
    unsafe {
        _check_set_test_info(function, filename, lineno);
    }
}

fn fail(
    file: *const ::core::ffi::c_char,
    line: ::core::ffi::c_int,
    msg: *const ::core::ffi::c_char,
) -> ! {
    unsafe { _fail(file, line, msg) }
}

fn expat_version() -> Option<(XML_Expat_Version, &'static std::ffi::CStr)> {
    unsafe {
        let version_text = XML_ExpatVersion();
        if version_text.is_null() {
            None
        } else {
            Some((
                XML_ExpatVersionInfo(),
                std::ffi::CStr::from_ptr(version_text),
            ))
        }
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
    set_test_info(
        b"test_misc_version\0".as_ptr() as *const ::core::ffi::c_char,
        b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
        201 as ::core::ffi::c_int,
    );

    let (read_version, version_text) = match expat_version() {
        Some(version) => version,
        None => fail(
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            208 as ::core::ffi::c_int,
            b"Could not obtain version text\0".as_ptr() as *const ::core::ffi::c_char,
        ),
    };

    let parsed_version = match parse_version(version_text) {
        Some(version) => version,
        None => fail(
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            211 as ::core::ffi::c_int,
            b"Unable to parse version text\0".as_ptr() as *const ::core::ffi::c_char,
        ),
    };

    if !versions_equal(&read_version, &parsed_version) {
        fail(
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            213 as ::core::ffi::c_int,
            b"Version mismatch\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }

    if version_text.to_bytes() != b"expat_2.7.4" {
        fail(
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            217 as ::core::ffi::c_int,
            b"XML_*_VERSION in expat.h out of sync?\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn test_misc_features() {
    unsafe {
        _check_set_test_info(
            b"test_misc_features\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            222 as ::core::ffi::c_int,
        );
        let mut features: *const XML_Feature = XML_GetFeatureList();
        g_parser = ::core::ptr::null_mut::<XML_ParserStruct>();
        if features.is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                228 as ::core::ffi::c_int,
                b"Failed to get feature information\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            while (*features).feature as ::core::ffi::c_uint
                != XML_FEATURE_END as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                match (*features).feature as ::core::ffi::c_uint {
                    6 => {
                        if (*features).value as usize != ::core::mem::size_of::<XML_Char>() as usize
                        {
                            _fail(
                                b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                235 as ::core::ffi::c_int,
                                b"Incorrect size of XML_Char\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    7 => {
                        if (*features).value as usize
                            != ::core::mem::size_of::<XML_LChar>() as usize
                        {
                            _fail(
                                b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                239 as ::core::ffi::c_int,
                                b"Incorrect size of XML_LChar\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    _ => {}
                }
                features = features.offset(1);
            }
        };
    }
}
unsafe extern "C" fn test_misc_attribute_leak() {
    unsafe {
        _check_set_test_info(
            b"test_misc_attribute_leak\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            253 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<D xmlns:L=\"D\" l:a='' L:a=''/>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut memsuite: XML_Memory_Handling_Suite = XML_Memory_Handling_Suite {
            malloc_fcn: Some(
                tracking_malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void,
            ),
            realloc_fcn: Some(
                tracking_realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
            free_fcn: Some(tracking_free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
        };
        g_parser = XML_ParserCreate_MM(
            b"UTF-8\0".as_ptr() as *const XML_Char,
            &raw mut memsuite,
            b"\n\0".as_ptr() as *const XML_Char,
        );
        _expect_failure(
            text,
            XML_ERROR_UNBOUND_PREFIX,
            b"Unbound prefixes not found\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            259 as ::core::ffi::c_int,
        );
        XML_ParserFree(g_parser);
        g_parser = ::core::ptr::null_mut::<XML_ParserStruct>();
        if tracking_report() == 0 {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                265 as ::core::ffi::c_int,
                b"Memory leak found\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_misc_utf16le() {
    unsafe {
        _check_set_test_info(
            b"test_misc_utf16le\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            270 as ::core::ffi::c_int,
        );
        let text: [::core::ffi::c_char; 61] = ::core::mem::transmute::<
            [u8; 61],
            [::core::ffi::c_char; 61],
        >(
            *b"<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\x001\0.\x000\0'\0?\0>\0<\0q\0>\0H\0i\0<\0/\0q\0>\0\0",
        );
        let mut expected: *const XML_Char = b"Hi\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        g_parser = XML_ParserCreate(b"UTF-16LE\0".as_ptr() as *const XML_Char);
        if g_parser.is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                281 as ::core::ffi::c_int,
                b"Parser not created\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        CharData_Init(&raw mut storage);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            &raw const text as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 61]>() as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                288 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
unsafe extern "C" fn test_misc_stop_during_end_handler_issue_240_1() {
    unsafe {
        _check_set_test_info(
            b"test_misc_stop_during_end_handler_issue_240_1\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            293 as ::core::ffi::c_int,
        );
        let mut parser: XML_Parser = ::core::ptr::null_mut::<XML_ParserStruct>();
        let mut mydata: *mut DataIssue240 = ::core::ptr::null_mut::<DataIssue240>();
        let mut result: XML_Status = XML_STATUS_ERROR;
        let doc1: *const ::core::ffi::c_char =
            b"<doc><e1/><e><foo/></e></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        XML_SetElementHandler(
            parser,
            Some(
                start_element_issue_240
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
            Some(
                end_element_issue_240
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        mydata = malloc(::core::mem::size_of::<DataIssue240>() as size_t) as *mut DataIssue240;
        if mydata.is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                302 as ::core::ffi::c_int,
                b"check failed: mydata != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        (*mydata).parser = parser;
        (*mydata).deep = 0 as ::core::ffi::c_int;
        XML_SetUserData(parser, mydata as *mut ::core::ffi::c_void);
        result = _XML_Parse_SINGLE_BYTES(
            parser,
            doc1,
            strlen(doc1) as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        XML_ParserFree(parser);
        free(mydata as *mut ::core::ffi::c_void);
        if result as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                311 as ::core::ffi::c_int,
                b"Stopping the parser did not work as expected\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_misc_stop_during_end_handler_issue_240_2() {
    unsafe {
        _check_set_test_info(
            b"test_misc_stop_during_end_handler_issue_240_2\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            315 as ::core::ffi::c_int,
        );
        let mut parser: XML_Parser = ::core::ptr::null_mut::<XML_ParserStruct>();
        let mut mydata: *mut DataIssue240 = ::core::ptr::null_mut::<DataIssue240>();
        let mut result: XML_Status = XML_STATUS_ERROR;
        let doc2: *const ::core::ffi::c_char =
            b"<doc><elem/></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        XML_SetElementHandler(
            parser,
            Some(
                start_element_issue_240
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
            Some(
                end_element_issue_240
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
            ),
        );
        mydata = malloc(::core::mem::size_of::<DataIssue240>() as size_t) as *mut DataIssue240;
        if mydata.is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                324 as ::core::ffi::c_int,
                b"check failed: mydata != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        (*mydata).parser = parser;
        (*mydata).deep = 0 as ::core::ffi::c_int;
        XML_SetUserData(parser, mydata as *mut ::core::ffi::c_void);
        result = _XML_Parse_SINGLE_BYTES(
            parser,
            doc2,
            strlen(doc2) as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        XML_ParserFree(parser);
        free(mydata as *mut ::core::ffi::c_void);
        if result as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                333 as ::core::ffi::c_int,
                b"Stopping the parser did not work as expected\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
unsafe extern "C" fn test_misc_deny_internal_entity_closing_doctype_issue_317() {
    unsafe {
        _check_set_test_info(
            b"test_misc_deny_internal_entity_closing_doctype_issue_317\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            337 as ::core::ffi::c_int,
        );
        let inputOne: *const ::core::ffi::c_char = b"<!DOCTYPE d [\n<!ENTITY % element_d '<!ELEMENT d (#PCDATA)*>'>\n%element_d;\n<!ENTITY % e ']><d/>'>\n\n%e;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let inputTwo: *const ::core::ffi::c_char = b"<!DOCTYPE d [\n<!ENTITY % element_d '<!ELEMENT d (#PCDATA)*>'>\n%element_d;\n<!ENTITY % e1 ']><d/>'><!ENTITY % e2 '&#37;e1;'>\n\n%e2;\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let inputThree: *const ::core::ffi::c_char = b"<!DOCTYPE d [\n<!ENTITY % element_d '<!ELEMENT d (#PCDATA)*>'>\n%element_d;\n<!ENTITY % e ']><d'>\n\n%e;/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let inputIssue317: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY % element_doc '<!ELEMENT doc (#PCDATA)*>'>\n%element_doc;\n<!ENTITY % foo ']>\n<doc>Hell<oc (#PCDATA)*>'>\n%foo;\n]>\n<doc>Hello, world</dVc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let inputs: [*const ::core::ffi::c_char; 4] =
            [inputOne, inputTwo, inputThree, inputIssue317];
        let suspendOrNot: [XML_Bool; 2] = [XML_FALSE, XML_TRUE];
        let mut inputIndex: size_t = 0 as size_t;
        while inputIndex
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        {
            let mut suspendOrNotIndex: size_t = 0 as size_t;
            while suspendOrNotIndex
                < (::core::mem::size_of::<[XML_Bool; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<XML_Bool>() as usize)
            {
                let input: *const ::core::ffi::c_char = inputs[inputIndex as usize];
                let suspend: XML_Bool = suspendOrNot[suspendOrNotIndex as usize];
                if suspend as ::core::ffi::c_int != 0 && g_chunkSize > 0 as ::core::ffi::c_int {
                    return;
                }
                set_subtest(
                    b"[input=%d suspend=%s] %s\0".as_ptr() as *const ::core::ffi::c_char,
                    inputIndex as ::core::ffi::c_int,
                    if suspend as ::core::ffi::c_int != 0 {
                        b"true\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"false\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    input,
                );
                let mut parser: XML_Parser = ::core::ptr::null_mut::<XML_ParserStruct>();
                let mut parseResult: XML_Status = XML_STATUS_ERROR;
                let mut setParamEntityResult: ::core::ffi::c_int = 0;
                let mut lineNumber: XML_Size = 0;
                let mut columnNumber: XML_Size = 0;
                parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
                setParamEntityResult =
                    XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
                if setParamEntityResult != 1 as ::core::ffi::c_int {
                    _fail(
                        b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        398 as ::core::ffi::c_int,
                        b"Failed to set XML_PARAM_ENTITY_PARSING_ALWAYS.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                if suspend != 0 {
                    XML_SetUserData(parser, parser as *mut ::core::ffi::c_void);
                    XML_SetElementDeclHandler(
                        parser,
                        Some(
                            suspend_after_element_declaration
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *const XML_Char,
                                    *mut XML_Content,
                                ) -> (),
                        ),
                    );
                }
                if suspend != 0 {
                    parseResult = XML_Parse(
                        parser,
                        input,
                        strlen(input) as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    while parseResult as ::core::ffi::c_uint
                        == XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        parseResult = XML_ResumeParser(parser);
                    }
                    if parseResult as ::core::ffi::c_uint
                        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        parseResult = XML_Parse(
                            parser,
                            b"\0".as_ptr() as *const ::core::ffi::c_char,
                            0 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                    }
                    while parseResult as ::core::ffi::c_uint
                        == XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        parseResult = XML_ResumeParser(parser);
                    }
                } else {
                    parseResult = _XML_Parse_SINGLE_BYTES(
                        parser,
                        input,
                        strlen(input) as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    if parseResult as ::core::ffi::c_uint
                        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        parseResult = _XML_Parse_SINGLE_BYTES(
                            parser,
                            b"\0".as_ptr() as *const ::core::ffi::c_char,
                            0 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                    }
                }
                if parseResult as ::core::ffi::c_uint
                    != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    _fail(
                        b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        435 as ::core::ffi::c_int,
                        b"Parsing was expected to fail but succeeded.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                if XML_GetErrorCode(parser) as ::core::ffi::c_uint
                    != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    _fail(
                        b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        439 as ::core::ffi::c_int,
                        b"Error code does not match XML_ERROR_INVALID_TOKEN\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                lineNumber = XML_GetCurrentLineNumber(parser);
                if lineNumber != 6 as XML_Size {
                    _fail(
                        b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        443 as ::core::ffi::c_int,
                        b"XML_GetCurrentLineNumber does not work as expected.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                columnNumber = XML_GetCurrentColumnNumber(parser);
                if columnNumber != 0 as XML_Size {
                    _fail(
                        b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        447 as ::core::ffi::c_int,
                        b"XML_GetCurrentColumnNumber does not work as expected.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                XML_ParserFree(parser);
                suspendOrNotIndex = suspendOrNotIndex.wrapping_add(1);
            }
            inputIndex = inputIndex.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_misc_tag_mismatch_reset_leak() {
    unsafe {
        _check_set_test_info(
            b"test_misc_tag_mismatch_reset_leak\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            455 as ::core::ffi::c_int,
        );
        let text: *const ::core::ffi::c_char = b"<open xmlns='https://namespace1.test'></close>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut parser: XML_Parser =
            XML_ParserCreateNS(::core::ptr::null::<XML_Char>(), '\n' as i32 as XML_Char);
        if _XML_Parse_SINGLE_BYTES(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                462 as ::core::ffi::c_int,
                b"Call to parse was expected to fail\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(parser) as ::core::ffi::c_uint
            != XML_ERROR_TAG_MISMATCH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                464 as ::core::ffi::c_int,
                b"Call to parse was expected to fail from a closing tag mismatch\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserReset(parser, ::core::ptr::null::<XML_Char>());
        if _XML_Parse_SINGLE_BYTES(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                470 as ::core::ffi::c_int,
                b"Call to parse was expected to fail\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(parser) as ::core::ffi::c_uint
            != XML_ERROR_TAG_MISMATCH as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                472 as ::core::ffi::c_int,
                b"Call to parse was expected to fail from a closing tag mismatch\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
    }
}
unsafe extern "C" fn test_misc_create_external_entity_parser_with_null_context() {
    unsafe {
        _check_set_test_info(
            b"test_misc_create_external_entity_parser_with_null_context\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            479 as ::core::ffi::c_int,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        let mut ext_parser: XML_Parser = XML_ExternalEntityParserCreate(
            parser,
            ::core::ptr::null::<XML_Char>(),
            ::core::ptr::null::<XML_Char>(),
        );
        if ext_parser.is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                487 as ::core::ffi::c_int,
                b"check failed: ext_parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(ext_parser);
        XML_ParserFree(parser);
    }
}
unsafe extern "C" fn test_misc_general_entities_support() {
    unsafe {
        _check_set_test_info(
            b"test_misc_general_entities_support\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            496 as ::core::ffi::c_int,
        );
        let doc: *const ::core::ffi::c_char = b"<!DOCTYPE r [\n<!ENTITY e1 'v1'>\n<!ENTITY e2 SYSTEM 'v2'>\n]>\n<r a1='[&e1;]'>[&e1;][&e2;][&amp;&apos;&gt;&lt;&quot;]</r>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        XML_SetUserData(parser, &raw mut storage as *mut ::core::ffi::c_void);
        XML_SetStartElementHandler(
            parser,
            Some(
                accumulate_start_element
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetExternalEntityRefHandler(
            parser,
            Some(
                external_entity_failer__if_not_xml_ge
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetEntityDeclHandler(
            parser,
            Some(
                accumulate_entity_decl
                    as unsafe extern "C" fn(
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
            ),
        );
        XML_SetCharacterDataHandler(
            parser,
            Some(
                accumulate_characters
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            parser,
            doc,
            strlen(doc) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                517 as ::core::ffi::c_int,
            );
        }
        XML_ParserFree(parser);
        CharData_CheckXMLChars(
            &raw mut storage,
            b"e1=v1\ne2=(null)\n(r(a1=[v1]))\n[v1][][&'><\"]\0".as_ptr() as *const XML_Char,
        );
    }
}
unsafe extern "C" fn resumable_stopping_character_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    unsafe {
        let mut parser: XML_Parser = userData as XML_Parser;
        XML_StopParser(parser, XML_TRUE);
    }
}
unsafe extern "C" fn test_misc_char_handler_stop_without_leak() {
    unsafe {
        _check_set_test_info(
            b"test_misc_char_handler_stop_without_leak\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            550 as ::core::ffi::c_int,
        );
        let data: *const ::core::ffi::c_char =
            b"<!DOCTYPE t1[<!ENTITY e1 'angle<'><!ENTITY e2 '&e1;'>]><t1>&e2;\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                554 as ::core::ffi::c_int,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_SetUserData(parser, parser as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            parser,
            Some(
                resumable_stopping_character_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        _XML_Parse_SINGLE_BYTES(
            parser,
            data,
            strlen(data) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        );
        XML_ParserFree(parser);
    }
}
unsafe extern "C" fn test_misc_resumeparser_not_crashing() {
    unsafe {
        _check_set_test_info(
            b"test_misc_resumeparser_not_crashing\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            562 as ::core::ffi::c_int,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        XML_GetBuffer(parser, 1 as ::core::ffi::c_int);
        XML_StopParser(parser, XML_TRUE);
        XML_ResumeParser(parser);
        XML_ParserFree(parser);
    }
}
unsafe extern "C" fn test_misc_stopparser_rejects_unstarted_parser() {
    unsafe {
        _check_set_test_info(
            b"test_misc_stopparser_rejects_unstarted_parser\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            571 as ::core::ffi::c_int,
        );
        let cases: [XML_Bool; 2] = [XML_TRUE, XML_FALSE];
        let mut i: size_t = 0 as size_t;
        while i
            < (::core::mem::size_of::<[XML_Bool; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<XML_Bool>() as usize)
        {
            let resumable: XML_Bool = cases[i as usize];
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if !(XML_GetErrorCode(parser) as ::core::ffi::c_uint
                == XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                _fail(
                    b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                    576 as ::core::ffi::c_int,
                    b"check failed: XML_GetErrorCode(parser) == XML_ERROR_NONE\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if !(XML_StopParser(parser, resumable) as ::core::ffi::c_uint
                == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                _fail(
                    b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                    577 as ::core::ffi::c_int,
                    b"check failed: XML_StopParser(parser, resumable) == XML_STATUS_ERROR\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !(XML_GetErrorCode(parser) as ::core::ffi::c_uint
                == XML_ERROR_NOT_STARTED as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                _fail(
                    b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                    578 as ::core::ffi::c_int,
                    b"check failed: XML_GetErrorCode(parser) == XML_ERROR_NOT_STARTED\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            XML_ParserFree(parser);
            i = i.wrapping_add(1);
        }
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
unsafe extern "C" fn test_renter_loop_finite_content() {
    unsafe {
        _check_set_test_info(
            b"test_renter_loop_finite_content\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            594 as ::core::ffi::c_int,
        );
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        let text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY e1 '&e2;'>\n<!ENTITY e2 '&e3;'>\n<!ENTITY e3 SYSTEM '012.ent'>\n<!ENTITY e4 '&e5;'>\n<!ENTITY e5 '(e5)'>\n<!ELEMENT doc (#PCDATA)>\n]>\n<doc>&e1;</doc>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut test_data: ExtHdlrData = ext_hdlr_data {
            parse_text: b"&e4;\n\0".as_ptr() as *const ::core::ffi::c_char,
            handler: Some(
                external_entity_null_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
            storage: &raw mut storage,
        };
        let expected: *const XML_Char = b"(e5)\n\0".as_ptr() as *const XML_Char;
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                610 as ::core::ffi::c_int,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_SetUserData(parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetExternalEntityRefHandler(
            parser,
            Some(
                external_entity_oneshot_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetCharacterDataHandler(
            parser,
            Some(
                accumulate_characters_ext_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                parser,
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                616 as ::core::ffi::c_int,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
        XML_ParserFree(parser);
    }
}
unsafe extern "C" fn dup_original_string(mut parser: XML_Parser) -> *mut ::core::ffi::c_char {
    unsafe {
        let byte_count: ::core::ffi::c_int = XML_GetCurrentByteCount(parser) as ::core::ffi::c_int;
        if !(byte_count >= 0 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                628 as ::core::ffi::c_int,
                b"check failed: byte_count >= 0\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut offset: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut size: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let context: *const ::core::ffi::c_char =
            XML_GetInputContext(parser, &raw mut offset, &raw mut size)
                as *const ::core::ffi::c_char;
        if context.is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                636 as ::core::ffi::c_int,
                b"check failed: context != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(offset >= 0 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                637 as ::core::ffi::c_int,
                b"check failed: offset >= 0\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(size >= 0 as ::core::ffi::c_int) {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                638 as ::core::ffi::c_int,
                b"check failed: size >= 0\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return portable_strndup(context.offset(offset as isize), byte_count as size_t);
    }
}
unsafe extern "C" fn on_characters_issue_980(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    unsafe {
        let mut parser: XML_Parser = userData as XML_Parser;
        let original_string: *mut ::core::ffi::c_char =
            dup_original_string(parser) as *mut ::core::ffi::c_char;
        if original_string.is_null() {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                655 as ::core::ffi::c_int,
                b"check failed: original_string != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(strcmp(
            original_string,
            b"&draft.day;\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int)
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                656 as ::core::ffi::c_int,
                b"check failed: strcmp(original_string, \"&draft.day;\") == 0\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        free(original_string as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn test_misc_expected_event_ptr_issue_980() {
    unsafe {
        _check_set_test_info(
            b"test_misc_expected_event_ptr_issue_980\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            663 as ::core::ffi::c_int,
        );
        let doc: *const ::core::ffi::c_char =
            b"<!DOCTYPE day [\n  <!ENTITY draft.day '10'>\n]>\n<day>&draft.day;</day>\n\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        XML_SetUserData(parser, parser as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            parser,
            Some(
                on_characters_issue_980
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if !(_XML_Parse_SINGLE_BYTES(
            parser,
            doc,
            strlen(doc) as ::core::ffi::c_int,
            1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                677 as ::core::ffi::c_int,
                b"check failed: _XML_Parse_SINGLE_BYTES(parser, doc, (int)strlen(doc), XML_TRUE) == XML_STATUS_OK\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
    }
}
unsafe extern "C" fn test_misc_sync_entity_tolerated() {
    unsafe {
        _check_set_test_info(
            b"test_misc_sync_entity_tolerated\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            683 as ::core::ffi::c_int,
        );
        let doc: *const ::core::ffi::c_char = b"<!DOCTYPE t0 [\n   <!ENTITY a '<t1></t1>'>\n   <!ENTITY b '<t2>two</t2>'>\n   <!ENTITY c '<t3>three<t4>four</t4>three</t3>'>\n   <!ENTITY d '<t5>&b;</t5>'>\n]>\n<t0>&a;&b;&c;&d;</t0>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if !(_XML_Parse_SINGLE_BYTES(
            parser,
            doc,
            strlen(doc) as ::core::ffi::c_int,
            1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            _fail(
                b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                695 as ::core::ffi::c_int,
                b"check failed: _XML_Parse_SINGLE_BYTES(parser, doc, (int)strlen(doc), XML_TRUE) == XML_STATUS_OK\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parser);
    }
}
unsafe extern "C" fn test_misc_async_entity_rejected() {
    unsafe {
        _check_set_test_info(
            b"test_misc_async_entity_rejected\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
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
        let mut i: size_t = 0 as size_t;
        while i
            < (::core::mem::size_of::<[test_case; 5]>() as usize)
                .wrapping_div(::core::mem::size_of::<test_case>() as usize)
        {
            let testCase: test_case = cases[i as usize];
            set_subtest(
                b"cases[%d]\0".as_ptr() as *const ::core::ffi::c_char,
                i as ::core::ffi::c_int,
            );
            let doc: *const ::core::ffi::c_char = testCase.doc;
            let expectedStatus: XML_Status = XML_STATUS_ERROR;
            let expectedError: XML_Error = XML_ERROR_ASYNC_ENTITY;
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            if !(_XML_Parse_SINGLE_BYTES(
                parser,
                doc,
                strlen(doc) as ::core::ffi::c_int,
                1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                == expectedStatus as ::core::ffi::c_uint)
            {
                _fail(
                    b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    763 as ::core::ffi::c_int,
                    b"check failed: _XML_Parse_SINGLE_BYTES(parser, doc, (int)strlen(doc), XML_TRUE) == expectedStatus\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !(XML_GetErrorCode(parser) as ::core::ffi::c_uint
                == expectedError as ::core::ffi::c_uint)
            {
                _fail(
                    b"/root/work/expat/tests/misc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                    764 as ::core::ffi::c_int,
                    b"check failed: XML_GetErrorCode(parser) == expectedError\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            if !(XML_GetCurrentLineNumber(parser) == testCase.expectedErrorLine) {
                _fail(
                    b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    766 as ::core::ffi::c_int,
                    b"check failed: XML_GetCurrentLineNumber(parser) == testCase.expectedErrorLine\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !(XML_GetCurrentColumnNumber(parser) == testCase.expectedErrorColumn) {
                _fail(
                    b"/root/work/expat/tests/misc_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    768 as ::core::ffi::c_int,
                    b"check failed: XML_GetCurrentColumnNumber(parser) == testCase.expectedErrorColumn\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
            XML_ParserFree(parser);
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_miscellaneous_test_case(mut s: *mut Suite) {
    unsafe {
        let mut tc_misc: *mut TCase =
            tcase_create(b"miscellaneous tests\0".as_ptr() as *const ::core::ffi::c_char);
        suite_add_tcase(s, tc_misc);
        tcase_add_checked_fixture(
            tc_misc,
            None,
            Some(basic_teardown as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_alloc_create_parser as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_alloc_create_parser_with_encoding as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_null_parser as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_error_string as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_version as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_features as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_attribute_leak as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_utf16le as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_stop_during_end_handler_issue_240_1 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_stop_during_end_handler_issue_240_2 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_misc,
            Some(
                test_misc_deny_internal_entity_closing_doctype_issue_317
                    as unsafe extern "C" fn() -> (),
            ),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_tag_mismatch_reset_leak as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(
                test_misc_create_external_entity_parser_with_null_context
                    as unsafe extern "C" fn() -> (),
            ),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_general_entities_support as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_char_handler_stop_without_leak as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_resumeparser_not_crashing as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_stopparser_rejects_unstarted_parser as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_misc,
            Some(test_renter_loop_finite_content as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_expected_event_ptr_issue_980 as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_sync_entity_tolerated as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_misc,
            Some(test_misc_async_entity_rejected as unsafe extern "C" fn() -> ()),
        );
    }
}
