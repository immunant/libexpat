pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::expat_external_h::XML_Char;
pub use crate::src::tests::chardata::CharData;
pub use crate::src::tests::chardata::CharData_CheckXMLChars;
pub use crate::src::tests::chardata::CharData_Init;
use crate::src::tests::common::_XML_Parse_SINGLE_BYTES;
use crate::src::tests::common::_expect_failure;
use crate::src::tests::common::_run_character_check;
use crate::src::tests::common::_xml_failure;
use crate::src::tests::common::basic_teardown;
use crate::src::tests::common::g_parser;
use crate::src::tests::common::tcase_add_test__if_xml_ge;
use crate::src::tests::common::tcase_add_test__ifdef_xml_dtd;
pub use crate::src::tests::dummy::dummy_end_element;
pub use crate::src::tests::dummy::dummy_end_namespace_decl_handler;
pub use crate::src::tests::dummy::dummy_start_element;
pub use crate::src::tests::dummy::dummy_start_namespace_decl_handler;
pub use crate::src::tests::dummy::get_dummy_handler_flags;
pub use crate::src::tests::dummy::init_dummy_handlers;
pub use crate::src::tests::dummy::DUMMY_END_NS_DECL_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_START_NS_DECL_HANDLER_FLAG;

pub use crate::expat_h::XML_Bool;
pub use crate::expat_h::XML_Encoding;
pub use crate::expat_h::XML_EndElementHandler;
pub use crate::expat_h::XML_EndNamespaceDeclHandler;
pub use crate::expat_h::XML_Error;
pub use crate::expat_h::XML_ExternalEntityRefHandler;
pub use crate::expat_h::XML_ParamEntityParsing;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ParserStruct;
pub use crate::expat_h::XML_Parsing;
pub use crate::expat_h::XML_ParsingStatus;
pub use crate::expat_h::XML_StartElementHandler;
pub use crate::expat_h::XML_StartNamespaceDeclHandler;
pub use crate::expat_h::XML_Status;
pub use crate::expat_h::XML_UnknownEncodingHandler;
pub use crate::expat_h::XML_ERROR_ABORTED;
pub use crate::expat_h::XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
pub use crate::expat_h::XML_ERROR_ASYNC_ENTITY;
pub use crate::expat_h::XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_BAD_CHAR_REF;
pub use crate::expat_h::XML_ERROR_BINARY_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
pub use crate::expat_h::XML_ERROR_DUPLICATE_ATTRIBUTE;
pub use crate::expat_h::XML_ERROR_ENTITY_DECLARED_IN_PE;
pub use crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING;
pub use crate::expat_h::XML_ERROR_FEATURE_REQUIRES_XML_DTD;
pub use crate::expat_h::XML_ERROR_FINISHED;
pub use crate::expat_h::XML_ERROR_INCOMPLETE_PE;
pub use crate::expat_h::XML_ERROR_INCORRECT_ENCODING;
pub use crate::expat_h::XML_ERROR_INVALID_ARGUMENT;
pub use crate::expat_h::XML_ERROR_INVALID_TOKEN;
pub use crate::expat_h::XML_ERROR_JUNK_AFTER_DOC_ELEMENT;
pub use crate::expat_h::XML_ERROR_MISPLACED_XML_PI;
pub use crate::expat_h::XML_ERROR_NONE;
pub use crate::expat_h::XML_ERROR_NOT_STANDALONE;
pub use crate::expat_h::XML_ERROR_NOT_STARTED;
pub use crate::expat_h::XML_ERROR_NOT_SUSPENDED;
pub use crate::expat_h::XML_ERROR_NO_BUFFER;
pub use crate::expat_h::XML_ERROR_NO_ELEMENTS;
pub use crate::expat_h::XML_ERROR_NO_MEMORY;
pub use crate::expat_h::XML_ERROR_PARAM_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_PARTIAL_CHAR;
pub use crate::expat_h::XML_ERROR_PUBLICID;
pub use crate::expat_h::XML_ERROR_RECURSIVE_ENTITY_REF;
pub use crate::expat_h::XML_ERROR_RESERVED_NAMESPACE_URI;
pub use crate::expat_h::XML_ERROR_RESERVED_PREFIX_XML;
pub use crate::expat_h::XML_ERROR_RESERVED_PREFIX_XMLNS;
pub use crate::expat_h::XML_ERROR_SUSPENDED;
pub use crate::expat_h::XML_ERROR_SUSPEND_PE;
pub use crate::expat_h::XML_ERROR_SYNTAX;
pub use crate::expat_h::XML_ERROR_TAG_MISMATCH;
pub use crate::expat_h::XML_ERROR_TEXT_DECL;
pub use crate::expat_h::XML_ERROR_UNBOUND_PREFIX;
pub use crate::expat_h::XML_ERROR_UNCLOSED_CDATA_SECTION;
pub use crate::expat_h::XML_ERROR_UNCLOSED_TOKEN;
pub use crate::expat_h::XML_ERROR_UNDECLARING_PREFIX;
pub use crate::expat_h::XML_ERROR_UNDEFINED_ENTITY;
pub use crate::expat_h::XML_ERROR_UNEXPECTED_STATE;
pub use crate::expat_h::XML_ERROR_UNKNOWN_ENCODING;
pub use crate::expat_h::XML_ERROR_XML_DECL;
pub use crate::expat_h::XML_FALSE;
pub use crate::expat_h::XML_FINISHED;
pub use crate::expat_h::XML_INITIALIZED;
pub use crate::expat_h::XML_PARAM_ENTITY_PARSING_ALWAYS;
pub use crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
pub use crate::expat_h::XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE;
pub use crate::expat_h::XML_PARSING;
pub use crate::expat_h::XML_STATUS_ERROR;
pub use crate::expat_h::XML_STATUS_OK;
pub use crate::expat_h::XML_STATUS_SUSPENDED;
pub use crate::expat_h::XML_SUSPENDED;
pub use crate::expat_h::XML_TRUE;
pub use crate::src::lib::xmlparse::XML_GetErrorCode;
pub use crate::src::lib::xmlparse::XML_GetParsingStatus;
pub use crate::src::lib::xmlparse::XML_ParserCreateNS;
pub use crate::src::lib::xmlparse::XML_ParserFree;
pub use crate::src::lib::xmlparse::XML_ParserReset;
pub use crate::src::lib::xmlparse::XML_SetElementHandler;
pub use crate::src::lib::xmlparse::XML_SetEndElementHandler;
pub use crate::src::lib::xmlparse::XML_SetEndNamespaceDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler;
pub use crate::src::lib::xmlparse::XML_SetNamespaceDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetParamEntityParsing;
pub use crate::src::lib::xmlparse::XML_SetReturnNSTriplet;
pub use crate::src::lib::xmlparse::XML_SetStartElementHandler;
pub use crate::src::lib::xmlparse::XML_SetStartNamespaceDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetUnknownEncodingHandler;
pub use crate::src::lib::xmlparse::XML_SetUserData;
pub use crate::src::lib::xmlparse::XML_UseParserAsHandlerArg;
use crate::src::tests::handlers::accumulate_attribute;
use crate::src::tests::handlers::external_entity_handler;
use crate::src::tests::handlers::g_triplet_end_flag;
use crate::src::tests::handlers::g_triplet_start_flag;
use crate::src::tests::handlers::overwrite_end_checker;
use crate::src::tests::handlers::overwrite_start_checker;
use crate::src::tests::handlers::start_element_event_handler;
use crate::src::tests::handlers::start_element_fail;
use crate::src::tests::handlers::start_ns_clearing_start_element;
use crate::src::tests::handlers::triplet_end_checker;
use crate::src::tests::handlers::triplet_start_checker;
use crate::src::tests::handlers::MiscEncodingHandler;
pub use crate::src::tests::minicheck::set_subtest;
pub use crate::src::tests::minicheck::tcase_setup_function;
pub use crate::src::tests::minicheck::tcase_teardown_function;
pub use crate::src::tests::minicheck::tcase_test_function;
pub use crate::src::tests::minicheck::Suite;
pub use crate::src::tests::minicheck::TCase;
pub use crate::src::tests::minicheck::_check_set_test_info;
pub use crate::src::tests::minicheck::_fail;
pub use crate::src::tests::minicheck::suite_add_tcase;
pub use crate::src::tests::minicheck::tcase_add_checked_fixture;
pub use crate::src::tests::minicheck::tcase_add_test;
pub use crate::src::tests::minicheck::tcase_create;
use crate::stdlib::strlen;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct test_case {
    pub expectedStatus: XML_Status,
    pub doc: *const ::core::ffi::c_char,
    pub namesep: XML_Char,
}

