use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type XML_ParserStruct;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn XML_ParserCreate(encoding: *const XML_Char) -> XML_Parser;
    fn XML_SetExternalEntityRefHandler(parser: XML_Parser, handler: XML_ExternalEntityRefHandler);
    fn XML_SetUserData(parser: XML_Parser, userData: *mut ::core::ffi::c_void);
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
    fn XML_ParserFree(parser: XML_Parser);
    fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
        parser: XML_Parser,
        maximumAmplificationFactor: ::core::ffi::c_float,
    ) -> XML_Bool;
    fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
        parser: XML_Parser,
        activationThresholdBytes: ::core::ffi::c_ulonglong,
    ) -> XML_Bool;
    fn testingAccountingGetCountBytesDirect(parser: XML_Parser) -> ::core::ffi::c_ulonglong;
    fn testingAccountingGetCountBytesIndirect(parser: XML_Parser) -> ::core::ffi::c_ulonglong;
    fn unsignedCharToPrintable(c: ::core::ffi::c_uchar) -> *const ::core::ffi::c_char;
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
    fn tcase_add_test(tc: *mut TCase, test: tcase_test_function);
    fn tcase_add_test__ifdef_xml_dtd(tc: *mut TCase, test: tcase_test_function);
    fn _xml_failure(parser: XML_Parser, file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);
    fn _XML_Parse_SINGLE_BYTES(
        parser: XML_Parser,
        s: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        isFinal: ::core::ffi::c_int,
    ) -> XML_Status;
    fn accounting_external_entity_ref_handler(
        parser: XML_Parser,
        context: *const XML_Char,
        base: *const XML_Char,
        systemId: *const XML_Char,
        publicId: *const XML_Char,
    ) -> ::core::ffi::c_int;
}
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub struct AccountingTestCase {
    pub primaryText: *const ::core::ffi::c_char,
    pub firstExternalText: *const ::core::ffi::c_char,
    pub secondExternalText: *const ::core::ffi::c_char,
    pub expectedCountBytesIndirectExtra: ::core::ffi::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestCase {
    pub offsetOfThreshold: ::core::ffi::c_int,
    pub expectedStatus: XML_Status,
}
pub const XML_TRUE: XML_Bool = 1 as ::core::ffi::c_int as XML_Bool;
pub const XML_FALSE: XML_Bool = 0 as ::core::ffi::c_int as XML_Bool;
unsafe extern "C" fn test_accounting_precision() {
    unsafe {
        _check_set_test_info(
            b"test_accounting_precision\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            59 as ::core::ffi::c_int,
        );
        let mut cases: [AccountingTestCase; 36] = [
            AccountingTestCase {
                primaryText: b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e></e>\0".as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e k1=\"v2\" k2=\"v2\"/>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e k1=\"v2\" k2=\"v2\"></e>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<p:e xmlns:p=\"https://domain.invalid/\" />\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e k=\"&amp;&apos;&gt;&lt;&quot;\" />\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(5 as usize) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e1 xmlns='https://example.org/'>\n  <e2 xmlns=''/>\n</e1>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e>text</e>\0".as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e1><e2>text1<e3/>text2</e2></e1>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e>&amp;&apos;&gt;&lt;&quot;</e>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(5 as usize) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e>&#65;&#41;</e>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<?xml version=\"1.0\"?><root/>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"  <e1>  <e2>  </e2>  </e1>  \0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e1  ><e2  /></e1  >\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e1><e2 k = \"v\"/><e3 k = 'v'/></e1>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!-- Comment --><e><!-- Comment --></e>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<?xml-stylesheet type=\"text/xsl\" href=\"https://domain.invalid/\" media=\"all\"?><e/>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<?pi0?><?pi1 ?><?pi2  ?><r/><?pi4?>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<?pi0?><?pi1 ?><?pi2  ?><!DOCTYPE r SYSTEM 'first.ent'><r/>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: b"<?pi3?><!ENTITY % e1 SYSTEM 'second.ent'><?pi4?>%e1;<?pi5?>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                secondExternalText: b"<?pi6?>\0".as_ptr() as *const ::core::ffi::c_char,
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<e><![CDATA[one two three]]></e>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r [\n<!ENTITY e \"111<![CDATA[2 <= 2]]>333\">\n]>\n<r>&e;</r>\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        strlen(
                            b"111<![CDATA[2 <= 2]]>333\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) as usize,
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r [\n<!ENTITY % draft 'INCLUDE'>\n<!ENTITY % final 'IGNORE'>\n<!ENTITY % import SYSTEM \"first.ent\">\n%import;\n]>\n<r/>\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: b"<![%draft;[<!--1-->]]>\n<![%final;[<!--22-->]]>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        (strlen(b"INCLUDE\0".as_ptr() as *const ::core::ffi::c_char)
                            as usize)
                            .wrapping_add(
                                strlen(b"IGNORE\0".as_ptr() as *const ::core::ffi::c_char)
                                    as usize,
                            ),
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE root [\n<!ENTITY nine \"123456789\">\n]>\n<root>&nine;</root>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        strlen(b"123456789\0".as_ptr() as *const ::core::ffi::c_char)
                            as usize,
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE root [\n<!ENTITY nine \"123456789\">\n]>\n<root k1=\"&nine;\"/>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        strlen(b"123456789\0".as_ptr() as *const ::core::ffi::c_char)
                            as usize,
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE root [\n<!ENTITY nine \"123456789\">\n<!ENTITY nine2 \"&nine;&nine;\">\n]>\n<root>&nine2;&nine2;&nine2;</root>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(3 as usize)
                    .wrapping_mul(2 as usize)
                    .wrapping_mul(
                        (strlen(b"&nine;\0".as_ptr() as *const ::core::ffi::c_char)
                            as usize)
                            .wrapping_add(
                                strlen(
                                    b"123456789\0".as_ptr() as *const ::core::ffi::c_char,
                                ) as usize,
                            ),
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r [\n  <!ENTITY five SYSTEM 'first.ent'>\n]>\n<r>&five;</r>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: b"12345\0".as_ptr() as *const ::core::ffi::c_char,
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r [\n  <!ENTITY five SYSTEM 'first.ent'>\n]>\n<r>&five;</r>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: b"\xEF\xBB\xBF\0".as_ptr()
                    as *const ::core::ffi::c_char,
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r [\n<!ENTITY % comment \"<!---->\">\n%comment;\n]>\n<r/>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        strlen(b"<!---->\0".as_ptr() as *const ::core::ffi::c_char)
                            as usize,
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r [\n<!ENTITY % ninedef \"&#60;!ENTITY nine &#34;123456789&#34;&#62;\">\n%ninedef;\n]>\n<r>&nine;</r>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        (strlen(
                            b"<!ENTITY nine \"123456789\">\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) as usize)
                            .wrapping_add(
                                strlen(
                                    b"123456789\0".as_ptr() as *const ::core::ffi::c_char,
                                ) as usize,
                            ),
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r [\n<!ENTITY % comment \"<!--1-->\">\n<!ENTITY % comment2 \"&#37;comment;<!--22-->&#37;comment;\">\n%comment2;\n]>\n<r/>\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        (strlen(
                            b"%comment;<!--22-->%comment;\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) as usize)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        strlen(b"<!--1-->\0".as_ptr() as *const ::core::ffi::c_char)
                                            as usize,
                                    ),
                            ),
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r [\n  <!ENTITY % five \"12345\">\n  <!ENTITY % five2def \"&#60;!ENTITY five2 &#34;[&#37;five;][&#37;five;]]]]&#34;&#62;\">\n  %five2def;\n]>\n<r>&five2;</r>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        (strlen(
                            b"<!ENTITY five2 \"[%five;][%five;]]]]\">\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) as usize)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        strlen(b"12345\0".as_ptr() as *const ::core::ffi::c_char)
                                            as usize,
                                    ),
                            )
                            .wrapping_add(
                                strlen(
                                    b"[12345][12345]]]]\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                ) as usize,
                            ),
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r SYSTEM \"first.ent\">\n<r/>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: b"<!ENTITY % comment '<!--1-->'>\n<!ENTITY % comment2 '<!--22-->%comment;<!--22-->%comment;<!--22-->'>\n%comment2;\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                secondExternalText: ::core::ptr::null::<::core::ffi::c_char>(),
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        (strlen(
                            b"<!--22-->%comment;<!--22-->%comment;<!--22-->\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) as usize)
                            .wrapping_add(
                                (2 as usize)
                                    .wrapping_mul(
                                        strlen(b"<!---->\0".as_ptr() as *const ::core::ffi::c_char)
                                            as usize,
                                    ),
                            ),
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r SYSTEM 'first.ent'>\n<r/>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: b"<!ENTITY % e1 PUBLIC 'foo' 'second.ent'>\n<!ENTITY % e2 '<!--22-->%e1;<!--22-->'>\n%e2;\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                secondExternalText: b"<!--1-->\0".as_ptr() as *const ::core::ffi::c_char,
                expectedCountBytesIndirectExtra: (::core::mem::size_of::<XML_Char>()
                    as usize)
                    .wrapping_mul(
                        strlen(
                            b"<!--22--><!--1--><!--22-->\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        ) as usize,
                    ) as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r SYSTEM 'first.ent'>\n<r/>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: b"<!ENTITY % e1 SYSTEM 'second.ent'>\n<!ENTITY % e2 '%e1;'>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                secondExternalText: b"<?xml version='1.0' encoding='utf-8'?>\nhello\nxml\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE r SYSTEM 'first.ent'>\n<r/>\0".as_ptr()
                    as *const ::core::ffi::c_char,
                firstExternalText: b"<!ENTITY % e1 SYSTEM 'second.ent'>\n<!ENTITY % e2 '%e1;'>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                secondExternalText: b"<?xml version='1.0' encoding='utf-8'?>\nhello\nxml\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expectedCountBytesIndirectExtra: 0 as ::core::ffi::c_ulonglong,
            },
            AccountingTestCase {
                primaryText: b"<!DOCTYPE doc SYSTEM 'first.ent'>\n<doc></doc>\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                firstExternalText: b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM 'second.ent'>\n<!ENTITY % e2 '%e1;'>\n%e1;\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                secondExternalText: b"\xEF\xBB\xBF<!ATTLIST doc a1 CDATA 'value'>\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                expectedCountBytesIndirectExtra: strlen(
                    b"\xEF\xBB\xBF<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr()
                        as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_ulonglong,
            },
        ];
        let countCases: size_t = (::core::mem::size_of::<[AccountingTestCase; 36]>() as size_t)
            .wrapping_div(::core::mem::size_of::<AccountingTestCase>() as size_t);
        let mut u: size_t = 0 as size_t;
        while u < countCases {
            let expectedCountBytesDirect: ::core::ffi::c_ulonglong =
                strlen(cases[u as usize].primaryText) as ::core::ffi::c_ulonglong;
            let expectedCountBytesIndirect: ::core::ffi::c_ulonglong =
                ((if !cases[u as usize].firstExternalText.is_null() {
                    strlen(cases[u as usize].firstExternalText)
                } else {
                    0 as size_t
                })
                .wrapping_add(
                    (if !cases[u as usize].secondExternalText.is_null() {
                        strlen(cases[u as usize].secondExternalText)
                    } else {
                        0 as size_t
                    }),
                ) as ::core::ffi::c_ulonglong)
                    .wrapping_add(cases[u as usize].expectedCountBytesIndirectExtra);
            let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
            XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
            if !cases[u as usize].firstExternalText.is_null() {
                XML_SetExternalEntityRefHandler(
                    parser,
                    Some(
                        accounting_external_entity_ref_handler
                            as unsafe extern "C" fn(
                                XML_Parser,
                                *const XML_Char,
                                *const XML_Char,
                                *const XML_Char,
                                *const XML_Char,
                            )
                                -> ::core::ffi::c_int,
                    ),
                );
                XML_SetUserData(
                    parser,
                    (&raw mut cases as *mut AccountingTestCase).offset(u as isize)
                        as *mut AccountingTestCase as *mut ::core::ffi::c_void,
                );
            }
            let mut status: XML_Status = _XML_Parse_SINGLE_BYTES(
                parser,
                cases[u as usize].primaryText,
                strlen(cases[u as usize].primaryText) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            );
            if status as ::core::ffi::c_uint
                != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    parser,
                    b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                    264 as ::core::ffi::c_int,
                );
            }
            let actualCountBytesDirect: ::core::ffi::c_ulonglong =
                testingAccountingGetCountBytesDirect(parser) as ::core::ffi::c_ulonglong;
            let actualCountBytesIndirect: ::core::ffi::c_ulonglong =
                testingAccountingGetCountBytesIndirect(parser) as ::core::ffi::c_ulonglong;
            XML_ParserFree(parser);
            if actualCountBytesDirect != expectedCountBytesDirect {
                fprintf(
                    stderr,
                    b"Document %lu of %lu: Expected %llu count direct bytes, got %llu instead.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    u.wrapping_add(1 as size_t),
                    countCases,
                    expectedCountBytesDirect,
                    actualCountBytesDirect,
                );
                _fail(
                    b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                    280 as ::core::ffi::c_int,
                    b"Count of direct bytes is off\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if actualCountBytesIndirect != expectedCountBytesIndirect {
                fprintf(
                    stderr,
                    b"Document %lu of %lu: Expected %llu count indirect bytes, got %llu instead.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    u.wrapping_add(1 as size_t),
                    countCases,
                    expectedCountBytesIndirect,
                    actualCountBytesIndirect,
                );
                _fail(
                    b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                    290 as ::core::ffi::c_int,
                    b"Count of indirect bytes is off\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            u = u.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn test_billion_laughs_attack_protection_api() {
    unsafe {
        _check_set_test_info(
            b"test_billion_laughs_attack_protection_api\0".as_ptr() as *const ::core::ffi::c_char,
            b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
            296 as ::core::ffi::c_int,
        );
        let mut parserWithoutParent: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        let mut parserWithParent: XML_Parser = XML_ExternalEntityParserCreate(
            parserWithoutParent,
            b"entity123\0".as_ptr() as *const XML_Char,
            ::core::ptr::null::<XML_Char>(),
        );
        if parserWithoutParent.is_null() {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                301 as ::core::ffi::c_int,
                b"parserWithoutParent is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if parserWithParent.is_null() {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                303 as ::core::ffi::c_int,
                b"parserWithParent is NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionMaximumAmplification(
            ::core::ptr::null_mut::<XML_ParserStruct>(),
            123.0f32,
        ) as ::core::ffi::c_int
            == XML_TRUE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                308 as ::core::ffi::c_int,
                b"Call with NULL parser is NOT supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionMaximumAmplification(parserWithParent, 123.0f32)
            as ::core::ffi::c_int
            == XML_TRUE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                312 as ::core::ffi::c_int,
                b"Call with non-root parser is NOT supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionMaximumAmplification(
            parserWithoutParent,
            ::core::f32::NAN,
        ) as ::core::ffi::c_int
            == XML_TRUE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                316 as ::core::ffi::c_int,
                b"Call with NaN limit is NOT supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionMaximumAmplification(parserWithoutParent, -1.0f32)
            as ::core::ffi::c_int
            == XML_TRUE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                320 as ::core::ffi::c_int,
                b"Call with negative limit is NOT supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionMaximumAmplification(parserWithoutParent, 0.9f32)
            as ::core::ffi::c_int
            == XML_TRUE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                324 as ::core::ffi::c_int,
                b"Call with positive limit <1.0 is NOT supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionMaximumAmplification(parserWithoutParent, 1.0f32)
            as ::core::ffi::c_int
            == XML_FALSE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                330 as ::core::ffi::c_int,
                b"Call with positive limit >=1.0 is supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionMaximumAmplification(
            parserWithoutParent,
            123456.789f32,
        ) as ::core::ffi::c_int
            == XML_FALSE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                334 as ::core::ffi::c_int,
                b"Call with positive limit >=1.0 is supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionMaximumAmplification(
            parserWithoutParent,
            ::core::f32::INFINITY,
        ) as ::core::ffi::c_int
            == XML_FALSE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                338 as ::core::ffi::c_int,
                b"Call with positive limit >=1.0 is supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionActivationThreshold(
            ::core::ptr::null_mut::<XML_ParserStruct>(),
            123 as ::core::ffi::c_ulonglong,
        ) as ::core::ffi::c_int
            == XML_TRUE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                343 as ::core::ffi::c_int,
                b"Call with NULL parser is NOT supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionActivationThreshold(
            parserWithParent,
            123 as ::core::ffi::c_ulonglong,
        ) as ::core::ffi::c_int
            == XML_TRUE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                347 as ::core::ffi::c_int,
                b"Call with non-root parser is NOT supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_SetBillionLaughsAttackProtectionActivationThreshold(
            parserWithoutParent,
            123 as ::core::ffi::c_ulonglong,
        ) as ::core::ffi::c_int
            == XML_FALSE as ::core::ffi::c_int
        {
            _fail(
                b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                353 as ::core::ffi::c_int,
                b"Call with non-NULL parentless parser is supposed to succeed\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        XML_ParserFree(parserWithParent);
        XML_ParserFree(parserWithoutParent);
    }
}
enum AccountingCallbackTest {
    UnsignedCharToPrintable,
    AmplificationIsolatedExternalParser,
}

fn run_accounting_callback_test(test: AccountingCallbackTest) {
    unsafe {
        match test {
            AccountingCallbackTest::UnsignedCharToPrintable => {
                _check_set_test_info(
                    b"test_helper_unsigned_char_to_printable\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                    360 as ::core::ffi::c_int,
                );
                let mut uc: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
                loop {
                    set_subtest(
                        b"char %u\0".as_ptr() as *const ::core::ffi::c_char,
                        uc as ::core::ffi::c_uint,
                    );
                    let printable = unsignedCharToPrintable(uc) as *const ::core::ffi::c_char;
                    if printable.is_null() {
                        _fail(
                            b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            367 as ::core::ffi::c_int,
                            b"unsignedCharToPrintable returned NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    } else if strlen(printable) < 1 as ::core::ffi::c_int as size_t {
                        _fail(
                            b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            369 as ::core::ffi::c_int,
                            b"unsignedCharToPrintable returned empty string\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if uc as ::core::ffi::c_int
                        == -(1 as ::core::ffi::c_int) as ::core::ffi::c_uchar as ::core::ffi::c_int
                    {
                        break;
                    }
                    uc = uc.wrapping_add(1);
                }
                set_subtest(b"char 'A'\0".as_ptr() as *const ::core::ffi::c_char);
                if strcmp(
                    unsignedCharToPrintable('A' as i32 as ::core::ffi::c_uchar),
                    b"A\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    _fail(
                        b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        378 as ::core::ffi::c_int,
                        b"unsignedCharToPrintable result mistaken\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                set_subtest(b"char '\\'\0".as_ptr() as *const ::core::ffi::c_char);
                if strcmp(
                    unsignedCharToPrintable('\\' as i32 as ::core::ffi::c_uchar),
                    b"\\\\\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0 as ::core::ffi::c_int
                {
                    _fail(
                        b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        381 as ::core::ffi::c_int,
                        b"unsignedCharToPrintable result mistaken\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            }
            AccountingCallbackTest::AmplificationIsolatedExternalParser => {
                _check_set_test_info(
                    b"test_amplification_isolated_external_parser\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/root/work/expat/tests/acc_tests.c\0".as_ptr() as *const ::core::ffi::c_char,
                    385 as ::core::ffi::c_int,
                );
                let doc = b"<!ENTITY % p1 '123456789_123456789_1234567'>\0";
                let doc_len = doc.len() as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
                let maximum_tolerated_amplification: ::core::ffi::c_float = 2.0f32;
                let cases = [
                    TestCase {
                        offsetOfThreshold: -(2 as ::core::ffi::c_int),
                        expectedStatus: XML_STATUS_ERROR,
                    },
                    TestCase {
                        offsetOfThreshold: -(1 as ::core::ffi::c_int),
                        expectedStatus: XML_STATUS_ERROR,
                    },
                    TestCase {
                        offsetOfThreshold: 0 as ::core::ffi::c_int,
                        expectedStatus: XML_STATUS_ERROR,
                    },
                    TestCase {
                        offsetOfThreshold: 1 as ::core::ffi::c_int,
                        expectedStatus: XML_STATUS_OK,
                    },
                    TestCase {
                        offsetOfThreshold: 2 as ::core::ffi::c_int,
                        expectedStatus: XML_STATUS_OK,
                    },
                ];

                for case in cases {
                    let offset_of_threshold = case.offsetOfThreshold;
                    let expected_status = case.expectedStatus;
                    let activation_threshold_bytes =
                        (doc_len + offset_of_threshold) as ::core::ffi::c_ulonglong;
                    set_subtest(
                        b"offsetOfThreshold=%d, expectedStatus=%d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        offset_of_threshold,
                        expected_status as ::core::ffi::c_uint,
                    );
                    let parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
                    if parser.is_null() {
                        _fail(
                            b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            414 as ::core::ffi::c_int,
                            b"check failed: parser != NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if !(XML_SetBillionLaughsAttackProtectionMaximumAmplification(
                        parser,
                        maximum_tolerated_amplification,
                    ) as ::core::ffi::c_int
                        == 1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int)
                    {
                        _fail(
                            b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            418 as ::core::ffi::c_int,
                            b"check failed: XML_SetBillionLaughsAttackProtectionMaximumAmplification( parser, maximumToleratedAmplification) == XML_TRUE\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    if !(XML_SetBillionLaughsAttackProtectionActivationThreshold(
                        parser,
                        activation_threshold_bytes,
                    ) as ::core::ffi::c_int
                        == 1 as ::core::ffi::c_int as XML_Bool as ::core::ffi::c_int)
                    {
                        _fail(
                            b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            421 as ::core::ffi::c_int,
                            b"check failed: XML_SetBillionLaughsAttackProtectionActivationThreshold( parser, activationThresholdBytes) == XML_TRUE\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    let ext_parser = XML_ExternalEntityParserCreate(
                        parser,
                        ::core::ptr::null::<XML_Char>(),
                        ::core::ptr::null::<XML_Char>(),
                    );
                    if ext_parser.is_null() {
                        _fail(
                            b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            424 as ::core::ffi::c_int,
                            b"check failed: ext_parser != NULL\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    let actual_status = _XML_Parse_SINGLE_BYTES(
                        ext_parser,
                        doc.as_ptr().cast::<::core::ffi::c_char>(),
                        doc_len,
                        XML_TRUE as ::core::ffi::c_int,
                    ) as XML_Status;
                    if !(actual_status as ::core::ffi::c_uint
                        == expected_status as ::core::ffi::c_uint)
                    {
                        _fail(
                            b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            429 as ::core::ffi::c_int,
                            b"check failed: actualStatus == expectedStatus\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                    if actual_status as ::core::ffi::c_uint
                        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !(XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
                            == XML_ERROR_AMPLIFICATION_LIMIT_BREACH as ::core::ffi::c_int
                                as ::core::ffi::c_uint)
                    {
                        _fail(
                            b"/root/work/expat/tests/acc_tests.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            432 as ::core::ffi::c_int,
                            b"check failed: XML_GetErrorCode(ext_parser) == XML_ERROR_AMPLIFICATION_LIMIT_BREACH\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    XML_ParserFree(ext_parser);
                    XML_ParserFree(parser);
                }
            }
        }
    }
}

extern "C" fn test_helper_unsigned_char_to_printable() {
    run_accounting_callback_test(AccountingCallbackTest::UnsignedCharToPrintable);
}

extern "C" fn test_amplification_isolated_external_parser() {
    run_accounting_callback_test(AccountingCallbackTest::AmplificationIsolatedExternalParser);
}
#[no_mangle]
pub unsafe extern "C" fn make_accounting_test_case(mut s: *mut Suite) {
    unsafe {
        let mut tc_accounting: *mut TCase =
            tcase_create(b"accounting tests\0".as_ptr() as *const ::core::ffi::c_char);
        suite_add_tcase(s, tc_accounting);
        tcase_add_test(
            tc_accounting,
            Some(test_accounting_precision as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_accounting,
            Some(test_billion_laughs_attack_protection_api as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test(
            tc_accounting,
            Some(test_helper_unsigned_char_to_printable as unsafe extern "C" fn() -> ()),
        );
        tcase_add_test__ifdef_xml_dtd(
            tc_accounting,
            Some(test_amplification_isolated_external_parser as unsafe extern "C" fn() -> ()),
        );
    }
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
