use ::c2rust_bitfields;

pub use crate::__stddef_size_t_h::size_t;
pub use crate::expat_external_h::XML_Char;
use crate::src::tests::common::_XML_Parse_SINGLE_BYTES;
use crate::src::tests::common::_xml_failure;
use crate::src::tests::common::tcase_add_test__ifdef_xml_dtd;

pub use crate::__stddef_null_h::NULL;
pub use crate::expat_h::XML_Bool;
pub use crate::expat_h::XML_Error;
pub use crate::expat_h::XML_ExternalEntityRefHandler;
pub use crate::expat_h::XML_ParamEntityParsing;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ParserStruct;
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
use crate::src::lib::xmlparse::testingAccountingGetCountBytesDirect;
use crate::src::lib::xmlparse::testingAccountingGetCountBytesIndirect;
use crate::src::lib::xmlparse::unsignedCharToPrintable;
pub use crate::src::lib::xmlparse::XML_ExternalEntityParserCreate;
pub use crate::src::lib::xmlparse::XML_GetErrorCode;
pub use crate::src::lib::xmlparse::XML_ParserCreate;
pub use crate::src::lib::xmlparse::XML_ParserFree;
pub use crate::src::lib::xmlparse::XML_SetBillionLaughsAttackProtectionActivationThreshold;
pub use crate::src::lib::xmlparse::XML_SetBillionLaughsAttackProtectionMaximumAmplification;
pub use crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler;
pub use crate::src::lib::xmlparse::XML_SetParamEntityParsing;
pub use crate::src::lib::xmlparse::XML_SetUserData;
pub use crate::src::tests::handlers::accounting_external_entity_ref_handler;
pub use crate::src::tests::handlers::AccountingTestCase;
pub use crate::src::tests::minicheck::set_subtest;
pub use crate::src::tests::minicheck::tcase_setup_function;
pub use crate::src::tests::minicheck::tcase_teardown_function;
pub use crate::src::tests::minicheck::tcase_test_function;
pub use crate::src::tests::minicheck::Suite;
pub use crate::src::tests::minicheck::TCase;
pub use crate::src::tests::minicheck::_check_set_test_info;
pub use crate::src::tests::minicheck::_fail;
pub use crate::src::tests::minicheck::suite_add_tcase;
pub use crate::src::tests::minicheck::tcase_add_test;
pub use crate::src::tests::minicheck::tcase_create;
pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__uint64_t;
use crate::stdlib::fprintf;
use crate::stdlib::stderr;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
pub use crate::stdlib::FILE;
pub use crate::stdlib::_IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct TestCase {
    pub offsetOfThreshold: ::core::ffi::c_int,
    pub expectedStatus: XML_Status,
}

