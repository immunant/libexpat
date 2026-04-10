pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::common_h::ALLOC_ALWAYS_SUCCEED;
pub use crate::common_h::REALLOC_ALWAYS_SUCCEED;
pub use crate::expat_external_h::XML_Char;
pub use crate::src::tests::common::_XML_Parse_SINGLE_BYTES;
pub use crate::src::tests::common::_xml_failure;
pub use crate::src::tests::common::basic_teardown;
pub use crate::src::tests::common::duff_allocator;
pub use crate::src::tests::common::duff_reallocator;
pub use crate::src::tests::common::g_allocation_count;
pub use crate::src::tests::common::g_parser;
pub use crate::src::tests::common::g_reallocation_count;
pub use crate::src::tests::common::g_resumable;
pub use crate::src::tests::common::tcase_add_test__if_xml_ge;
use crate::src::tests::dummy::dummy_default_handler;
pub use crate::stdlib::__assert_fail;
pub use crate::stdlib::__ASSERT_FUNCTION_3;

pub use crate::expat_h::XML_Bool;
pub use crate::expat_h::XML_CharacterDataHandler;
pub use crate::expat_h::XML_DefaultHandler;
pub use crate::expat_h::XML_EndElementHandler;
pub use crate::expat_h::XML_Error;
pub use crate::expat_h::XML_ExternalEntityRefHandler;
pub use crate::expat_h::XML_Memory_Handling_Suite;
pub use crate::expat_h::XML_ParamEntityParsing;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ParserStruct;
pub use crate::expat_h::XML_StartElementHandler;
pub use crate::expat_h::XML_Status;
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
pub use crate::src::lib::xmlparse::XML_GetBuffer;
pub use crate::src::lib::xmlparse::XML_GetErrorCode;
pub use crate::src::lib::xmlparse::XML_ParseBuffer;
pub use crate::src::lib::xmlparse::XML_ParserCreate_MM;
pub use crate::src::lib::xmlparse::XML_ParserReset;
pub use crate::src::lib::xmlparse::XML_ResumeParser;
pub use crate::src::lib::xmlparse::XML_SetCharacterDataHandler;
pub use crate::src::lib::xmlparse::XML_SetDefaultHandler;
pub use crate::src::lib::xmlparse::XML_SetElementHandler;
pub use crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler;
pub use crate::src::lib::xmlparse::XML_SetParamEntityParsing;
pub use crate::src::lib::xmlparse::XML_SetReturnNSTriplet;
pub use crate::src::lib::xmlparse::XML_SetUserData;
pub use crate::src::tests::handlers::clearing_aborting_character_handler;
pub use crate::src::tests::handlers::external_entity_optioner;
pub use crate::src::tests::handlers::triplet_end_checker;
pub use crate::src::tests::handlers::triplet_start_checker;
pub use crate::src::tests::handlers::ExtOption;
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
use crate::stdlib::free;
use crate::stdlib::memcpy;
use crate::stdlib::strlen;

unsafe extern "C" fn nsalloc_setup() {
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
    let mut ns_sep: [XML_Char; 2] = [
        
        ' ' as XML_Char,
        
        '\0' as XML_Char,
    ];
    g_allocation_count = ALLOC_ALWAYS_SUCCEED;
    g_reallocation_count = REALLOC_ALWAYS_SUCCEED;
    g_parser = XML_ParserCreate_MM(
        ::core::ptr::null::<XML_Char>(),
        
        &raw mut memsuite as *const XML_Memory_Handling_Suite,
        &raw mut ns_sep as *mut XML_Char,
    );
    if g_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            67i32,
            b"Parser not created\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn nsalloc_teardown() {
    basic_teardown();
}