unsafe extern "C" fn namespace_setup() {
    g_parser = XML_ParserCreateNS(
        ::core::ptr::null::<XML_Char>(),
        ' ' as i32 as XML_Char,
    );
    if g_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            60 as ::core::ffi::c_int,
            b"Parser not created.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn namespace_teardown() {
    basic_teardown();
}

unsafe extern "C" fn test_return_ns_triplet() {
    _check_set_test_info(
        b"test_return_ns_triplet\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
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
    XML_SetReturnNSTriplet(
        g_parser,
        XML_TRUE as ::core::ffi::c_int,
    );
    XML_SetUserData(
        g_parser,
        &raw mut elemstr as *mut *const XML_Char
            as *mut ::core::ffi::c_void,
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
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetNamespaceDeclHandler(
        g_parser,
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
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                ) -> (),
        ),
    );
    g_triplet_start_flag =
        XML_FALSE as ::core::ffi::c_int;
    g_triplet_end_flag =
        XML_FALSE as ::core::ffi::c_int;
    init_dummy_handlers();
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            84 as ::core::ffi::c_int,
        );
    }
    XML_SetReturnNSTriplet(
        g_parser,
        XML_FALSE as ::core::ffi::c_int,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        epilog,
        strlen(epilog) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            89 as ::core::ffi::c_int,
        );
    }
    if g_triplet_start_flag == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            91 as ::core::ffi::c_int,
            b"triplet_start_checker not invoked\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if g_triplet_end_flag == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            93 as ::core::ffi::c_int,
            b"triplet_end_checker not invoked\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_START_NS_DECL_HANDLER_FLAG
            | DUMMY_END_NS_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            96 as ::core::ffi::c_int,
            b"Namespace handlers not called\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_ns_parser_reset() {
    _check_set_test_info(
        b"test_ns_parser_reset\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        104 as ::core::ffi::c_int,
    );
    let mut status: XML_ParsingStatus = XML_ParsingStatus { parsing:  XML_INITIALIZED, finalBuffer:  0 };
    XML_GetParsingStatus(
        g_parser,
        &raw mut status as *mut _ as *mut XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        != XML_INITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            109 as ::core::ffi::c_int,
            b"parsing status doesn't start INITIALIZED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    test_return_ns_triplet();
    XML_GetParsingStatus(
        g_parser,
        &raw mut status as *mut _ as *mut XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        != XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            113 as ::core::ffi::c_int,
            b"parsing status doesn't end FINISHED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(
        g_parser,
        ::core::ptr::null::<XML_Char>(),
    );
    XML_GetParsingStatus(
        g_parser,
        &raw mut status as *mut _ as *mut XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        != XML_INITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            117 as ::core::ffi::c_int,
            b"parsing status doesn't reset to INITIALIZED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn run_ns_tagname_overwrite_test(
    mut text: *const ::core::ffi::c_char,
    mut result: *const XML_Char,
) {
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    CharData_Init(
        &raw mut storage as *mut _ as *mut CharData,
    );
    XML_SetUserData(
        g_parser,
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    XML_SetElementHandler(
        g_parser,
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
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
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
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            130 as ::core::ffi::c_int,
        );
    }
    CharData_CheckXMLChars(
        &raw mut storage as *mut _ as *mut CharData,
        result,
    );
}

unsafe extern "C" fn test_ns_tagname_overwrite() {
    _check_set_test_info(
        b"test_ns_tagname_overwrite\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        135 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<n:e xmlns:n='http://example.org/'>\n  <n:f n:attr='foo'/>\n  <n:g n:attr2='bar'/>\n</n:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut result: *const XML_Char = b"start http://example.org/ e\nstart http://example.org/ f\nattribute http://example.org/ attr\nend http://example.org/ f\nstart http://example.org/ g\nattribute http://example.org/ attr2\nend http://example.org/ g\nend http://example.org/ e\n\0"
        .as_ptr() as *const XML_Char;
    run_ns_tagname_overwrite_test(text, result);
}

unsafe extern "C" fn test_ns_tagname_overwrite_triplet() {
    _check_set_test_info(
        b"test_ns_tagname_overwrite_triplet\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        153 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<n:e xmlns:n='http://example.org/'>\n  <n:f n:attr='foo'/>\n  <n:g n:attr2='bar'/>\n</n:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut result: *const XML_Char = b"start http://example.org/ e n\nstart http://example.org/ f n\nattribute http://example.org/ attr n\nend http://example.org/ f n\nstart http://example.org/ g n\nattribute http://example.org/ attr2 n\nend http://example.org/ g n\nend http://example.org/ e n\n\0"
        .as_ptr() as *const XML_Char;
    XML_SetReturnNSTriplet(
        g_parser,
        XML_TRUE as ::core::ffi::c_int,
    );
    run_ns_tagname_overwrite_test(text, result);
}

unsafe extern "C" fn test_start_ns_clears_start_element() {
    _check_set_test_info(
        b"test_start_ns_clears_start_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        172 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<e xmlns='http://example.org/'></e>\0".as_ptr() as *const ::core::ffi::c_char;
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_fail
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetStartNamespaceDeclHandler(
        g_parser,
        Some(
            start_ns_clearing_start_element
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndNamespaceDeclHandler(
        g_parser,
        Some(
            dummy_end_namespace_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_UseParserAsHandlerArg(g_parser);
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            185 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_default_ns_from_ext_subset_and_ext_ge() {
    _check_set_test_info(
        b"test_default_ns_from_ext_subset_and_ext_ge\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        190 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0'?>\n<!DOCTYPE doc SYSTEM 'http://example.org/doc.dtd' [\n  <!ENTITY en SYSTEM 'http://example.org/entity.ent'>\n]>\n<doc xmlns='http://example.org/ns1'>\n&en;\n</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetParamEntityParsing(
        g_parser,
        XML_PARAM_ENTITY_PARSING_ALWAYS,
    );
    XML_SetExternalEntityRefHandler(
        g_parser,
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
    XML_SetStartElementHandler(
        g_parser,
        Some(
            dummy_start_element
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        NULL,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            206 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_prefix_with_empty_uri_1() {
    _check_set_test_info(
        b"test_ns_prefix_with_empty_uri_1\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        211 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc xmlns:prefix='http://example.org/'>\n  <e xmlns:prefix=''/>\n</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_UNDECLARING_PREFIX,
        b"Did not report re-setting namespace URI with prefix to ''.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        218 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_prefix_with_empty_uri_2() {
    _check_set_test_info(
        b"test_ns_prefix_with_empty_uri_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        223 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0'?>\n<docelem xmlns:pre=''/>\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_UNDECLARING_PREFIX,
        b"Did not report setting namespace URI with prefix to ''.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        228 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_prefix_with_empty_uri_3() {
    _check_set_test_info(
        b"test_ns_prefix_with_empty_uri_3\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        233 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ELEMENT doc EMPTY>\n  <!ATTLIST doc\n    xmlns:prefix CDATA ''>\n]>\n<doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_UNDECLARING_PREFIX,
        b"Didn't report attr default setting NS w/ prefix to ''.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        242 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_prefix_with_empty_uri_4() {
    _check_set_test_info(
        b"test_ns_prefix_with_empty_uri_4\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        247 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ELEMENT prefix:doc EMPTY>\n  <!ATTLIST prefix:doc\n    xmlns:prefix CDATA 'http://example.org/'>\n]>\n<prefix:doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut elemstr: [*const XML_Char; 1] =
        [b"http://example.org/ doc prefix\0".as_ptr() as *const ::core::ffi::c_char];
    XML_SetReturnNSTriplet(
        g_parser,
        XML_TRUE as ::core::ffi::c_int,
    );
    XML_SetUserData(
        g_parser,
        &raw mut elemstr as *mut *const XML_Char
            as *mut ::core::ffi::c_void,
    );
    XML_SetEndElementHandler(
        g_parser,
        Some(
            triplet_end_checker
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
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
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            263 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_unbound_prefix() {
    _check_set_test_info(
        b"test_ns_unbound_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        268 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ELEMENT prefix:doc EMPTY>\n  <!ATTLIST prefix:doc\n    notxmlns:prefix CDATA 'http://example.org/'>\n]>\n<prefix:doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            278 as ::core::ffi::c_int,
            b"Unbound prefix incorrectly passed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(g_parser)
        as ::core::ffi::c_uint
        != XML_ERROR_UNBOUND_PREFIX as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            280 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_default_with_empty_uri() {
    _check_set_test_info(
        b"test_ns_default_with_empty_uri\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        284 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc xmlns='http://example.org/'>\n  <e xmlns=''/>\n</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    XML_SetStartNamespaceDeclHandler(
        g_parser,
        Some(
            dummy_start_namespace_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetEndNamespaceDeclHandler(
        g_parser,
        Some(
            dummy_end_namespace_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
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
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            294 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_duplicate_attrs_diff_prefixes() {
    _check_set_test_info(
        b"test_ns_duplicate_attrs_diff_prefixes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        299 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<doc xmlns:a='http://example.org/a'\n     xmlns:b='http://example.org/a'\n     a:a='v' b:a='v' />\0"
        .as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_DUPLICATE_ATTRIBUTE,
        b"did not report multiple attributes with same URI+name\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        304 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_duplicate_hashes() {
    _check_set_test_info(
        b"test_ns_duplicate_hashes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        308 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc xmlns:a='http://example.org/a'\n     a:a='v' a:i='w' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            329 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_unbound_prefix_on_attribute() {
    _check_set_test_info(
        b"test_ns_unbound_prefix_on_attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        334 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc a:attr=''/>\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_UNBOUND_PREFIX,
        b"did not report unbound prefix on attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        337 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_unbound_prefix_on_element() {
    _check_set_test_info(
        b"test_ns_unbound_prefix_on_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        342 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<a:doc/>\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_UNBOUND_PREFIX,
        b"did not report unbound prefix on element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        345 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_long_element() {
    _check_set_test_info(
        b"test_ns_long_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        350 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:thisisalongenoughelementnametotriggerareallocation\n xmlns:foo='http://example.org/' bar:a='12'\n xmlns:bar='http://example.org/'></foo:thisisalongenoughelementnametotriggerareallocation>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ thisisalongenoughelementnametotriggerareallocation foo\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"http://example.org/ a bar\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    XML_SetReturnNSTriplet(
        g_parser,
        XML_TRUE as ::core::ffi::c_int,
    );
    XML_SetUserData(
        g_parser,
        &raw mut elemstr as *mut *const XML_Char
            as *mut ::core::ffi::c_void,
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
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
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
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            366 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_mixed_prefix_atts() {
    _check_set_test_info(
        b"test_ns_mixed_prefix_atts\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        371 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<e a='12' bar:b='13'\n xmlns:bar='http://example.org/'></e>\0".as_ptr()
            as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            378 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_extend_uri_buffer() {
    _check_set_test_info(
        b"test_ns_extend_uri_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        386 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/'> <foo:thisisalongenoughnametotriggerallocationaction   foo:a='12' /></foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            393 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_reserved_attributes() {
    _check_set_test_info(
        b"test_ns_reserved_attributes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        400 as ::core::ffi::c_int,
    );
    let mut text1: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://example.org/' xmlns:xmlns='12' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://example.org/' foo:xmlns='12' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text1,
        XML_ERROR_RESERVED_PREFIX_XMLNS,
        b"xmlns not rejected as an attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        406 as ::core::ffi::c_int,
    );
    XML_ParserReset(
        g_parser,
        ::core::ptr::null::<XML_Char>(),
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text2,
        strlen(text2) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            410 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_reserved_attributes_2() {
    _check_set_test_info(
        b"test_ns_reserved_attributes_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
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
    _expect_failure(
        text1,
        XML_ERROR_RESERVED_PREFIX_XML,
        b"xml not rejected as an attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        423 as ::core::ffi::c_int,
    );
    XML_ParserReset(
        g_parser,
        ::core::ptr::null::<XML_Char>(),
    );
    _expect_failure(
        text2,
        XML_ERROR_RESERVED_NAMESPACE_URI,
        b"Use of w3.org URL not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        426 as ::core::ffi::c_int,
    );
    XML_ParserReset(
        g_parser,
        ::core::ptr::null::<XML_Char>(),
    );
    _expect_failure(
        text3,
        XML_ERROR_RESERVED_NAMESPACE_URI,
        b"Use of w3.org xmlns URL not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        429 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_extremely_long_prefix() {
    _check_set_test_info(
        b"test_ns_extremely_long_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        435 as ::core::ffi::c_int,
    );
    let mut text1: *const ::core::ffi::c_char = b"<doc ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:a='12'\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char = b" xmlns:ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP='foo'\n></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text1,
        strlen(text1) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            516 as ::core::ffi::c_int,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        text2,
        strlen(text2) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            519 as ::core::ffi::c_int,
        );
    }
}

unsafe extern "C" fn test_ns_unknown_encoding_success() {
    _check_set_test_info(
        b"test_ns_unknown_encoding_success\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        524 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='prefix-conv'?>\n<foo:e xmlns:foo='http://example.org/'>Hi</foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    _run_character_check(
        text,
        b"Hi\0".as_ptr() as *const XML_Char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        529 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_double_colon() {
    _check_set_test_info(
        b"test_ns_double_colon\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        534 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://example.org/' foo:a:b='bar' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    let status: XML_Status = _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as XML_Status;
    if status as ::core::ffi::c_uint
        == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        || XML_GetErrorCode(g_parser)
            as ::core::ffi::c_uint
            != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            542 as ::core::ffi::c_int,
            b"Double colon in attribute name not faulted (despite active namespace support)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_ns_double_colon_element() {
    _check_set_test_info(
        b"test_ns_double_colon_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        553 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<foo:bar:e xmlns:foo='http://example.org/' />\0".as_ptr() as *const ::core::ffi::c_char;
    let status: XML_Status = _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as XML_Status;
    if status as ::core::ffi::c_uint
        == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        || XML_GetErrorCode(g_parser)
            as ::core::ffi::c_uint
            != XML_ERROR_INVALID_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            561 as ::core::ffi::c_int,
            b"Double colon in element name not faulted (despite active namespace support)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_ns_bad_attr_leafname() {
    _check_set_test_info(
        b"test_ns_bad_attr_leafname\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        573 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<foo:e xmlns:foo='http://example.org/' foo:?ar='baz' />\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character in leafname not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        577 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_bad_element_leafname() {
    _check_set_test_info(
        b"test_ns_bad_element_leafname\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        581 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<foo:?oc xmlns:foo='http://example.org/' />\0".as_ptr() as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character in element leafname not faulted\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        585 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_utf16_leafname() {
    _check_set_test_info(
        b"test_ns_utf16_leafname\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        590 as ::core::ffi::c_int,
    );
    let text: [::core::ffi::c_char; 59] = ::core::mem::transmute::<
        [u8; 59],
        [::core::ffi::c_char; 59],
    >(
        *b"<\0n\0:\0e\0 \0x\0m\0l\0n\0s\0:\0n\0=\0'\0U\0R\0I\0'\0 \0n\0:\0\x04\x0E=\0'\0a\0'\0 \0/\0>\0\0",
    );
    let mut expected: *const XML_Char =
        b"a\0".as_ptr() as *const XML_Char;
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    CharData_Init(
        &raw mut storage as *mut _ as *mut CharData,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 59]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            605 as ::core::ffi::c_int,
        );
    }
    CharData_CheckXMLChars(
        &raw mut storage as *mut _ as *mut CharData,
        expected,
    );
}

unsafe extern "C" fn test_ns_utf16_element_leafname() {
    _check_set_test_info(
        b"test_ns_utf16_element_leafname\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        610 as ::core::ffi::c_int,
    );
    let text: [::core::ffi::c_char; 41] =
        ::core::mem::transmute::<[u8; 41], [::core::ffi::c_char; 41]>(
            *b"\0<\0n\0:\x0E\x04\0 \0x\0m\0l\0n\0s\0:\0n\0=\0'\0U\0R\0I\0'\0/\0>\0",
        );
    let mut expected: *const XML_Char =
        b"URI \xE0\xB8\x84\0".as_ptr() as *const XML_Char;
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    CharData_Init(
        &raw mut storage as *mut _ as *mut CharData,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_event_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUserData(
        g_parser,
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 41]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            628 as ::core::ffi::c_int,
        );
    }
    CharData_CheckXMLChars(
        &raw mut storage as *mut _ as *mut CharData,
        expected,
    );
}

unsafe extern "C" fn test_ns_utf16_doctype() {
    _check_set_test_info(
        b"test_ns_utf16_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        633 as ::core::ffi::c_int,
    );
    let text: [::core::ffi::c_char; 155] = ::core::mem::transmute::<
        [u8; 155],
        [::core::ffi::c_char; 155],
    >(
        *b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0f\0o\0o\0:\x0E\x04\0 \0[\0 \0<\0!\0E\0N\0T\0I\0T\0Y\0 \0b\0a\0r\0 \0'\0b\0a\0z\0'\0>\0 \0]\0>\0\n\0<\0f\0o\0o\0:\x0E\x04\0 \0x\0m\0l\0n\0s\0:\0f\0o\0o\0=\0'\0U\0R\0I\0'\0>\0&\0b\0a\0r\0;\0<\0/\0f\0o\0o\0:\x0E\x04\0>\0",
    );
    let mut expected: *const XML_Char =
        b"URI \xE0\xB8\x84\0".as_ptr() as *const XML_Char;
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    CharData_Init(
        &raw mut storage as *mut _ as *mut CharData,
    );
    XML_SetUserData(
        g_parser,
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    XML_SetStartElementHandler(
        g_parser,
        Some(
            start_element_event_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    XML_SetUnknownEncodingHandler(
        g_parser,
        ::core::mem::transmute(Some(
            MiscEncodingHandler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Encoding,
                ) -> ::core::ffi::c_int,
        )),
        NULL,
    );
    if _XML_Parse_SINGLE_BYTES(
        g_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 155]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            659 as ::core::ffi::c_int,
        );
    }
    CharData_CheckXMLChars(
        &raw mut storage as *mut _ as *mut CharData,
        expected,
    );
}

unsafe extern "C" fn test_ns_invalid_doctype() {
    _check_set_test_info(
        b"test_ns_invalid_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        664 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE foo:!bad [ <!ENTITY bar 'baz' ]>\n<foo:!bad>&bar;</foo:!bad>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_INVALID_TOKEN,
        b"Invalid character in document local name not faulted\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        669 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_double_colon_doctype() {
    _check_set_test_info(
        b"test_ns_double_colon_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        673 as ::core::ffi::c_int,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE foo:a:doc [ <!ENTITY bar 'baz' ]>\n<foo:a:doc>&bar;</foo:a:doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    _expect_failure(
        text,
        XML_ERROR_SYNTAX,
        b"Double colon in document name not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        678 as ::core::ffi::c_int,
    );
}

unsafe extern "C" fn test_ns_separator_in_uri() {
    _check_set_test_info(
        b"test_ns_separator_in_uri\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
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
        set_subtest(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            cases[i as usize].doc,
        );
        let mut parser: XML_Parser = XML_ParserCreateNS(
            ::core::ptr::null::<XML_Char>(),
            cases[i as usize].namesep,
        );
        XML_SetElementHandler(
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
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            parser,
            cases[i as usize].doc,
            strlen(cases[i as usize].doc) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != cases[i as usize].expectedStatus as ::core::ffi::c_uint
        {
            failCount = failCount.wrapping_add(1);
        }
        XML_ParserFree(parser);
        i = i.wrapping_add(1);
    }
    if failCount != 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/ns_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            709 as ::core::ffi::c_int,
            b"Namespace separator handling is broken\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn make_namespace_test_case(mut s: *mut Suite) {
    let mut tc_namespace: *mut TCase =
        tcase_create(
            b"XML namespaces\0".as_ptr() as *const ::core::ffi::c_char
        ) as *mut TCase;
    suite_add_tcase(
        s as *mut Suite,
        tc_namespace as *mut TCase,
    );
    tcase_add_checked_fixture(
        tc_namespace as *mut TCase,
        Some(namespace_setup as unsafe extern "C" fn() -> ()),
        Some(namespace_teardown as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_return_ns_triplet as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_parser_reset as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_tagname_overwrite as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_tagname_overwrite_triplet as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_start_ns_clears_start_element as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        tc_namespace as *mut TCase,
        Some(test_default_ns_from_ext_subset_and_ext_ge as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_prefix_with_empty_uri_1 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_prefix_with_empty_uri_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_prefix_with_empty_uri_3 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_prefix_with_empty_uri_4 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_unbound_prefix as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_default_with_empty_uri as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_duplicate_attrs_diff_prefixes as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_duplicate_hashes as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_unbound_prefix_on_attribute as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_unbound_prefix_on_element as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_long_element as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_mixed_prefix_atts as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_extend_uri_buffer as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_reserved_attributes as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_reserved_attributes_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_extremely_long_prefix as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_unknown_encoding_success as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_double_colon as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_double_colon_element as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_bad_attr_leafname as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_bad_element_leafname as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_utf16_leafname as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_utf16_element_leafname as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        tc_namespace as *mut TCase,
        Some(test_ns_utf16_doctype as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_invalid_doctype as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_double_colon_doctype as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        tc_namespace as *mut TCase,
        Some(test_ns_separator_in_uri as unsafe extern "C" fn() -> ()),
    );
}
