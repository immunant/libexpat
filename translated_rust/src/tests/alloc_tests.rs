use std::ffi::CStr;
use std::ptr::NonNull;

extern "C" {
    pub type XML_ParserStruct;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn XML_SetElementDeclHandler(parser: XML_Parser, eldecl: XML_ElementDeclHandler);
    fn XML_SetAttlistDeclHandler(parser: XML_Parser, attdecl: XML_AttlistDeclHandler);
    fn XML_SetXmlDeclHandler(parser: XML_Parser, xmldecl: XML_XmlDeclHandler);
    fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser;
    fn XML_ParserCreate_MM(
        encoding: *const XML_Char,
        memsuite: *const XML_Memory_Handling_Suite,
        namespaceSeparator: *const XML_Char,
    ) -> XML_Parser;
    fn XML_ParserReset(parser: XML_Parser, encoding: *const XML_Char) -> XML_Bool;
    fn XML_SetEntityDeclHandler(parser: XML_Parser, handler: XML_EntityDeclHandler);
    fn XML_SetStartElementHandler(parser: XML_Parser, handler: XML_StartElementHandler);
    fn XML_SetCharacterDataHandler(parser: XML_Parser, handler: XML_CharacterDataHandler);
    fn XML_SetProcessingInstructionHandler(
        parser: XML_Parser,
        handler: XML_ProcessingInstructionHandler,
    );
    fn XML_SetCommentHandler(parser: XML_Parser, handler: XML_CommentHandler);
    fn XML_SetCdataSectionHandler(
        parser: XML_Parser,
        start: XML_StartCdataSectionHandler,
        end: XML_EndCdataSectionHandler,
    );
    fn XML_SetDefaultHandler(parser: XML_Parser, handler: XML_DefaultHandler);
    fn XML_SetDoctypeDeclHandler(
        parser: XML_Parser,
        start: XML_StartDoctypeDeclHandler,
        end: XML_EndDoctypeDeclHandler,
    );
    fn XML_SetUnparsedEntityDeclHandler(parser: XML_Parser, handler: XML_UnparsedEntityDeclHandler);
    fn XML_SetNotationDeclHandler(parser: XML_Parser, handler: XML_NotationDeclHandler);
    fn XML_SetExternalEntityRefHandler(parser: XML_Parser, handler: XML_ExternalEntityRefHandler);
    fn XML_SetUnknownEncodingHandler(
        parser: XML_Parser,
        handler: XML_UnknownEncodingHandler,
        encodingHandlerData: *mut ::core::ffi::c_void,
    );
    fn XML_SetUserData(parser: XML_Parser, userData: *mut ::core::ffi::c_void);
    fn XML_SetEncoding(parser: XML_Parser, encoding: *const XML_Char) -> XML_Status;
    fn XML_UseForeignDTD(parser: XML_Parser, useDTD: XML_Bool) -> XML_Error;
    fn XML_SetBase(parser: XML_Parser, base: *const XML_Char) -> XML_Status;
    fn XML_GetBuffer(parser: XML_Parser, len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    fn XML_ParseBuffer(
        parser: XML_Parser,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
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
    fn XML_MemMalloc(parser: XML_Parser, size: size_t) -> *mut ::core::ffi::c_void;
    fn XML_MemRealloc(
        parser: XML_Parser,
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn XML_MemFree(parser: XML_Parser, ptr: *mut ::core::ffi::c_void);
    fn XML_ParserFree(parser: XML_Parser);
    fn XML_SetAllocTrackerMaximumAmplification(
        parser: XML_Parser,
        maximumAmplificationFactor: ::core::ffi::c_float,
    ) -> XML_Bool;
    fn XML_SetAllocTrackerActivationThreshold(
        parser: XML_Parser,
        activationThresholdBytes: ::core::ffi::c_ulonglong,
    ) -> XML_Bool;
    static mut g_reparseDeferralEnabledDefault: XML_Bool;
    fn expat_malloc(
        parser: XML_Parser,
        size: size_t,
        sourceLine: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn expat_free(
        parser: XML_Parser,
        ptr: *mut ::core::ffi::c_void,
        sourceLine: ::core::ffi::c_int,
    );
    fn expat_realloc(
        parser: XML_Parser,
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
        sourceLine: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
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
    static mut g_parser: XML_Parser;
    static mut get_buffer_test_text: *const ::core::ffi::c_char;
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
    static mut g_reallocation_count: ::core::ffi::c_int;
    fn duff_allocator(size: size_t) -> *mut ::core::ffi::c_void;
    fn duff_reallocator(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn init_dummy_handlers();
    fn get_dummy_handler_flags() -> ::core::ffi::c_ulong;
    fn dummy_xdecl_handler(
        userData: *mut ::core::ffi::c_void,
        version: *const XML_Char,
        encoding: *const XML_Char,
        standalone: ::core::ffi::c_int,
    );
    fn dummy_start_doctype_handler(
        userData: *mut ::core::ffi::c_void,
        doctypeName: *const XML_Char,
        sysid: *const XML_Char,
        pubid: *const XML_Char,
        has_internal_subset: ::core::ffi::c_int,
    );
    fn dummy_end_doctype_handler(userData: *mut ::core::ffi::c_void);
    fn dummy_entity_decl_handler(
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
    fn dummy_notation_decl_handler(
        userData: *mut ::core::ffi::c_void,
        notationName: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    );
    fn dummy_element_decl_handler(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        model: *mut XML_Content,
    );
    fn dummy_attlist_decl_handler(
        userData: *mut ::core::ffi::c_void,
        elname: *const XML_Char,
        attname: *const XML_Char,
        att_type: *const XML_Char,
        dflt: *const XML_Char,
        isrequired: ::core::ffi::c_int,
    );
    fn dummy_comment_handler(userData: *mut ::core::ffi::c_void, data: *const XML_Char);
    fn dummy_pi_handler(
        userData: *mut ::core::ffi::c_void,
        target: *const XML_Char,
        data: *const XML_Char,
    );
    fn dummy_start_cdata_handler(userData: *mut ::core::ffi::c_void);
    fn dummy_end_cdata_handler(userData: *mut ::core::ffi::c_void);
    fn dummy_unparsed_entity_decl_handler(
        userData: *mut ::core::ffi::c_void,
        entityName: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
        notationName: *const XML_Char,
    );
    fn dummy_start_doctype_decl_handler(
        userData: *mut ::core::ffi::c_void,
        doctypeName: *const XML_Char,
        sysid: *const XML_Char,
        pubid: *const XML_Char,
        has_internal_subset: ::core::ffi::c_int,
    );
    fn dummy_end_doctype_decl_handler(userData: *mut ::core::ffi::c_void);
    fn unknown_released_encoding_handler(
        data: *mut ::core::ffi::c_void,
        encoding: *const XML_Char,
        info: *mut XML_Encoding,
    ) -> ::core::ffi::c_int;
    fn long_encoding_handler(
        userData: *mut ::core::ffi::c_void,
        encoding: *const XML_Char,
        info: *mut XML_Encoding,
    ) -> ::core::ffi::c_int;
    fn external_entity_optioner(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_faulter(
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
    fn external_entity_public(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_duff_loader(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_dbl_handler(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_dbl_handler_2(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_alloc_set_encoding(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_reallocator(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_alloc(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn external_entity_parser_create_alloc_fail_handler(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
    fn record_element_start_handler(
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
pub type XML_AttlistDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type XML_XmlDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
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
pub type XML_CharacterDataHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_ProcessingInstructionHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *const XML_Char) -> ()>;
pub type XML_CommentHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> ()>;
pub type XML_StartCdataSectionHandler =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_EndCdataSectionHandler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type XML_DefaultHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
pub type XML_StartDoctypeDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type XML_EndDoctypeDeclHandler = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
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
pub type XML_UnparsedEntityDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
        *const XML_Char,
    ) -> (),
>;
pub type XML_NotationDeclHandler = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
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
pub type XML_ParamEntityParsing = ::core::ffi::c_uint;
pub const XML_PARAM_ENTITY_PARSING_ALWAYS: XML_ParamEntityParsing = 2;
pub const XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE: XML_ParamEntityParsing = 1;
pub const XML_PARAM_ENTITY_PARSING_NEVER: XML_ParamEntityParsing = 0;
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
pub struct ExtOption {
    pub system_id: *const XML_Char,
    pub parse_text: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ext_faults {
    pub parse_text: *const ::core::ffi::c_char,
    pub fail_text: *const ::core::ffi::c_char,
    pub encoding: *const XML_Char,
    pub error: XML_Error,
}
pub type ExtFaults = ext_faults;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 37] = unsafe {
    ::core::mem::transmute::<[u8; 37], [::core::ffi::c_char; 37]>(
        *b"void test_alloc_realloc_buffer(void)\0",
    )
};
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
pub const EXPAT_MALLOC_ALIGNMENT: usize = ::core::mem::size_of::<::core::ffi::c_longlong>();
pub const EXPAT_MALLOC_PADDING: usize = (::core::mem::size_of::<::core::ffi::c_longlong>()
    as usize)
    .wrapping_sub(::core::mem::size_of::<size_t>() as usize);
pub const ALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const REALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const DUMMY_START_DOCTYPE_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 0 as ::core::ffi::c_int;
pub const DUMMY_END_DOCTYPE_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 1 as ::core::ffi::c_int;
pub const DUMMY_ENTITY_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 2 as ::core::ffi::c_int;
pub const DUMMY_NOTATION_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 3 as ::core::ffi::c_int;
pub const DUMMY_ELEMENT_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 4 as ::core::ffi::c_int;
pub const DUMMY_ATTLIST_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 5 as ::core::ffi::c_int;
pub const DUMMY_COMMENT_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 6 as ::core::ffi::c_int;
pub const DUMMY_PI_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 7 as ::core::ffi::c_int;
pub const DUMMY_START_CDATA_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 9 as ::core::ffi::c_int;
pub const DUMMY_END_CDATA_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 10 as ::core::ffi::c_int;
pub const DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 11 as ::core::ffi::c_int;
pub const DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 14 as ::core::ffi::c_int;
pub const DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 15 as ::core::ffi::c_int;

fn c_str(bytes: &'static [u8]) -> &'static CStr {
    CStr::from_bytes_with_nul(bytes).expect("static C string literal")
}

fn alloc_tests_file() -> &'static CStr {
    c_str(b"/root/work/expat/tests/alloc_tests.c\0")
}

macro_rules! ffi_call {
    ($function:path $(, $arg:expr)* $(,)?) => {{
        unsafe { $function($($arg),*) }
    }};
}

macro_rules! unsafe_global_get {
    ($name:ident) => {{
        unsafe { $name }
    }};
}

macro_rules! unsafe_global_set {
    ($name:ident, $value:expr) => {{
        unsafe {
            $name = $value;
        }
    }};
}

enum AllocTestAction {
    SetInfo(&'static CStr, ::core::ffi::c_int),
    SetCurrentParser(XML_Parser),
    SetAllocationCount(::core::ffi::c_int),
    SetReallocationCount(::core::ffi::c_int),
    SetXmlDeclHandler(XML_XmlDeclHandler),
    SetUnknownEncodingHandler(XML_UnknownEncodingHandler),
    SetProcessingInstructionHandler(XML_ProcessingInstructionHandler),
    SetCommentHandler(XML_CommentHandler),
    Teardown,
}

fn alloc_test_action(action: AllocTestAction) {
    match action {
        AllocTestAction::SetInfo(test_name, line) => {
            ffi_call!(
                _check_set_test_info,
                test_name.as_ptr(),
                alloc_tests_file().as_ptr(),
                line
            );
        }
        AllocTestAction::SetCurrentParser(parser) => {
            unsafe_global_set!(g_parser, parser);
        }
        AllocTestAction::SetAllocationCount(count) => {
            unsafe_global_set!(g_allocation_count, count);
        }
        AllocTestAction::SetReallocationCount(count) => {
            unsafe_global_set!(g_reallocation_count, count);
        }
        AllocTestAction::SetXmlDeclHandler(handler) => {
            let parser = unsafe_global_get!(g_parser);
            ffi_call!(XML_SetXmlDeclHandler, parser, handler);
        }
        AllocTestAction::SetUnknownEncodingHandler(handler) => {
            let parser = unsafe_global_get!(g_parser);
            ffi_call!(XML_SetUnknownEncodingHandler, parser, handler, NULL);
        }
        AllocTestAction::SetProcessingInstructionHandler(handler) => {
            let parser = unsafe_global_get!(g_parser);
            ffi_call!(XML_SetProcessingInstructionHandler, parser, handler);
        }
        AllocTestAction::SetCommentHandler(handler) => {
            let parser = unsafe_global_get!(g_parser);
            ffi_call!(XML_SetCommentHandler, parser, handler);
        }
        AllocTestAction::Teardown => {
            ffi_call!(basic_teardown);
        }
    }
}

fn set_alloc_test_info(test_name: &'static CStr, line: ::core::ffi::c_int) {
    alloc_test_action(AllocTestAction::SetInfo(test_name, line));
}

fn fail_alloc_test(line: ::core::ffi::c_int, message: &'static CStr) -> ! {
    ffi_call!(_fail, alloc_tests_file().as_ptr(), line, message.as_ptr())
}

fn set_current_parser(parser: XML_Parser) {
    alloc_test_action(AllocTestAction::SetCurrentParser(parser));
}

fn set_allocation_count(count: ::core::ffi::c_int) {
    alloc_test_action(AllocTestAction::SetAllocationCount(count));
}

fn set_reallocation_count(count: ::core::ffi::c_int) {
    alloc_test_action(AllocTestAction::SetReallocationCount(count));
}

fn create_alloc_parser() -> XML_Parser {
    let mut memsuite = XML_Memory_Handling_Suite {
        malloc_fcn: Some(duff_allocator),
        realloc_fcn: Some(duff_reallocator),
        free_fcn: Some(free),
    };

    ffi_call!(
        XML_ParserCreate_MM,
        ::core::ptr::null::<XML_Char>(),
        &raw mut memsuite,
        ::core::ptr::null::<XML_Char>(),
    )
}

fn parser_text_len(text: &CStr) -> ::core::ffi::c_int {
    (text.to_bytes().len()) as ::core::ffi::c_int
}

fn parse_single_bytes(text: &CStr, is_final: ::core::ffi::c_int) -> XML_Status {
    let parser = unsafe_global_get!(g_parser);
    ffi_call!(
        _XML_Parse_SINGLE_BYTES,
        parser,
        text.as_ptr(),
        parser_text_len(text),
        is_final,
    )
}

fn current_parser() -> XML_Parser {
    unsafe_global_get!(g_parser)
}

fn set_param_entity_parsing(parsing: XML_ParamEntityParsing) {
    ffi_call!(XML_SetParamEntityParsing, current_parser(), parsing);
}

fn set_external_entity_ref_handler(handler: XML_ExternalEntityRefHandler) {
    ffi_call!(XML_SetExternalEntityRefHandler, current_parser(), handler);
}

fn set_user_data(user_data: *mut ::core::ffi::c_void) {
    ffi_call!(XML_SetUserData, current_parser(), user_data);
}

fn xml_failure_current_parser(line: ::core::ffi::c_int) {
    ffi_call!(
        _xml_failure,
        current_parser(),
        alloc_tests_file().as_ptr(),
        line
    );
}

fn c_char_array<const N: usize>(bytes: [u8; N]) -> [::core::ffi::c_char; N] {
    bytes.map(|byte| byte as ::core::ffi::c_char)
}

fn set_xml_decl_handler(handler: XML_XmlDeclHandler) {
    alloc_test_action(AllocTestAction::SetXmlDeclHandler(handler));
}

fn set_unknown_encoding_handler(handler: XML_UnknownEncodingHandler) {
    alloc_test_action(AllocTestAction::SetUnknownEncodingHandler(handler));
}

fn set_processing_instruction_handler(handler: XML_ProcessingInstructionHandler) {
    alloc_test_action(AllocTestAction::SetProcessingInstructionHandler(handler));
}

fn set_comment_handler(handler: XML_CommentHandler) {
    alloc_test_action(AllocTestAction::SetCommentHandler(handler));
}

fn run_alloc_failure_parse_test(
    test_name: &'static CStr,
    test_line: ::core::ffi::c_int,
    text: &'static CStr,
    max_alloc_count: ::core::ffi::c_int,
    fail_on_success_line: ::core::ffi::c_int,
    fail_on_max_alloc_line: ::core::ffi::c_int,
    mut configure_parser: impl FnMut(),
) {
    set_alloc_test_info(test_name, test_line);

    let mut i = 0;
    while i < max_alloc_count {
        set_allocation_count(i);
        configure_parser();
        if parse_single_bytes(text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }

    if i == 0 {
        fail_alloc_test(
            fail_on_success_line,
            c_str(b"Parse succeeded despite failing allocator\0"),
        );
    }
    if i == max_alloc_count {
        fail_alloc_test(
            fail_on_max_alloc_line,
            c_str(b"Parse failed with max allocations\0"),
        );
    }
}

extern "C" fn alloc_setup() {
    set_allocation_count(ALLOC_ALWAYS_SUCCEED);
    set_reallocation_count(REALLOC_ALWAYS_SUCCEED);
    let parser = create_alloc_parser();
    set_current_parser(parser);
    if parser.is_null() {
        fail_alloc_test(74 as ::core::ffi::c_int, c_str(b"Parser not created\0"));
    }
}
extern "C" fn alloc_teardown() {
    alloc_test_action(AllocTestAction::Teardown)
}
extern "C" fn test_alloc_parse_xdecl() {
    run_alloc_failure_parse_test(
        c_str(b"test_alloc_parse_xdecl\0"),
        83 as ::core::ffi::c_int,
        c_str(b"<?xml version='1.0' encoding='utf-8'?>\n<doc>Hello, world</doc>\0"),
        15 as ::core::ffi::c_int,
        104 as ::core::ffi::c_int,
        106 as ::core::ffi::c_int,
        || set_xml_decl_handler(Some(dummy_xdecl_handler)),
    )
}
extern "C" fn test_alloc_parse_xdecl_2() {
    run_alloc_failure_parse_test(
        c_str(b"test_alloc_parse_xdecl_2\0"),
        113 as ::core::ffi::c_int,
        c_str(b"<?xml version='1.0' encoding='ThisIsAStupidlyLongEncodingNameIntendedToTriggerPoolGrowth123456ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMN'?><doc>Hello, world</doc>\0"),
        20 as ::core::ffi::c_int,
        150 as ::core::ffi::c_int,
        152 as ::core::ffi::c_int,
        || {
            set_xml_decl_handler(Some(dummy_xdecl_handler));
            set_unknown_encoding_handler(Some(long_encoding_handler));
        },
    )
}
extern "C" fn test_alloc_parse_pi() {
    run_alloc_failure_parse_test(
        c_str(b"test_alloc_parse_pi\0"),
        157 as ::core::ffi::c_int,
        c_str(b"<?xml version='1.0' encoding='utf-8'?>\n<?pi unknown?>\n<doc>Hello, world</doc>\0"),
        15 as ::core::ffi::c_int,
        177 as ::core::ffi::c_int,
        179 as ::core::ffi::c_int,
        || set_processing_instruction_handler(Some(dummy_pi_handler)),
    )
}
extern "C" fn test_alloc_parse_pi_2() {
    run_alloc_failure_parse_test(
        c_str(b"test_alloc_parse_pi_2\0"),
        183 as ::core::ffi::c_int,
        c_str(b"<?xml version='1.0' encoding='utf-8'?>\n<doc>Hello, world<?pi unknown?>\n</doc>\0"),
        15 as ::core::ffi::c_int,
        203 as ::core::ffi::c_int,
        205 as ::core::ffi::c_int,
        || set_processing_instruction_handler(Some(dummy_pi_handler)),
    )
}
extern "C" fn test_alloc_parse_pi_3() {
    run_alloc_failure_parse_test(
        c_str(b"test_alloc_parse_pi_3\0"),
        209 as ::core::ffi::c_int,
        c_str(b"<?This processing instruction should be long enough to ensure thatit triggers the growth of an internal string pool when the      allocator fails at a cruicial moment FGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPQ?><doc/>\0"),
        20 as ::core::ffi::c_int,
        244 as ::core::ffi::c_int,
        246 as ::core::ffi::c_int,
        || set_processing_instruction_handler(Some(dummy_pi_handler)),
    )
}
extern "C" fn test_alloc_parse_comment() {
    run_alloc_failure_parse_test(
        c_str(b"test_alloc_parse_comment\0"),
        250 as ::core::ffi::c_int,
        c_str(b"<?xml version='1.0' encoding='utf-8'?>\n<!-- Test parsing this comment --><doc>Hi</doc>\0"),
        15 as ::core::ffi::c_int,
        268 as ::core::ffi::c_int,
        270 as ::core::ffi::c_int,
        || set_comment_handler(Some(dummy_comment_handler)),
    )
}
extern "C" fn test_alloc_parse_comment_2() {
    run_alloc_failure_parse_test(
        c_str(b"test_alloc_parse_comment_2\0"),
        274 as ::core::ffi::c_int,
        c_str(b"<?xml version='1.0' encoding='utf-8'?>\n<doc>Hello, world<!-- Parse this comment too --></doc>\0"),
        15 as ::core::ffi::c_int,
        294 as ::core::ffi::c_int,
        296 as ::core::ffi::c_int,
        || set_comment_handler(Some(dummy_comment_handler)),
    )
}
extern "C" fn test_alloc_create_external_parser() {
    set_alloc_test_info(
        c_str(b"test_alloc_create_external_parser\0"),
        303 as ::core::ffi::c_int,
    );
    let text = c_str(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );
    let mut foo_text = c_char_array(*b"<!ELEMENT doc (#PCDATA)*>\0");

    set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    set_user_data(&raw mut foo_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    set_external_entity_ref_handler(Some(external_entity_duff_loader));

    if parse_single_bytes(text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_alloc_test(
            314 as ::core::ffi::c_int,
            c_str(b"External parser allocator returned success incorrectly\0"),
        );
    }
}
extern "C" fn test_alloc_run_external_parser() {
    set_alloc_test_info(
        c_str(b"test_alloc_run_external_parser\0"),
        320 as ::core::ffi::c_int,
    );
    let text = c_str(
        b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0",
    );
    let mut foo_text = c_char_array(*b"<!ELEMENT doc (#PCDATA)*>\0");
    let mut i: ::core::ffi::c_uint = 0;
    let max_alloc_count: ::core::ffi::c_uint = 15 as ::core::ffi::c_uint;

    while i < max_alloc_count {
        set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
        set_user_data(&raw mut foo_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        set_external_entity_ref_handler(Some(external_entity_null_loader));
        set_allocation_count(i as ::core::ffi::c_int);
        if parse_single_bytes(text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i = i.wrapping_add(1);
    }

    if i == 0 as ::core::ffi::c_uint {
        fail_alloc_test(
            341 as ::core::ffi::c_int,
            c_str(b"Parsing ignored failing allocator\0"),
        );
    } else if i == max_alloc_count {
        fail_alloc_test(
            343 as ::core::ffi::c_int,
            c_str(b"Parsing failed with allocation count 10\0"),
        );
    }
}
extern "C" fn test_alloc_dtd_copy_default_atts() {
    set_alloc_test_info(
        c_str(b"test_alloc_dtd_copy_default_atts\0"),
        350 as ::core::ffi::c_int,
    );
    let text = c_str(
        b"<?xml version='1.0'?>\n<!DOCTYPE doc SYSTEM 'http://example.org/doc.dtd' [\n  <!ENTITY en SYSTEM 'http://example.org/entity.ent'>\n]>\n<doc xmlns='http://example.org/ns1'>\n&en;\n</doc>\0",
    );
    let mut callno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
    set_external_entity_ref_handler(Some(external_entity_dbl_handler));
    set_user_data(&raw mut callno as *mut ::core::ffi::c_void);
    if parse_single_bytes(text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        xml_failure_current_parser(365 as ::core::ffi::c_int);
    }
}
extern "C" fn test_alloc_external_entity() {
    set_alloc_test_info(
        c_str(b"test_alloc_external_entity\0"),
        370 as ::core::ffi::c_int,
    );
    let text = c_str(
        b"<?xml version='1.0'?>\n<!DOCTYPE doc SYSTEM 'http://example.org/doc.dtd' [\n  <!ENTITY en SYSTEM 'http://example.org/entity.ent'>\n]>\n<doc xmlns='http://example.org/ns1'>\n&en;\n</doc>\0",
    );
    let mut i: ::core::ffi::c_int = 0;
    let alloc_test_max_repeats: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut callno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

    while i < alloc_test_max_repeats {
        set_allocation_count(-(1 as ::core::ffi::c_int));
        set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
        set_external_entity_ref_handler(Some(external_entity_dbl_handler_2));
        callno = 0 as ::core::ffi::c_int;
        set_user_data(&raw mut callno as *mut ::core::ffi::c_void);
        set_allocation_count(i);
        if parse_single_bytes(text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }

    set_allocation_count(-(1 as ::core::ffi::c_int));
    if i == 0 as ::core::ffi::c_int {
        fail_alloc_test(
            398 as ::core::ffi::c_int,
            c_str(b"External entity parsed despite duff allocator\0"),
        );
    }
    if i == alloc_test_max_repeats {
        fail_alloc_test(
            400 as ::core::ffi::c_int,
            c_str(b"External entity not parsed at max allocation count\0"),
        );
    }
}
extern "C" fn test_alloc_ext_entity_set_encoding() {
    set_alloc_test_info(
        c_str(b"test_alloc_ext_entity_set_encoding\0"),
        405 as ::core::ffi::c_int,
    );
    let text = c_str(
        b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0",
    );
    let mut i: ::core::ffi::c_int = 0;
    let max_allocation_count: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

    while i < max_allocation_count {
        set_external_entity_ref_handler(Some(external_entity_alloc_set_encoding));
        set_allocation_count(i);
        if parse_single_bytes(text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        set_allocation_count(-(1 as ::core::ffi::c_int));
        alloc_teardown();
        alloc_setup();
        i += 1;
    }

    if i == 0 as ::core::ffi::c_int {
        fail_alloc_test(
            426 as ::core::ffi::c_int,
            c_str(b"Encoding check succeeded despite failing allocator\0"),
        );
    }
    if i == max_allocation_count {
        fail_alloc_test(
            428 as ::core::ffi::c_int,
            c_str(b"Encoding failed at max allocation count\0"),
        );
    }
}
extern "C" fn test_alloc_internal_entity() {
    set_alloc_test_info(
        c_str(b"test_alloc_internal_entity\0"),
        435 as ::core::ffi::c_int,
    );
    let text = c_str(
        concat!(
            "<?xml version='1.0' encoding='unsupported-encoding'?>\n",
            "<!DOCTYPE test [<!ENTITY foo 'bar'>]>\n",
            "<test a='&foo;'/>",
            "\0"
        )
        .as_bytes(),
    );
    let mut i: ::core::ffi::c_uint = 0;
    let max_alloc_count: ::core::ffi::c_uint = 20 as ::core::ffi::c_uint;

    while i < max_alloc_count {
        set_allocation_count(i as ::core::ffi::c_int);
        set_unknown_encoding_handler(Some(unknown_released_encoding_handler));
        if parse_single_bytes(text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i = i.wrapping_add(1);
    }

    if i == 0 as ::core::ffi::c_uint {
        fail_alloc_test(
            454 as ::core::ffi::c_int,
            c_str(b"Internal entity worked despite failing allocations\0"),
        );
    } else if i == max_alloc_count {
        fail_alloc_test(
            456 as ::core::ffi::c_int,
            c_str(b"Internal entity failed at max allocation count\0"),
        );
    }
}
extern "C" fn test_alloc_parameter_entity() {
    set_alloc_test_info(
        c_str(b"test_alloc_parameter_entity\0"),
        460 as ::core::ffi::c_int,
    );
    let text = c_str(
        b"<!DOCTYPE foo [<!ENTITY % param1 \"<!ENTITY internal 'some_text'>\">%param1;]> <foo>&internal;content</foo>\0",
    );
    let mut i: ::core::ffi::c_int = 0;
    let alloc_test_max_repeats: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

    while i < alloc_test_max_repeats {
        set_allocation_count(i);
        set_param_entity_parsing(XML_PARAM_ENTITY_PARSING_ALWAYS);
        if parse_single_bytes(text, XML_TRUE as ::core::ffi::c_int) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }

    set_allocation_count(-(1 as ::core::ffi::c_int));
    if i == 0 as ::core::ffi::c_int {
        fail_alloc_test(
            479 as ::core::ffi::c_int,
            c_str(b"Parameter entity processed despite duff allocator\0"),
        );
    }
    if i == alloc_test_max_repeats {
        fail_alloc_test(
            481 as ::core::ffi::c_int,
            c_str(b"Parameter entity not processed at max allocation count\0"),
        );
    }
}
extern "C" fn test_alloc_dtd_default_handling() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_dtd_default_handling\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            488 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY e SYSTEM 'http://example.org/e'>\n<!NOTATION n SYSTEM 'http://example.org/n'>\n<!ENTITY e1 SYSTEM 'http://example.org/e' NDATA n>\n<!ELEMENT doc (#PCDATA)>\n<!ATTLIST doc a CDATA #IMPLIED>\n<?pi in dtd?>\n<!--comment in dtd-->\n]>\n<doc><![CDATA[text in doc]]></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut expected: *const XML_Char =
            b"\n\n\n\n\n\n\n\n\n<doc>text in doc</doc>\0".as_ptr() as *const XML_Char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            init_dummy_handlers();
            XML_SetDefaultHandler(
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
            XML_SetDoctypeDeclHandler(
                g_parser,
                Some(
                    dummy_start_doctype_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                Some(
                    dummy_end_doctype_handler
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
            XML_SetEntityDeclHandler(
                g_parser,
                Some(
                    dummy_entity_decl_handler
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
            XML_SetNotationDeclHandler(
                g_parser,
                Some(
                    dummy_notation_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> (),
                ),
            );
            XML_SetElementDeclHandler(
                g_parser,
                Some(
                    dummy_element_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut XML_Content,
                        ) -> (),
                ),
            );
            XML_SetAttlistDeclHandler(
                g_parser,
                Some(
                    dummy_attlist_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
            );
            XML_SetProcessingInstructionHandler(
                g_parser,
                Some(
                    dummy_pi_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> (),
                ),
            );
            XML_SetCommentHandler(
                g_parser,
                Some(
                    dummy_comment_handler
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
                ),
            );
            XML_SetCdataSectionHandler(
                g_parser,
                Some(
                    dummy_start_cdata_handler
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
                Some(
                    dummy_end_cdata_handler as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
                ),
            );
            XML_SetUnparsedEntityDeclHandler(
                g_parser,
                Some(
                    dummy_unparsed_entity_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> (),
                ),
            );
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
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                531 as ::core::ffi::c_int,
                b"Default DTD parsed despite allocation failures\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                533 as ::core::ffi::c_int,
                b"Default DTD not parsed with maximum alloc count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
        if get_dummy_handler_flags()
            != DUMMY_START_DOCTYPE_HANDLER_FLAG
                | DUMMY_END_DOCTYPE_HANDLER_FLAG
                | DUMMY_ENTITY_DECL_HANDLER_FLAG
                | DUMMY_NOTATION_DECL_HANDLER_FLAG
                | DUMMY_ELEMENT_DECL_HANDLER_FLAG
                | DUMMY_ATTLIST_DECL_HANDLER_FLAG
                | DUMMY_COMMENT_HANDLER_FLAG
                | DUMMY_PI_HANDLER_FLAG
                | DUMMY_START_CDATA_HANDLER_FLAG
                | DUMMY_END_CDATA_HANDLER_FLAG
                | DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG
        {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                542 as ::core::ffi::c_int,
                b"Not all handlers were called\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_explicit_encoding() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_explicit_encoding\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            547 as ::core::ffi::c_int,
        );
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            if XML_SetEncoding(g_parser, b"us-ascii\0".as_ptr() as *const XML_Char)
                as ::core::ffi::c_uint
                == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                557 as ::core::ffi::c_int,
                b"Encoding set despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                559 as ::core::ffi::c_int,
                b"Encoding not set at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_set_base() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_set_base\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            564 as ::core::ffi::c_int,
        );
        let mut new_base: *const XML_Char = b"/local/file/name.xml\0".as_ptr() as *const XML_Char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            if XML_SetBase(g_parser, new_base) as ::core::ffi::c_uint
                == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                575 as ::core::ffi::c_int,
                b"Base set despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                577 as ::core::ffi::c_int,
                b"Base not set with max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_buffer() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_buffer\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            582 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = get_buffer_test_text;
        let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            buffer = XML_GetBuffer(g_parser, 1536 as ::core::ffi::c_int);
            if buffer.is_null() {
                _fail(
                    b"/root/work/expat/tests/alloc_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    593 as ::core::ffi::c_int,
                    b"1.5K buffer reallocation failed\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if !buffer.is_null() {
            } else {
                __assert_fail(
                    b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/root/work/expat/tests/alloc_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    594 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
            memcpy(buffer, text as *const ::core::ffi::c_void, strlen(text));
            if XML_ParseBuffer(
                g_parser,
                strlen(text) as ::core::ffi::c_int,
                XML_FALSE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        g_reallocation_count = -(1 as ::core::ffi::c_int);
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                605 as ::core::ffi::c_int,
                b"Parse succeeded with no reallocation\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                607 as ::core::ffi::c_int,
                b"Parse failed with max reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_ext_entity_realloc_buffer() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_ext_entity_realloc_buffer\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            612 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_reallocator
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            XML_SetUserData(g_parser, &raw mut i as *mut ::core::ffi::c_void);
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                631 as ::core::ffi::c_int,
                b"Succeeded with no reallocations\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                633 as ::core::ffi::c_int,
                b"Failed with max reallocations\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_many_attributes() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_many_attributes\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            638 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ATTLIST doc za CDATA 'default'>\n<!ATTLIST doc zb CDATA 'def2'>\n<!ATTLIST doc zc CDATA 'def3'>\n]>\n<doc a='1'     b='2'     c='3'     d='4'     e='5'     f='6'     g='7'     h='8'     i='9'     j='10'     k='11'     l='12'     m='13'     n='14'     p='15'     q='16'     r='17'     s='18'></doc>\0"
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                676 as ::core::ffi::c_int,
                b"Parse succeeded despite no reallocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                678 as ::core::ffi::c_int,
                b"Parse failed at max reallocations\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_public_entity_value() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_public_entity_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            683 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc></doc>\n\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut dtd_text: [::core::ffi::c_char; 1109] = ::core::mem::transmute::<
            [u8; 1109],
            [::core::ffi::c_char; 1109],
        >(
            *b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 PUBLIC 'foo' 'bar.ent'>\n<!ENTITY % ThisIsAStupidlyLongParameterNameIntendedToTriggerPoolGrowth12345ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP '%e1;'>\n%e1;\n\0",
        );
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            init_dummy_handlers();
            XML_SetUserData(
                g_parser,
                &raw mut dtd_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_public
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
                g_parser,
                Some(
                    dummy_entity_decl_handler
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                728 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocation\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                730 as ::core::ffi::c_int,
                b"Parsing failed at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if get_dummy_handler_flags() != DUMMY_ENTITY_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                732 as ::core::ffi::c_int,
                b"Entity declaration handler not called\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_subst_public_entity_value() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_subst_public_entity_value\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            736 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc></doc>\n\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut dtd_text: [::core::ffi::c_char; 2108] = ::core::mem::transmute::<
            [u8; 2108],
            [::core::ffi::c_char; 2108],
        >(
            *b"<!ELEMENT doc EMPTY>\n<!ENTITY % ThisIsAStupidlyLongParameterNameIntendedToTriggerPoolGrowth12345ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP PUBLIC 'foo' 'bar.ent'>\n%ThisIsAStupidlyLongParameterNameIntendedToTriggerPoolGrowth12345ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP;\0",
        );
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut dtd_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_public
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                792 as ::core::ffi::c_int,
                b"Parsing worked despite failing reallocation\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                794 as ::core::ffi::c_int,
                b"Parsing failed at max reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_parse_public_doctype() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_parse_public_doctype\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            798 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='utf-8'?>\n<!DOCTYPE doc PUBLIC 'http://example.com/a/long/enough/name/to/trigger/pool/growth/zz/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/' 'test'>\n<doc></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            init_dummy_handlers();
            XML_SetDoctypeDeclHandler(
                g_parser,
                Some(
                    dummy_start_doctype_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                Some(
                    dummy_end_doctype_decl_handler
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                837 as ::core::ffi::c_int,
                b"Parse succeeded despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                839 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if get_dummy_handler_flags()
            != DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG | DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG
        {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                843 as ::core::ffi::c_int,
                b"Doctype handler functions not called\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_parse_public_doctype_long_name() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_parse_public_doctype_long_name\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            847 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='utf-8'?>\n<!DOCTYPE doc PUBLIC 'http://example.com/foo' 'ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOP'>\n<doc></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetDoctypeDeclHandler(
                g_parser,
                Some(
                    dummy_start_doctype_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            ::core::ffi::c_int,
                        ) -> (),
                ),
                Some(
                    dummy_end_doctype_decl_handler
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                885 as ::core::ffi::c_int,
                b"Parse succeeded despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                887 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_set_foreign_dtd() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_set_foreign_dtd\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            892 as ::core::ffi::c_int,
        );
        let mut text1: *const ::core::ffi::c_char =
            b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut text2: [::core::ffi::c_char; 26] = ::core::mem::transmute::<
            [u8; 26],
            [::core::ffi::c_char; 26],
        >(*b"<!ELEMENT doc (#PCDATA)*>\0");
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetUserData(g_parser, &raw mut text2 as *mut ::core::ffi::c_void);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_alloc
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if XML_UseForeignDTD(g_parser, XML_TRUE) as ::core::ffi::c_uint
                != XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _fail(
                    b"/root/work/expat/tests/alloc_tests.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    905 as ::core::ffi::c_int,
                    b"Could not set foreign DTD\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if _XML_Parse_SINGLE_BYTES(
                g_parser,
                text1,
                strlen(text1) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            ) as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                break;
            }
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                914 as ::core::ffi::c_int,
                b"Parse succeeded despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                916 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_attribute_enum_value() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_attribute_enum_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            921 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' standalone='no'?>\n<!DOCTYPE animal SYSTEM 'test.dtd'>\n<animal>This is a \n    <a/>  \n\nyellow tiger</animal>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut dtd_text: [::core::ffi::c_char; 108] = ::core::mem::transmute::<
            [u8; 108],
            [::core::ffi::c_char; 108],
        >(
            *b"<!ELEMENT animal (#PCDATA|a)*>\n<!ELEMENT a EMPTY>\n<!ATTLIST animal xml:space (default|preserve) 'preserve'>\0",
        );
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_alloc
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            XML_SetUserData(
                g_parser,
                &raw mut dtd_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetAttlistDeclHandler(
                g_parser,
                Some(
                    dummy_attlist_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                946 as ::core::ffi::c_int,
                b"Parse succeeded despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                948 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_attribute_enum_value() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_attribute_enum_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            953 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' standalone='no'?>\n<!DOCTYPE animal SYSTEM 'test.dtd'>\n<animal>This is a yellow tiger</animal>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut dtd_text: [::core::ffi::c_char; 1097] = ::core::mem::transmute::<
            [u8; 1097],
            [::core::ffi::c_char; 1097],
        >(
            *b"<!ELEMENT animal (#PCDATA)*>\n<!ATTLIST animal thing (default|ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|BBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|CBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|DBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|EBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|FBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|GBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|HBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|IBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|JBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|KBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|LBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|MBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|NBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|OBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|PBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO) 'default'>\0",
        );
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_alloc
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            XML_SetUserData(
                g_parser,
                &raw mut dtd_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetAttlistDeclHandler(
                g_parser,
                Some(
                    dummy_attlist_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1002 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1004 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_implied_attribute() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_implied_attribute\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1009 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc EMPTY>\n<!ATTLIST doc a (ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|BBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|CBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|DBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|EBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|FBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|GBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|HBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|IBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|JBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|KBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|LBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|MBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|NBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|OBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|PBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMN) #IMPLIED>\n]><doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            XML_SetAttlistDeclHandler(
                g_parser,
                Some(
                    dummy_attlist_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1052 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1054 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_default_attribute() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_default_attribute\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1059 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc EMPTY>\n<!ATTLIST doc a (ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|BBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|CBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|DBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|EBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|FBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|GBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|HBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|IBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|JBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|KBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|LBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|MBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|NBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|OBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|PBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMN) 'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO'>\n]><doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            XML_SetAttlistDeclHandler(
                g_parser,
                Some(
                    dummy_attlist_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1102 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1104 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_notation() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_notation\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1109 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!NOTATION ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP SYSTEM 'http://example.org/n'>\n<!ENTITY e SYSTEM 'http://example.org/e' NDATA ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            init_dummy_handlers();
            XML_SetNotationDeclHandler(
                g_parser,
                Some(
                    dummy_notation_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> (),
                ),
            );
            XML_SetEntityDeclHandler(
                g_parser,
                Some(
                    dummy_entity_decl_handler
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1167 as ::core::ffi::c_int,
                b"Parse succeeded despite allocation failures\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1169 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if get_dummy_handler_flags()
            != DUMMY_ENTITY_DECL_HANDLER_FLAG | DUMMY_NOTATION_DECL_HANDLER_FLAG
        {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1172 as ::core::ffi::c_int,
                b"Entity declaration handler not called\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_public_notation() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_public_notation\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1177 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!NOTATION note PUBLIC 'http://example.com/a/long/enough/name/to/trigger/pool/growth/zz/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/' 'foo'>\n<!ENTITY e SYSTEM 'http://example.com/e' NDATA note>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            init_dummy_handlers();
            XML_SetNotationDeclHandler(
                g_parser,
                Some(
                    dummy_notation_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1217 as ::core::ffi::c_int,
                b"Parse succeeded despite allocation failures\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1219 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if get_dummy_handler_flags() != DUMMY_NOTATION_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1221 as ::core::ffi::c_int,
                b"Notation handler not called\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_system_notation() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_system_notation\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1226 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!NOTATION note SYSTEM 'http://example.com/a/long/enough/name/to/trigger/pool/growth/zz/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/'>\n<!ENTITY e SYSTEM 'http://example.com/e' NDATA note>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            init_dummy_handlers();
            XML_SetNotationDeclHandler(
                g_parser,
                Some(
                    dummy_notation_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1266 as ::core::ffi::c_int,
                b"Parse succeeded despite allocation failures\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1268 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if get_dummy_handler_flags() != DUMMY_NOTATION_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1270 as ::core::ffi::c_int,
                b"Notation handler not called\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_nested_groups() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_nested_groups\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1274 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (e,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?))))))))))))))))))))))))))))))))>\n<!ELEMENT e EMPTY>]>\n<doc><e/></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            CharData_Init(&raw mut storage);
            XML_SetElementDeclHandler(
                g_parser,
                Some(
                    dummy_element_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut XML_Content,
                        ) -> (),
                ),
            );
            XML_SetStartElementHandler(
                g_parser,
                Some(
                    record_element_start_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut *const XML_Char,
                        ) -> (),
                ),
            );
            XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
            init_dummy_handlers();
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1305 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1307 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"doce\0".as_ptr() as *const XML_Char);
        if get_dummy_handler_flags() != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1310 as ::core::ffi::c_int,
                b"Element handler not fired\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_nested_groups() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_nested_groups\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1314 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (e,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?))))))))))))))))))))))))))))))))>\n<!ELEMENT e EMPTY>]>\n<doc><e/></doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            CharData_Init(&raw mut storage);
            XML_SetElementDeclHandler(
                g_parser,
                Some(
                    dummy_element_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut XML_Content,
                        ) -> (),
                ),
            );
            XML_SetStartElementHandler(
                g_parser,
                Some(
                    record_element_start_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut *const XML_Char,
                        ) -> (),
                ),
            );
            XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
            init_dummy_handlers();
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1345 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1347 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        CharData_CheckXMLChars(&raw mut storage, b"doce\0".as_ptr() as *const XML_Char);
        if get_dummy_handler_flags() != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1350 as ::core::ffi::c_int,
                b"Element handler not fired\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_large_group() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_large_group\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1354 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (a1|a2|a3|a4|a5|a6|a7|a8|b1|b2|b3|b4|b5|b6|b7|b8|c1|c2|c3|c4|c5|c6|c7|c8|d1|d2|d3|d4|d5|d6|d7|d8|e1)+>\n]>\n<doc>\n<a1/>\n</doc>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetElementDeclHandler(
                g_parser,
                Some(
                    dummy_element_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut XML_Content,
                        ) -> (),
                ),
            );
            init_dummy_handlers();
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1382 as ::core::ffi::c_int,
                b"Parse succeeded despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1384 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if get_dummy_handler_flags() != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1386 as ::core::ffi::c_int,
                b"Element handler flag not raised\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_group_choice() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_group_choice\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1390 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (a1|a2|a3|a4|a5|a6|a7|a8|b1|b2|b3|b4|b5|b6|b7|b8|c1|c2|c3|c4|c5|c6|c7|c8|d1|d2|d3|d4|d5|d6|d7|d8|e1)+>\n]>\n<doc>\n<a1/>\n<b2 attr='foo'>This is a foo</b2>\n<c3></c3>\n</doc>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            XML_SetElementDeclHandler(
                g_parser,
                Some(
                    dummy_element_decl_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *mut XML_Content,
                        ) -> (),
                ),
            );
            init_dummy_handlers();
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1420 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1422 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if get_dummy_handler_flags() != DUMMY_ELEMENT_DECL_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1424 as ::core::ffi::c_int,
                b"Element handler flag not raised\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_pi_in_epilog() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_pi_in_epilog\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1428 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc></doc>\n<?pi in epilog?>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetProcessingInstructionHandler(
                g_parser,
                Some(
                    dummy_pi_handler
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> (),
                ),
            );
            init_dummy_handlers();
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1446 as ::core::ffi::c_int,
                b"Parse completed despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1448 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if get_dummy_handler_flags() != DUMMY_PI_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1450 as ::core::ffi::c_int,
                b"Processing instruction handler not invoked\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_comment_in_epilog() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_comment_in_epilog\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1454 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc></doc>\n<!-- comment in epilog -->\0".as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetCommentHandler(
                g_parser,
                Some(
                    dummy_comment_handler
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char) -> (),
                ),
            );
            init_dummy_handlers();
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1472 as ::core::ffi::c_int,
                b"Parse completed despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1474 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if get_dummy_handler_flags() != DUMMY_COMMENT_HANDLER_FLAG {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1476 as ::core::ffi::c_int,
                b"Processing instruction handler not invoked\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_long_attribute_value() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_long_attribute_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1480 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [<!ENTITY foo 'This entity will be substituted as an attribute value, and is   calculated to be exactly long enough that the terminating NUL   that the library adds internally will trigger the string pool togrow. GHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP'>]>\n<doc a='&foo;'></doc>\0"
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1515 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1517 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_attribute_whitespace() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_attribute_whitespace\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1521 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc a=' '></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1536 as ::core::ffi::c_int,
                b"Parse succeeded despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1538 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_attribute_predefined_entity() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_attribute_predefined_entity\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1542 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<doc a='&amp;'></doc>\0".as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1557 as ::core::ffi::c_int,
                b"Parse succeeded despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1559 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_long_attr_default_with_char_ref() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_long_attr_default_with_char_ref\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1567 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [<!ATTLIST doc a CDATA 'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHI&#x31;'>]>\n<doc/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1602 as ::core::ffi::c_int,
                b"Parse succeeded despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1604 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_long_attr_value() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_long_attr_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1611 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE test [<!ENTITY foo '\nABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP'>]>\n<test a='&foo;'/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1646 as ::core::ffi::c_int,
                b"Parse succeeded despite failing allocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1648 as ::core::ffi::c_int,
                b"Parse failed at maximum allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_nested_entities() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_nested_entities\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1657 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'http://example.org/one.ent'>\n<doc />\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut test_data: ExtFaults = ext_faults {
            parse_text: b"<!ENTITY % pe1 'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP'>\n<!ENTITY % pe2 '%pe1;'>\n<!ENTITY % pe3 '%pe2;'>\0"
                .as_ptr() as *const ::core::ffi::c_char,
            fail_text: b"Memory Fail not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
            encoding: ::core::ptr::null::<XML_Char>(),
            error: XML_ERROR_NO_MEMORY,
        };
        g_allocation_count = 12 as ::core::ffi::c_int;
        XML_SetUserData(g_parser, &raw mut test_data as *mut ::core::ffi::c_void);
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_faulter
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        _expect_failure(
            text,
            XML_ERROR_EXTERNAL_ENTITY_HANDLING,
            b"Entity allocation failure not noted\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1690 as ::core::ffi::c_int,
        );
    }
}
extern "C" fn test_alloc_realloc_param_entity_newline() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_param_entity_newline\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1694 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut dtd_text: [::core::ffi::c_char; 1048] = ::core::mem::transmute::<
            [u8; 1048],
            [::core::ffi::c_char; 1048],
        >(
            *b"<!ENTITY % pe '<!ATTLIST doc att CDATA \"This default value is carefully crafted so that the carriage    return right at the end of the entity string causes an internal string pool to have to grow.  This allows us to test the alloc  failure path from that point. OPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDE\">\n'>%pe;\n\0",
        );
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut dtd_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_alloc
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1734 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1736 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_ce_extends_pe() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_ce_extends_pe\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1740 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc/>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut dtd_text: [::core::ffi::c_char; 1056] = ::core::mem::transmute::<
            [u8; 1056],
            [::core::ffi::c_char; 1056],
        >(
            *b"<!ENTITY % pe '<!ATTLIST doc att CDATA \"This default value is carefully crafted so that the character   entity at the end causes an internal string pool to have to     grow.  This allows us to test the allocation failure path from  that point onwards. EFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFG&#x51;\">\n'>%pe;\n\0",
        );
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_realloc_count {
            g_reallocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut dtd_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_alloc
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1780 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1782 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_realloc_attributes() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_realloc_attributes\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1786 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ATTLIST doc\n    a1  (a|b|c)   'a'\n    a2  (foo|bar) #IMPLIED\n    a3  NMTOKEN   #IMPLIED\n    a4  NMTOKENS  #IMPLIED\n    a5  ID        #IMPLIED\n    a6  IDREF     #IMPLIED\n    a7  IDREFS    #IMPLIED\n    a8  ENTITY    #IMPLIED\n    a9  ENTITIES  #IMPLIED\n    a10 CDATA     #IMPLIED\n  >]>\n<doc>wombat</doc>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_realloc_count: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1815 as ::core::ffi::c_int,
                b"Parse succeeded despite failing reallocator\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if i == max_realloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1817 as ::core::ffi::c_int,
                b"Parse failed at maximum reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_long_doc_name() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_long_doc_name\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1821 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<LongRootElementNameThatWillCauseTheNextAllocationToExpandTheStringPoolForTheDTDQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ a='1'/>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1854 as ::core::ffi::c_int,
                b"Parsing worked despite failing reallocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1856 as ::core::ffi::c_int,
                b"Parsing failed even at max reallocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_long_base() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_long_base\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1860 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc [\n  <!ENTITY e SYSTEM 'foo'>\n]>\n<doc>&e;</doc>\0".as_ptr()
                as *const ::core::ffi::c_char;
        let mut entity_text: [::core::ffi::c_char; 12] =
            ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"Hello world\0");
        let mut base: *const XML_Char = b"LongBaseURI/that/will/overflow/an/internal/buffer/and/cause/it/to/have/to/grow/PQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/\0"
            .as_ptr() as *const XML_Char;
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut entity_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_alloc
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                            *const XML_Char,
                        ) -> ::core::ffi::c_int,
                ),
            );
            if XML_SetBase(g_parser, base) as ::core::ffi::c_uint
                == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
            } else {
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
                alloc_teardown();
                alloc_setup();
            }
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1906 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1908 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_long_public_id() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_long_public_id\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1912 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY e PUBLIC 'LongPublicIDThatShouldResultInAnInternalStringPoolGrowingAtASpecificMomentKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB' 'bar'>\n]>\n<doc>&e;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut entity_text: [::core::ffi::c_char; 12] =
            ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"Hello world\0");
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut entity_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_alloc
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1953 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                1955 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_long_entity_value() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_long_entity_value\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            1959 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY e1 'Long entity value that should provoke a string pool to grow while setting up to parse the external entity below. xyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB'>\n  <!ENTITY e2 SYSTEM 'bar'>\n]>\n<doc>&e2;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut entity_text: [::core::ffi::c_char; 12] =
            ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"Hello world\0");
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = i;
            XML_SetUserData(
                g_parser,
                &raw mut entity_text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            );
            XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            XML_SetExternalEntityRefHandler(
                g_parser,
                Some(
                    external_entity_alloc
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2001 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2003 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_long_notation() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_long_notation\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2007 as ::core::ffi::c_int,
        );
        let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!NOTATION note SYSTEM 'ALongNotationNameThatShouldProvokeStringPoolGrowthWhileCallingAnExternalEntityParserUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB'>\n  <!ENTITY e1 SYSTEM 'foo' NDATA ALongNotationNameThatShouldProvokeStringPoolGrowthWhileCallingAnExternalEntityParserUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB>\n  <!ENTITY e2 SYSTEM 'bar'>\n]>\n<doc>&e2;</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
        let mut options: [ExtOption; 3] = [
            ExtOption {
                system_id: b"foo\0".as_ptr() as *const XML_Char,
                parse_text: b"Entity Foo\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: b"bar\0".as_ptr() as *const XML_Char,
                parse_text: b"Entity Bar\0".as_ptr() as *const ::core::ffi::c_char,
            },
            ExtOption {
                system_id: ::core::ptr::null::<XML_Char>(),
                parse_text: ::core::ptr::null::<::core::ffi::c_char>(),
            },
        ];
        let mut i: ::core::ffi::c_int = 0;
        let max_alloc_count: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
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
            alloc_teardown();
            alloc_setup();
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2070 as ::core::ffi::c_int,
                b"Parsing worked despite failing allocations\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2072 as ::core::ffi::c_int,
                b"Parsing failed even at max allocation count\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
}
extern "C" fn test_alloc_reset_after_external_entity_parser_create_fail() {
    unsafe {
        _check_set_test_info(
            b"test_alloc_reset_after_external_entity_parser_create_fail\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            2076 as ::core::ffi::c_int,
        );
        let text: *const ::core::ffi::c_char =
            b"<!DOCTYPE doc SYSTEM 'foo'><doc/>\0".as_ptr() as *const ::core::ffi::c_char;
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_parser_create_alloc_fail_handler
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2085 as ::core::ffi::c_int,
                b"Call to parse was expected to fail\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint
            != XML_ERROR_EXTERNAL_ENTITY_HANDLING as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/root/work/expat/tests/alloc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                2088 as ::core::ffi::c_int,
                b"Call to parse was expected to fail from the external entity handler\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserReset(g_parser, ::core::ptr::null::<XML_Char>());
    }
}
extern "C" fn sizeRecordedFor(mut ptr: *mut ::core::ffi::c_void) -> size_t {
    OwnedParser::allocation_size_recorded_for(ptr)
}

struct OwnedParser(NonNull<XML_ParserStruct>);

impl OwnedParser {
    fn new(encoding: *const XML_Char) -> Option<Self> {
        NonNull::new(unsafe { XML_ParserCreate(encoding) }).map(Self)
    }

    fn new_with_memsuite(
        encoding: *const XML_Char,
        memsuite: &mut XML_Memory_Handling_Suite,
        namespace_separator: *const XML_Char,
    ) -> Option<Self> {
        NonNull::new(unsafe {
            XML_ParserCreate_MM(
                encoding,
                memsuite as *mut XML_Memory_Handling_Suite,
                namespace_separator,
            )
        })
        .map(Self)
    }

    fn new_external_entity(
        parent: &Self,
        context: *const XML_Char,
        encoding: *const XML_Char,
    ) -> Option<Self> {
        NonNull::new(unsafe { XML_ExternalEntityParserCreate(parent.as_raw(), context, encoding) })
            .map(Self)
    }

    fn as_raw(&self) -> XML_Parser {
        self.0.as_ptr()
    }

    fn expat_malloc(&self, size: size_t) -> *mut ::core::ffi::c_void {
        unsafe { expat_malloc(self.as_raw(), size, -(1 as ::core::ffi::c_int)) }
    }

    fn expat_realloc(
        &self,
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void {
        unsafe { expat_realloc(self.as_raw(), ptr, size, -(1 as ::core::ffi::c_int)) }
    }

    fn expat_free(&self, ptr: *mut ::core::ffi::c_void) {
        unsafe { expat_free(self.as_raw(), ptr, -(1 as ::core::ffi::c_int)) }
    }

    fn mem_malloc(&self, size: size_t) -> *mut ::core::ffi::c_void {
        unsafe { XML_MemMalloc(self.as_raw(), size) }
    }

    fn mem_realloc(&self, ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void {
        unsafe { XML_MemRealloc(self.as_raw(), ptr, size) }
    }

    fn mem_free(&self, ptr: *mut ::core::ffi::c_void) {
        unsafe { XML_MemFree(self.as_raw(), ptr) }
    }

    fn set_alloc_tracker_maximum_amplification(
        &self,
        maximum_amplification_factor: ::core::ffi::c_float,
    ) -> bool {
        Self::set_alloc_tracker_maximum_amplification_raw(
            self.as_raw(),
            maximum_amplification_factor,
        )
    }

    fn set_alloc_tracker_activation_threshold(
        &self,
        activation_threshold_bytes: ::core::ffi::c_ulonglong,
    ) -> bool {
        Self::set_alloc_tracker_activation_threshold_raw(self.as_raw(), activation_threshold_bytes)
    }

    fn parse_single_bytes(
        &self,
        text: *const ::core::ffi::c_char,
        text_len: ::core::ffi::c_int,
        is_final: ::core::ffi::c_int,
    ) -> XML_Status {
        unsafe { _XML_Parse_SINGLE_BYTES(self.as_raw(), text, text_len, is_final) }
    }

    fn get_buffer(&self, len: ::core::ffi::c_int) -> *mut ::core::ffi::c_void {
        unsafe { XML_GetBuffer(self.as_raw(), len) }
    }

    fn allocation_size_recorded_for(ptr: *mut ::core::ffi::c_void) -> size_t {
        let header = ptr
            .cast::<u8>()
            .wrapping_sub(EXPAT_MALLOC_PADDING + ::core::mem::size_of::<size_t>())
            .cast::<size_t>();
        unsafe { header.read_unaligned() }
    }

    fn reparse_deferral_enabled_default() -> XML_Bool {
        unsafe { g_reparseDeferralEnabledDefault }
    }

    fn set_alloc_tracker_maximum_amplification_raw(
        parser: XML_Parser,
        maximum_amplification_factor: ::core::ffi::c_float,
    ) -> bool {
        (unsafe { XML_SetAllocTrackerMaximumAmplification(parser, maximum_amplification_factor) })
            == XML_TRUE
    }

    fn set_alloc_tracker_activation_threshold_raw(
        parser: XML_Parser,
        activation_threshold_bytes: ::core::ffi::c_ulonglong,
    ) -> bool {
        (unsafe { XML_SetAllocTrackerActivationThreshold(parser, activation_threshold_bytes) })
            == XML_TRUE
    }

    fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> *mut ::core::ffi::c_void {
        unsafe { memcpy(dest, src, len) }
    }

    fn memset(
        dest: *mut ::core::ffi::c_void,
        value: ::core::ffi::c_int,
        len: size_t,
    ) -> *mut ::core::ffi::c_void {
        unsafe { memset(dest, value, len) }
    }

    fn strlen(value: *const ::core::ffi::c_char) -> size_t {
        unsafe { strlen(value) }
    }
}

impl Drop for OwnedParser {
    fn drop(&mut self) {
        unsafe { XML_ParserFree(self.as_raw()) }
    }
}

extern "C" fn test_alloc_tracker_size_recorded() {
    set_alloc_test_info(
        c_str(b"test_alloc_tracker_size_recorded\0"),
        2101 as ::core::ffi::c_int,
    );
    let mut memsuite = XML_Memory_Handling_Suite {
        malloc_fcn: Some(malloc),
        realloc_fcn: Some(realloc),
        free_fcn: Some(free),
    };
    for &use_mem_suite in &[true_0 != 0, false_0 != 0] {
        ffi_call!(
            set_subtest,
            b"useMemSuite=%d\0".as_ptr() as *const ::core::ffi::c_char,
            use_mem_suite as ::core::ffi::c_int,
        );
        let parser = if use_mem_suite {
            OwnedParser::new_with_memsuite(
                ::core::ptr::null::<XML_Char>(),
                &mut memsuite,
                b"|\0".as_ptr() as *const XML_Char,
            )
        } else {
            OwnedParser::new(::core::ptr::null::<XML_Char>())
        };
        let parser = match parser {
            Some(parser) => parser,
            None => {
                fail_alloc_test(
                    2112 as ::core::ffi::c_int,
                    c_str(b"check failed: parser != NULL\0"),
                );
            }
        };
        let mut ptr = parser.expat_malloc(10 as size_t);
        if ptr.is_null() {
            fail_alloc_test(
                2115 as ::core::ffi::c_int,
                c_str(b"check failed: ptr != NULL\0"),
            );
        }
        if OwnedParser::allocation_size_recorded_for(ptr) != 10 as size_t {
            fail_alloc_test(
                2116 as ::core::ffi::c_int,
                c_str(b"check failed: sizeRecordedFor(ptr) == 10\0"),
            );
        }
        if !parser
            .expat_realloc(ptr, size_t::MAX.wrapping_div(2 as size_t))
            .is_null()
        {
            fail_alloc_test(
                2118 as ::core::ffi::c_int,
                c_str(b"check failed: expat_realloc(parser, ptr, SIZE_MAX / 2, -1) == NULL\0"),
            );
        }
        if OwnedParser::allocation_size_recorded_for(ptr) != 10 as size_t {
            fail_alloc_test(
                2120 as ::core::ffi::c_int,
                c_str(b"check failed: sizeRecordedFor(ptr) == 10\0"),
            );
        }
        ptr = parser.expat_realloc(ptr, 20 as size_t);
        if ptr.is_null() {
            fail_alloc_test(
                2124 as ::core::ffi::c_int,
                c_str(b"check failed: ptr != NULL\0"),
            );
        }
        if OwnedParser::allocation_size_recorded_for(ptr) != 20 as size_t {
            fail_alloc_test(
                2125 as ::core::ffi::c_int,
                c_str(b"check failed: sizeRecordedFor(ptr) == 20\0"),
            );
        }
        parser.expat_free(ptr);
    }
}
extern "C" fn test_alloc_tracker_pointer_alignment() {
    set_alloc_test_info(
        c_str(b"test_alloc_tracker_pointer_alignment\0"),
        2135 as ::core::ffi::c_int,
    );
    let parser = match OwnedParser::new(::core::ptr::null::<XML_Char>()) {
        Some(parser) => parser,
        None => fail_alloc_test(
            2136 as ::core::ffi::c_int,
            c_str(b"check failed: parser != NULL\0"),
        ),
    };
    if ::core::mem::size_of::<::core::ffi::c_longlong>() < ::core::mem::size_of::<size_t>() {
        fail_alloc_test(
            2138 as ::core::ffi::c_int,
            c_str(b"check failed: sizeof(long long) >= sizeof(size_t)\0"),
        );
    }
    let ptr = parser.expat_malloc(
        (4 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_longlong>() as size_t),
    ) as *mut ::core::ffi::c_longlong;
    let values: [::core::ffi::c_longlong; 4] = [0, 1, 2, 3];
    OwnedParser::memcpy(
        ptr.cast::<::core::ffi::c_void>(),
        values.as_ptr().cast::<::core::ffi::c_void>(),
        ::core::mem::size_of_val(&values),
    );
    parser.expat_free(ptr.cast::<::core::ffi::c_void>());
}
extern "C" fn test_alloc_tracker_maximum_amplification() {
    set_alloc_test_info(
        c_str(b"test_alloc_tracker_maximum_amplification\0"),
        2151 as ::core::ffi::c_int,
    );
    if OwnedParser::reparse_deferral_enabled_default() as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        return;
    }
    let parser = match OwnedParser::new(::core::ptr::null::<XML_Char>()) {
        Some(parser) => parser,
        None => fail_alloc_test(
            2154 as ::core::ffi::c_int,
            c_str(b"check failed: parser != NULL\0"),
        ),
    };
    let chunk = b"<e>\0".as_ptr() as *const ::core::ffi::c_char;
    let chunk_len = OwnedParser::strlen(chunk) as ::core::ffi::c_int;
    if parser.parse_single_bytes(chunk, chunk_len, XML_FALSE as ::core::ffi::c_int)
        as ::core::ffi::c_uint
        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fail_alloc_test(
            2162 as ::core::ffi::c_int,
            c_str(b"check failed: _XML_Parse_SINGLE_BYTES(parser, chunk, (int)strlen(chunk), XML_FALSE) == XML_STATUS_OK\0"),
        );
    }
    if !parser.set_alloc_tracker_activation_threshold(0 as ::core::ffi::c_ulonglong) {
        fail_alloc_test(
            2166 as ::core::ffi::c_int,
            c_str(b"check failed: XML_SetAllocTrackerActivationThreshold(parser, 0) == XML_TRUE\0"),
        );
    }
    if !parser.expat_malloc(1000 as size_t).is_null() {
        fail_alloc_test(
            2169 as ::core::ffi::c_int,
            c_str(b"check failed: expat_malloc(parser, 1000, -1) == NULL\0"),
        );
    }
    if !parser.set_alloc_tracker_maximum_amplification(3000.0f32) {
        fail_alloc_test(
            2174 as ::core::ffi::c_int,
            c_str(b"check failed: XML_SetAllocTrackerMaximumAmplification(parser, 3000.0f) == XML_TRUE\0"),
        );
    }
    let ptr = parser.expat_malloc(1000 as size_t);
    if ptr.is_null() {
        fail_alloc_test(
            2177 as ::core::ffi::c_int,
            c_str(b"check failed: ptr != NULL\0"),
        );
    }
    parser.expat_free(ptr);
}
extern "C" fn test_alloc_tracker_threshold() {
    set_alloc_test_info(
        c_str(b"test_alloc_tracker_threshold\0"),
        2185 as ::core::ffi::c_int,
    );
    let parser = match OwnedParser::new(::core::ptr::null::<XML_Char>()) {
        Some(parser) => parser,
        None => fail_alloc_test(
            2187 as ::core::ffi::c_int,
            c_str(b"check failed: parser != NULL\0"),
        ),
    };
    let ptr = parser.expat_malloc(1000 as size_t);
    if ptr.is_null() {
        fail_alloc_test(
            2191 as ::core::ffi::c_int,
            c_str(b"check failed: ptr != NULL\0"),
        );
    }
    parser.expat_free(ptr);
    if !parser.set_alloc_tracker_activation_threshold(999 as ::core::ffi::c_ulonglong) {
        fail_alloc_test(
            2195 as ::core::ffi::c_int,
            c_str(
                b"check failed: XML_SetAllocTrackerActivationThreshold(parser, 999) == XML_TRUE\0",
            ),
        );
    }
    if !parser.expat_malloc(1000 as size_t).is_null() {
        fail_alloc_test(
            2196 as ::core::ffi::c_int,
            c_str(b"check failed: expat_malloc(parser, 1000, -1) == NULL\0"),
        );
    }
}
extern "C" fn test_alloc_tracker_getbuffer_unlimited() {
    set_alloc_test_info(
        c_str(b"test_alloc_tracker_getbuffer_unlimited\0"),
        2203 as ::core::ffi::c_int,
    );
    let parser = match OwnedParser::new(::core::ptr::null::<XML_Char>()) {
        Some(parser) => parser,
        None => fail_alloc_test(
            2204 as ::core::ffi::c_int,
            c_str(b"check failed: parser != NULL\0"),
        ),
    };
    if !parser.set_alloc_tracker_activation_threshold(0 as ::core::ffi::c_ulonglong) {
        fail_alloc_test(
            2208 as ::core::ffi::c_int,
            c_str(b"check failed: XML_SetAllocTrackerActivationThreshold(parser, 0) == XML_TRUE\0"),
        );
    }
    if !parser.expat_malloc(1000 as size_t).is_null() {
        fail_alloc_test(
            2211 as ::core::ffi::c_int,
            c_str(b"check failed: expat_malloc(parser, 1000, -1) == NULL\0"),
        );
    }
    if parser.get_buffer(1000 as ::core::ffi::c_int).is_null() {
        fail_alloc_test(
            2214 as ::core::ffi::c_int,
            c_str(b"check failed: XML_GetBuffer(parser, 1000) != NULL\0"),
        );
    }
}
extern "C" fn test_alloc_tracker_api() {
    set_alloc_test_info(
        c_str(b"test_alloc_tracker_api\0"),
        2220 as ::core::ffi::c_int,
    );
    let parser_without_parent = match OwnedParser::new(::core::ptr::null::<XML_Char>()) {
        Some(parser) => parser,
        None => fail_alloc_test(
            2225 as ::core::ffi::c_int,
            c_str(b"parserWithoutParent is NULL\0"),
        ),
    };
    let parser_with_parent = match OwnedParser::new_external_entity(
        &parser_without_parent,
        b"entity123\0".as_ptr() as *const XML_Char,
        ::core::ptr::null::<XML_Char>(),
    ) {
        Some(parser) => parser,
        None => fail_alloc_test(
            2227 as ::core::ffi::c_int,
            c_str(b"parserWithParent is NULL\0"),
        ),
    };
    if parser_without_parent.as_raw().is_null() {
        fail_alloc_test(
            2225 as ::core::ffi::c_int,
            c_str(b"parserWithoutParent is NULL\0"),
        );
    }
    if parser_with_parent.as_raw().is_null() {
        fail_alloc_test(
            2227 as ::core::ffi::c_int,
            c_str(b"parserWithParent is NULL\0"),
        );
    }
    if OwnedParser::set_alloc_tracker_maximum_amplification_raw(
        ::core::ptr::null_mut::<XML_ParserStruct>(),
        123.0f32,
    ) {
        fail_alloc_test(
            2232 as ::core::ffi::c_int,
            c_str(b"Call with NULL parser is NOT supposed to succeed\0"),
        );
    }
    if parser_with_parent.set_alloc_tracker_maximum_amplification(123.0f32) {
        fail_alloc_test(
            2235 as ::core::ffi::c_int,
            c_str(b"Call with non-root parser is NOT supposed to succeed\0"),
        );
    }
    if parser_without_parent.set_alloc_tracker_maximum_amplification(::core::f32::NAN) {
        fail_alloc_test(
            2238 as ::core::ffi::c_int,
            c_str(b"Call with NaN limit is NOT supposed to succeed\0"),
        );
    }
    if parser_without_parent.set_alloc_tracker_maximum_amplification(-1.0f32) {
        fail_alloc_test(
            2241 as ::core::ffi::c_int,
            c_str(b"Call with negative limit is NOT supposed to succeed\0"),
        );
    }
    if parser_without_parent.set_alloc_tracker_maximum_amplification(0.9f32) {
        fail_alloc_test(
            2244 as ::core::ffi::c_int,
            c_str(b"Call with positive limit <1.0 is NOT supposed to succeed\0"),
        );
    }
    if !parser_without_parent.set_alloc_tracker_maximum_amplification(1.0f32) {
        fail_alloc_test(
            2249 as ::core::ffi::c_int,
            c_str(b"Call with positive limit >=1.0 is supposed to succeed\0"),
        );
    }
    if !parser_without_parent.set_alloc_tracker_maximum_amplification(123456.789f32) {
        fail_alloc_test(
            2252 as ::core::ffi::c_int,
            c_str(b"Call with positive limit >=1.0 is supposed to succeed\0"),
        );
    }
    if !parser_without_parent.set_alloc_tracker_maximum_amplification(::core::f32::INFINITY) {
        fail_alloc_test(
            2255 as ::core::ffi::c_int,
            c_str(b"Call with positive limit >=1.0 is supposed to succeed\0"),
        );
    }
    if OwnedParser::set_alloc_tracker_activation_threshold_raw(
        ::core::ptr::null_mut::<XML_ParserStruct>(),
        123 as ::core::ffi::c_ulonglong,
    ) {
        fail_alloc_test(
            2259 as ::core::ffi::c_int,
            c_str(b"Call with NULL parser is NOT supposed to succeed\0"),
        );
    }
    if parser_with_parent.set_alloc_tracker_activation_threshold(123 as ::core::ffi::c_ulonglong) {
        fail_alloc_test(
            2261 as ::core::ffi::c_int,
            c_str(b"Call with non-root parser is NOT supposed to succeed\0"),
        );
    }
    if !parser_without_parent
        .set_alloc_tracker_activation_threshold(123 as ::core::ffi::c_ulonglong)
    {
        fail_alloc_test(
            2266 as ::core::ffi::c_int,
            c_str(b"Call with non-NULL parentless parser is supposed to succeed\0"),
        );
    }
}
extern "C" fn test_mem_api_cycle() {
    set_alloc_test_info(c_str(b"test_mem_api_cycle\0"), 2274 as ::core::ffi::c_int);
    let parser = match OwnedParser::new(::core::ptr::null::<XML_Char>()) {
        Some(parser) => parser,
        None => fail_alloc_test(
            2275 as ::core::ffi::c_int,
            c_str(b"check failed: parser != NULL\0"),
        ),
    };
    let mut ptr = parser.mem_malloc(10 as size_t);
    if ptr.is_null() {
        fail_alloc_test(
            2279 as ::core::ffi::c_int,
            c_str(b"check failed: ptr != NULL\0"),
        );
    }
    OwnedParser::memset(ptr, 'x' as i32, 10 as size_t);
    ptr = parser.mem_realloc(ptr, 20 as size_t);
    if ptr.is_null() {
        fail_alloc_test(
            2284 as ::core::ffi::c_int,
            c_str(b"check failed: ptr != NULL\0"),
        );
    }
    OwnedParser::memset(ptr, 'y' as i32, 20 as size_t);
    parser.mem_free(ptr);
}
extern "C" fn test_mem_api_unlimited() {
    set_alloc_test_info(
        c_str(b"test_mem_api_unlimited\0"),
        2293 as ::core::ffi::c_int,
    );
    let parser = match OwnedParser::new(::core::ptr::null::<XML_Char>()) {
        Some(parser) => parser,
        None => fail_alloc_test(
            2294 as ::core::ffi::c_int,
            c_str(b"check failed: parser != NULL\0"),
        ),
    };
    if !parser.set_alloc_tracker_activation_threshold(0 as ::core::ffi::c_ulonglong) {
        fail_alloc_test(
            2297 as ::core::ffi::c_int,
            c_str(b"check failed: XML_SetAllocTrackerActivationThreshold(parser, 0) == XML_TRUE\0"),
        );
    }
    let mut ptr = parser.mem_malloc(1000 as size_t);
    if ptr.is_null() {
        fail_alloc_test(
            2302 as ::core::ffi::c_int,
            c_str(b"check failed: ptr != NULL\0"),
        );
    }
    ptr = parser.mem_realloc(ptr, 2000 as size_t);
    if ptr.is_null() {
        fail_alloc_test(
            2306 as ::core::ffi::c_int,
            c_str(b"check failed: ptr != NULL\0"),
        );
    }
    parser.mem_free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn make_alloc_test_case(mut s: *mut Suite) {
    unsafe {
        let mut tc_alloc: *mut TCase =
            tcase_create(b"allocation tests\0".as_ptr() as *const ::core::ffi::c_char);
        suite_add_tcase(s, tc_alloc);
        tcase_add_checked_fixture(
            tc_alloc,
            Some(alloc_setup as extern "C" fn() -> ()),
            Some(alloc_teardown as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_parse_xdecl as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_parse_xdecl_2 as extern "C" fn() -> ()),
        );
        tcase_add_test(tc_alloc, Some(test_alloc_parse_pi as extern "C" fn() -> ()));
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_parse_pi_2 as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_parse_pi_3 as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_parse_comment as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_parse_comment_2 as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_create_external_parser as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_run_external_parser as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_dtd_copy_default_atts as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_external_entity as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_ext_entity_set_encoding as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_internal_entity as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_parameter_entity as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_dtd_default_handling as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_explicit_encoding as extern "C" fn() -> ()),
        );
        tcase_add_test(tc_alloc, Some(test_alloc_set_base as extern "C" fn() -> ()));
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_realloc_buffer as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_ext_entity_realloc_buffer as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_realloc_many_attributes as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_public_entity_value as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_subst_public_entity_value as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_parse_public_doctype as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_parse_public_doctype_long_name as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_set_foreign_dtd as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_attribute_enum_value as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_attribute_enum_value as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_implied_attribute as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_default_attribute as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(tc_alloc, Some(test_alloc_notation as extern "C" fn() -> ()));
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_public_notation as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_system_notation as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_nested_groups as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_nested_groups as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_large_group as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_group_choice as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_pi_in_epilog as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_comment_in_epilog as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_long_attribute_value as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_attribute_whitespace as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_attribute_predefined_entity as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_long_attr_default_with_char_ref as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_long_attr_value as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_nested_entities as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_param_entity_newline as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_ce_extends_pe as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(test_alloc_realloc_attributes as extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_alloc,
            Some(test_alloc_long_doc_name as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_long_base as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_long_public_id as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_long_entity_value as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_long_notation as extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_alloc,
            Some(
                test_alloc_reset_after_external_entity_parser_create_fail as extern "C" fn() -> (),
            ),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_tracker_size_recorded as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_tracker_pointer_alignment as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_tracker_maximum_amplification as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_tracker_threshold as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_tracker_getbuffer_unlimited as extern "C" fn() -> ()),
        );
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_alloc_tracker_api as extern "C" fn() -> ()),
        );
        tcase_add_test(tc_alloc, Some(test_mem_api_cycle as extern "C" fn() -> ()));
        tcase_add_test__if_xml_ge(
            tc_alloc,
            Some(test_mem_api_unlimited as extern "C" fn() -> ()),
        );
    }
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
