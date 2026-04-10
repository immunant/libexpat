extern "C" {
    pub static mut g_parser: XML_Parser;
}
// =============== BEGIN common_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtTest {
    pub parse_text: *const ::core::ffi::c_char,
    pub encoding: *const XML_Char,
    pub storage: *mut CharData,
}
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::tests::chardata::CharData;
pub use crate::src::tests::chardata::CharData_CheckXMLChars;
pub use crate::src::tests::chardata::CharData_Init;
pub use crate::stdlib::__assert_fail;
use crate::stdlib::__errno_location;
pub use crate::stdlib::EINVAL;
pub use crate::stdlib::ENOMEM;
pub use crate::stdlib::__ASSERT_FUNCTION_1;

pub use crate::expat_external_h::XML_Char;
pub use crate::expat_external_h::XML_LChar;
pub use crate::expat_external_h::XML_Size;
pub use crate::expat_h::XML_Bool;
pub use crate::expat_h::XML_CharacterDataHandler;
pub use crate::expat_h::XML_Error;
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
pub use crate::expat_h::XML_STATUS_ERROR;
pub use crate::expat_h::XML_STATUS_OK;
pub use crate::expat_h::XML_STATUS_SUSPENDED;
pub use crate::expat_h::XML_TRUE;
pub use crate::src::lib::xmlparse::XML_ErrorString;
pub use crate::src::lib::xmlparse::XML_GetCurrentColumnNumber;
pub use crate::src::lib::xmlparse::XML_GetCurrentLineNumber;
pub use crate::src::lib::xmlparse::XML_GetErrorCode;
pub use crate::src::lib::xmlparse::XML_Parse;
pub use crate::src::lib::xmlparse::XML_ParserFree;
pub use crate::src::lib::xmlparse::XML_SetCharacterDataHandler;
pub use crate::src::lib::xmlparse::XML_SetStartElementHandler;
pub use crate::src::lib::xmlparse::XML_SetUserData;
use crate::src::tests::handlers::accumulate_attribute;
use crate::src::tests::handlers::accumulate_characters;
use crate::src::tests::handlers::ext_accumulate_characters;
pub use crate::src::tests::minicheck::tcase_setup_function;
pub use crate::src::tests::minicheck::tcase_teardown_function;
pub use crate::src::tests::minicheck::tcase_test_function;
pub use crate::src::tests::minicheck::TCase;
pub use crate::src::tests::minicheck::_fail;
pub use crate::src::tests::minicheck::tcase_add_test;
use crate::stdlib::snprintf;
pub use crate::stdlib::SIZE_MAX;

use crate::stdlib::free;
use crate::stdlib::malloc;
use crate::stdlib::memchr;
use crate::stdlib::memcpy;
use crate::stdlib::realloc;
use crate::stdlib::strlen;
#[no_mangle]

pub static mut long_character_data_text: *const ::core::ffi::c_char = b"<?xml version='1.0' encoding='iso-8859-1'?><s>012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789</s>\0"
    .as_ptr() as *const ::core::ffi::c_char;
#[no_mangle]

pub static mut long_cdata_text: *const ::core::ffi::c_char = b"<s><![CDATA[012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789]]></s>\0"
    .as_ptr() as *const ::core::ffi::c_char;
#[no_mangle]

pub static mut get_buffer_test_text: *const ::core::ffi::c_char = b"<documentwitharidiculouslylongelementnametoteaseaparticularcorneroftheallocationinXML_GetBuffersothatwecanimprovethecoverageyetagain012345678901123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789>\n<ef0\0"
    .as_ptr() as *const ::core::ffi::c_char;
#[no_mangle]

pub static mut g_resumable: XML_Bool = XML_FALSE;
#[no_mangle]

pub static mut g_abortable: XML_Bool = XML_FALSE;
#[no_mangle]

pub static mut g_chunkSize: ::core::ffi::c_int = 1;
#[no_mangle]

