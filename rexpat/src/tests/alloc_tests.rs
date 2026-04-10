pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::common_h::ALLOC_ALWAYS_SUCCEED;
pub use crate::common_h::REALLOC_ALWAYS_SUCCEED;
pub use crate::expat_external_h::XML_Char;
pub use crate::src::tests::chardata::CharData;
pub use crate::src::tests::chardata::CharData_CheckXMLChars;
pub use crate::src::tests::chardata::CharData_Init;
pub use crate::src::tests::common::_XML_Parse_SINGLE_BYTES;
pub use crate::src::tests::common::_expect_failure;
pub use crate::src::tests::common::_xml_failure;
pub use crate::src::tests::common::basic_teardown;
pub use crate::src::tests::common::duff_allocator;
pub use crate::src::tests::common::duff_reallocator;
pub use crate::src::tests::common::g_allocation_count;
pub use crate::src::tests::common::g_parser;
pub use crate::src::tests::common::g_reallocation_count;
pub use crate::src::tests::common::get_buffer_test_text;
pub use crate::src::tests::common::tcase_add_test__if_xml_ge;
pub use crate::src::tests::common::tcase_add_test__ifdef_xml_dtd;
pub use crate::src::tests::dummy::dummy_attlist_decl_handler;
pub use crate::src::tests::dummy::dummy_comment_handler;
pub use crate::src::tests::dummy::dummy_element_decl_handler;
pub use crate::src::tests::dummy::dummy_end_cdata_handler;
pub use crate::src::tests::dummy::dummy_end_doctype_decl_handler;
pub use crate::src::tests::dummy::dummy_end_doctype_handler;
pub use crate::src::tests::dummy::dummy_entity_decl_handler;
pub use crate::src::tests::dummy::dummy_notation_decl_handler;
pub use crate::src::tests::dummy::dummy_pi_handler;
pub use crate::src::tests::dummy::dummy_start_cdata_handler;
pub use crate::src::tests::dummy::dummy_start_doctype_decl_handler;
pub use crate::src::tests::dummy::dummy_start_doctype_handler;
pub use crate::src::tests::dummy::dummy_unparsed_entity_decl_handler;
pub use crate::src::tests::dummy::dummy_xdecl_handler;
pub use crate::src::tests::dummy::get_dummy_handler_flags;
pub use crate::src::tests::dummy::init_dummy_handlers;
pub use crate::src::tests::dummy::DUMMY_ATTLIST_DECL_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_COMMENT_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_ELEMENT_DECL_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_END_CDATA_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_END_DOCTYPE_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_ENTITY_DECL_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_NOTATION_DECL_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_PI_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_START_CDATA_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_START_DOCTYPE_HANDLER_FLAG;
pub use crate::src::tests::dummy::DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG;
pub use crate::stdlib::__assert_fail;
pub use crate::stdlib::__ASSERT_FUNCTION;

pub use crate::expat_h::XML_AttlistDeclHandler;
pub use crate::expat_h::XML_Bool;
pub use crate::expat_h::XML_CharacterDataHandler;
pub use crate::expat_h::XML_CommentHandler;
pub use crate::expat_h::XML_Content;
pub use crate::expat_h::XML_Content_Quant;
pub use crate::expat_h::XML_Content_Type;
pub use crate::expat_h::XML_DefaultHandler;
pub use crate::expat_h::XML_ElementDeclHandler;
pub use crate::expat_h::XML_Encoding;
pub use crate::expat_h::XML_EndCdataSectionHandler;
pub use crate::expat_h::XML_EndDoctypeDeclHandler;
pub use crate::expat_h::XML_EntityDeclHandler;
pub use crate::expat_h::XML_Error;
pub use crate::expat_h::XML_ExternalEntityRefHandler;
pub use crate::expat_h::XML_Memory_Handling_Suite;
pub use crate::expat_h::XML_NotationDeclHandler;
pub use crate::expat_h::XML_ParamEntityParsing;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ParserStruct;
pub use crate::expat_h::XML_ProcessingInstructionHandler;
pub use crate::expat_h::XML_StartCdataSectionHandler;
pub use crate::expat_h::XML_StartDoctypeDeclHandler;
pub use crate::expat_h::XML_StartElementHandler;
pub use crate::expat_h::XML_Status;
pub use crate::expat_h::XML_UnknownEncodingHandler;
pub use crate::expat_h::XML_UnparsedEntityDeclHandler;
pub use crate::expat_h::XML_XmlDeclHandler;
pub use crate::expat_h::XML_cp;
pub use crate::expat_h::XML_CQUANT_NONE;
pub use crate::expat_h::XML_CQUANT_OPT;
pub use crate::expat_h::XML_CQUANT_PLUS;
pub use crate::expat_h::XML_CQUANT_REP;
pub use crate::expat_h::XML_CTYPE_ANY;
pub use crate::expat_h::XML_CTYPE_CHOICE;
pub use crate::expat_h::XML_CTYPE_EMPTY;
pub use crate::expat_h::XML_CTYPE_MIXED;
pub use crate::expat_h::XML_CTYPE_NAME;
pub use crate::expat_h::XML_CTYPE_SEQ;
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
pub use crate::expat_h::XML_PARAM_ENTITY_PARSING_ALWAYS;
pub use crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
pub use crate::expat_h::XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE;
pub use crate::expat_h::XML_STATUS_ERROR;
pub use crate::expat_h::XML_STATUS_OK;
pub use crate::expat_h::XML_STATUS_SUSPENDED;
pub use crate::expat_h::XML_TRUE;
pub use crate::internal_h::EXPAT_MALLOC_ALIGNMENT;
pub use crate::internal_h::EXPAT_MALLOC_PADDING;
pub use crate::src::lib::xmlparse::expat_free;
pub use crate::src::lib::xmlparse::expat_malloc;
pub use crate::src::lib::xmlparse::expat_realloc;
pub use crate::src::lib::xmlparse::g_reparseDeferralEnabledDefault;
pub use crate::src::lib::xmlparse::XML_ExternalEntityParserCreate;
pub use crate::src::lib::xmlparse::XML_GetBuffer;
pub use crate::src::lib::xmlparse::XML_GetErrorCode;
pub use crate::src::lib::xmlparse::XML_MemFree;
pub use crate::src::lib::xmlparse::XML_MemMalloc;
pub use crate::src::lib::xmlparse::XML_MemRealloc;
pub use crate::src::lib::xmlparse::XML_ParseBuffer;
pub use crate::src::lib::xmlparse::XML_ParserCreate;
pub use crate::src::lib::xmlparse::XML_ParserCreate_MM;
pub use crate::src::lib::xmlparse::XML_ParserFree;
pub use crate::src::lib::xmlparse::XML_ParserReset;
pub use crate::src::lib::xmlparse::XML_SetAllocTrackerActivationThreshold;
pub use crate::src::lib::xmlparse::XML_SetAllocTrackerMaximumAmplification;
pub use crate::src::lib::xmlparse::XML_SetAttlistDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetBase;
pub use crate::src::lib::xmlparse::XML_SetCdataSectionHandler;
pub use crate::src::lib::xmlparse::XML_SetCharacterDataHandler;
pub use crate::src::lib::xmlparse::XML_SetCommentHandler;
pub use crate::src::lib::xmlparse::XML_SetDefaultHandler;
pub use crate::src::lib::xmlparse::XML_SetDoctypeDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetElementDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetEncoding;
pub use crate::src::lib::xmlparse::XML_SetEntityDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler;
pub use crate::src::lib::xmlparse::XML_SetNotationDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetParamEntityParsing;
pub use crate::src::lib::xmlparse::XML_SetProcessingInstructionHandler;
pub use crate::src::lib::xmlparse::XML_SetStartElementHandler;
pub use crate::src::lib::xmlparse::XML_SetUnknownEncodingHandler;
pub use crate::src::lib::xmlparse::XML_SetUnparsedEntityDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetUserData;
pub use crate::src::lib::xmlparse::XML_SetXmlDeclHandler;
pub use crate::src::lib::xmlparse::XML_UseForeignDTD;
pub use crate::src::tests::handlers::accumulate_characters;
pub use crate::src::tests::handlers::ext_faults;
pub use crate::src::tests::handlers::external_entity_alloc;
pub use crate::src::tests::handlers::external_entity_alloc_set_encoding;
pub use crate::src::tests::handlers::external_entity_dbl_handler;
pub use crate::src::tests::handlers::external_entity_dbl_handler_2;
pub use crate::src::tests::handlers::external_entity_duff_loader;
pub use crate::src::tests::handlers::external_entity_faulter;
pub use crate::src::tests::handlers::external_entity_null_loader;
pub use crate::src::tests::handlers::external_entity_optioner;
pub use crate::src::tests::handlers::external_entity_parser_create_alloc_fail_handler;
pub use crate::src::tests::handlers::external_entity_public;
pub use crate::src::tests::handlers::external_entity_reallocator;
pub use crate::src::tests::handlers::long_encoding_handler;
pub use crate::src::tests::handlers::record_element_start_handler;
pub use crate::src::tests::handlers::unknown_released_encoding_handler;
pub use crate::src::tests::handlers::ExtFaults;
pub use crate::src::tests::handlers::ExtOption;
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
pub use crate::stdbool_h::false_0;
pub use crate::stdbool_h::true_0;
use crate::stdlib::free;
use crate::stdlib::malloc;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::realloc;
use crate::stdlib::strlen;

