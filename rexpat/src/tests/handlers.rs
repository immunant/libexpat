// =============== BEGIN handlers_h ================
pub const STRUCT_START_TAG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const STRUCT_END_TAG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub type AttrInfo = crate::src::tests::handlers::attrInfo;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct attrInfo {
    pub name: *const crate::expat_external_h::XML_Char,
    pub value: *const crate::expat_external_h::XML_Char,
}

pub type ElementInfo = crate::src::tests::handlers::elementInfo;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct elementInfo {
    pub name: *const crate::expat_external_h::XML_Char,
    pub attr_count: ::core::ffi::c_int,
    pub id_name: *const crate::expat_external_h::XML_Char,
    pub attributes: *mut crate::src::tests::handlers::AttrInfo,
}

pub type ParserAndElementInfo = crate::src::tests::handlers::StructParserAndElementInfo;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct StructParserAndElementInfo {
    pub parser: crate::expat_h::XML_Parser,
    pub info: *mut crate::src::tests::handlers::ElementInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DataIssue240 {
    pub parser: crate::expat_h::XML_Parser,
    pub deep: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ExtOption {
    pub system_id: *const crate::expat_external_h::XML_Char,
    pub parse_text: *const ::core::ffi::c_char,
}

pub type ExtFaults = crate::src::tests::handlers::ext_faults;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ext_faults {
    pub parse_text: *const ::core::ffi::c_char,
    pub fail_text: *const ::core::ffi::c_char,
    pub encoding: *const crate::expat_external_h::XML_Char,
    pub error: crate::expat_h::XML_Error,
}

pub type ExtHdlrData = crate::src::tests::handlers::ext_hdlr_data;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ext_hdlr_data {
    pub parse_text: *const ::core::ffi::c_char,
    pub handler: crate::expat_h::XML_ExternalEntityRefHandler,
    pub storage: *mut crate::src::tests::chardata::CharData,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ExtTest2 {
    pub parse_text: *const ::core::ffi::c_char,
    pub parse_len: ::core::ffi::c_int,
    pub encoding: *const crate::expat_external_h::XML_Char,
    pub storage: *mut crate::src::tests::chardata::CharData,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ExtFaults2 {
    pub parse_text: *const ::core::ffi::c_char,
    pub parse_len: ::core::ffi::c_int,
    pub fail_text: *const ::core::ffi::c_char,
    pub encoding: *const crate::expat_external_h::XML_Char,
    pub error: crate::expat_h::XML_Error,
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

pub struct AttTest {
    pub definition: *const ::core::ffi::c_char,
    pub element_name: *const crate::expat_external_h::XML_Char,
    pub attr_name: *const crate::expat_external_h::XML_Char,
    pub attr_type: *const crate::expat_external_h::XML_Char,
    pub default_value: *const crate::expat_external_h::XML_Char,
    pub is_required: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ByteTestData {
    pub start_element_len: ::core::ffi::c_int,
    pub cdata_len: ::core::ffi::c_int,
    pub total_string_len: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct handler_record_entry {
    pub name: *const ::core::ffi::c_char,
    pub arg: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct handler_record_list {
    pub count: ::core::ffi::c_int,
    pub entries: [crate::src::tests::handlers::handler_record_entry; 50],
}

pub const ENTITY_MATCH_FAIL: ::core::ffi::c_int = -1 as ::core::ffi::c_int;

pub const ENTITY_MATCH_NOT_FOUND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const ENTITY_MATCH_SUCCESS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub type DefaultCheck = crate::src::tests::handlers::default_check;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct default_check {
    pub expected: *const crate::expat_external_h::XML_Char,
    pub expectedLen: ::core::ffi::c_int,
    pub seen: crate::expat_h::XML_Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ParserPlusStorage {
    pub parser: crate::expat_h::XML_Parser,
    pub storage: *mut crate::src::tests::chardata::CharData,
}
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;
use crate::stdlib::__assert_fail;

pub use crate::common_h::ALLOC_ALWAYS_SUCCEED;
pub use crate::expat_external_h::XML_Char;
pub use crate::expat_external_h::XML_Index;
pub use crate::expat_external_h::XML_Size;
pub use crate::expat_h::XML_Bool;
pub use crate::expat_h::XML_CharacterDataHandler;
pub use crate::expat_h::XML_Content;
pub use crate::expat_h::XML_Content_Quant;
pub use crate::expat_h::XML_Content_Type;
pub use crate::expat_h::XML_DefaultHandler;
pub use crate::expat_h::XML_ElementDeclHandler;
pub use crate::expat_h::XML_Encoding;
pub use crate::expat_h::XML_Error;
pub use crate::expat_h::XML_ExternalEntityRefHandler;
pub use crate::expat_h::XML_NotStandaloneHandler;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ParserStruct;
pub use crate::expat_h::XML_Parsing;
pub use crate::expat_h::XML_ParsingStatus;
pub use crate::expat_h::XML_StartElementHandler;
pub use crate::expat_h::XML_Status;
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
pub use crate::expat_h::XML_FINISHED;
pub use crate::expat_h::XML_INITIALIZED;
pub use crate::expat_h::XML_PARSING;
pub use crate::expat_h::XML_STATUS_ERROR;
pub use crate::expat_h::XML_STATUS_OK;
pub use crate::expat_h::XML_STATUS_SUSPENDED;
pub use crate::expat_h::XML_SUSPENDED;
pub use crate::expat_h::XML_TRUE;
pub use crate::src::lib::xmlparse::XML_DefaultCurrent;
pub use crate::src::lib::xmlparse::XML_ExternalEntityParserCreate;
pub use crate::src::lib::xmlparse::XML_FreeContentModel;
pub use crate::src::lib::xmlparse::XML_GetBuffer;
pub use crate::src::lib::xmlparse::XML_GetCurrentByteCount;
pub use crate::src::lib::xmlparse::XML_GetCurrentByteIndex;
pub use crate::src::lib::xmlparse::XML_GetCurrentColumnNumber;
pub use crate::src::lib::xmlparse::XML_GetCurrentLineNumber;
pub use crate::src::lib::xmlparse::XML_GetErrorCode;
pub use crate::src::lib::xmlparse::XML_GetIdAttributeIndex;
pub use crate::src::lib::xmlparse::XML_GetInputContext;
pub use crate::src::lib::xmlparse::XML_GetParsingStatus;
pub use crate::src::lib::xmlparse::XML_GetSpecifiedAttributeCount;
pub use crate::src::lib::xmlparse::XML_ParseBuffer;
pub use crate::src::lib::xmlparse::XML_ParserFree;
pub use crate::src::lib::xmlparse::XML_ParserReset;
pub use crate::src::lib::xmlparse::XML_ResumeParser;
pub use crate::src::lib::xmlparse::XML_SetCharacterDataHandler;
pub use crate::src::lib::xmlparse::XML_SetDefaultHandler;
pub use crate::src::lib::xmlparse::XML_SetElementDeclHandler;
pub use crate::src::lib::xmlparse::XML_SetEncoding;
pub use crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler;
pub use crate::src::lib::xmlparse::XML_SetNotStandaloneHandler;
pub use crate::src::lib::xmlparse::XML_SetStartElementHandler;
pub use crate::src::lib::xmlparse::XML_SetUserData;
pub use crate::src::lib::xmlparse::XML_SetXmlDeclHandler;
pub use crate::src::lib::xmlparse::XML_StopParser;
pub use crate::src::tests::chardata::CharData;
pub use crate::src::tests::chardata::CharData_AppendXMLChars;
pub use crate::src::tests::chardata::CharData_CheckXMLChars;
pub use crate::src::tests::chardata::CharData_Init;
pub use crate::src::tests::common::g_abortable;
pub use crate::src::tests::common::g_parser;
pub use crate::src::tests::common::g_resumable;
pub use crate::src::tests::common::get_buffer_test_text;
pub use crate::src::tests::common::ExtTest;
pub use crate::src::tests::common::_XML_Parse_SINGLE_BYTES;
pub use crate::src::tests::common::_xml_failure;
pub use crate::src::tests::common::g_allocation_count;
pub use crate::src::tests::common::g_reallocation_count;
use crate::src::tests::minicheck::_fail;
pub use crate::stdlib::intptr_t;
use crate::stdlib::snprintf;

pub use crate::src::tests::structdata::StructData;
pub use crate::src::tests::structdata::StructDataEntry;
pub use crate::src::tests::structdata::StructData_AddItem;
use crate::stdlib::memcmp;
use crate::stdlib::memcpy;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
use crate::stdlib::strncmp;
#[no_mangle]

pub static mut g_handler_data: *const ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>();
#[no_mangle]

pub static mut g_comment_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub static mut g_skip_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub static mut g_xdecl_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]

pub unsafe extern "C" fn start_element_event_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    crate::src::tests::chardata::CharData_AppendXMLChars(
        userData as *mut crate::src::tests::chardata::CharData
            as *mut crate::src::tests::chardata::CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn end_element_event_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"/\0".as_ptr() as *const crate::expat_external_h::XML_Char,
        1 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_element_event_handler2(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut attr: *mut *const crate::expat_external_h::XML_Char,
) {
    let mut storage: *mut crate::src::tests::structdata::StructData =
        userData as *mut crate::src::tests::structdata::StructData;
    crate::src::tests::structdata::StructData_AddItem(
        storage as *mut crate::src::tests::structdata::StructData,
        name,
        crate::src::lib::xmlparse::XML_GetCurrentColumnNumber(crate::src::tests::common::g_parser)
            as ::core::ffi::c_int,
        crate::src::lib::xmlparse::XML_GetCurrentLineNumber(crate::src::tests::common::g_parser)
            as ::core::ffi::c_int,
        crate::src::tests::handlers::STRUCT_START_TAG,
    );
}
#[no_mangle]

pub unsafe extern "C" fn end_element_event_handler2(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut storage: *mut crate::src::tests::structdata::StructData =
        userData as *mut crate::src::tests::structdata::StructData;
    crate::src::tests::structdata::StructData_AddItem(
        storage as *mut crate::src::tests::structdata::StructData,
        name,
        crate::src::lib::xmlparse::XML_GetCurrentColumnNumber(crate::src::tests::common::g_parser)
            as ::core::ffi::c_int,
        crate::src::lib::xmlparse::XML_GetCurrentLineNumber(crate::src::tests::common::g_parser)
            as ::core::ffi::c_int,
        crate::src::tests::handlers::STRUCT_END_TAG,
    );
}
#[no_mangle]

pub unsafe extern "C" fn counting_start_element_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let parserAndElementInfos: *mut crate::src::tests::handlers::ParserAndElementInfo =
        userData as *mut crate::src::tests::handlers::ParserAndElementInfo;
    let mut info: *mut crate::src::tests::handlers::ElementInfo = (*parserAndElementInfos).info;
    let mut attr: *mut crate::src::tests::handlers::AttrInfo =
        ::core::ptr::null_mut::<crate::src::tests::handlers::AttrInfo>();
    let mut count: ::core::ffi::c_int = 0;
    let mut id: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    while !(*info).name.is_null() {
        if crate::stdlib::strcmp(
            name as *const ::core::ffi::c_char,
            (*info).name as *const ::core::ffi::c_char,
        ) == 0
        {
            break;
        }
        info = info.offset(1);
    }
    if (*info).name.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            119 as ::core::ffi::c_int,
            b"Element not recognised\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    count =
        crate::src::lib::xmlparse::XML_GetSpecifiedAttributeCount((*parserAndElementInfos).parser);
    if (*info).attr_count * 2 as ::core::ffi::c_int != count {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            128 as ::core::ffi::c_int,
            b"Not got expected attribute count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    id = crate::src::lib::xmlparse::XML_GetIdAttributeIndex((*parserAndElementInfos).parser);
    if id == -1 as ::core::ffi::c_int && !(*info).id_name.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            133 as ::core::ffi::c_int,
            b"ID not present\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if id != -1 as ::core::ffi::c_int
        && crate::stdlib::strcmp(
            *atts.offset(id as isize) as *const ::core::ffi::c_char,
            (*info).id_name as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            137 as ::core::ffi::c_int,
            b"ID does not have the correct name\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*info).attr_count {
        attr = (*info).attributes;
        while !(*attr).name.is_null() {
            if crate::stdlib::strcmp(
                *atts.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                (*attr).name as *const ::core::ffi::c_char,
            ) == 0
            {
                break;
            }
            attr = attr.offset(1);
        }
        if (*attr).name.is_null() {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                148 as ::core::ffi::c_int,
                b"Attribute not recognised\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if crate::stdlib::strcmp(
            *atts.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            (*attr).value as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                152 as ::core::ffi::c_int,
                b"Attribute has wrong value\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        atts = atts.offset(2 as ::core::ffi::c_int as isize);
        i += 1;
    }
}
#[no_mangle]

pub unsafe extern "C" fn suspending_end_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
) {
    crate::src::lib::xmlparse::XML_StopParser(
        userData as crate::expat_h::XML_Parser,
        1 as crate::expat_h::XML_Bool,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_element_suspender(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    if crate::stdlib::strcmp(
        name as *const ::core::ffi::c_char,
        b"suspend\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        crate::src::lib::xmlparse::XML_StopParser(
            crate::src::tests::common::g_parser,
            crate::expat_h::XML_TRUE,
        );
    }
    if crate::stdlib::strcmp(
        name as *const ::core::ffi::c_char,
        b"abort\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        crate::src::lib::xmlparse::XML_StopParser(
            crate::src::tests::common::g_parser,
            crate::expat_h::XML_FALSE,
        );
    }
}
#[no_mangle]

pub static mut g_triplet_start_flag: ::core::ffi::c_int =
    crate::expat_h::XML_FALSE as ::core::ffi::c_int;
#[no_mangle]

pub static mut g_triplet_end_flag: ::core::ffi::c_int =
    crate::expat_h::XML_FALSE as ::core::ffi::c_int;
#[no_mangle]

pub unsafe extern "C" fn triplet_start_checker(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let mut elemstr: *mut *mut crate::expat_external_h::XML_Char =
        userData as *mut *mut crate::expat_external_h::XML_Char;
    let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    if crate::stdlib::strcmp(
        *elemstr.offset(0 as ::core::ffi::c_int as isize),
        name as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        crate::stdlib::snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                as crate::__stddef_size_t_h::size_t,
            b"unexpected start string: '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            193 as ::core::ffi::c_int,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
    if crate::stdlib::strcmp(
        *elemstr.offset(1 as ::core::ffi::c_int as isize),
        *atts.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        crate::stdlib::snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                as crate::__stddef_size_t_h::size_t,
            b"unexpected attribute string: '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            *atts.offset(0 as ::core::ffi::c_int as isize),
        );
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            198 as ::core::ffi::c_int,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
    g_triplet_start_flag = crate::expat_h::XML_TRUE as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn triplet_end_checker(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut elemstr: *mut *mut crate::expat_external_h::XML_Char =
        userData as *mut *mut crate::expat_external_h::XML_Char;
    if crate::stdlib::strcmp(
        *elemstr.offset(0 as ::core::ffi::c_int as isize),
        name as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
        crate::stdlib::snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                as crate::__stddef_size_t_h::size_t,
            b"unexpected end string: '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            214 as ::core::ffi::c_int,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
    g_triplet_end_flag = crate::expat_h::XML_TRUE as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn overwrite_start_checker(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let mut storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"start \0".as_ptr() as *const crate::expat_external_h::XML_Char,
        6 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
    while !(*atts).is_null() {
        crate::src::tests::chardata::CharData_AppendXMLChars(
            storage as *mut crate::src::tests::chardata::CharData,
            b"\nattribute \0".as_ptr() as *const crate::expat_external_h::XML_Char,
            11 as ::core::ffi::c_int,
        );
        crate::src::tests::chardata::CharData_AppendXMLChars(
            storage as *mut crate::src::tests::chardata::CharData,
            *atts,
            -1 as ::core::ffi::c_int,
        );
        atts = atts.offset(2 as ::core::ffi::c_int as isize);
    }
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"\n\0".as_ptr() as *const crate::expat_external_h::XML_Char,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn overwrite_end_checker(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"end \0".as_ptr() as *const crate::expat_external_h::XML_Char,
        4 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"\n\0".as_ptr() as *const crate::expat_external_h::XML_Char,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_element_fail(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    crate::src::tests::minicheck::_fail(
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        249 as ::core::ffi::c_int,
        b"should never reach start_element_fail()\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_ns_clearing_start_element(
    mut userData: *mut ::core::ffi::c_void,
    mut prefix: *const crate::expat_external_h::XML_Char,
    mut uri: *const crate::expat_external_h::XML_Char,
) {
    crate::src::lib::xmlparse::XML_SetStartElementHandler(
        userData as crate::expat_h::XML_Parser,
        None,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_element_issue_240(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let mut mydata: *mut crate::src::tests::handlers::DataIssue240 =
        userData as *mut crate::src::tests::handlers::DataIssue240;
    (*mydata).deep += 1;
}
#[no_mangle]

pub unsafe extern "C" fn end_element_issue_240(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut mydata: *mut crate::src::tests::handlers::DataIssue240 =
        userData as *mut crate::src::tests::handlers::DataIssue240;
    (*mydata).deep -= 1;
    if (*mydata).deep == 0 as ::core::ffi::c_int {
        crate::src::lib::xmlparse::XML_StopParser((*mydata).parser, 0 as crate::expat_h::XML_Bool);
    }
}
#[no_mangle]

pub unsafe extern "C" fn UnknownEncodingHandler(
    mut data: *mut ::core::ffi::c_void,
    mut encoding: *const crate::expat_external_h::XML_Char,
    mut info: *mut crate::expat_h::XML_Encoding,
) -> ::core::ffi::c_int {
    if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"unsupported-encoding\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            (*info).map[i as usize] = i;
            i += 1;
        }
        (*info).data = crate::__stddef_null_h::NULL;
        (*info).convert = None;
        (*info).release = None;
        return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
    }
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}

unsafe extern "C" fn dummy_release(mut data: *mut ::core::ffi::c_void) {}
#[no_mangle]

pub unsafe extern "C" fn UnrecognisedEncodingHandler(
    mut data: *mut ::core::ffi::c_void,
    mut encoding: *const crate::expat_external_h::XML_Char,
    mut info: *mut crate::expat_h::XML_Encoding,
) -> ::core::ffi::c_int {
    (*info).data = crate::__stddef_null_h::NULL;
    (*info).convert = None;
    (*info).release = Some(dummy_release as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn unknown_released_encoding_handler(
    mut data: *mut ::core::ffi::c_void,
    mut encoding: *const crate::expat_external_h::XML_Char,
    mut info: *mut crate::expat_h::XML_Encoding,
) -> ::core::ffi::c_int {
    if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"unsupported-encoding\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            (*info).map[i as usize] = i;
            i += 1;
        }
        (*info).data = crate::__stddef_null_h::NULL;
        (*info).convert = None;
        (*info).release =
            Some(dummy_release as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
    }
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}

unsafe extern "C" fn failing_converter(
    mut data: *mut ::core::ffi::c_void,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return -1 as ::core::ffi::c_int;
}

unsafe extern "C" fn prefix_converter(
    mut data: *mut ::core::ffi::c_void,
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if *s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == -1 as ::core::ffi::c_int as ::core::ffi::c_char as ::core::ffi::c_int
    {
        return -1 as ::core::ffi::c_int;
    }
    return *s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + (*s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x7f as ::core::ffi::c_int)
        & 0x1ff as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn MiscEncodingHandler(
    mut data: *mut ::core::ffi::c_void,
    mut encoding: *const crate::expat_external_h::XML_Char,
    mut info: *mut crate::expat_h::XML_Encoding,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut high_map: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
    if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-9\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
        || crate::stdlib::strcmp(
            encoding as *const ::core::ffi::c_char,
            b"ascii-like\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        || crate::stdlib::strcmp(
            encoding as *const ::core::ffi::c_char,
            b"invalid-len\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        || crate::stdlib::strcmp(
            encoding as *const ::core::ffi::c_char,
            b"invalid-a\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        || crate::stdlib::strcmp(
            encoding as *const ::core::ffi::c_char,
            b"invalid-surrogate\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        || crate::stdlib::strcmp(
            encoding as *const ::core::ffi::c_char,
            b"invalid-high\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        high_map = -1 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 128 as ::core::ffi::c_int {
        (*info).map[i as usize] = i;
        i += 1;
    }
    while i < 256 as ::core::ffi::c_int {
        (*info).map[i as usize] = high_map;
        i += 1;
    }
    if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-9\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[9 as ::core::ffi::c_int as usize] = 5 as ::core::ffi::c_int;
    }
    if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-len\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[0x81 as ::core::ffi::c_int as usize] = -5 as ::core::ffi::c_int;
    }
    if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-a\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[0x82 as ::core::ffi::c_int as usize] = 'a' as i32;
    }
    if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-surrogate\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[0x83 as ::core::ffi::c_int as usize] = 0xd801 as ::core::ffi::c_int;
    }
    if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-high\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[0x84 as ::core::ffi::c_int as usize] = 0x10101 as ::core::ffi::c_int;
    }
    (*info).data = data;
    (*info).release = None;
    if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"failing-conv\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).convert = Some(
            failing_converter
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >;
    } else if crate::stdlib::strcmp(
        encoding as *const ::core::ffi::c_char,
        b"prefix-conv\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).convert = Some(
            prefix_converter
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >;
    } else {
        (*info).convert = None;
    }
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn long_encoding_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut encoding: *const crate::expat_external_h::XML_Char,
    mut info: *mut crate::expat_h::XML_Encoding,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        (*info).map[i as usize] = i;
        i += 1;
    }
    (*info).data = crate::__stddef_null_h::NULL;
    (*info).convert = None;
    (*info).release = None;
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn user_data_checking_unknown_encoding_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut encoding: *const crate::expat_external_h::XML_Char,
    mut info: *mut crate::expat_h::XML_Encoding,
) -> ::core::ffi::c_int {
    let number: crate::stdlib::intptr_t = userData as crate::stdlib::intptr_t;
    if !(number == 0xc0ffee as ::core::ffi::c_int as crate::stdlib::intptr_t) {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            416 as ::core::ffi::c_int,
            b"check failed: number == 0xC0FFEE\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return long_encoding_handler(userData, encoding, info);
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_optioner(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut options: *mut crate::src::tests::handlers::ExtOption =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtOption;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    while !(*options).parse_text.is_null() {
        if crate::stdlib::strcmp(
            systemId as *const ::core::ffi::c_char,
            (*options).system_id as *const ::core::ffi::c_char,
        ) == 0
        {
            let mut rc: crate::expat_h::XML_Status = crate::expat_h::XML_STATUS_ERROR;
            ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
                parser,
                context,
                ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
            );
            if ext_parser.is_null() {
                return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
            }
            rc = crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
                ext_parser,
                (*options).parse_text,
                crate::stdlib::strlen((*options).parse_text) as ::core::ffi::c_int,
                crate::expat_h::XML_TRUE as ::core::ffi::c_int,
            );
            crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
            return rc as ::core::ffi::c_int;
        }
        options = options.offset(1);
    }
    crate::src::tests::minicheck::_fail(
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        444 as ::core::ffi::c_int,
        b"No suitable option found\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_loader(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut test_data: *mut crate::src::tests::common::ExtTest =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::common::ExtTest;
    let mut extparser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    extparser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if extparser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            460 as ::core::ffi::c_int,
            b"Could not create external entity parser.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(*test_data).encoding.is_null() {
        if crate::src::lib::xmlparse::XML_SetEncoding(extparser, (*test_data).encoding) as u64 == 0
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                463 as ::core::ffi::c_int,
                b"XML_SetEncoding() ignored for external entity\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        extparser,
        (*test_data).parse_text,
        crate::stdlib::strlen((*test_data).parse_text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            extparser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            468 as ::core::ffi::c_int,
        );
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    crate::src::lib::xmlparse::XML_ParserFree(extparser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_faulter(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut fault: *mut crate::src::tests::handlers::ExtFaults =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtFaults;
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            487 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(*fault).encoding.is_null() {
        if crate::src::lib::xmlparse::XML_SetEncoding(ext_parser, (*fault).encoding) as u64 == 0 {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                490 as ::core::ffi::c_int,
                b"XML_SetEncoding failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        (*fault).parse_text,
        crate::stdlib::strlen((*fault).parse_text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            495 as ::core::ffi::c_int,
            (*fault).fail_text,
        );
    }
    if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != (*fault).error as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            497 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_null_loader(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_resetter(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut status: crate::expat_h::XML_ParsingStatus = crate::expat_h::XML_ParsingStatus {
        parsing: crate::expat_h::XML_INITIALIZED,
        finalBuffer: 0,
    };
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            528 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_GetParsingStatus(
        ext_parser,
        &raw mut status as *mut _ as *mut crate::expat_h::XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        != crate::expat_h::XML_INITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            531 as ::core::ffi::c_int,
            b"Parsing status is not INITIALIZED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            536 as ::core::ffi::c_int,
        );
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    crate::src::lib::xmlparse::XML_GetParsingStatus(
        ext_parser,
        &raw mut status as *mut _ as *mut crate::expat_h::XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        != crate::expat_h::XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            541 as ::core::ffi::c_int,
            b"Parsing status is not FINISHED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            547 as ::core::ffi::c_int,
            b"Parsing when finished not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            549 as ::core::ffi::c_int,
            b"Parsing when finished faulted with wrong code\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_ParserReset(
        ext_parser,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    crate::src::lib::xmlparse::XML_GetParsingStatus(
        ext_parser,
        &raw mut status as *mut _ as *mut crate::expat_h::XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        != crate::expat_h::XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            553 as ::core::ffi::c_int,
            b"Parsing status not still FINISHED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn entity_suspending_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut model: *mut crate::expat_h::XML_Content,
) {
    let mut ext_parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    if crate::src::lib::xmlparse::XML_StopParser(ext_parser, crate::expat_h::XML_TRUE)
        as ::core::ffi::c_uint
        != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            567 as ::core::ffi::c_int,
            b"Attempting to suspend a subordinate parser not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_SUSPEND_PE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            569 as ::core::ffi::c_int,
            b"Suspending subordinate parser get wrong code\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_SetElementDeclHandler(
        ext_parser,
        None as ::std::option::Option<
            unsafe extern "C" fn(
                _: *mut ::std::ffi::c_void,
                _: *const i8,
                _: *mut crate::expat_h::XML_cp,
            ) -> (),
        >,
    );
    crate::src::lib::xmlparse::XML_FreeContentModel(
        crate::src::tests::common::g_parser,
        model as *mut crate::expat_h::XML_cp,
    );
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_suspender(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            586 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_SetElementDeclHandler(
        ext_parser,
        ::core::mem::transmute(Some(
            entity_suspending_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const crate::expat_external_h::XML_Char,
                    *mut crate::expat_h::XML_Content,
                ) -> (),
        )),
    );
    crate::src::lib::xmlparse::XML_SetUserData(ext_parser, ext_parser as *mut ::core::ffi::c_void);
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            591 as ::core::ffi::c_int,
        );
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn entity_suspending_xdecl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut version: *const crate::expat_external_h::XML_Char,
    mut encoding: *const crate::expat_external_h::XML_Char,
    mut standalone: ::core::ffi::c_int,
) {
    let mut ext_parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    crate::src::lib::xmlparse::XML_StopParser(ext_parser, crate::src::tests::common::g_resumable);
    crate::src::lib::xmlparse::XML_SetXmlDeclHandler(ext_parser, None);
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_suspend_xmldecl(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='us-ascii'?>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut status: crate::expat_h::XML_ParsingStatus = crate::expat_h::XML_ParsingStatus {
        parsing: crate::expat_h::XML_INITIALIZED,
        finalBuffer: 0,
    };
    let mut rc: crate::expat_h::XML_Status = crate::expat_h::XML_STATUS_ERROR;
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            624 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_SetXmlDeclHandler(
        ext_parser,
        Some(
            entity_suspending_xdecl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const crate::expat_external_h::XML_Char,
                    *const crate::expat_external_h::XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    crate::src::lib::xmlparse::XML_SetUserData(ext_parser, ext_parser as *mut ::core::ffi::c_void);
    rc = crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    );
    crate::src::lib::xmlparse::XML_GetParsingStatus(
        ext_parser,
        &raw mut status as *mut _ as *mut crate::expat_h::XML_ParsingStatus,
    );
    if crate::src::tests::common::g_resumable != 0 {
        if rc as ::core::ffi::c_uint
            == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                631 as ::core::ffi::c_int,
            );
        }
        if status.parsing as ::core::ffi::c_uint
            != crate::expat_h::XML_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                633 as ::core::ffi::c_int,
                b"Ext Parsing status not SUSPENDED\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    } else {
        if rc as ::core::ffi::c_uint
            != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                636 as ::core::ffi::c_int,
                b"Ext parsing not aborted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_ABORTED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                638 as ::core::ffi::c_int,
            );
        }
        if status.parsing as ::core::ffi::c_uint
            != crate::expat_h::XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                640 as ::core::ffi::c_int,
                b"Ext Parsing status not FINISHED\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_suspending_faulter(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut fault: *mut crate::src::tests::handlers::ExtFaults =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtFaults;
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut parse_len: ::core::ffi::c_int =
        crate::stdlib::strlen((*fault).parse_text) as ::core::ffi::c_int;
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            662 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_SetXmlDeclHandler(
        ext_parser,
        Some(
            entity_suspending_xdecl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const crate::expat_external_h::XML_Char,
                    *const crate::expat_external_h::XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    crate::src::lib::xmlparse::XML_SetUserData(ext_parser, ext_parser as *mut ::core::ffi::c_void);
    crate::src::tests::common::g_resumable = crate::expat_h::XML_TRUE;
    buffer = crate::src::lib::xmlparse::XML_GetBuffer(ext_parser, parse_len);
    if buffer.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            668 as ::core::ffi::c_int,
            b"Could not allocate parse buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            669 as ::core::ffi::c_uint,
            b"int external_entity_suspending_faulter(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    crate::stdlib::memcpy(
        buffer,
        (*fault).parse_text as *const ::core::ffi::c_void,
        parse_len as crate::__stddef_size_t_h::size_t,
    );
    if crate::src::lib::xmlparse::XML_ParseBuffer(
        ext_parser,
        parse_len,
        crate::expat_h::XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != crate::expat_h::XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            672 as ::core::ffi::c_int,
            b"XML declaration did not suspend\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::lib::xmlparse::XML_ResumeParser(ext_parser) as ::core::ffi::c_uint
        != crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            674 as ::core::ffi::c_int,
        );
    }
    if crate::src::lib::xmlparse::XML_ParseBuffer(
        ext_parser,
        0 as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            676 as ::core::ffi::c_int,
            (*fault).fail_text,
        );
    }
    if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != (*fault).error as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            678 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_failer__if_not_xml_ge(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_cr_catcher(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = b"\r\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            714 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const crate::expat_external_h::XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            718 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_bad_cr_catcher(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = b"<tag>\r\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            735 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const crate::expat_external_h::XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            739 as ::core::ffi::c_int,
            b"Async entity error not caught\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_ASYNC_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            741 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_rsqb_catcher(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = b"<tag>]\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            758 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            rsqb_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const crate::expat_external_h::XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            762 as ::core::ffi::c_int,
            b"Async entity error not caught\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != crate::expat_h::XML_ERROR_ASYNC_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            764 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_good_cdata_ascii(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<a><![CDATA[<greeting>Hello, world!</greeting>]]></a>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut expected: *const crate::expat_external_h::XML_Char =
        b"<greeting>Hello, world!</greeting>\0".as_ptr()
            as *const crate::expat_external_h::XML_Char;
    let mut storage: crate::src::tests::chardata::CharData =
        crate::src::tests::chardata::CharData {
            count: 0,
            data: [0; 2048],
        };
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    crate::src::tests::chardata::CharData_Init(
        &raw mut storage as *mut _ as *mut crate::src::tests::chardata::CharData,
    );
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            784 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_SetUserData(
        ext_parser,
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    crate::src::lib::xmlparse::XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            accumulate_characters
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const crate::expat_external_h::XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            790 as ::core::ffi::c_int,
        );
    }
    crate::src::tests::chardata::CharData_CheckXMLChars(
        &raw mut storage as *mut _ as *mut crate::src::tests::chardata::CharData,
        expected,
    );
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_param_checker(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!-- Subordinate parser -->\n<!ELEMENT doc (#PCDATA)*>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            810 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    g_handler_data = ext_parser as *const ::core::ffi::c_void;
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            814 as ::core::ffi::c_int,
        );
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    g_handler_data = parser as *const ::core::ffi::c_void;
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_ref_param_checker(
    mut parameter: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    if parameter as *mut ::core::ffi::c_void != g_handler_data as *mut ::core::ffi::c_void {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            834 as ::core::ffi::c_int,
            b"External entity ref handler parameter not correct\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        crate::src::tests::common::g_parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            839 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            842 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_param(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM '004-2.ent'>\n<!ENTITY % e2 '%e1;'>\n%e1;\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<!ELEMENT el EMPTY>\n<el/>\n\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    if systemId.is_null() {
        return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            867 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-1.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            ext_parser,
            text1,
            crate::stdlib::strlen(text1) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                872 as ::core::ffi::c_int,
                b"Inner DTD with invalid tag not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_EXTERNAL_ENTITY_HANDLING as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                874 as ::core::ffi::c_int,
            );
        }
    } else if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-2.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            ext_parser,
            text2,
            crate::stdlib::strlen(text2) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                878 as ::core::ffi::c_int,
                b"Invalid tag in external param not rejected\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_SYNTAX as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                880 as ::core::ffi::c_int,
            );
        }
    } else {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            882 as ::core::ffi::c_int,
            b"Unknown system ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_load_ignore(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            901 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            904 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_load_ignore_utf16(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let text: [::core::ffi::c_char; 73] = ::core::mem::transmute::<
        [u8; 73],
        [::core::ffi::c_char; 73],
    >(
        *b"<\0!\0[\0I\0G\0N\0O\0R\0E\0[\0<\0!\0E\0L\0E\0M\0E\0N\0T\0 \0e\0 \0(\0#\0P\0C\0D\0A\0T\0A\0)\0*\0>\0]\0]\0>\0\0",
    );
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            927 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 73]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            930 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_load_ignore_utf16_be(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let text: [::core::ffi::c_char; 73] = ::core::mem::transmute::<
        [u8; 73],
        [::core::ffi::c_char; 73],
    >(
        *b"\0<\0!\0[\0I\0G\0N\0O\0R\0E\0[\0<\0!\0E\0L\0E\0M\0E\0N\0T\0 \0e\0 \0(\0#\0P\0C\0D\0A\0T\0A\0)\0*\0>\0]\0]\0>\0",
    );
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            953 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 73]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            956 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_valuer(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM '004-2.ent'>\n<!ENTITY % e2 '%e1;'>\n%e1;\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    if systemId.is_null() {
        return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            978 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-1.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            ext_parser,
            text1,
            crate::stdlib::strlen(text1) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                982 as ::core::ffi::c_int,
            );
        }
    } else if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-2.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut fault: *mut crate::src::tests::handlers::ExtFaults = *(parser
            as *mut *mut ::core::ffi::c_void)
            as *mut crate::src::tests::handlers::ExtFaults;
        let mut status: crate::expat_h::XML_Status = crate::expat_h::XML_STATUS_ERROR;
        let mut error: crate::expat_h::XML_Error = crate::expat_h::XML_ERROR_NONE;
        status = crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            ext_parser,
            (*fault).parse_text,
            crate::stdlib::strlen((*fault).parse_text) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        );
        if (*fault).error as ::core::ffi::c_uint
            == crate::expat_h::XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if status as ::core::ffi::c_uint
                == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                crate::src::tests::common::_xml_failure(
                    ext_parser,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int,
                );
            }
        } else {
            if status as ::core::ffi::c_uint
                != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                crate::src::tests::minicheck::_fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    995 as ::core::ffi::c_int,
                    (*fault).fail_text,
                );
            }
            error = crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser);
            if error as ::core::ffi::c_uint != (*fault).error as ::core::ffi::c_uint
                && ((*fault).error as ::core::ffi::c_uint
                    != crate::expat_h::XML_ERROR_XML_DECL as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    || error as ::core::ffi::c_uint
                        != crate::expat_h::XML_ERROR_TEXT_DECL as ::core::ffi::c_int
                            as ::core::ffi::c_uint)
            {
                crate::src::tests::common::_xml_failure(
                    ext_parser,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1000 as ::core::ffi::c_int,
                );
            }
        }
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_not_standalone(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM 'bar'>\n%e1;\n\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    if systemId.is_null() {
        return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1024 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"foo\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        crate::src::lib::xmlparse::XML_SetNotStandaloneHandler(
            ext_parser,
            Some(
                reject_not_standalone_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
        );
        if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            ext_parser,
            text1,
            crate::stdlib::strlen(text1) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1029 as ::core::ffi::c_int,
                b"Expected not standalone rejection\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_NOT_STANDALONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1031 as ::core::ffi::c_int,
            );
        }
        crate::src::lib::xmlparse::XML_SetNotStandaloneHandler(ext_parser, None);
        crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    } else if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"bar\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            ext_parser,
            text2,
            crate::stdlib::strlen(text2) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1038 as ::core::ffi::c_int,
            );
        }
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_value_aborter(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM '004-2.ent'>\n<!ENTITY % e2 '%e1;'>\n%e1;\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='utf-8'?>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    if systemId.is_null() {
        return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1062 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-1.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            ext_parser,
            text1,
            crate::stdlib::strlen(text1) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1066 as ::core::ffi::c_int,
            );
        }
    }
    if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-2.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        crate::src::lib::xmlparse::XML_SetXmlDeclHandler(
            ext_parser,
            Some(
                entity_suspending_xdecl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const crate::expat_external_h::XML_Char,
                        *const crate::expat_external_h::XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        crate::src::lib::xmlparse::XML_SetUserData(
            ext_parser,
            ext_parser as *mut ::core::ffi::c_void,
        );
        if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            ext_parser,
            text2,
            crate::stdlib::strlen(text2) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1073 as ::core::ffi::c_int,
                b"Aborted parse not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if crate::src::lib::xmlparse::XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_ABORTED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1075 as ::core::ffi::c_int,
            );
        }
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_public(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        *(parser as *mut *mut ::core::ffi::c_void) as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<!ATTLIST doc a CDATA 'value'>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut parse_res: ::core::ffi::c_int = 0;
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    if !systemId.is_null()
        && crate::stdlib::strcmp(
            systemId as *const ::core::ffi::c_char,
            b"http://example.org/\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        text = text1;
    } else if !publicId.is_null()
        && crate::stdlib::strcmp(
            publicId as *const ::core::ffi::c_char,
            b"foo\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        text = text2;
    } else {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1101 as ::core::ffi::c_int,
            b"Unexpected parameters to external entity parser\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !text.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"text != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1102 as ::core::ffi::c_uint,
            b"int external_entity_public(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    parse_res = crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return parse_res;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_devaluer(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM 'bar'>\n%e1;\n\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut clear_handler_flag: ::core::ffi::c_int =
        !(*(parser as *mut *mut ::core::ffi::c_void)).is_null() as ::core::ffi::c_int;
    if systemId.is_null()
        || crate::stdlib::strcmp(
            systemId as *const ::core::ffi::c_char,
            b"bar\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
    }
    if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"foo\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1124 as ::core::ffi::c_int,
            b"Unexpected system ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1127 as ::core::ffi::c_int,
            b"Could note create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if clear_handler_flag != 0 {
        crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler(ext_parser, None);
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1132 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_oneshot_loader(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut test_data: *mut crate::src::tests::handlers::ExtHdlrData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtHdlrData;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1150 as ::core::ffi::c_int,
            b"Could not create external entity parser.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_SetExternalEntityRefHandler(ext_parser, (*test_data).handler);
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        (*test_data).parse_text,
        crate::stdlib::strlen((*test_data).parse_text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1156 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_loader2(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut test_data: *mut crate::src::tests::handlers::ExtTest2 =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtTest2;
    let mut extparser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    extparser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if extparser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1175 as ::core::ffi::c_int,
            b"Coulr not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(*test_data).encoding.is_null() {
        if crate::src::lib::xmlparse::XML_SetEncoding(extparser, (*test_data).encoding) as u64 == 0
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1178 as ::core::ffi::c_int,
                b"XML_SetEncoding() ignored for external entity\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        extparser,
        (*test_data).parse_text,
        (*test_data).parse_len,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            extparser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1183 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(extparser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_faulter2(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut test_data: *mut crate::src::tests::handlers::ExtFaults2 =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtFaults2;
    let mut extparser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    extparser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if extparser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1202 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(*test_data).encoding.is_null() {
        if crate::src::lib::xmlparse::XML_SetEncoding(extparser, (*test_data).encoding) as u64 == 0
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1205 as ::core::ffi::c_int,
                b"XML_SetEncoding() ignored for external entity\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        extparser,
        (*test_data).parse_text,
        (*test_data).parse_len,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1210 as ::core::ffi::c_int,
            (*test_data).fail_text,
        );
    }
    if crate::src::lib::xmlparse::XML_GetErrorCode(extparser) as ::core::ffi::c_uint
        != (*test_data).error as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            extparser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1212 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(extparser);
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_unfinished_attlist(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = b"<!ELEMENT barf ANY>\n<!ATTLIST barf my_attr (blah|%blah;a|foo) #REQUIRED>\n<!--COMMENT-->\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    if systemId.is_null() {
        return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1235 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1239 as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_handler(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut user_data: *mut ::core::ffi::c_void = *(parser as *mut *mut ::core::ffi::c_void);
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p2: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    if user_data.is_null() {
        text = b"<!ELEMENT doc (e+)>\n<!ATTLIST doc xmlns CDATA #IMPLIED>\n<!ELEMENT e EMPTY>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else {
        text = b"<?xml version='1.0' encoding='us-ascii'?><e/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    }
    crate::src::lib::xmlparse::XML_SetUserData(parser, parser as *mut ::core::ffi::c_void);
    p2 = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        p2,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            p2,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1269 as ::core::ffi::c_int,
        );
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    crate::src::lib::xmlparse::XML_ParserFree(p2);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_duff_loader(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut new_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut i: ::core::ffi::c_uint = 0;
    let max_alloc_count: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while i < max_alloc_count {
        crate::src::tests::common::g_allocation_count = i as ::core::ffi::c_int;
        new_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        );
        if !new_parser.is_null() {
            crate::src::lib::xmlparse::XML_ParserFree(new_parser);
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if i == 0 as ::core::ffi::c_uint {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1297 as ::core::ffi::c_int,
            b"External parser creation ignored failing allocator\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1299 as ::core::ffi::c_int,
            b"Extern parser not created with max allocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    crate::src::tests::common::g_allocation_count = crate::common_h::ALLOC_ALWAYS_SUCCEED;
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_dbl_handler(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut pcallno: *mut ::core::ffi::c_int =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut ::core::ffi::c_int;
    let mut callno: ::core::ffi::c_int = *pcallno;
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut new_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    if callno == 0 as ::core::ffi::c_int {
        text = b"<!ELEMENT doc (e+)>\n<!ATTLIST doc xmlns CDATA #IMPLIED>\n<!ELEMENT e EMPTY>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        crate::src::tests::common::g_allocation_count = 10000 as ::core::ffi::c_int;
        new_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        );
        if new_parser.is_null() {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1330 as ::core::ffi::c_int,
                b"Unable to allocate first external parser\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        *pcallno = 10000 as ::core::ffi::c_int - crate::src::tests::common::g_allocation_count;
    } else {
        text = b"<?xml version='1.0' encoding='us-ascii'?><e/>\0".as_ptr()
            as *const ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            crate::src::tests::common::g_allocation_count = callno + i;
            new_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
                parser,
                context,
                ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
            );
            if !new_parser.is_null() {
                break;
            }
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1346 as ::core::ffi::c_int,
                b"Second external parser unexpectedly created\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1350 as ::core::ffi::c_int,
                b"Second external parser not created\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    crate::src::tests::common::g_allocation_count = crate::common_h::ALLOC_ALWAYS_SUCCEED;
    if crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        new_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::src::tests::common::_xml_failure(
            new_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1358 as ::core::ffi::c_int,
        );
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    crate::src::lib::xmlparse::XML_ParserFree(new_parser);
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_dbl_handler_2(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut pcallno: *mut ::core::ffi::c_int =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut ::core::ffi::c_int;
    let mut callno: ::core::ffi::c_int = *pcallno;
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut new_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut rv: crate::expat_h::XML_Status = crate::expat_h::XML_STATUS_ERROR;
    if callno == 0 as ::core::ffi::c_int {
        text = b"<!ELEMENT doc (e+)>\n<!ATTLIST doc xmlns CDATA #IMPLIED>\n<!ELEMENT e EMPTY>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        *pcallno = 1 as ::core::ffi::c_int;
        new_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        );
        if new_parser.is_null() {
            return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
        }
        rv = crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            new_parser,
            text,
            crate::stdlib::strlen(text) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        );
    } else {
        text = b"<?xml version='1.0' encoding='us-ascii'?><e/>\0".as_ptr()
            as *const ::core::ffi::c_char;
        new_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        );
        if new_parser.is_null() {
            return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
        }
        rv = crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
            new_parser,
            text,
            crate::stdlib::strlen(text) as ::core::ffi::c_int,
            crate::expat_h::XML_TRUE as ::core::ffi::c_int,
        );
    }
    crate::src::lib::xmlparse::XML_ParserFree(new_parser);
    if rv as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_alloc_set_encoding(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<?xml encoding='iso-8859-3'?>\xC3\xA9\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut status: crate::expat_h::XML_Status = crate::expat_h::XML_STATUS_ERROR;
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    if crate::src::lib::xmlparse::XML_SetEncoding(
        ext_parser,
        b"utf-8\0".as_ptr() as *const crate::expat_external_h::XML_Char,
    ) as u64
        == 0
    {
        crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    status = crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    );
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    if status as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_reallocator(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = crate::src::tests::common::get_buffer_test_text;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut status: crate::expat_h::XML_Status = crate::expat_h::XML_STATUS_ERROR;
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1446 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::tests::common::g_reallocation_count =
        *(*(parser as *mut *mut ::core::ffi::c_void) as *mut ::core::ffi::c_int);
    buffer = crate::src::lib::xmlparse::XML_GetBuffer(ext_parser, 1536 as ::core::ffi::c_int);
    if buffer.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1451 as ::core::ffi::c_int,
            b"Buffer allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1452 as ::core::ffi::c_uint,
            b"int external_entity_reallocator(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    crate::stdlib::memcpy(
        buffer,
        text as *const ::core::ffi::c_void,
        crate::stdlib::strlen(text),
    );
    status = crate::src::lib::xmlparse::XML_ParseBuffer(
        ext_parser,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_FALSE as ::core::ffi::c_int,
    );
    crate::src::tests::common::g_reallocation_count = -1 as ::core::ffi::c_int;
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return if status as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int
    } else {
        crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int
    };
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_alloc(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        *(parser as *mut *mut ::core::ffi::c_void) as *const ::core::ffi::c_char;
    let mut ext_parser: crate::expat_h::XML_Parser =
        ::core::ptr::null_mut::<crate::expat_h::XML_ParserStruct>();
    let mut parse_res: ::core::ffi::c_int = 0;
    ext_parser = crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
    );
    if ext_parser.is_null() {
        return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    parse_res = crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        crate::stdlib::strlen(text) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    crate::src::lib::xmlparse::XML_ParserFree(ext_parser);
    return parse_res;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_parser_create_alloc_fail_handler(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    if !context.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1491 as ::core::ffi::c_int,
            b"Unexpected non-NULL context\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    crate::src::tests::common::g_allocation_count = 3 as ::core::ffi::c_int;
    let encodingName: *const crate::expat_external_h::XML_Char =
        b"UTF-8\0".as_ptr() as *const crate::expat_external_h::XML_Char;
    let ext_parser: crate::expat_h::XML_Parser =
        crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(parser, context, encodingName)
            as crate::expat_h::XML_Parser;
    if !ext_parser.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1503 as ::core::ffi::c_int,
            b"Call to XML_ExternalEntityParserCreate was expected to fail out-of-memory\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    crate::src::tests::common::g_allocation_count = crate::common_h::ALLOC_ALWAYS_SUCCEED;
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn accounting_external_entity_ref_handler(
    mut parser: crate::expat_h::XML_Parser,
    mut context: *const crate::expat_external_h::XML_Char,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
) -> ::core::ffi::c_int {
    let testCase: *const crate::src::tests::handlers::AccountingTestCase = *(parser
        as *mut *mut ::core::ffi::c_void)
        as *const crate::src::tests::handlers::AccountingTestCase;
    let mut externalText: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"first.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        externalText = (*testCase).firstExternalText;
    } else if crate::stdlib::strcmp(
        systemId as *const ::core::ffi::c_char,
        b"second.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        externalText = (*testCase).secondExternalText;
    } else {
        if (b"systemId is neither \"first.ent\" nor \"second.ent\"\0".as_ptr()
            as *const ::core::ffi::c_char)
            .is_null()
        {
        } else {
            crate::stdlib::__assert_fail(
                b"! \"systemId is neither \\\"first.ent\\\" nor \\\"second.ent\\\"\"\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1528 as ::core::ffi::c_uint,
                b"int accounting_external_entity_ref_handler(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        };
    }
    if !externalText.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"externalText\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1530 as ::core::ffi::c_uint,
            b"int accounting_external_entity_ref_handler(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    let mut entParser: crate::expat_h::XML_Parser =
        crate::src::lib::xmlparse::XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<crate::expat_external_h::XML_Char>(),
        );
    if !entParser.is_null() {
    } else {
        crate::stdlib::__assert_fail(
            b"entParser\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1533 as ::core::ffi::c_uint,
            b"int accounting_external_entity_ref_handler(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    let status: crate::expat_h::XML_Status = crate::src::tests::common::_XML_Parse_SINGLE_BYTES(
        entParser,
        externalText,
        crate::stdlib::strlen(externalText) as ::core::ffi::c_int,
        crate::expat_h::XML_TRUE as ::core::ffi::c_int,
    ) as crate::expat_h::XML_Status;
    crate::src::lib::xmlparse::XML_ParserFree(entParser);
    return status as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn reject_not_standalone_handler(
    mut userData: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn accept_not_standalone_handler(
    mut userData: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn verify_attlist_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut element_name: *const crate::expat_external_h::XML_Char,
    mut attr_name: *const crate::expat_external_h::XML_Char,
    mut attr_type: *const crate::expat_external_h::XML_Char,
    mut default_value: *const crate::expat_external_h::XML_Char,
    mut is_required: ::core::ffi::c_int,
) {
    let mut at: *mut crate::src::tests::handlers::AttTest =
        userData as *mut crate::src::tests::handlers::AttTest;
    if crate::stdlib::strcmp(
        element_name as *const ::core::ffi::c_char,
        (*at).element_name as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1566 as ::core::ffi::c_int,
            b"Unexpected element name in attribute declaration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if crate::stdlib::strcmp(
        attr_name as *const ::core::ffi::c_char,
        (*at).attr_name as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1568 as ::core::ffi::c_int,
            b"Unexpected attribute name in attribute declaration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if crate::stdlib::strcmp(
        attr_type as *const ::core::ffi::c_char,
        (*at).attr_type as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1570 as ::core::ffi::c_int,
            b"Unexpected attribute type in attribute declaration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if default_value.is_null() && !(*at).default_value.is_null()
        || !default_value.is_null() && (*at).default_value.is_null()
        || !default_value.is_null()
            && crate::stdlib::strcmp(
                default_value as *const ::core::ffi::c_char,
                (*at).default_value as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1575 as ::core::ffi::c_int,
            b"Unexpected default value in attribute declaration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if is_required != (*at).is_required {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1577 as ::core::ffi::c_int,
            b"Requirement mismatch in attribute declaration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn clearing_aborting_character_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    crate::src::lib::xmlparse::XML_StopParser(
        crate::src::tests::common::g_parser,
        crate::src::tests::common::g_resumable,
    );
    crate::src::lib::xmlparse::XML_SetCharacterDataHandler(
        crate::src::tests::common::g_parser,
        None,
    );
}
#[no_mangle]

pub unsafe extern "C" fn parser_stop_character_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut status: crate::expat_h::XML_ParsingStatus = crate::expat_h::XML_ParsingStatus {
        parsing: crate::expat_h::XML_INITIALIZED,
        finalBuffer: 0,
    };
    crate::src::lib::xmlparse::XML_GetParsingStatus(
        crate::src::tests::common::g_parser,
        &raw mut status as *mut _ as *mut crate::expat_h::XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        == crate::expat_h::XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    crate::src::lib::xmlparse::XML_StopParser(
        crate::src::tests::common::g_parser,
        crate::src::tests::common::g_resumable,
    );
    crate::src::lib::xmlparse::XML_SetCharacterDataHandler(
        crate::src::tests::common::g_parser,
        None,
    );
    if crate::src::tests::common::g_resumable == 0 {
        if crate::src::lib::xmlparse::XML_StopParser(
            crate::src::tests::common::g_parser,
            crate::expat_h::XML_FALSE,
        ) as ::core::ffi::c_uint
            != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1607 as ::core::ffi::c_int,
                b"Aborting aborted parser not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if crate::src::lib::xmlparse::XML_GetErrorCode(crate::src::tests::common::g_parser)
            as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                crate::src::tests::common::g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1609 as ::core::ffi::c_int,
            );
        }
    } else if crate::src::tests::common::g_abortable != 0 {
        if crate::src::lib::xmlparse::XML_StopParser(
            crate::src::tests::common::g_parser,
            crate::expat_h::XML_FALSE,
        ) as ::core::ffi::c_uint
            == crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                crate::src::tests::common::g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1613 as ::core::ffi::c_int,
            );
        }
    } else {
        if crate::src::lib::xmlparse::XML_StopParser(
            crate::src::tests::common::g_parser,
            crate::expat_h::XML_TRUE,
        ) as ::core::ffi::c_uint
            != crate::expat_h::XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::minicheck::_fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1617 as ::core::ffi::c_int,
                b"Suspending suspended parser not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if crate::src::lib::xmlparse::XML_GetErrorCode(crate::src::tests::common::g_parser)
            as ::core::ffi::c_uint
            != crate::expat_h::XML_ERROR_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::src::tests::common::_xml_failure(
                crate::src::tests::common::g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1619 as ::core::ffi::c_int,
            );
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn cr_cdata_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut pfound: *mut ::core::ffi::c_int = userData as *mut ::core::ffi::c_int;
    if len == 1 as ::core::ffi::c_int
        && (*s as ::core::ffi::c_int == '\n' as i32 || *s as ::core::ffi::c_int == '\r' as i32)
    {
        *pfound = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]

pub unsafe extern "C" fn rsqb_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut pfound: *mut ::core::ffi::c_int = userData as *mut ::core::ffi::c_int;
    if len == 1 as ::core::ffi::c_int && *s as ::core::ffi::c_int == ']' as i32 {
        *pfound = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]

pub unsafe extern "C" fn byte_character_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut offset: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut buffer: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut data: *mut crate::src::tests::handlers::ByteTestData =
        userData as *mut crate::src::tests::handlers::ByteTestData;
    buffer = crate::src::lib::xmlparse::XML_GetInputContext(
        crate::src::tests::common::g_parser,
        &raw mut offset,
        &raw mut size,
    );
    if buffer.is_null() {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1652 as ::core::ffi::c_int,
            b"Failed to get context buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if offset != (*data).start_element_len {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1654 as ::core::ffi::c_int,
            b"Context offset in unexpected position\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if len != (*data).cdata_len {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1656 as ::core::ffi::c_int,
            b"CDATA length reported incorrectly\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if size != (*data).total_string_len {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1658 as ::core::ffi::c_int,
            b"Context size is not full buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::lib::xmlparse::XML_GetCurrentByteIndex(crate::src::tests::common::g_parser)
        != offset as crate::expat_external_h::XML_Index
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1660 as ::core::ffi::c_int,
            b"Character byte index incorrect\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if crate::src::lib::xmlparse::XML_GetCurrentByteCount(crate::src::tests::common::g_parser)
        != len
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1662 as ::core::ffi::c_int,
            b"Character byte count incorrect\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
#[no_mangle]

pub unsafe extern "C" fn ext2_accumulate_characters(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut test_data: *mut crate::src::tests::handlers::ExtTest2 =
        userData as *mut crate::src::tests::handlers::ExtTest2;
    accumulate_characters((*test_data).storage as *mut ::core::ffi::c_void, s, len);
}

unsafe extern "C" fn record_call(
    rec: *mut crate::src::tests::handlers::handler_record_list,
    mut funcname: *const ::core::ffi::c_char,
    arg: ::core::ffi::c_int,
) {
    let max_entries: ::core::ffi::c_int = (::core::mem::size_of::<
        [crate::src::tests::handlers::handler_record_entry; 50],
    >() as usize)
        .wrapping_div(
            ::core::mem::size_of::<crate::src::tests::handlers::handler_record_entry>() as usize,
        ) as ::core::ffi::c_int;
    if !((*rec).count < max_entries) {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1682 as ::core::ffi::c_int,
            b"check failed: rec->count < max_entries\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let c2rust_fresh0 = (*rec).count;
    (*rec).count = (*rec).count + 1;
    let e: *mut crate::src::tests::handlers::handler_record_entry = (&raw mut (*rec).entries
        as *mut crate::src::tests::handlers::handler_record_entry)
        .offset(c2rust_fresh0 as isize)
        as *mut crate::src::tests::handlers::handler_record_entry;
    (*e).name = funcname;
    (*e).arg = arg;
}
#[no_mangle]

pub unsafe extern "C" fn record_default_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    record_call(
        userData as *mut crate::src::tests::handlers::handler_record_list,
        b"record_default_handler\0".as_ptr() as *const ::core::ffi::c_char,
        len,
    );
}
#[no_mangle]

pub unsafe extern "C" fn record_cdata_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    record_call(
        userData as *mut crate::src::tests::handlers::handler_record_list,
        b"record_cdata_handler\0".as_ptr() as *const ::core::ffi::c_char,
        len,
    );
    crate::src::lib::xmlparse::XML_DefaultCurrent(crate::src::tests::common::g_parser);
}
#[no_mangle]

pub unsafe extern "C" fn record_cdata_nodefault_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    record_call(
        userData as *mut crate::src::tests::handlers::handler_record_list,
        b"record_cdata_nodefault_handler\0".as_ptr() as *const ::core::ffi::c_char,
        len,
    );
}
#[no_mangle]

pub unsafe extern "C" fn record_skip_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut entityName: *const crate::expat_external_h::XML_Char,
    mut is_parameter_entity: ::core::ffi::c_int,
) {
    record_call(
        userData as *mut crate::src::tests::handlers::handler_record_list,
        b"record_skip_handler\0".as_ptr() as *const ::core::ffi::c_char,
        is_parameter_entity,
    );
}
#[no_mangle]

pub unsafe extern "C" fn record_element_start_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    crate::src::tests::chardata::CharData_AppendXMLChars(
        userData as *mut crate::src::tests::chardata::CharData
            as *mut crate::src::tests::chardata::CharData,
        name,
        crate::stdlib::strlen(name as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn record_element_end_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
) {
    let mut storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"/\0".as_ptr() as *const crate::expat_external_h::XML_Char,
        1 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn _handler_record_get(
    mut storage: *const crate::src::tests::handlers::handler_record_list,
    mut index: ::core::ffi::c_int,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) -> *const crate::src::tests::handlers::handler_record_entry {
    if (*storage).count <= index {
        crate::src::tests::minicheck::_fail(
            file,
            line,
            b"too few handler calls\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return (&raw const (*storage).entries
        as *const crate::src::tests::handlers::handler_record_entry)
        .offset(index as isize)
        as *const crate::src::tests::handlers::handler_record_entry;
}

static mut entity_name_to_match: *const crate::expat_external_h::XML_Char =
    ::core::ptr::null::<crate::expat_external_h::XML_Char>();

static mut entity_value_to_match: *const crate::expat_external_h::XML_Char =
    ::core::ptr::null::<crate::expat_external_h::XML_Char>();

static mut entity_match_flag: ::core::ffi::c_int =
    crate::src::tests::handlers::ENTITY_MATCH_NOT_FOUND;
#[no_mangle]

pub unsafe extern "C" fn param_entity_match_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut entityName: *const crate::expat_external_h::XML_Char,
    mut is_parameter_entity: ::core::ffi::c_int,
    mut value: *const crate::expat_external_h::XML_Char,
    mut value_length: ::core::ffi::c_int,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
    mut notationName: *const crate::expat_external_h::XML_Char,
) {
    if is_parameter_entity == 0 || entity_name_to_match.is_null() || entity_value_to_match.is_null()
    {
        return;
    }
    if crate::stdlib::strcmp(
        entityName as *const ::core::ffi::c_char,
        entity_name_to_match as *const ::core::ffi::c_char,
    ) == 0
    {
        if value_length
            != crate::stdlib::strlen(entity_value_to_match as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
            || crate::stdlib::strncmp(
                value as *const ::core::ffi::c_char,
                entity_value_to_match as *const ::core::ffi::c_char,
                value_length as crate::__stddef_size_t_h::size_t,
            ) != 0 as ::core::ffi::c_int
        {
            entity_match_flag = crate::src::tests::handlers::ENTITY_MATCH_FAIL;
        } else {
            entity_match_flag = crate::src::tests::handlers::ENTITY_MATCH_SUCCESS;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn param_entity_match_init(
    mut name: *const crate::expat_external_h::XML_Char,
    mut value: *const crate::expat_external_h::XML_Char,
) {
    entity_name_to_match = name;
    entity_value_to_match = value;
    entity_match_flag = crate::src::tests::handlers::ENTITY_MATCH_NOT_FOUND;
}
#[no_mangle]

pub unsafe extern "C" fn get_param_entity_match_flag() -> ::core::ffi::c_int {
    return entity_match_flag;
}
#[no_mangle]

pub unsafe extern "C" fn xml_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut version: *const crate::expat_external_h::XML_Char,
    mut encoding: *const crate::expat_external_h::XML_Char,
    mut standalone: ::core::ffi::c_int,
) {
    if userData != g_handler_data as *mut ::core::ffi::c_void {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1794 as ::core::ffi::c_int,
            b"User data (xml decl) not correctly set\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if standalone != -1 as ::core::ffi::c_int {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1796 as ::core::ffi::c_int,
            b"Standalone not flagged as not present in XML decl\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    g_xdecl_count += 1;
}
#[no_mangle]

pub unsafe extern "C" fn param_check_skip_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut entityName: *const crate::expat_external_h::XML_Char,
    mut is_parameter_entity: ::core::ffi::c_int,
) {
    if userData != g_handler_data as *mut ::core::ffi::c_void {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1806 as ::core::ffi::c_int,
            b"User data (skip) not correctly set\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    g_skip_count += 1;
}
#[no_mangle]

pub unsafe extern "C" fn data_check_comment_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut data: *const crate::expat_external_h::XML_Char,
) {
    if userData != g_handler_data as *mut ::core::ffi::c_void {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1815 as ::core::ffi::c_int,
            b"User data (parser) not correctly set\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if *(userData as *mut *mut ::core::ffi::c_void)
        != 1 as ::core::ffi::c_int as *mut ::core::ffi::c_void
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1818 as ::core::ffi::c_int,
            b"User data in parser not correctly set\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    g_comment_count += 1;
}
#[no_mangle]

pub unsafe extern "C" fn selective_aborting_default_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let trigger_char: crate::expat_external_h::XML_Char =
        *(userData as *const crate::expat_external_h::XML_Char);
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        if *s.offset(i as isize) as ::core::ffi::c_int == trigger_char as ::core::ffi::c_int {
            found = 1 as ::core::ffi::c_int;
            break;
        } else {
            i += 1;
        }
    }
    if found != 0 {
        crate::src::lib::xmlparse::XML_StopParser(
            crate::src::tests::common::g_parser,
            crate::src::tests::common::g_resumable,
        );
        crate::src::lib::xmlparse::XML_SetDefaultHandler(crate::src::tests::common::g_parser, None);
    }
}
#[no_mangle]

pub unsafe extern "C" fn suspending_comment_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut data: *const crate::expat_external_h::XML_Char,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    crate::src::lib::xmlparse::XML_StopParser(parser, crate::expat_h::XML_TRUE);
}
#[no_mangle]

pub unsafe extern "C" fn element_decl_suspender(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut model: *mut crate::expat_h::XML_Content,
) {
    crate::src::lib::xmlparse::XML_StopParser(
        crate::src::tests::common::g_parser,
        crate::expat_h::XML_TRUE,
    );
    crate::src::lib::xmlparse::XML_FreeContentModel(
        crate::src::tests::common::g_parser,
        model as *mut crate::expat_h::XML_cp,
    );
}
#[no_mangle]

pub unsafe extern "C" fn suspend_after_element_declaration(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut model: *mut crate::expat_h::XML_Content,
) {
    let mut parser: crate::expat_h::XML_Parser = userData as crate::expat_h::XML_Parser;
    if !(crate::src::lib::xmlparse::XML_StopParser(
        parser,
        1 as ::core::ffi::c_int as crate::expat_h::XML_Bool,
    ) as ::core::ffi::c_uint
        == crate::expat_h::XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        crate::src::tests::minicheck::_fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1861 as ::core::ffi::c_int,
            b"check failed: XML_StopParser(parser, XML_TRUE) == XML_STATUS_OK\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    crate::src::lib::xmlparse::XML_FreeContentModel(parser, model as *mut crate::expat_h::XML_cp);
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_pi_characters(
    mut userData: *mut ::core::ffi::c_void,
    mut target: *const crate::expat_external_h::XML_Char,
    mut data: *const crate::expat_external_h::XML_Char,
) {
    let mut storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        target,
        -1 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b": \0".as_ptr() as *const crate::expat_external_h::XML_Char,
        2 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        data,
        -1 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"\n\0".as_ptr() as *const crate::expat_external_h::XML_Char,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_comment(
    mut userData: *mut ::core::ffi::c_void,
    mut data: *const crate::expat_external_h::XML_Char,
) {
    let mut storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        data,
        -1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_entity_decl(
    mut userData: *mut ::core::ffi::c_void,
    mut entityName: *const crate::expat_external_h::XML_Char,
    mut is_parameter_entity: ::core::ffi::c_int,
    mut value: *const crate::expat_external_h::XML_Char,
    mut value_length: ::core::ffi::c_int,
    mut base: *const crate::expat_external_h::XML_Char,
    mut systemId: *const crate::expat_external_h::XML_Char,
    mut publicId: *const crate::expat_external_h::XML_Char,
    mut notationName: *const crate::expat_external_h::XML_Char,
) {
    let mut storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        entityName,
        -1 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"=\0".as_ptr() as *const crate::expat_external_h::XML_Char,
        1 as ::core::ffi::c_int,
    );
    if value.is_null() {
        crate::src::tests::chardata::CharData_AppendXMLChars(
            storage as *mut crate::src::tests::chardata::CharData,
            b"(null)\0".as_ptr() as *const crate::expat_external_h::XML_Char,
            -1 as ::core::ffi::c_int,
        );
    } else {
        crate::src::tests::chardata::CharData_AppendXMLChars(
            storage as *mut crate::src::tests::chardata::CharData,
            value,
            value_length,
        );
    }
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"\n\0".as_ptr() as *const crate::expat_external_h::XML_Char,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_char_data_and_suspend(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let parserPlusStorage: *mut crate::src::tests::handlers::ParserPlusStorage =
        userData as *mut crate::src::tests::handlers::ParserPlusStorage;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        (*parserPlusStorage).storage as *mut crate::src::tests::chardata::CharData,
        s,
        len,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        if *s.offset(i as isize) as ::core::ffi::c_int == 'Z' as i32 {
            crate::src::lib::xmlparse::XML_StopParser(
                (*parserPlusStorage).parser,
                crate::expat_h::XML_TRUE,
            );
            break;
        } else {
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_start_element(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b"(\0".as_ptr() as *const crate::expat_external_h::XML_Char,
        1 as ::core::ffi::c_int,
    );
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
    if !atts.is_null() && !(*atts.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        crate::src::tests::chardata::CharData_AppendXMLChars(
            storage as *mut crate::src::tests::chardata::CharData,
            b"(\0".as_ptr() as *const crate::expat_external_h::XML_Char,
            1 as ::core::ffi::c_int,
        );
        while !(*atts.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            crate::src::tests::chardata::CharData_AppendXMLChars(
                storage as *mut crate::src::tests::chardata::CharData,
                *atts.offset(0 as ::core::ffi::c_int as isize),
                -1 as ::core::ffi::c_int,
            );
            crate::src::tests::chardata::CharData_AppendXMLChars(
                storage as *mut crate::src::tests::chardata::CharData,
                b"=\0".as_ptr() as *const crate::expat_external_h::XML_Char,
                1 as ::core::ffi::c_int,
            );
            crate::src::tests::chardata::CharData_AppendXMLChars(
                storage as *mut crate::src::tests::chardata::CharData,
                *atts.offset(1 as ::core::ffi::c_int as isize),
                -1 as ::core::ffi::c_int,
            );
            atts = atts.offset(2 as ::core::ffi::c_int as isize);
            if !(*atts.offset(0 as ::core::ffi::c_int as isize)).is_null() {
                crate::src::tests::chardata::CharData_AppendXMLChars(
                    storage as *mut crate::src::tests::chardata::CharData,
                    b",\0".as_ptr() as *const crate::expat_external_h::XML_Char,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        crate::src::tests::chardata::CharData_AppendXMLChars(
            storage as *mut crate::src::tests::chardata::CharData,
            b")\0".as_ptr() as *const crate::expat_external_h::XML_Char,
            1 as ::core::ffi::c_int,
        );
    }
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        b")\n\0".as_ptr() as *const crate::expat_external_h::XML_Char,
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_characters(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    crate::src::tests::chardata::CharData_AppendXMLChars(
        storage as *mut crate::src::tests::chardata::CharData,
        s,
        len,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_attribute(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const crate::expat_external_h::XML_Char,
    mut atts: *mut *const crate::expat_external_h::XML_Char,
) {
    let storage: *mut crate::src::tests::chardata::CharData =
        userData as *mut crate::src::tests::chardata::CharData;
    if atts.is_null() {
        return;
    }
    while (*storage).count < 0 as ::core::ffi::c_int
        && !(*atts.offset(0 as ::core::ffi::c_int as isize)).is_null()
    {
        crate::src::tests::chardata::CharData_AppendXMLChars(
            storage as *mut crate::src::tests::chardata::CharData,
            *atts.offset(1 as ::core::ffi::c_int as isize),
            -1 as ::core::ffi::c_int,
        );
        atts = atts.offset(2 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]

pub unsafe extern "C" fn ext_accumulate_characters(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let test_data: *mut crate::src::tests::common::ExtTest =
        userData as *mut crate::src::tests::common::ExtTest;
    accumulate_characters((*test_data).storage as *mut ::core::ffi::c_void, s, len);
}
#[no_mangle]

pub unsafe extern "C" fn checking_default_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const crate::expat_external_h::XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut data: *mut crate::src::tests::handlers::DefaultCheck =
        userData as *mut crate::src::tests::handlers::DefaultCheck;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !(*data.offset(i as isize)).expected.is_null() {
        if (*data.offset(i as isize)).expectedLen == len
            && crate::stdlib::memcmp(
                (*data.offset(i as isize)).expected as *const ::core::ffi::c_void,
                s as *const ::core::ffi::c_void,
                (len as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::expat_external_h::XML_Char>()
                        as crate::__stddef_size_t_h::size_t),
            ) == 0
        {
            (*data.offset(i as isize)).seen = crate::expat_h::XML_TRUE;
            break;
        } else {
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_and_suspend_comment_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut data: *const crate::expat_external_h::XML_Char,
) {
    let parserPlusStorage: *mut crate::src::tests::handlers::ParserPlusStorage =
        userData as *mut crate::src::tests::handlers::ParserPlusStorage;
    accumulate_comment(
        (*parserPlusStorage).storage as *mut ::core::ffi::c_void,
        data,
    );
    crate::src::lib::xmlparse::XML_StopParser(
        (*parserPlusStorage).parser,
        crate::expat_h::XML_TRUE,
    );
}