pub unsafe extern "C" fn tcase_add_test__ifdef_xml_dtd(
    mut tc: *mut TCase,
    mut test: tcase_test_function,
) {
    tcase_add_test(
        
        tc,
        test,
    );
}
#[no_mangle]

pub unsafe extern "C" fn tcase_add_test__if_xml_ge(
    mut tc: *mut TCase,
    mut test: tcase_test_function,
) {
    tcase_add_test(
        
        tc,
        test,
    );
}
#[no_mangle]

pub unsafe extern "C" fn basic_teardown() {
    if !crate::src::tests::common::g_parser.is_null() {
        XML_ParserFree(crate::src::tests::common::g_parser);
        crate::src::tests::common::g_parser =
            ::core::ptr::null_mut::<XML_ParserStruct>();
    }
}
#[no_mangle]

pub unsafe extern "C" fn _xml_failure(
    mut parser: XML_Parser,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut err: XML_Error = XML_GetErrorCode(parser);
    snprintf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
        b"    %d: %s (line %lu, offset %lu)\n    reported from %s, line %d\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        
        err,
        XML_ErrorString(err),
        XML_GetCurrentLineNumber(parser),
        XML_GetCurrentColumnNumber(parser),
        file,
        line,
    );
    _fail(file, line, &raw mut buffer as *mut ::core::ffi::c_char);
}
#[no_mangle]