unsafe extern "C" fn alloc_setup() {
    let mut memsuite: XML_Memory_Handling_Suite =
        XML_Memory_Handling_Suite {
    malloc_fcn:  Some(
                duff_allocator
                    as unsafe extern "C" fn(
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
    realloc_fcn:  Some(
                duff_reallocator
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
    free_fcn:  Some(
                free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
};
    g_allocation_count = ALLOC_ALWAYS_SUCCEED;
    g_reallocation_count = REALLOC_ALWAYS_SUCCEED;
    g_parser = XML_ParserCreate_MM(
        ::core::ptr::null::<XML_Char>(),
        
        &raw mut memsuite as *const XML_Memory_Handling_Suite,
        ::core::ptr::null::<XML_Char>(),
    );
    if g_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            74i32,
            b"Parser not created\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn alloc_teardown() {
    basic_teardown();
}

unsafe extern "C" fn test_alloc_parse_xdecl() {
    _check_set_test_info(
        b"test_alloc_parse_xdecl\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        83,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='utf-8'?>\n<doc>Hello, world</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 15;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetXmlDeclHandler(
            g_parser,
            Some(
                dummy_xdecl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            104i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            106i32,
            b"Parse failed with max allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_parse_xdecl_2() {
    _check_set_test_info(
        b"test_alloc_parse_xdecl_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        113,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='ThisIsAStupidlyLongEncodingNameIntendedToTriggerPoolGrowth123456ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMN'?><doc>Hello, world</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetXmlDeclHandler(
            g_parser,
            Some(
                dummy_xdecl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUnknownEncodingHandler(
            g_parser,
            ::core::mem::transmute(Some(
                long_encoding_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Encoding,
                    ) -> ::core::ffi::c_int,
            )),
            NULL,
        );
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            150i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            152i32,
            b"Parse failed with max allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_parse_pi() {
    _check_set_test_info(
        b"test_alloc_parse_pi\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        157,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='utf-8'?>\n<?pi unknown?>\n<doc>Hello, world</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 15;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            177i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            179i32,
            b"Parse failed with max allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_parse_pi_2() {
    _check_set_test_info(
        b"test_alloc_parse_pi_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        183,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='utf-8'?>\n<doc>Hello, world<?pi unknown?>\n</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 15;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            203i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            205i32,
            b"Parse failed with max allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_parse_pi_3() {
    _check_set_test_info(
        b"test_alloc_parse_pi_3\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        209,
    );
    let mut text: *const ::core::ffi::c_char = b"<?This processing instruction should be long enough to ensure thatit triggers the growth of an internal string pool when the      allocator fails at a cruicial moment FGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPQ?><doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            244i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            246i32,
            b"Parse failed with max allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_parse_comment() {
    _check_set_test_info(
        b"test_alloc_parse_comment\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        250,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='utf-8'?>\n<!-- Test parsing this comment --><doc>Hi</doc>\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 15;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetCommentHandler(
            g_parser,
            Some(
                dummy_comment_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            268i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            270i32,
            b"Parse failed with max allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_parse_comment_2() {
    _check_set_test_info(
        b"test_alloc_parse_comment_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        274,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='utf-8'?>\n<doc>Hello, world<!-- Parse this comment too --></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 15;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetCommentHandler(
            g_parser,
            Some(
                dummy_comment_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            294i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            296i32,
            b"Parse failed with max allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_create_external_parser() {
    _check_set_test_info(
        b"test_alloc_create_external_parser\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        303,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut foo_text: [::core::ffi::c_char; 26] = ::core::mem::transmute::<
        [u8; 26],
        [::core::ffi::c_char; 26],
    >(*b"<!ELEMENT doc (#PCDATA)*>\0");
    XML_SetParamEntityParsing(
        g_parser,
        XML_PARAM_ENTITY_PARSING_ALWAYS,
    );
    XML_SetUserData(
        g_parser,
        
        &raw mut foo_text as *mut ::core::ffi::c_void,
    );
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_duff_loader
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                    *const XML_Char,
                ) -> ::core::ffi::c_int,
        ),
    );
    if  _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        !=  XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            314i32,
            b"External parser allocator returned success incorrectly\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_run_external_parser() {
    _check_set_test_info(
        b"test_alloc_run_external_parser\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        320,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut foo_text: [::core::ffi::c_char; 26] = ::core::mem::transmute::<
        [u8; 26],
        [::core::ffi::c_char; 26],
    >(*b"<!ELEMENT doc (#PCDATA)*>\0");
    let mut i: ::core::ffi::c_uint = 0;
    let max_alloc_count: ::core::ffi::c_uint = 15;
    i = 0;
    while i < max_alloc_count {
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
        XML_SetUserData(
            g_parser,
            
            &raw mut foo_text as *mut ::core::ffi::c_void,
        );
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_null_loader
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        g_allocation_count = i as ::core::ffi::c_int;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i = i.wrapping_add(1);
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            341i32,
            b"Parsing ignored failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            343i32,
            b"Parsing failed with allocation count 10\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_dtd_copy_default_atts() {
    _check_set_test_info(
        b"test_alloc_dtd_copy_default_atts\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        350,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0'?>\n<!DOCTYPE doc SYSTEM 'http://example.org/doc.dtd' [\n  <!ENTITY en SYSTEM 'http://example.org/entity.ent'>\n]>\n<doc xmlns='http://example.org/ns1'>\n&en;\n</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut callno: ::core::ffi::c_int = 0;
    XML_SetParamEntityParsing(
        g_parser,
        XML_PARAM_ENTITY_PARSING_ALWAYS,
    );
    XML_SetExternalEntityRefHandler(
        g_parser,
        Some(
            external_entity_dbl_handler
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
        &raw mut callno as *mut ::core::ffi::c_void,
    );
    if  _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        ==  XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            365i32,
        );
    }
}

unsafe extern "C" fn test_alloc_external_entity() {
    _check_set_test_info(
        b"test_alloc_external_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        370,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0'?>\n<!DOCTYPE doc SYSTEM 'http://example.org/doc.dtd' [\n  <!ENTITY en SYSTEM 'http://example.org/entity.ent'>\n]>\n<doc xmlns='http://example.org/ns1'>\n&en;\n</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let alloc_test_max_repeats: ::core::ffi::c_int = 50;
    let mut callno: ::core::ffi::c_int = 0;
    i = 0;
    while i < alloc_test_max_repeats {
        g_allocation_count = -1;
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_dbl_handler_2
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        callno = 0;
        XML_SetUserData(
            g_parser,
            &raw mut callno as *mut ::core::ffi::c_void,
        );
        g_allocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            ==  XML_STATUS_OK
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    g_allocation_count = -1;
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            398i32,
            b"External entity parsed despite duff allocator\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if i == alloc_test_max_repeats {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            400i32,
            b"External entity not parsed at max allocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_ext_entity_set_encoding() {
    _check_set_test_info(
        b"test_alloc_ext_entity_set_encoding\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        405,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_allocation_count: ::core::ffi::c_int = 30;
    i = 0;
    while i < max_allocation_count {
        XML_SetExternalEntityRefHandler(
            g_parser,
            Some(
                external_entity_alloc_set_encoding
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                        *const XML_Char,
                    ) -> ::core::ffi::c_int,
            ),
        );
        g_allocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            ==  XML_STATUS_OK
        {
            break;
        }
        g_allocation_count = -1;
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            426i32,
            b"Encoding check succeeded despite failing allocator\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if i == max_allocation_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            428i32,
            b"Encoding failed at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_internal_entity() {
    _check_set_test_info(
        b"test_alloc_internal_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        435,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='unsupported-encoding'?>\n<!DOCTYPE test [<!ENTITY foo 'bar'>]>\n<test a='&foo;'/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_uint = 0;
    let max_alloc_count: ::core::ffi::c_uint = 20;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i as ::core::ffi::c_int;
        XML_SetUnknownEncodingHandler(
            g_parser,
            ::core::mem::transmute(Some(
                unknown_released_encoding_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Encoding,
                    ) -> ::core::ffi::c_int,
            )),
            NULL,
        );
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i = i.wrapping_add(1);
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            454i32,
            b"Internal entity worked despite failing allocations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            456i32,
            b"Internal entity failed at max allocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_parameter_entity() {
    _check_set_test_info(
        b"test_alloc_parameter_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        460,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE foo [<!ENTITY % param1 \"<!ENTITY internal 'some_text'>\">%param1;]> <foo>&internal;content</foo>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let alloc_test_max_repeats: ::core::ffi::c_int = 30;
    i = 0;
    while i < alloc_test_max_repeats {
        g_allocation_count = i;
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    g_allocation_count = -1;
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            479i32,
            b"Parameter entity processed despite duff allocator\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if i == alloc_test_max_repeats {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            481i32,
            b"Parameter entity not processed at max allocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_dtd_default_handling() {
    _check_set_test_info(
        b"test_alloc_dtd_default_handling\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        488,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ENTITY e SYSTEM 'http://example.org/e'>\n<!NOTATION n SYSTEM 'http://example.org/n'>\n<!ENTITY e1 SYSTEM 'http://example.org/e' NDATA n>\n<!ELEMENT doc (#PCDATA)>\n<!ATTLIST doc a CDATA #IMPLIED>\n<?pi in dtd?>\n<!--comment in dtd-->\n]>\n<doc><![CDATA[text in doc]]></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"\n\n\n\n\n\n\n\n\n<doc>text in doc</doc>\0".as_ptr()
            as *const XML_Char;
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 25;
    i = 0;
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
            ::core::mem::transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            )),
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
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        XML_SetCdataSectionHandler(
            g_parser,
            Some(
                dummy_start_cdata_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
            Some(
                dummy_end_cdata_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
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
        CharData_Init(
            
            &raw mut storage,
        );
        XML_SetUserData(
            g_parser,
            &raw mut storage as *mut ::core::ffi::c_void,
        );
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            531i32,
            b"Default DTD parsed despite allocation failures\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            533i32,
            b"Default DTD not parsed with maximum alloc count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    CharData_CheckXMLChars(
        
        &raw mut storage,
        expected,
    );
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
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            542i32,
            b"Not all handlers were called\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_explicit_encoding() {
    _check_set_test_info(
        b"test_alloc_explicit_encoding\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        547,
    );
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 5;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        if  XML_SetEncoding(
            g_parser,
            b"us-ascii\0".as_ptr() as *const XML_Char,
        )
            ==  XML_STATUS_OK
        {
            break;
        }
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            557i32,
            b"Encoding set despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            559i32,
            b"Encoding not set at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_set_base() {
    _check_set_test_info(
        b"test_alloc_set_base\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        564,
    );
    let mut new_base: *const XML_Char =
        b"/local/file/name.xml\0".as_ptr() as *const XML_Char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 5;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        if  XML_SetBase(g_parser, new_base)
            ==  XML_STATUS_OK
        {
            break;
        }
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            575i32,
            b"Base set despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            577i32,
            b"Base not set with max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_buffer() {
    _check_set_test_info(
        b"test_alloc_realloc_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        582,
    );
    let mut text: *const ::core::ffi::c_char = get_buffer_test_text;
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
    while i < max_realloc_count {
        g_reallocation_count = i;
        buffer = XML_GetBuffer(
            g_parser,
            1536,
        );
        if buffer.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                593i32,
                b"1.5K buffer reallocation failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !buffer.is_null() {
        } else {
            __assert_fail(
                b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                594u32,
                __ASSERT_FUNCTION.as_ptr(),
            );
        };
        memcpy(
            buffer,
            text as *const ::core::ffi::c_void,
            strlen(text),
        );
        if  XML_ParseBuffer(
            g_parser,
            strlen(text) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        )
            ==  XML_STATUS_OK
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    g_reallocation_count = -1;
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            605i32,
            b"Parse succeeded with no reallocation\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            607i32,
            b"Parse failed with max reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_ext_entity_realloc_buffer() {
    _check_set_test_info(
        b"test_alloc_ext_entity_realloc_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        612,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
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
        XML_SetUserData(
            g_parser,
            &raw mut i as *mut ::core::ffi::c_void,
        );
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            ==  XML_STATUS_OK
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            631i32,
            b"Succeeded with no reallocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            633i32,
            b"Failed with max reallocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_many_attributes() {
    _check_set_test_info(
        b"test_alloc_realloc_many_attributes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        638,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ATTLIST doc za CDATA 'default'>\n<!ATTLIST doc zb CDATA 'def2'>\n<!ATTLIST doc zc CDATA 'def3'>\n]>\n<doc a='1'     b='2'     c='3'     d='4'     e='5'     f='6'     g='7'     h='8'     i='9'     j='10'     k='11'     l='12'     m='13'     n='14'     p='15'     q='16'     r='17'     s='18'></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
    while i < max_realloc_count {
        g_reallocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            676i32,
            b"Parse succeeded despite no reallocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            678i32,
            b"Parse failed at max reallocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_public_entity_value() {
    _check_set_test_info(
        b"test_alloc_public_entity_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        683,
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
    let max_alloc_count: ::core::ffi::c_int = 50;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        init_dummy_handlers();
        XML_SetUserData(
            g_parser,
            
            &raw mut dtd_text as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            728i32,
            b"Parsing worked despite failing allocation\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            730i32,
            b"Parsing failed at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_ENTITY_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            732i32,
            b"Entity declaration handler not called\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_subst_public_entity_value() {
    _check_set_test_info(
        b"test_alloc_realloc_subst_public_entity_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        736,
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
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
    while i < max_realloc_count {
        g_reallocation_count = i;
        XML_SetUserData(
            g_parser,
            
            &raw mut dtd_text as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            792i32,
            b"Parsing worked despite failing reallocation\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            794i32,
            b"Parsing failed at max reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_parse_public_doctype() {
    _check_set_test_info(
        b"test_alloc_parse_public_doctype\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        798,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='utf-8'?>\n<!DOCTYPE doc PUBLIC 'http://example.com/a/long/enough/name/to/trigger/pool/growth/zz/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/' 'test'>\n<doc></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 25;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            837i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            839i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG
            | DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            843i32,
            b"Doctype handler functions not called\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_parse_public_doctype_long_name() {
    _check_set_test_info(
        b"test_alloc_parse_public_doctype_long_name\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        847,
    );
    let mut text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='utf-8'?>\n<!DOCTYPE doc PUBLIC 'http://example.com/foo' 'ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOPABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOP'>\n<doc></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 25;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            885i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            887i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_set_foreign_dtd() {
    _check_set_test_info(
        b"test_alloc_set_foreign_dtd\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        892,
    );
    let mut text1: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut text2: [::core::ffi::c_char; 26] = ::core::mem::transmute::<
        [u8; 26],
        [::core::ffi::c_char; 26],
    >(*b"<!ELEMENT doc (#PCDATA)*>\0");
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 25;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
        XML_SetUserData(
            g_parser,
            &raw mut text2 as *mut ::core::ffi::c_void,
        );
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
        if  XML_UseForeignDTD(
            g_parser,
            XML_TRUE,
        )
            !=  XML_ERROR_NONE
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                905i32,
                b"Could not set foreign DTD\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text1,
            strlen(text1) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            914i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            916i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_attribute_enum_value() {
    _check_set_test_info(
        b"test_alloc_attribute_enum_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        921,
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
    let max_alloc_count: ::core::ffi::c_int = 30;
    i = 0;
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
            
            &raw mut dtd_text as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            946i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            948i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_attribute_enum_value() {
    _check_set_test_info(
        b"test_alloc_realloc_attribute_enum_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        953,
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
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
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
            
            &raw mut dtd_text as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1002i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1004i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_implied_attribute() {
    _check_set_test_info(
        b"test_alloc_realloc_implied_attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1009,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc EMPTY>\n<!ATTLIST doc a (ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|BBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|CBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|DBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|EBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|FBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|GBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|HBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|IBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|JBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|KBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|LBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|MBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|NBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|OBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|PBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMN) #IMPLIED>\n]><doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1052i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1054i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_default_attribute() {
    _check_set_test_info(
        b"test_alloc_realloc_default_attribute\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1059,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc EMPTY>\n<!ATTLIST doc a (ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|BBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|CBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|DBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|EBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|FBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|GBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|HBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|IBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|JBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|KBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|LBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|MBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|NBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|OBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO|PBCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMN) 'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNO'>\n]><doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1102i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1104i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_notation() {
    _check_set_test_info(
        b"test_alloc_notation\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1109,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!NOTATION ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP SYSTEM 'http://example.org/n'>\n<!ENTITY e SYSTEM 'http://example.org/e' NDATA ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1167i32,
            b"Parse succeeded despite allocation failures\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1169i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_ENTITY_DECL_HANDLER_FLAG
            | DUMMY_NOTATION_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1172i32,
            b"Entity declaration handler not called\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_public_notation() {
    _check_set_test_info(
        b"test_alloc_public_notation\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1177,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!NOTATION note PUBLIC 'http://example.com/a/long/enough/name/to/trigger/pool/growth/zz/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/' 'foo'>\n<!ENTITY e SYSTEM 'http://example.com/e' NDATA note>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1217i32,
            b"Parse succeeded despite allocation failures\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1219i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_NOTATION_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1221i32,
            b"Notation handler not called\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_system_notation() {
    _check_set_test_info(
        b"test_alloc_system_notation\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1226,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!NOTATION note SYSTEM 'http://example.com/a/long/enough/name/to/trigger/pool/growth/zz/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/'>\n<!ENTITY e SYSTEM 'http://example.com/e' NDATA note>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1266i32,
            b"Parse succeeded despite allocation failures\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1268i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_NOTATION_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1270i32,
            b"Notation handler not called\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_nested_groups() {
    _check_set_test_info(
        b"test_alloc_nested_groups\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1274,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (e,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?))))))))))))))))))))))))))))))))>\n<!ELEMENT e EMPTY>]>\n<doc><e/></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        CharData_Init(
            
            &raw mut storage,
        );
        XML_SetElementDeclHandler(
            g_parser,
            ::core::mem::transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            )),
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
        XML_SetUserData(
            g_parser,
            &raw mut storage as *mut ::core::ffi::c_void,
        );
        init_dummy_handlers();
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1305i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1307i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    CharData_CheckXMLChars(
        
        &raw mut storage,
        b"doce\0".as_ptr() as *const XML_Char,
    );
    if get_dummy_handler_flags()
        != DUMMY_ELEMENT_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1310i32,
            b"Element handler not fired\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_nested_groups() {
    _check_set_test_info(
        b"test_alloc_realloc_nested_groups\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1314,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (e,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?,(e?))))))))))))))))))))))))))))))))>\n<!ELEMENT e EMPTY>]>\n<doc><e/></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
    while i < max_realloc_count {
        g_reallocation_count = i;
        CharData_Init(
            
            &raw mut storage,
        );
        XML_SetElementDeclHandler(
            g_parser,
            ::core::mem::transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            )),
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
        XML_SetUserData(
            g_parser,
            &raw mut storage as *mut ::core::ffi::c_void,
        );
        init_dummy_handlers();
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1345i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1347i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    CharData_CheckXMLChars(
        
        &raw mut storage,
        b"doce\0".as_ptr() as *const XML_Char,
    );
    if get_dummy_handler_flags()
        != DUMMY_ELEMENT_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1350i32,
            b"Element handler not fired\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_large_group() {
    _check_set_test_info(
        b"test_alloc_large_group\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1354,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (a1|a2|a3|a4|a5|a6|a7|a8|b1|b2|b3|b4|b5|b6|b7|b8|c1|c2|c3|c4|c5|c6|c7|c8|d1|d2|d3|d4|d5|d6|d7|d8|e1)+>\n]>\n<doc>\n<a1/>\n</doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 50;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetElementDeclHandler(
            g_parser,
            ::core::mem::transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            )),
        );
        init_dummy_handlers();
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1382i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1384i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_ELEMENT_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1386i32,
            b"Element handler flag not raised\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_group_choice() {
    _check_set_test_info(
        b"test_alloc_realloc_group_choice\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1390,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n<!ELEMENT doc (a1|a2|a3|a4|a5|a6|a7|a8|b1|b2|b3|b4|b5|b6|b7|b8|c1|c2|c3|c4|c5|c6|c7|c8|d1|d2|d3|d4|d5|d6|d7|d8|e1)+>\n]>\n<doc>\n<a1/>\n<b2 attr='foo'>This is a foo</b2>\n<c3></c3>\n</doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
    while i < max_realloc_count {
        g_reallocation_count = i;
        XML_SetElementDeclHandler(
            g_parser,
            ::core::mem::transmute(Some(
                dummy_element_decl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *mut XML_Content,
                    ) -> (),
            )),
        );
        init_dummy_handlers();
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1420i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1422i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_ELEMENT_DECL_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1424i32,
            b"Element handler flag not raised\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_pi_in_epilog() {
    _check_set_test_info(
        b"test_alloc_pi_in_epilog\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1428,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></doc>\n<?pi in epilog?>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 15;
    i = 0;
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1446i32,
            b"Parse completed despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1448i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_PI_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1450i32,
            b"Processing instruction handler not invoked\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_comment_in_epilog() {
    _check_set_test_info(
        b"test_alloc_comment_in_epilog\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1454,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc></doc>\n<!-- comment in epilog -->\0".as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 15;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetCommentHandler(
            g_parser,
            Some(
                dummy_comment_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                    ) -> (),
            ),
        );
        init_dummy_handlers();
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1472i32,
            b"Parse completed despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1474i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if get_dummy_handler_flags()
        != DUMMY_COMMENT_HANDLER_FLAG
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1476i32,
            b"Processing instruction handler not invoked\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_long_attribute_value() {
    _check_set_test_info(
        b"test_alloc_realloc_long_attribute_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1480,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [<!ENTITY foo 'This entity will be substituted as an attribute value, and is   calculated to be exactly long enough that the terminating NUL   that the library adds internally will trigger the string pool togrow. GHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP'>]>\n<doc a='&foo;'></doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10;
    i = 0;
    while i < max_realloc_count {
        g_reallocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1515i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1517i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_attribute_whitespace() {
    _check_set_test_info(
        b"test_alloc_attribute_whitespace\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1521,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc a=' '></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 15;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1536i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1538i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_attribute_predefined_entity() {
    _check_set_test_info(
        b"test_alloc_attribute_predefined_entity\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1542,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc a='&amp;'></doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 15;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1557i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1559i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_long_attr_default_with_char_ref() {
    _check_set_test_info(
        b"test_alloc_long_attr_default_with_char_ref\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1567,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [<!ATTLIST doc a CDATA 'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHI&#x31;'>]>\n<doc/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1602i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1604i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_long_attr_value() {
    _check_set_test_info(
        b"test_alloc_long_attr_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1611,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE test [<!ENTITY foo '\nABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP'>]>\n<test a='&foo;'/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 25;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1646i32,
            b"Parse succeeded despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1648i32,
            b"Parse failed at maximum allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_nested_entities() {
    _check_set_test_info(
        b"test_alloc_nested_entities\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1657,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc SYSTEM 'http://example.org/one.ent'>\n<doc />\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut test_data: ExtFaults = ext_faults {
    parse_text:   b"<!ENTITY % pe1 'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP'>\n<!ENTITY % pe2 '%pe1;'>\n<!ENTITY % pe3 '%pe2;'>\0"
            .as_ptr() as *const ::core::ffi::c_char,
    fail_text:   b"Memory Fail not faulted\0".as_ptr() as *const ::core::ffi::c_char,
    encoding:   ::core::ptr::null::<XML_Char>(),
    error:   XML_ERROR_NO_MEMORY,
};
    g_allocation_count = 12;
    XML_SetUserData(
        g_parser,
        &raw mut test_data as *mut ::core::ffi::c_void,
    );
    XML_SetParamEntityParsing(
        g_parser,
        XML_PARAM_ENTITY_PARSING_ALWAYS,
    );
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
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1690,
    );
}

unsafe extern "C" fn test_alloc_realloc_param_entity_newline() {
    _check_set_test_info(
        b"test_alloc_realloc_param_entity_newline\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1694,
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
    let max_realloc_count: ::core::ffi::c_int = 5;
    i = 0;
    while i < max_realloc_count {
        g_reallocation_count = i;
        XML_SetUserData(
            g_parser,
            
            &raw mut dtd_text as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1734i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1736i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_ce_extends_pe() {
    _check_set_test_info(
        b"test_alloc_realloc_ce_extends_pe\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1740,
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
    let max_realloc_count: ::core::ffi::c_int = 5;
    i = 0;
    while i < max_realloc_count {
        g_reallocation_count = i;
        XML_SetUserData(
            g_parser,
            
            &raw mut dtd_text as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1780i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1782i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_realloc_attributes() {
    _check_set_test_info(
        b"test_alloc_realloc_attributes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1786,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ATTLIST doc\n    a1  (a|b|c)   'a'\n    a2  (foo|bar) #IMPLIED\n    a3  NMTOKEN   #IMPLIED\n    a4  NMTOKENS  #IMPLIED\n    a5  ID        #IMPLIED\n    a6  IDREF     #IMPLIED\n    a7  IDREFS    #IMPLIED\n    a8  ENTITY    #IMPLIED\n    a9  ENTITIES  #IMPLIED\n    a10 CDATA     #IMPLIED\n  >]>\n<doc>wombat</doc>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 5;
    i = 0;
    while i < max_realloc_count {
        g_reallocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1815i32,
            b"Parse succeeded despite failing reallocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1817i32,
            b"Parse failed at maximum reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_long_doc_name() {
    _check_set_test_info(
        b"test_alloc_long_doc_name\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1821,
    );
    let mut text: *const ::core::ffi::c_char = b"<LongRootElementNameThatWillCauseTheNextAllocationToExpandTheStringPoolForTheDTDQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ a='1'/>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1854i32,
            b"Parsing worked despite failing reallocations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1856i32,
            b"Parsing failed even at max reallocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_long_base() {
    _check_set_test_info(
        b"test_alloc_long_base\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1860,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<!DOCTYPE doc [\n  <!ENTITY e SYSTEM 'foo'>\n]>\n<doc>&e;</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut entity_text: [::core::ffi::c_char; 12] =
        ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"Hello world\0");
    let mut base: *const XML_Char = b"LongBaseURI/that/will/overflow/an/internal/buffer/and/cause/it/to/have/to/grow/PQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/\0"
        .as_ptr() as *const XML_Char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 25;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetUserData(
            g_parser,
            
            &raw mut entity_text as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
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
        if  XML_SetBase(g_parser, base)
            ==  XML_STATUS_ERROR
        {
            XML_ParserReset(
                g_parser,
                ::core::ptr::null::<XML_Char>(),
            );
        } else {
            if  _XML_Parse_SINGLE_BYTES(
                g_parser,
                text,
                strlen(text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            )
                !=  XML_STATUS_ERROR
            {
                break;
            }
            alloc_teardown();
            alloc_setup();
        }
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1906i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1908i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_long_public_id() {
    _check_set_test_info(
        b"test_alloc_long_public_id\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1912,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY e PUBLIC 'LongPublicIDThatShouldResultInAnInternalStringPoolGrowingAtASpecificMomentKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB' 'bar'>\n]>\n<doc>&e;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut entity_text: [::core::ffi::c_char; 12] =
        ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"Hello world\0");
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetUserData(
            g_parser,
            
            &raw mut entity_text as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1953i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1955i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_long_entity_value() {
    _check_set_test_info(
        b"test_alloc_long_entity_value\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1959,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ENTITY e1 'Long entity value that should provoke a string pool to grow while setting up to parse the external entity below. xyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB'>\n  <!ENTITY e2 SYSTEM 'bar'>\n]>\n<doc>&e2;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut entity_text: [::core::ffi::c_char; 12] =
        ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"Hello world\0");
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetUserData(
            g_parser,
            
            &raw mut entity_text as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2001i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2003i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_long_notation() {
    _check_set_test_info(
        b"test_alloc_long_notation\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2007,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!NOTATION note SYSTEM 'ALongNotationNameThatShouldProvokeStringPoolGrowthWhileCallingAnExternalEntityParserUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB'>\n  <!ENTITY e1 SYSTEM 'foo' NDATA ALongNotationNameThatShouldProvokeStringPoolGrowthWhileCallingAnExternalEntityParserUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789ABABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AB>\n  <!ENTITY e2 SYSTEM 'bar'>\n]>\n<doc>&e2;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut options: [ExtOption; 3] = [
        ExtOption {
    system_id:  b"foo\0".as_ptr() as *const XML_Char,
    parse_text:  b"Entity Foo\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  b"bar\0".as_ptr() as *const XML_Char,
    parse_text:  b"Entity Bar\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  ::core::ptr::null::<XML_Char>(),
    parse_text:  ::core::ptr::null::<::core::ffi::c_char>(),
},
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40;
    i = 0;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetUserData(
            g_parser,
            
            &raw mut options
                as *mut ::core::ffi::c_void,
        );
        XML_SetParamEntityParsing(
            g_parser,
            XML_PARAM_ENTITY_PARSING_ALWAYS,
        );
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
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        alloc_teardown();
        alloc_setup();
        i += 1;
    }
    if i == 0 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2070i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2072i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_alloc_reset_after_external_entity_parser_create_fail() {
    _check_set_test_info(
        b"test_alloc_reset_after_external_entity_parser_create_fail\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2076,
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
    XML_SetParamEntityParsing(
        g_parser,
        XML_PARAM_ENTITY_PARSING_ALWAYS,
    );
    if  _XML_Parse_SINGLE_BYTES(
        g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        !=  XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2085i32,
            b"Call to parse was expected to fail\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if  XML_GetErrorCode(g_parser)
        !=  XML_ERROR_EXTERNAL_ENTITY_HANDLING
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2088i32,
            b"Call to parse was expected to fail from the external entity handler\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(
        g_parser,
        ::core::ptr::null::<XML_Char>(),
    );
}

unsafe extern "C" fn sizeRecordedFor(
    mut ptr: *mut ::core::ffi::c_void,
) -> size_t {
    return *((ptr as *mut ::core::ffi::c_char)
        .offset(-(EXPAT_MALLOC_PADDING as isize))
        .offset(-(::core::mem::size_of::<size_t>() as isize))
        as *mut size_t);
}

unsafe extern "C" fn test_alloc_tracker_size_recorded() {
    _check_set_test_info(
        b"test_alloc_tracker_size_recorded\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2101,
    );
    let mut memsuite: XML_Memory_Handling_Suite =
        XML_Memory_Handling_Suite {
    malloc_fcn:  Some(
                malloc
                    as unsafe extern "C" fn(
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
    realloc_fcn:  Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        size_t,
                    ) -> *mut ::core::ffi::c_void,
            ),
    free_fcn:  Some(
                free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
            ),
};
    let mut values: [bool; 2] = [
        true_0 != 0,
        false_0 != 0,
    ];
    let mut i: size_t = 0;
    while i
        < (::core::mem::size_of::<[bool; 2]>())
            .wrapping_div(::core::mem::size_of::<bool>())
    {
        let useMemSuite: bool = values[i];
        set_subtest(
            b"useMemSuite=%d\0".as_ptr() as *const ::core::ffi::c_char,
            useMemSuite as ::core::ffi::c_int,
        );
        let mut parser: XML_Parser = if useMemSuite as ::core::ffi::c_int != 0 {
            XML_ParserCreate_MM(
                ::core::ptr::null::<XML_Char>(),
                
                &raw mut memsuite as *const XML_Memory_Handling_Suite,
                b"|\0".as_ptr() as *const XML_Char,
            )
        } else {
            XML_ParserCreate(::core::ptr::null::<
                XML_Char,
            >())
        };
        let mut ptr: *mut ::core::ffi::c_void = expat_malloc(
            parser,
            10,
            -1,
        );
        if ptr.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2115i32,
                b"check failed: ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(sizeRecordedFor(ptr) == 10) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2116i32,
                b"check failed: sizeRecordedFor(ptr) == 10\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !expat_realloc(
            parser,
            ptr,
            (18446744073709551615 as size_t)
                .wrapping_div(2usize),
            -1,
        )
        .is_null()
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2118i32,
                b"check failed: expat_realloc(parser, ptr, SIZE_MAX / 2, -1) == NULL\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !(sizeRecordedFor(ptr) == 10) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2120i32,
                b"check failed: sizeRecordedFor(ptr) == 10\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        ptr = expat_realloc(
            parser,
            ptr,
            20,
            -1,
        );
        if ptr.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2124i32,
                b"check failed: ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(sizeRecordedFor(ptr) == 20) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                2125i32,
                b"check failed: sizeRecordedFor(ptr) == 20\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        expat_free(parser, ptr, -1);
        XML_ParserFree(parser);
        i = i.wrapping_add(1);
    }
}

unsafe extern "C" fn test_alloc_tracker_pointer_alignment() {
    _check_set_test_info(
        b"test_alloc_tracker_pointer_alignment\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2135,
    );
    let mut parser: XML_Parser =
        XML_ParserCreate(::core::ptr::null::<
            XML_Char,
        >());
    if !(::core::mem::size_of::<::core::ffi::c_longlong>()
        >=  ::core::mem::size_of::<size_t>())
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2138i32,
            b"check failed: sizeof(long long) >= sizeof(size_t)\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    let ptr: *mut ::core::ffi::c_longlong = expat_malloc(
        parser,
        (4usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_longlong>()),
        -1,
    ) as *mut ::core::ffi::c_longlong;
    *ptr.offset(0) = 0i64;
    *ptr.offset(1) = 1i64;
    *ptr.offset(2) = 2i64;
    *ptr.offset(3) = 3i64;
    expat_free(
        parser,
        ptr as *mut ::core::ffi::c_void,
        -1,
    );
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_alloc_tracker_maximum_amplification() {
    _check_set_test_info(
        b"test_alloc_tracker_maximum_amplification\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2151,
    );
    if g_reparseDeferralEnabledDefault as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        return;
    }
    let mut parser: XML_Parser =
        XML_ParserCreate(::core::ptr::null::<
            XML_Char,
        >());
    let chunk: *const ::core::ffi::c_char = b"<e>\0".as_ptr() as *const ::core::ffi::c_char;
    if !(_XML_Parse_SINGLE_BYTES(
        parser,
        chunk,
        strlen(chunk) as ::core::ffi::c_int,
        0,
    )
        ==  XML_STATUS_OK)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            2162i32,
            b"check failed: _XML_Parse_SINGLE_BYTES(parser, chunk, (int)strlen(chunk), XML_FALSE) == XML_STATUS_OK\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(XML_SetAllocTrackerActivationThreshold(
        parser,
        0,
    ) as ::core::ffi::c_int
        == 1)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2166i32,
            b"check failed: XML_SetAllocTrackerActivationThreshold(parser, 0) == XML_TRUE\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !expat_malloc(
        parser,
        1000,
        -1,
    )
    .is_null()
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2169i32,
            b"check failed: expat_malloc(parser, 1000, -1) == NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !(XML_SetAllocTrackerMaximumAmplification(parser, 3000.0)
        as ::core::ffi::c_int
        == 1)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2174i32,
            b"check failed: XML_SetAllocTrackerMaximumAmplification(parser, 3000.0f) == XML_TRUE\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let ptr: *mut ::core::ffi::c_void =  expat_malloc(
        parser,
        1000,
        -1,
    );
    if ptr.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2177i32,
            b"check failed: ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    expat_free(parser, ptr, -1);
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_alloc_tracker_threshold() {
    _check_set_test_info(
        b"test_alloc_tracker_threshold\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2185,
    );
    let mut parser: XML_Parser =
        XML_ParserCreate(::core::ptr::null::<
            XML_Char,
        >());
    let ptr: *mut ::core::ffi::c_void =  expat_malloc(
        parser,
        1000,
        -1,
    );
    if ptr.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2191i32,
            b"check failed: ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    expat_free(parser, ptr, -1);
    if !(XML_SetAllocTrackerActivationThreshold(
        parser,
        999,
    ) as ::core::ffi::c_int
        == 1)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2195i32,
            b"check failed: XML_SetAllocTrackerActivationThreshold(parser, 999) == XML_TRUE\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !expat_malloc(
        parser,
        1000,
        -1,
    )
    .is_null()
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2196i32,
            b"check failed: expat_malloc(parser, 1000, -1) == NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_alloc_tracker_getbuffer_unlimited() {
    _check_set_test_info(
        b"test_alloc_tracker_getbuffer_unlimited\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2203,
    );
    let mut parser: XML_Parser =
        XML_ParserCreate(::core::ptr::null::<
            XML_Char,
        >());
    if !(XML_SetAllocTrackerActivationThreshold(
        parser,
        0,
    ) as ::core::ffi::c_int
        == 1)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2208i32,
            b"check failed: XML_SetAllocTrackerActivationThreshold(parser, 0) == XML_TRUE\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !expat_malloc(
        parser,
        1000,
        -1,
    )
    .is_null()
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2211i32,
            b"check failed: expat_malloc(parser, 1000, -1) == NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_GetBuffer(parser, 1000).is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2214i32,
            b"check failed: XML_GetBuffer(parser, 1000) != NULL\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_alloc_tracker_api() {
    _check_set_test_info(
        b"test_alloc_tracker_api\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2220,
    );
    let mut parserWithoutParent: XML_Parser =
        XML_ParserCreate(::core::ptr::null::<
            XML_Char,
        >());
    let mut parserWithParent: XML_Parser =
        XML_ExternalEntityParserCreate(
            parserWithoutParent,
            b"entity123\0".as_ptr() as *const XML_Char,
            ::core::ptr::null::<XML_Char>(),
        );
    if parserWithoutParent.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2225i32,
            b"parserWithoutParent is NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if parserWithParent.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2227i32,
            b"parserWithParent is NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerMaximumAmplification(
        ::core::ptr::null_mut::<XML_ParserStruct>(),
        123.0,
    ) as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2232i32,
            b"Call with NULL parser is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerMaximumAmplification(
        parserWithParent,
        123.0,
    ) as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2235i32,
            b"Call with non-root parser is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerMaximumAmplification(
        parserWithoutParent,
        ::core::f32::NAN,
    ) as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2238i32,
            b"Call with NaN limit is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerMaximumAmplification(
        parserWithoutParent,
        -1.0,
    ) as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2241i32,
            b"Call with negative limit is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerMaximumAmplification(
        parserWithoutParent,
        0.9,
    ) as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2244i32,
            b"Call with positive limit <1.0 is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerMaximumAmplification(
        parserWithoutParent,
        1.0,
    ) as ::core::ffi::c_int
        == XML_FALSE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2249i32,
            b"Call with positive limit >=1.0 is supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerMaximumAmplification(
        parserWithoutParent,
        123456.789,
    ) as ::core::ffi::c_int
        == XML_FALSE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2252i32,
            b"Call with positive limit >=1.0 is supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerMaximumAmplification(
        parserWithoutParent,
        ::core::f32::INFINITY,
    ) as ::core::ffi::c_int
        == XML_FALSE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2255i32,
            b"Call with positive limit >=1.0 is supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerActivationThreshold(
        ::core::ptr::null_mut::<XML_ParserStruct>(),
        123,
    ) as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2259i32,
            b"Call with NULL parser is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerActivationThreshold(
        parserWithParent,
        123,
    ) as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2261i32,
            b"Call with non-root parser is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetAllocTrackerActivationThreshold(
        parserWithoutParent,
        123,
    ) as ::core::ffi::c_int
        == XML_FALSE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2266i32,
            b"Call with non-NULL parentless parser is supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parserWithParent);
    XML_ParserFree(parserWithoutParent);
}

unsafe extern "C" fn test_mem_api_cycle() {
    _check_set_test_info(
        b"test_mem_api_cycle\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2274,
    );
    let mut parser: XML_Parser =
        XML_ParserCreate(::core::ptr::null::<
            XML_Char,
        >());
    let mut ptr: *mut ::core::ffi::c_void =
        XML_MemMalloc(parser, 10);
    if ptr.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2279i32,
            b"check failed: ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    memset(ptr, 'x' as i32, 10);
    ptr = XML_MemRealloc(
        parser,
        ptr,
        20,
    );
    if ptr.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2284i32,
            b"check failed: ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    memset(ptr, 'y' as i32, 20);
    XML_MemFree(parser, ptr);
    XML_ParserFree(parser);
}

unsafe extern "C" fn test_mem_api_unlimited() {
    _check_set_test_info(
        b"test_mem_api_unlimited\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        2293,
    );
    let mut parser: XML_Parser =
        XML_ParserCreate(::core::ptr::null::<
            XML_Char,
        >());
    if !(XML_SetAllocTrackerActivationThreshold(
        parser,
        0,
    ) as ::core::ffi::c_int
        == 1)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2297i32,
            b"check failed: XML_SetAllocTrackerActivationThreshold(parser, 0) == XML_TRUE\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut ptr: *mut ::core::ffi::c_void =
        XML_MemMalloc(parser, 1000);
    if ptr.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2302i32,
            b"check failed: ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    ptr = XML_MemRealloc(
        parser,
        ptr,
        2000,
    );
    if ptr.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/alloc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            2306i32,
            b"check failed: ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_MemFree(parser, ptr);
    XML_ParserFree(parser);
}
#[no_mangle]

pub unsafe extern "C" fn make_alloc_test_case(mut s: *mut Suite) {
    let mut tc_alloc: *mut TCase =
        
        tcase_create(
            b"allocation tests\0".as_ptr() as *const ::core::ffi::c_char
        );
    suite_add_tcase(
        
        s,
        
        tc_alloc,
    );
    tcase_add_checked_fixture(
        
        tc_alloc,
        Some(alloc_setup as unsafe extern "C" fn() -> ()),
        Some(alloc_teardown as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_parse_xdecl as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_parse_xdecl_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_parse_pi as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_parse_pi_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_parse_pi_3 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_parse_comment as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_parse_comment_2 as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_create_external_parser as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_run_external_parser as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_dtd_copy_default_atts as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_external_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_ext_entity_set_encoding as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_internal_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_parameter_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_dtd_default_handling as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_explicit_encoding as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_set_base as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_realloc_buffer as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_ext_entity_realloc_buffer as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_realloc_many_attributes as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_public_entity_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_subst_public_entity_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_parse_public_doctype as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_parse_public_doctype_long_name as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_set_foreign_dtd as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_attribute_enum_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_attribute_enum_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_implied_attribute as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_default_attribute as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_notation as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_public_notation as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_system_notation as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_nested_groups as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_nested_groups as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_large_group as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_group_choice as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_pi_in_epilog as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_comment_in_epilog as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_long_attribute_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_attribute_whitespace as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_attribute_predefined_entity as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_long_attr_default_with_char_ref as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_long_attr_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_nested_entities as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_param_entity_newline as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_ce_extends_pe as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(test_alloc_realloc_attributes as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_alloc_long_doc_name as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_long_base as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_long_public_id as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_long_entity_value as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_long_notation as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__ifdef_xml_dtd(
        
        tc_alloc,
        Some(
            test_alloc_reset_after_external_entity_parser_create_fail
                as unsafe extern "C" fn() -> (),
        ),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_tracker_size_recorded as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_tracker_pointer_alignment as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_tracker_maximum_amplification as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_tracker_threshold as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_tracker_getbuffer_unlimited as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_alloc_tracker_api as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test(
        
        tc_alloc,
        Some(test_mem_api_cycle as unsafe extern "C" fn() -> ()),
    );
    tcase_add_test__if_xml_ge(
        
        tc_alloc,
        Some(test_mem_api_unlimited as unsafe extern "C" fn() -> ()),
    );
}