unsafe extern "C" fn test_accounting_precision() {
    _check_set_test_info(
        b"test_accounting_precision\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        59,
    );
    let mut cases: [AccountingTestCase; 36] = [
        AccountingTestCase {
    primaryText:   b"<e/>\0".as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e></e>\0".as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e k1=\"v2\" k2=\"v2\"/>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e k1=\"v2\" k2=\"v2\"></e>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<p:e xmlns:p=\"https://domain.invalid/\" />\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e k=\"&amp;&apos;&gt;&lt;&quot;\" />\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(5usize) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<e1 xmlns='https://example.org/'>\n  <e2 xmlns=''/>\n</e1>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e>text</e>\0".as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e1><e2>text1<e3/>text2</e2></e1>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e>&amp;&apos;&gt;&lt;&quot;</e>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(5usize) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<e>&#65;&#41;</e>\0".as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<?xml version=\"1.0\"?><root/>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"  <e1>  <e2>  </e2>  </e1>  \0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e1  ><e2  /></e1  >\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e1><e2 k = \"v\"/><e3 k = 'v'/></e1>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<!-- Comment --><e><!-- Comment --></e>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<?xml-stylesheet type=\"text/xsl\" href=\"https://domain.invalid/\" media=\"all\"?><e/>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<?pi0?><?pi1 ?><?pi2  ?><r/><?pi4?>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<?pi0?><?pi1 ?><?pi2  ?><!DOCTYPE r SYSTEM 'first.ent'><r/>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   b"<?pi3?><!ENTITY % e1 SYSTEM 'second.ent'><?pi4?>%e1;<?pi5?>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    secondExternalText:   b"<?pi6?>\0".as_ptr() as *const ::core::ffi::c_char,
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<e><![CDATA[one two three]]></e>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r [\n<!ENTITY e \"111<![CDATA[2 <= 2]]>333\">\n]>\n<r>&e;</r>\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    
                    strlen(
                        b"111<![CDATA[2 <= 2]]>333\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r [\n<!ENTITY % draft 'INCLUDE'>\n<!ENTITY % final 'IGNORE'>\n<!ENTITY % import SYSTEM \"first.ent\">\n%import;\n]>\n<r/>\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   b"<![%draft;[<!--1-->]]>\n<![%final;[<!--22-->]]>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    (strlen(b"INCLUDE\0".as_ptr() as *const ::core::ffi::c_char))
                        .wrapping_add(
                            
                            strlen(b"IGNORE\0".as_ptr() as *const ::core::ffi::c_char),
                        ),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE root [\n<!ENTITY nine \"123456789\">\n]>\n<root>&nine;</root>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    
                    strlen(b"123456789\0".as_ptr() as *const ::core::ffi::c_char),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE root [\n<!ENTITY nine \"123456789\">\n]>\n<root k1=\"&nine;\"/>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    
                    strlen(b"123456789\0".as_ptr() as *const ::core::ffi::c_char),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE root [\n<!ENTITY nine \"123456789\">\n<!ENTITY nine2 \"&nine;&nine;\">\n]>\n<root>&nine2;&nine2;&nine2;</root>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(3usize)
                .wrapping_mul(2usize)
                .wrapping_mul(
                    (strlen(b"&nine;\0".as_ptr() as *const ::core::ffi::c_char))
                        .wrapping_add(
                            
                            strlen(b"123456789\0".as_ptr() as *const ::core::ffi::c_char),
                        ),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r [\n  <!ENTITY five SYSTEM 'first.ent'>\n]>\n<r>&five;</r>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   b"12345\0".as_ptr() as *const ::core::ffi::c_char,
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r [\n  <!ENTITY five SYSTEM 'first.ent'>\n]>\n<r>&five;</r>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   b"\xEF\xBB\xBF\0".as_ptr() as *const ::core::ffi::c_char,
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r [\n<!ENTITY % comment \"<!---->\">\n%comment;\n]>\n<r/>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    
                    strlen(b"<!---->\0".as_ptr() as *const ::core::ffi::c_char),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r [\n<!ENTITY % ninedef \"&#60;!ENTITY nine &#34;123456789&#34;&#62;\">\n%ninedef;\n]>\n<r>&nine;</r>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    (strlen(
                        b"<!ENTITY nine \"123456789\">\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ))
                        .wrapping_add(
                            
                            strlen(b"123456789\0".as_ptr() as *const ::core::ffi::c_char),
                        ),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r [\n<!ENTITY % comment \"<!--1-->\">\n<!ENTITY % comment2 \"&#37;comment;<!--22-->&#37;comment;\">\n%comment2;\n]>\n<r/>\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    (strlen(
                        b"%comment;<!--22-->%comment;\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ))
                        .wrapping_add(
                            (2usize)
                                .wrapping_mul(
                                    
                                    strlen(b"<!--1-->\0".as_ptr() as *const ::core::ffi::c_char),
                                ),
                        ),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r [\n  <!ENTITY % five \"12345\">\n  <!ENTITY % five2def \"&#60;!ENTITY five2 &#34;[&#37;five;][&#37;five;]]]]&#34;&#62;\">\n  %five2def;\n]>\n<r>&five2;</r>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    firstExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    (strlen(
                        b"<!ENTITY five2 \"[%five;][%five;]]]]\">\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ))
                        .wrapping_add(
                            (2usize)
                                .wrapping_mul(
                                    
                                    strlen(b"12345\0".as_ptr() as *const ::core::ffi::c_char),
                                ),
                        )
                        .wrapping_add(
                            
                            strlen(
                                b"[12345][12345]]]]\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            ),
                        ),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r SYSTEM \"first.ent\">\n<r/>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   b"<!ENTITY % comment '<!--1-->'>\n<!ENTITY % comment2 '<!--22-->%comment;<!--22-->%comment;<!--22-->'>\n%comment2;\0"
                .as_ptr() as *const ::core::ffi::c_char,
    secondExternalText:   ::core::ptr::null::<::core::ffi::c_char>(),
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    (strlen(
                        b"<!--22-->%comment;<!--22-->%comment;<!--22-->\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ))
                        .wrapping_add(
                            (2usize)
                                .wrapping_mul(
                                    
                                    strlen(b"<!---->\0".as_ptr() as *const ::core::ffi::c_char),
                                ),
                        ),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r SYSTEM 'first.ent'>\n<r/>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   b"<!ENTITY % e1 PUBLIC 'foo' 'second.ent'>\n<!ENTITY % e2 '<!--22-->%e1;<!--22-->'>\n%e2;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
    secondExternalText:   b"<!--1-->\0".as_ptr() as *const ::core::ffi::c_char,
    expectedCountBytesIndirectExtra:   (::core::mem::size_of::<XML_Char>())
                .wrapping_mul(
                    
                    strlen(
                        b"<!--22--><!--1--><!--22-->\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    ),
                ) as ::core::ffi::c_ulonglong,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r SYSTEM 'first.ent'>\n<r/>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   b"<!ENTITY % e1 SYSTEM 'second.ent'>\n<!ENTITY % e2 '%e1;'>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    secondExternalText:   b"<?xml version='1.0' encoding='utf-8'?>\nhello\nxml\0"
                .as_ptr() as *const ::core::ffi::c_char,
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE r SYSTEM 'first.ent'>\n<r/>\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   b"<!ENTITY % e1 SYSTEM 'second.ent'>\n<!ENTITY % e2 '%e1;'>\0"
                .as_ptr() as *const ::core::ffi::c_char,
    secondExternalText:   b"<?xml version='1.0' encoding='utf-8'?>\nhello\nxml\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
    expectedCountBytesIndirectExtra:   0u64,
},
        AccountingTestCase {
    primaryText:   b"<!DOCTYPE doc SYSTEM 'first.ent'>\n<doc></doc>\n\0".as_ptr()
                as *const ::core::ffi::c_char,
    firstExternalText:   b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM 'second.ent'>\n<!ENTITY % e2 '%e1;'>\n%e1;\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
    secondExternalText:   b"\xEF\xBB\xBF<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr()
                as *const ::core::ffi::c_char,
    expectedCountBytesIndirectExtra:   strlen(
                b"\xEF\xBB\xBF<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr()
                    as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_ulonglong,
},
    ];
    let countCases: size_t = (::core::mem::size_of::<[AccountingTestCase; 36]>())
        .wrapping_div(::core::mem::size_of::<AccountingTestCase>());
    let mut u: size_t = 0;
    while u < countCases {
        let expectedCountBytesDirect: ::core::ffi::c_ulonglong =
            strlen(cases[u].primaryText) as ::core::ffi::c_ulonglong;
        let expectedCountBytesIndirect: ::core::ffi::c_ulonglong =
            ((if !cases[u].firstExternalText.is_null() {
                strlen(cases[u].firstExternalText)
            } else {
                0usize
            })
            .wrapping_add(
                (if !cases[u].secondExternalText.is_null() {
                    strlen(cases[u].secondExternalText)
                } else {
                    0usize
                }),
            ) as ::core::ffi::c_ulonglong)
                .wrapping_add(cases[u].expectedCountBytesIndirectExtra);
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
        if !cases[u].firstExternalText.is_null() {
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
                        ) -> ::core::ffi::c_int,
                ),
            );
            XML_SetUserData(
                parser,
                (&raw mut cases as *mut AccountingTestCase).offset(u as isize)
                    as *mut ::core::ffi::c_void,
            );
        }
        let mut status: XML_Status = _XML_Parse_SINGLE_BYTES(
            parser,
            cases[u].primaryText,
            strlen(cases[u].primaryText) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        if status != XML_STATUS_OK {
            _xml_failure(
                parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                264i32,
            );
        }
        let actualCountBytesDirect: ::core::ffi::c_ulonglong =
            testingAccountingGetCountBytesDirect(parser);
        let actualCountBytesIndirect: ::core::ffi::c_ulonglong =
            testingAccountingGetCountBytesIndirect(parser);
        XML_ParserFree(parser);
        if actualCountBytesDirect != expectedCountBytesDirect {
            fprintf(
                stderr,
                b"Document %lu of %lu: Expected %llu count direct bytes, got %llu instead.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                u.wrapping_add(1usize),
                countCases,
                expectedCountBytesDirect,
                actualCountBytesDirect,
            );
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                280i32,
                b"Count of direct bytes is off\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if actualCountBytesIndirect != expectedCountBytesIndirect {
            fprintf(
                stderr,
                b"Document %lu of %lu: Expected %llu count indirect bytes, got %llu instead.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                u.wrapping_add(1usize),
                countCases,
                expectedCountBytesIndirect,
                actualCountBytesIndirect,
            );
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                290i32,
                b"Count of indirect bytes is off\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        u = u.wrapping_add(1);
    }
}

unsafe extern "C" fn test_billion_laughs_attack_protection_api() {
    _check_set_test_info(
        b"test_billion_laughs_attack_protection_api\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        296,
    );
    let mut parserWithoutParent: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
    let mut parserWithParent: XML_Parser = XML_ExternalEntityParserCreate(
        parserWithoutParent,
        b"entity123\0".as_ptr() as *const XML_Char,
        ::core::ptr::null::<XML_Char>(),
    );
    if parserWithoutParent.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            301i32,
            b"parserWithoutParent is NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if parserWithParent.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            303i32,
            b"parserWithParent is NULL\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBillionLaughsAttackProtectionMaximumAmplification(
        ::core::ptr::null_mut::<XML_ParserStruct>(),
        123.0,
    ) as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            308i32,
            b"Call with NULL parser is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBillionLaughsAttackProtectionMaximumAmplification(parserWithParent, 123.0)
        as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            312i32,
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
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            316i32,
            b"Call with NaN limit is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBillionLaughsAttackProtectionMaximumAmplification(parserWithoutParent, -1.0)
        as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            320i32,
            b"Call with negative limit is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBillionLaughsAttackProtectionMaximumAmplification(parserWithoutParent, 0.9)
        as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            324i32,
            b"Call with positive limit <1.0 is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBillionLaughsAttackProtectionMaximumAmplification(parserWithoutParent, 1.0)
        as ::core::ffi::c_int
        == XML_FALSE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            330i32,
            b"Call with positive limit >=1.0 is supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBillionLaughsAttackProtectionMaximumAmplification(parserWithoutParent, 123456.789)
        as ::core::ffi::c_int
        == XML_FALSE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            334i32,
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
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            338i32,
            b"Call with positive limit >=1.0 is supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBillionLaughsAttackProtectionActivationThreshold(
        ::core::ptr::null_mut::<XML_ParserStruct>(),
        123,
    ) as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            343i32,
            b"Call with NULL parser is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBillionLaughsAttackProtectionActivationThreshold(parserWithParent, 123)
        as ::core::ffi::c_int
        == XML_TRUE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            347i32,
            b"Call with non-root parser is NOT supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_SetBillionLaughsAttackProtectionActivationThreshold(parserWithoutParent, 123)
        as ::core::ffi::c_int
        == XML_FALSE as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            353i32,
            b"Call with non-NULL parentless parser is supposed to succeed\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(parserWithParent);
    XML_ParserFree(parserWithoutParent);
}

unsafe extern "C" fn test_helper_unsigned_char_to_printable() {
    _check_set_test_info(
        b"test_helper_unsigned_char_to_printable\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        360,
    );
    let mut uc: ::core::ffi::c_uchar = 0;
    loop {
        set_subtest(
            b"char %u\0".as_ptr() as *const ::core::ffi::c_char,
            uc as ::core::ffi::c_uint,
        );
        let printable: *const ::core::ffi::c_char = unsignedCharToPrintable(uc);
        if printable.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                367i32,
                b"unsignedCharToPrintable returned NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if strlen(printable) < 1 {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                369i32,
                b"unsignedCharToPrintable returned empty string\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if uc as ::core::ffi::c_int == -1i32 as ::core::ffi::c_uchar as ::core::ffi::c_int {
            break;
        }
        uc = uc.wrapping_add(1);
    }
    set_subtest(b"char 'A'\0".as_ptr() as *const ::core::ffi::c_char);
    if strcmp(
        unsignedCharToPrintable('A' as ::core::ffi::c_uchar),
        b"A\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            378i32,
            b"unsignedCharToPrintable result mistaken\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    set_subtest(b"char '\\'\0".as_ptr() as *const ::core::ffi::c_char);
    if strcmp(
        unsignedCharToPrintable('\\' as ::core::ffi::c_uchar),
        b"\\\\\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            381i32,
            b"unsignedCharToPrintable result mistaken\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}

unsafe extern "C" fn test_amplification_isolated_external_parser() {
    _check_set_test_info(
        b"test_amplification_isolated_external_parser\0".as_ptr() as *const ::core::ffi::c_char,
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        385,
    );
    let doc: [::core::ffi::c_char; 45] = ::core::mem::transmute::<
        [u8; 45],
        [::core::ffi::c_char; 45],
    >(*b"<!ENTITY % p1 '123456789_123456789_1234567'>\0");
    let docLen: ::core::ffi::c_int =
        ::core::mem::size_of::<[::core::ffi::c_char; 45]>() as ::core::ffi::c_int - 1;
    let maximumToleratedAmplification: ::core::ffi::c_float = 2.0;
    let mut cases: [TestCase; 5] = [
        TestCase {
            offsetOfThreshold: -2,
            expectedStatus: XML_STATUS_ERROR,
        },
        TestCase {
            offsetOfThreshold: -1,
            expectedStatus: XML_STATUS_ERROR,
        },
        TestCase {
            offsetOfThreshold: 0,
            expectedStatus: XML_STATUS_ERROR,
        },
        TestCase {
            offsetOfThreshold: 1,
            expectedStatus: XML_STATUS_OK,
        },
        TestCase {
            offsetOfThreshold: 2,
            expectedStatus: XML_STATUS_OK,
        },
    ];
    let mut i: size_t = 0;
    while i
        < (::core::mem::size_of::<[TestCase; 5]>()).wrapping_div(::core::mem::size_of::<TestCase>())
    {
        let offsetOfThreshold: ::core::ffi::c_int = cases[i].offsetOfThreshold;
        let expectedStatus: XML_Status = cases[i].expectedStatus;
        let activationThresholdBytes: ::core::ffi::c_ulonglong =
            (docLen + offsetOfThreshold) as ::core::ffi::c_ulonglong;
        set_subtest(
            b"offsetOfThreshold=%d, expectedStatus=%d\0".as_ptr() as *const ::core::ffi::c_char,
            offsetOfThreshold,
            expectedStatus,
        );
        let mut parser: XML_Parser = XML_ParserCreate(::core::ptr::null::<XML_Char>());
        if parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                414i32,
                b"check failed: parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(XML_SetBillionLaughsAttackProtectionMaximumAmplification(
            parser,
            maximumToleratedAmplification,
        ) as ::core::ffi::c_int
            == 1)
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                418i32,
                b"check failed: XML_SetBillionLaughsAttackProtectionMaximumAmplification( parser, maximumToleratedAmplification) == XML_TRUE\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if !(XML_SetBillionLaughsAttackProtectionActivationThreshold(
            parser,
            activationThresholdBytes,
        ) as ::core::ffi::c_int
            == 1)
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                421i32,
                b"check failed: XML_SetBillionLaughsAttackProtectionActivationThreshold( parser, activationThresholdBytes) == XML_TRUE\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut ext_parser: XML_Parser = XML_ExternalEntityParserCreate(
            parser,
            ::core::ptr::null::<XML_Char>(),
            ::core::ptr::null::<XML_Char>(),
        );
        if ext_parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                424i32,
                b"check failed: ext_parser != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let actualStatus: XML_Status = _XML_Parse_SINGLE_BYTES(
            ext_parser,
            &raw const doc as *const ::core::ffi::c_char,
            docLen,
            XML_TRUE as ::core::ffi::c_int,
        );
        if !(actualStatus == expectedStatus) {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                429i32,
                b"check failed: actualStatus == expectedStatus\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if actualStatus != XML_STATUS_OK {
            if !(XML_GetErrorCode(ext_parser) == XML_ERROR_AMPLIFICATION_LIMIT_BREACH) {
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/acc_tests.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    432i32,
                    b"check failed: XML_GetErrorCode(ext_parser) == XML_ERROR_AMPLIFICATION_LIMIT_BREACH\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
        XML_ParserFree(ext_parser);
        XML_ParserFree(parser);
        i = i.wrapping_add(1);
    }
}
#[no_mangle]

pub unsafe extern "C" fn make_accounting_test_case(mut s: *mut Suite) {
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