pub unsafe extern "C" fn _XML_Parse_SINGLE_BYTES(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut isFinal: ::core::ffi::c_int,
) -> XML_Status {
    if !parser.is_null() && !s.is_null() && len >= 0 {
    } else {
        __assert_fail(
            b"(parser != NULL) && (s != NULL) && (len >= 0)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/common.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            200u32,
            __ASSERT_FUNCTION_1.as_ptr(),
        );
    };
    let chunksize: ::core::ffi::c_int = g_chunkSize;
    if chunksize > 0 {
        while len > chunksize {
            let mut res: XML_Status = XML_Parse(
                parser,
                s,
                chunksize,
                XML_FALSE as ::core::ffi::c_int,
            );
            if  res
                !=  XML_STATUS_OK
            {
                if  res
                    ==  XML_STATUS_SUSPENDED
                    && len > chunksize
                {
                    _fail(
                        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/common.c\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        211i32,
                        b"Use of function _XML_Parse_SINGLE_BYTES with a chunk size greater than 0 (from g_chunkSize) does not work well with suspension. Please consider use of plain XML_Parse at this place in your test, instead.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                return res;
            }
            len -= chunksize;
            s = s.offset(chunksize as isize);
        }
    }
    return XML_Parse(parser, s, len, isFinal);
}
#[no_mangle]

pub unsafe extern "C" fn _expect_failure(
    mut text: *const ::core::ffi::c_char,
    mut errorCode: XML_Error,
    mut errorMessage: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut lineno: ::core::ffi::c_int,
) {
    if  _XML_Parse_SINGLE_BYTES(
        crate::src::tests::common::g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        ==  XML_STATUS_OK
    {
        _fail(file, lineno, errorMessage);
    }
    if  XML_GetErrorCode(crate::src::tests::common::g_parser)
        !=  errorCode
    {
        _xml_failure(crate::src::tests::common::g_parser, file, lineno);
    }
}
#[no_mangle]

pub unsafe extern "C" fn _run_character_check(
    mut text: *const ::core::ffi::c_char,
    mut expected: *const XML_Char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    CharData_Init(
        
        &raw mut storage,
    );
    XML_SetUserData(
        crate::src::tests::common::g_parser,
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    XML_SetCharacterDataHandler(
        crate::src::tests::common::g_parser,
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
        crate::src::tests::common::g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        ==  XML_STATUS_ERROR
    {
        _xml_failure(crate::src::tests::common::g_parser, file, line);
    }
    CharData_CheckXMLChars(
        
        &raw mut storage,
        expected,
    );
}
#[no_mangle]

pub unsafe extern "C" fn _run_attribute_check(
    mut text: *const ::core::ffi::c_char,
    mut expected: *const XML_Char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    CharData_Init(
        
        &raw mut storage,
    );
    XML_SetUserData(
        crate::src::tests::common::g_parser,
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    XML_SetStartElementHandler(
        crate::src::tests::common::g_parser,
        Some(
            accumulate_attribute
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut *const XML_Char,
                ) -> (),
        ),
    );
    if  _XML_Parse_SINGLE_BYTES(
        crate::src::tests::common::g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        ==  XML_STATUS_ERROR
    {
        _xml_failure(crate::src::tests::common::g_parser, file, line);
    }
    CharData_CheckXMLChars(
        
        &raw mut storage,
        expected,
    );
}
#[no_mangle]

pub unsafe extern "C" fn _run_ext_character_check(
    mut text: *const ::core::ffi::c_char,
    mut test_data: *mut crate::src::tests::common::ExtTest,
    mut expected: *const XML_Char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    let storage: *mut CharData = malloc(
        
        ::core::mem::size_of::<CharData>(),
    )
        as *mut CharData;
    CharData_Init(
        
        storage,
    );
    (*test_data).storage = storage;
    XML_SetUserData(
        crate::src::tests::common::g_parser,
        test_data as *mut ::core::ffi::c_void,
    );
    XML_SetCharacterDataHandler(
        crate::src::tests::common::g_parser,
        Some(
            ext_accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if  _XML_Parse_SINGLE_BYTES(
        crate::src::tests::common::g_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    )
        ==  XML_STATUS_ERROR
    {
        _xml_failure(crate::src::tests::common::g_parser, file, line);
    }
    CharData_CheckXMLChars(
        
        storage,
        expected,
    );
    free(storage as *mut ::core::ffi::c_void);
}

pub const ALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -1;

pub const REALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -1;
#[no_mangle]

pub static mut g_allocation_count: ::core::ffi::c_int = ALLOC_ALWAYS_SUCCEED;
#[no_mangle]

pub static mut g_reallocation_count: ::core::ffi::c_int = REALLOC_ALWAYS_SUCCEED;
#[no_mangle]

pub unsafe extern "C" fn duff_allocator(
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    if g_allocation_count == 0 {
        return NULL;
    }
    if g_allocation_count != ALLOC_ALWAYS_SUCCEED {
        g_allocation_count -= 1;
    }
    return malloc(size);
}
#[no_mangle]

pub unsafe extern "C" fn duff_reallocator(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    if g_reallocation_count == 0 {
        return NULL;
    }
    if g_reallocation_count != REALLOC_ALWAYS_SUCCEED {
        g_reallocation_count -= 1;
    }
    return realloc(ptr, size);
}

unsafe extern "C" fn portable_strnlen(
    mut s: *const ::core::ffi::c_char,
    mut maxlen: size_t,
) -> size_t {
    let end: *const ::core::ffi::c_char =
        memchr(s as *const ::core::ffi::c_void, '\0' as i32, maxlen)
            as *const ::core::ffi::c_char;
    return if end.is_null() {
        maxlen
    } else {
        
        end.offset_from(s) as size_t
    };
}
#[no_mangle]

pub unsafe extern "C" fn portable_strndup(
    mut s: *const ::core::ffi::c_char,
    mut n: size_t,
) -> *mut ::core::ffi::c_char {
    if s.is_null() || n == SIZE_MAX as size_t {
        *__errno_location() = EINVAL;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    n = portable_strnlen(s, n);
    let buffer: *mut ::core::ffi::c_char =
        malloc(n.wrapping_add(1usize))
            as *mut ::core::ffi::c_char;
    if buffer.is_null() {
        *__errno_location() = ENOMEM;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    *__errno_location() = 0;
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        n,
    );
    *buffer.offset(n as isize) =  '\0' as ::core::ffi::c_char;
    return buffer;
}
