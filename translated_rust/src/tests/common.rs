extern "C" {
    pub type XML_ParserStruct;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn XML_SetStartElementHandler(parser: XML_Parser, handler: XML_StartElementHandler);
    fn XML_SetCharacterDataHandler(parser: XML_Parser, handler: XML_CharacterDataHandler);
    fn XML_SetUserData(parser: XML_Parser, userData: *mut ::core::ffi::c_void);
    fn XML_Parse(
        parser: XML_Parser,
        s: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn XML_GetErrorCode(parser: XML_Parser) -> XML_Error;
    fn XML_GetCurrentLineNumber(parser: XML_Parser) -> XML_Size;
    fn XML_GetCurrentColumnNumber(parser: XML_Parser) -> XML_Size;
    fn XML_ParserFree(parser: XML_Parser);
    fn XML_ErrorString(code: XML_Error) -> *const XML_LChar;
    fn CharData_Init(storage: *mut CharData);
    fn CharData_CheckXMLChars(storage: *mut CharData, s: *const XML_Char) -> ::core::ffi::c_int;
    fn _fail(
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
        msg: *const ::core::ffi::c_char,
    ) -> !;
    fn tcase_add_test(tc: *mut TCase, test: tcase_test_function);
    static mut g_parser: XML_Parser;
    fn accumulate_characters(
        userData: *mut ::core::ffi::c_void,
        s: *const XML_Char,
        len: ::core::ffi::c_int,
    );
    fn accumulate_attribute(
        userData: *mut ::core::ffi::c_void,
        name: *const XML_Char,
        atts: *mut *const XML_Char,
    );
    fn ext_accumulate_characters(
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
pub type XML_StartElementHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, *mut *const XML_Char) -> (),
>;
pub type XML_CharacterDataHandler = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const XML_Char, ::core::ffi::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharData {
    pub count: ::core::ffi::c_int,
    pub data: [XML_Char; 2048],
}
pub type tcase_setup_function = Option<unsafe extern "C" fn() -> ()>;
pub type tcase_teardown_function = Option<unsafe extern "C" fn() -> ()>;
pub type tcase_test_function = Option<unsafe extern "C" fn() -> ()>;
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
pub struct ExtTest {
    pub parse_text: *const ::core::ffi::c_char,
    pub encoding: *const XML_Char,
    pub storage: *mut CharData,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 76] = unsafe {
    ::core::mem::transmute::<[u8; 76], [::core::ffi::c_char; 76]>(
        *b"enum XML_Status _XML_Parse_SINGLE_BYTES(XML_Parser, const char *, int, int)\0",
    )
};
pub const ENOMEM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
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
pub static mut g_chunkSize: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn tcase_add_test__ifdef_xml_dtd(
    mut tc: *mut TCase,
    mut test: tcase_test_function,
) {
    unsafe {
        tcase_add_test(tc, test);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tcase_add_test__if_xml_ge(
    mut tc: *mut TCase,
    mut test: tcase_test_function,
) {
    unsafe {
        tcase_add_test(tc, test);
    }
}
#[no_mangle]
pub unsafe extern "C" fn basic_teardown() {
    unsafe {
        if !g_parser.is_null() {
            XML_ParserFree(g_parser);
            g_parser = ::core::ptr::null_mut::<XML_ParserStruct>();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _xml_failure(
    mut parser: XML_Parser,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    unsafe {
        let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut err: XML_Error = XML_GetErrorCode(parser);
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
            b"    %d: %s (line %lu, offset %lu)\n    reported from %s, line %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            err as ::core::ffi::c_uint,
            XML_ErrorString(err),
            XML_GetCurrentLineNumber(parser),
            XML_GetCurrentColumnNumber(parser),
            file,
            line,
        );
        _fail(file, line, &raw mut buffer as *mut ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _XML_Parse_SINGLE_BYTES(
    mut parser: XML_Parser,
    mut s: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut isFinal: ::core::ffi::c_int,
) -> XML_Status {
    unsafe {
        if !parser.is_null() && !s.is_null() && len >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"(parser != NULL) && (s != NULL) && (len >= 0)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/root/work/expat/tests/common.c\0".as_ptr() as *const ::core::ffi::c_char,
                200 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        };
        let chunksize: ::core::ffi::c_int = g_chunkSize;
        if chunksize > 0 as ::core::ffi::c_int {
            while len > chunksize {
                let mut res: XML_Status =
                    XML_Parse(parser, s, chunksize, XML_FALSE as ::core::ffi::c_int);
                if res as ::core::ffi::c_uint
                    != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if res as ::core::ffi::c_uint
                        == XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
                        && len > chunksize
                    {
                        _fail(
                            b"/root/work/expat/tests/common.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            211 as ::core::ffi::c_int,
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
}
#[no_mangle]
pub unsafe extern "C" fn _expect_failure(
    mut text: *const ::core::ffi::c_char,
    mut errorCode: XML_Error,
    mut errorMessage: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut lineno: ::core::ffi::c_int,
) {
    unsafe {
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(file, lineno, errorMessage);
        }
        if XML_GetErrorCode(g_parser) as ::core::ffi::c_uint != errorCode as ::core::ffi::c_uint {
            _xml_failure(g_parser, file, lineno);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _run_character_check(
    mut text: *const ::core::ffi::c_char,
    mut expected: *const XML_Char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    unsafe {
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
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
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(g_parser, file, line);
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _run_attribute_check(
    mut text: *const ::core::ffi::c_char,
    mut expected: *const XML_Char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    unsafe {
        let mut storage: CharData = CharData {
            count: 0,
            data: [0; 2048],
        };
        CharData_Init(&raw mut storage);
        XML_SetUserData(g_parser, &raw mut storage as *mut ::core::ffi::c_void);
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
        if _XML_Parse_SINGLE_BYTES(
            g_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(g_parser, file, line);
        }
        CharData_CheckXMLChars(&raw mut storage, expected);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _run_ext_character_check(
    mut text: *const ::core::ffi::c_char,
    mut test_data: *mut ExtTest,
    mut expected: *const XML_Char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) {
    unsafe {
        let storage: *mut CharData =
            malloc(::core::mem::size_of::<CharData>() as size_t) as *mut CharData;
        CharData_Init(storage);
        (*test_data).storage = storage;
        XML_SetUserData(g_parser, test_data as *mut ::core::ffi::c_void);
        XML_SetCharacterDataHandler(
            g_parser,
            Some(
                ext_accumulate_characters
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
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(g_parser, file, line);
        }
        CharData_CheckXMLChars(storage, expected);
        free(storage as *mut ::core::ffi::c_void);
    }
}
pub const ALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const REALLOC_ALWAYS_SUCCEED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
#[no_mangle]
pub static mut g_allocation_count: ::core::ffi::c_int = ALLOC_ALWAYS_SUCCEED;
#[no_mangle]
pub static mut g_reallocation_count: ::core::ffi::c_int = REALLOC_ALWAYS_SUCCEED;
#[no_mangle]
pub unsafe extern "C" fn duff_allocator(mut size: size_t) -> *mut ::core::ffi::c_void {
    unsafe {
        if g_allocation_count == 0 as ::core::ffi::c_int {
            return NULL;
        }
        if g_allocation_count != ALLOC_ALWAYS_SUCCEED {
            g_allocation_count -= 1;
        }
        return malloc(size);
    }
}
#[no_mangle]
pub unsafe extern "C" fn duff_reallocator(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        if g_reallocation_count == 0 as ::core::ffi::c_int {
            return NULL;
        }
        if g_reallocation_count != REALLOC_ALWAYS_SUCCEED {
            g_reallocation_count -= 1;
        }
        return realloc(ptr, size);
    }
}
unsafe extern "C" fn portable_strnlen(
    mut s: *const ::core::ffi::c_char,
    mut maxlen: size_t,
) -> size_t {
    unsafe {
        let end: *const ::core::ffi::c_char =
            memchr(s as *const ::core::ffi::c_void, '\0' as i32, maxlen)
                as *const ::core::ffi::c_char;
        return if end.is_null() {
            maxlen
        } else {
            end.offset_from(s) as ::core::ffi::c_long as size_t
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn portable_strndup(
    mut s: *const ::core::ffi::c_char,
    mut n: size_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        if s.is_null() || n == SIZE_MAX as size_t {
            *__errno_location() = EINVAL;
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        n = portable_strnlen(s, n);
        let buffer: *mut ::core::ffi::c_char =
            malloc(n.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        if buffer.is_null() {
            *__errno_location() = ENOMEM;
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        *__errno_location() = 0 as ::core::ffi::c_int;
        memcpy(
            buffer as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            n,
        );
        *buffer.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
        return buffer;
    }
}