unsafe extern "C" fn test_nsalloc_xmlns() {
    _check_set_test_info(
        b"test_nsalloc_xmlns\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        78i32,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc xmlns='http://example.org/'>\n  <e xmlns=''/>\n</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_uint = 0;
    let max_alloc_count: ::core::ffi::c_uint = 30u32;
    i = 0u32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i = i.wrapping_add(1);
    }
    if i == 0u32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            101i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            103i32,
            b"Parsing failed even at maximum allocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_parse_buffer() {
    _check_set_test_info(
        b"test_nsalloc_parse_buffer\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        108i32,
    );
    let mut text: *const ::core::ffi::c_char =
        b"<doc>Hello</doc>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if  XML_ParseBuffer(
        g_parser,
        0i32,
        XML_FALSE as ::core::ffi::c_int,
    )
        !=  XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            115i32,
            b"Pre-init XML_ParseBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if  XML_GetErrorCode(g_parser)
        !=  XML_ERROR_NO_BUFFER
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            117i32,
            b"Pre-init XML_ParseBuffer faulted for wrong reason\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    buffer = XML_GetBuffer(
        g_parser,
        1i32,
    );
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            121i32,
            b"Could not acquire parse buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    g_allocation_count = 0i32;
    if  XML_ParseBuffer(
        g_parser,
        0i32,
        XML_FALSE as ::core::ffi::c_int,
    )
        !=  XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            125i32,
            b"Pre-init XML_ParseBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if  XML_GetErrorCode(g_parser)
        !=  XML_ERROR_NO_MEMORY
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            127i32,
            b"Pre-init XML_ParseBuffer faulted for wrong reason\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    g_allocation_count = ALLOC_ALWAYS_SUCCEED;
    if  XML_ParseBuffer(
        g_parser,
        0i32,
        XML_FALSE as ::core::ffi::c_int,
    )
        !=  XML_STATUS_OK
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            132i32,
        );
    }
    if  XML_ResumeParser(g_parser)
        !=  XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            136i32,
            b"Resuming unsuspended parser not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if  XML_GetErrorCode(g_parser)
        !=  XML_ERROR_NOT_SUSPENDED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            138i32,
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
    buffer = XML_GetBuffer(
        g_parser,
        strlen(text) as ::core::ffi::c_int,
    );
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            145i32,
            b"Could not acquire parse buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            146u32,
            __ASSERT_FUNCTION_3.as_ptr(),
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
        XML_TRUE as ::core::ffi::c_int,
    )
        !=  XML_STATUS_SUSPENDED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            150i32,
        );
    }
    if  XML_GetErrorCode(g_parser)
        !=  XML_ERROR_NONE
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            152i32,
        );
    }
    if  XML_ParseBuffer(
        g_parser,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        !=  XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            155i32,
            b"Suspended XML_ParseBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if  XML_GetErrorCode(g_parser)
        !=  XML_ERROR_SUSPENDED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            157i32,
        );
    }
    if !XML_GetBuffer(
        g_parser,
        strlen(text) as ::core::ffi::c_int,
    )
    .is_null()
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            159i32,
            b"Suspended XML_GetBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetCharacterDataHandler(
        g_parser,
        None,
    );
    if  XML_ResumeParser(g_parser)
        !=  XML_STATUS_OK
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            164i32,
        );
    }
    if  XML_ParseBuffer(
        g_parser,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        !=  XML_STATUS_ERROR
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            167i32,
            b"Post-finishing XML_ParseBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if  XML_GetErrorCode(g_parser)
        !=  XML_ERROR_FINISHED
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            169i32,
        );
    }
    if !XML_GetBuffer(
        g_parser,
        strlen(text) as ::core::ffi::c_int,
    )
    .is_null()
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            171i32,
            b"Post-finishing XML_GetBuffer not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_long_prefix() {
    _check_set_test_info(
        b"test_nsalloc_long_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        176i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            245i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            247i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_long_uri() {
    _check_set_test_info(
        b"test_nsalloc_long_uri\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        252i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/' bar:a='12'\nxmlns:bar='http://example.org/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789A/'></foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            305i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            307i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_long_attr() {
    _check_set_test_info(
        b"test_nsalloc_long_attr\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        312i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/' bar:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='12'\nxmlns:bar='http://example.org/'></foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            348i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            350i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_long_attr_prefix() {
    _check_set_test_info(
        b"test_nsalloc_long_attr_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        355i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/' ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:a='12'\nxmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'></foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ e foo\0".as_ptr() as *const ::core::ffi::c_char,
        b"http://example.org/ a ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40i32;
    i = 0i32;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetReturnNSTriplet(
            g_parser,
            XML_TRUE as ::core::ffi::c_int,
        );
        XML_SetUserData(
            g_parser,
            
            &raw mut elemstr
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            434i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            436i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_realloc_attributes() {
    _check_set_test_info(
        b"test_nsalloc_realloc_attributes\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        441i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:e xmlns:foo='http://example.org/' bar:a='12'\n       xmlns:bar='http://example.org/'></foo:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if !(i == 0i32) {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            459i32,
            b"check failed: i == 0\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_long_element() {
    _check_set_test_info(
        b"test_nsalloc_long_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        470i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<foo:thisisalongenoughelementnametotriggerareallocation\n xmlns:foo='http://example.org/' bar:a='12'\n xmlns:bar='http://example.org/'></foo:thisisalongenoughelementnametotriggerareallocation>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut elemstr: [*const XML_Char; 2] = [
        b"http://example.org/ thisisalongenoughelementnametotriggerareallocation foo\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"http://example.org/ a bar\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 30i32;
    i = 0i32;
    while i < max_alloc_count {
        g_allocation_count = i;
        XML_SetReturnNSTriplet(
            g_parser,
            XML_TRUE as ::core::ffi::c_int,
        );
        XML_SetUserData(
            g_parser,
            
            &raw mut elemstr
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            496i32,
            b"Parsing worked despite failing reallocations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            498i32,
            b"Parsing failed at max reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_realloc_binding_uri() {
    _check_set_test_info(
        b"test_nsalloc_realloc_binding_uri\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        512i32,
    );
    let mut first: *const ::core::ffi::c_char =
        b"<doc xmlns='http://example.org/'>\n  <e xmlns='' />\n</doc>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut second: *const ::core::ffi::c_char = b"<doc xmlns='http://example.org/long/enough/URI/to/reallocate/'>\n  <e xmlns='' />\n</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_uint = 0;
    let max_realloc_count: ::core::ffi::c_uint = 10u32;
    if  _XML_Parse_SINGLE_BYTES(
        g_parser,
        first,
        strlen(first) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        ==  XML_STATUS_ERROR
    {
        _xml_failure(
            g_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            526i32,
        );
    }
    i = 0u32;
    while i < max_realloc_count {
        XML_ParserReset(
            g_parser,
            ::core::ptr::null::<XML_Char>(),
        );
        g_reallocation_count = i as ::core::ffi::c_int;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            second,
            strlen(second) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
        {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i == 0u32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            537i32,
            b"Parsing worked despite failing reallocation\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            539i32,
            b"Parsing failed at max reallocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_realloc_long_prefix() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        544i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:foo>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 12i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            613i32,
            b"Parsing worked despite failing reallocations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            615i32,
            b"Parsing failed even at max reallocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_realloc_longer_prefix() {
    _check_set_test_info(
        b"test_nsalloc_realloc_longer_prefix\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        620i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ:foo xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ='http://example.org/'></ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZQ:foo>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 12i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            689i32,
            b"Parsing worked despite failing reallocations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            691i32,
            b"Parsing failed even at max reallocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_long_namespace() {
    _check_set_test_info(
        b"test_nsalloc_long_namespace\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        695i32,
    );
    let mut text1: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:e xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ='http://example.org/'>\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:f ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:attr='foo'/>\n</ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZ:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40i32;
    i = 0i32;
    while i < max_alloc_count {
        g_allocation_count = i;
        if  _XML_Parse_SINGLE_BYTES(
            g_parser,
            text1,
            strlen(text1) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
            &&  _XML_Parse_SINGLE_BYTES(
                g_parser,
                text2,
                strlen(text2) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            )
                !=  XML_STATUS_ERROR
        {
            break;
        }
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            803i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            805i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_less_long_namespace() {
    _check_set_test_info(
        b"test_nsalloc_less_long_namespace\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        812i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:e xmlns:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678='http://example.org/'>\n<ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:f ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:att='foo'/>\n</ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789AZABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz012345678:e>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 40i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            876i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            878i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_long_context() {
    _check_set_test_info(
        b"test_nsalloc_long_context\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        882i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ATTLIST doc baz ID #REQUIRED>\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKL' baz='2'>\n&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut options: [ExtOption; 3] = [
        ExtOption {
    system_id:  b"foo\0".as_ptr() as *const XML_Char,
    parse_text:  b"<!ELEMENT e EMPTY>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  b"bar\0".as_ptr() as *const XML_Char,
    parse_text:  b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  ::core::ptr::null::<XML_Char>(),
    parse_text:  ::core::ptr::null::<::core::ffi::c_char>(),
},
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 70i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            928i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            930i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn context_realloc_test(mut text: *const ::core::ffi::c_char) {
    let mut options: [ExtOption; 3] = [
        ExtOption {
    system_id:  b"foo\0".as_ptr() as *const XML_Char,
    parse_text:  b"<!ELEMENT e EMPTY>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  b"bar\0".as_ptr() as *const XML_Char,
    parse_text:  b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  ::core::ptr::null::<XML_Char>(),
    parse_text:  ::core::ptr::null::<::core::ffi::c_char>(),
},
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 6i32;
    i = 0i32;
    while i < max_realloc_count {
        g_reallocation_count = i;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            957i32,
            b"Parsing worked despite failing reallocations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            959i32,
            b"Parsing failed even at max reallocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_realloc_long_context() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_context\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        962i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKL'>\n&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_2() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_context_2\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        993i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJK'>\n&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_3() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_context_3\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1024i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGH'>\n&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_4() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_context_4\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1055i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO'>\n&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_5() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_context_5\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1086i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABC'>\n&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_6() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_context_6\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1117i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNOP'>\n&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_7() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_context_7\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1147i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLM'>\n&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    context_realloc_test(text);
}

unsafe extern "C" fn test_nsalloc_realloc_long_ge_name() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_ge_name\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1178i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP SYSTEM 'bar'>\n]>\n<doc xmlns='http://example.org/baz'>\n&ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut options: [ExtOption; 3] = [
        ExtOption {
    system_id:  b"foo\0".as_ptr() as *const XML_Char,
    parse_text:  b"<!ELEMENT el EMPTY>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  b"bar\0".as_ptr() as *const XML_Char,
    parse_text:  b"<el/>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  ::core::ptr::null::<XML_Char>(),
    parse_text:  ::core::ptr::null::<::core::ffi::c_char>(),
},
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 10i32;
    i = 0i32;
    while i < max_realloc_count {
        g_reallocation_count = i;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1240i32,
            b"Parsing worked despite failing reallocations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1242i32,
            b"Parsing failed even at max reallocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_realloc_long_context_in_dtd() {
    _check_set_test_info(
        b"test_nsalloc_realloc_long_context_in_dtd\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1251i32,
    );
    let mut text1: *const ::core::ffi::c_char = b"<!DOCTYPE ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:doc [\n  <!ENTITY First SYSTEM 'foo/First'>\n]>\n<ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:doc xmlns:ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP='foo/Second'>&First;\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char = b"</ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut options: [ExtOption; 2] = [
        ExtOption {
    system_id:  b"foo/First\0".as_ptr() as *const XML_Char,
    parse_text:  b"Hello world\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  ::core::ptr::null::<XML_Char>(),
    parse_text:  ::core::ptr::null::<::core::ffi::c_char>(),
},
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_realloc_count: ::core::ffi::c_int = 20i32;
    i = 0i32;
    while i < max_realloc_count {
        g_reallocation_count = i;
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
            text1,
            strlen(text1) as ::core::ffi::c_int,
            XML_FALSE as ::core::ffi::c_int,
        )
            !=  XML_STATUS_ERROR
            &&  _XML_Parse_SINGLE_BYTES(
                g_parser,
                text2,
                strlen(text2) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            )
                !=  XML_STATUS_ERROR
        {
            break;
        }
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1348i32,
            b"Parsing worked despite failing reallocations\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_realloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1350i32,
            b"Parsing failed even at max reallocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_long_default_in_ext() {
    _check_set_test_info(
        b"test_nsalloc_long_default_in_ext\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1354i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc [\n  <!ATTLIST e a1 CDATA 'ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP'>\n  <!ENTITY x SYSTEM 'foo'>\n]>\n<doc>&x;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut options: [ExtOption; 2] = [
        ExtOption {
    system_id:  b"foo\0".as_ptr() as *const XML_Char,
    parse_text:  b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  ::core::ptr::null::<XML_Char>(),
    parse_text:  ::core::ptr::null::<::core::ffi::c_char>(),
},
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 50i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1397i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1399i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_long_systemid_in_ext() {
    _check_set_test_info(
        b"test_nsalloc_long_systemid_in_ext\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1403i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE doc SYSTEM 'foo' [\n  <!ENTITY en SYSTEM 'ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/'>\n]>\n<doc>&en;</doc>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut options: [ExtOption; 3] = [
        ExtOption {
    system_id:   b"foo\0".as_ptr() as *const XML_Char,
    parse_text:   b"<!ELEMENT e EMPTY>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:   b"ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/ABCDEFGHIJKLMNO/\0"
                .as_ptr() as *const XML_Char,
    parse_text:   b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:   ::core::ptr::null::<XML_Char>(),
    parse_text:   ::core::ptr::null::<::core::ffi::c_char>(),
},
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 55i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1466i32,
            b"Parsing worked despite failing allocations\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1468i32,
            b"Parsing failed even at max allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_nsalloc_prefixed_element() {
    _check_set_test_info(
        b"test_nsalloc_prefixed_element\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        1475i32,
    );
    let mut text: *const ::core::ffi::c_char = b"<!DOCTYPE pfx:element SYSTEM 'foo' [\n  <!ATTLIST pfx:element baz ID #REQUIRED>\n  <!ENTITY en SYSTEM 'bar'>\n]>\n<pfx:element xmlns:pfx='http://example.org/' baz='2'>\n&en;</pfx:element>\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut options: [ExtOption; 3] = [
        ExtOption {
    system_id:  b"foo\0".as_ptr() as *const XML_Char,
    parse_text:  b"<!ELEMENT e EMPTY>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  b"bar\0".as_ptr() as *const XML_Char,
    parse_text:  b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
},
        ExtOption {
    system_id:  ::core::ptr::null::<XML_Char>(),
    parse_text:  ::core::ptr::null::<::core::ffi::c_char>(),
},
    ];
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 70i32;
    i = 0i32;
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
        nsalloc_teardown();
        nsalloc_setup();
        i += 1;
    }
    if i == 0i32 {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1502i32,
            b"Success despite failing allocator\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/nsalloc_tests.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1504i32,
            b"Failed even at full allocation count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn make_nsalloc_test_case(mut s: *mut Suite) {
    let mut tc_nsalloc: *mut TCase =
        
        tcase_create(
            b"namespace allocation tests\0".as_ptr() as *const ::core::ffi::c_char
        );
    suite_add_tcase(
        
        s,
        
        tc_nsalloc,
    );
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
