// =============== BEGIN handlers_h ================
pub const STRUCT_START_TAG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const STRUCT_END_TAG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub type AttrInfo = crate::src::tests::handlers::attrInfo;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct attrInfo {
    pub name: *const XML_Char,
    pub value: *const XML_Char,
}

pub type ElementInfo = crate::src::tests::handlers::elementInfo;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct elementInfo {
    pub name: *const XML_Char,
    pub attr_count: ::core::ffi::c_int,
    pub id_name: *const XML_Char,
    pub attributes: *mut crate::src::tests::handlers::AttrInfo,
}

pub type ParserAndElementInfo = crate::src::tests::handlers::StructParserAndElementInfo;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct StructParserAndElementInfo {
    pub parser: XML_Parser,
    pub info: *mut crate::src::tests::handlers::ElementInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DataIssue240 {
    pub parser: XML_Parser,
    pub deep: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ExtOption {
    pub system_id: *const XML_Char,
    pub parse_text: *const ::core::ffi::c_char,
}

pub type ExtFaults = crate::src::tests::handlers::ext_faults;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ext_faults {
    pub parse_text: *const ::core::ffi::c_char,
    pub fail_text: *const ::core::ffi::c_char,
    pub encoding: *const XML_Char,
    pub error: XML_Error,
}

pub type ExtHdlrData = crate::src::tests::handlers::ext_hdlr_data;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ext_hdlr_data {
    pub parse_text: *const ::core::ffi::c_char,
    pub handler: XML_ExternalEntityRefHandler,
    pub storage: *mut CharData,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ExtTest2 {
    pub parse_text: *const ::core::ffi::c_char,
    pub parse_len: ::core::ffi::c_int,
    pub encoding: *const XML_Char,
    pub storage: *mut CharData,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ExtFaults2 {
    pub parse_text: *const ::core::ffi::c_char,
    pub parse_len: ::core::ffi::c_int,
    pub fail_text: *const ::core::ffi::c_char,
    pub encoding: *const XML_Char,
    pub error: XML_Error,
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
    pub element_name: *const XML_Char,
    pub attr_name: *const XML_Char,
    pub attr_type: *const XML_Char,
    pub default_value: *const XML_Char,
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
    pub expected: *const XML_Char,
    pub expectedLen: ::core::ffi::c_int,
    pub seen: XML_Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ParserPlusStorage {
    pub parser: XML_Parser,
    pub storage: *mut CharData,
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
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    CharData_AppendXMLChars(
        userData as *mut CharData
            as *mut CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn end_element_event_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
) {
    let mut storage: *mut CharData =
        userData as *mut CharData;
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"/\0".as_ptr() as *const XML_Char,
        1 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_element_event_handler2(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut attr: *mut *const XML_Char,
) {
    let mut storage: *mut StructData =
        userData as *mut StructData;
    StructData_AddItem(
        storage as *mut StructData,
        name,
        XML_GetCurrentColumnNumber(g_parser)
            as ::core::ffi::c_int,
        XML_GetCurrentLineNumber(g_parser)
            as ::core::ffi::c_int,
        crate::src::tests::handlers::STRUCT_START_TAG,
    );
}
#[no_mangle]

pub unsafe extern "C" fn end_element_event_handler2(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
) {
    let mut storage: *mut StructData =
        userData as *mut StructData;
    StructData_AddItem(
        storage as *mut StructData,
        name,
        XML_GetCurrentColumnNumber(g_parser)
            as ::core::ffi::c_int,
        XML_GetCurrentLineNumber(g_parser)
            as ::core::ffi::c_int,
        crate::src::tests::handlers::STRUCT_END_TAG,
    );
}
#[no_mangle]

pub unsafe extern "C" fn counting_start_element_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
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
        if strcmp(
            name as *const ::core::ffi::c_char,
            (*info).name as *const ::core::ffi::c_char,
        ) == 0
        {
            break;
        }
        info = info.offset(1);
    }
    if (*info).name.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            119 as ::core::ffi::c_int,
            b"Element not recognised\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    count =
        XML_GetSpecifiedAttributeCount((*parserAndElementInfos).parser);
    if (*info).attr_count * 2 as ::core::ffi::c_int != count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            128 as ::core::ffi::c_int,
            b"Not got expected attribute count\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    id = XML_GetIdAttributeIndex((*parserAndElementInfos).parser);
    if id == -1 as ::core::ffi::c_int && !(*info).id_name.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            133 as ::core::ffi::c_int,
            b"ID not present\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if id != -1 as ::core::ffi::c_int
        && strcmp(
            *atts.offset(id as isize) as *const ::core::ffi::c_char,
            (*info).id_name as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
    {
        _fail(
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
            if strcmp(
                *atts.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
                (*attr).name as *const ::core::ffi::c_char,
            ) == 0
            {
                break;
            }
            attr = attr.offset(1);
        }
        if (*attr).name.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                148 as ::core::ffi::c_int,
                b"Attribute not recognised\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if strcmp(
            *atts.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
            (*attr).value as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            _fail(
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
    mut s: *const XML_Char,
) {
    XML_StopParser(
        userData as XML_Parser,
        1 as XML_Bool,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_element_suspender(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    if strcmp(
        name as *const ::core::ffi::c_char,
        b"suspend\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        XML_StopParser(
            g_parser,
            XML_TRUE,
        );
    }
    if strcmp(
        name as *const ::core::ffi::c_char,
        b"abort\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        XML_StopParser(
            g_parser,
            XML_FALSE,
        );
    }
}
#[no_mangle]

pub static mut g_triplet_start_flag: ::core::ffi::c_int =
    XML_FALSE as ::core::ffi::c_int;
#[no_mangle]

pub static mut g_triplet_end_flag: ::core::ffi::c_int =
    XML_FALSE as ::core::ffi::c_int;
#[no_mangle]

pub unsafe extern "C" fn triplet_start_checker(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut elemstr: *mut *mut XML_Char =
        userData as *mut *mut XML_Char;
    let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    if strcmp(
        *elemstr.offset(0 as ::core::ffi::c_int as isize),
        name as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                as size_t,
            b"unexpected start string: '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            193 as ::core::ffi::c_int,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
    if strcmp(
        *elemstr.offset(1 as ::core::ffi::c_int as isize),
        *atts.offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                as size_t,
            b"unexpected attribute string: '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            *atts.offset(0 as ::core::ffi::c_int as isize),
        );
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            198 as ::core::ffi::c_int,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
    g_triplet_start_flag = XML_TRUE as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn triplet_end_checker(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
) {
    let mut elemstr: *mut *mut XML_Char =
        userData as *mut *mut XML_Char;
    if strcmp(
        *elemstr.offset(0 as ::core::ffi::c_int as isize),
        name as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        let mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
        snprintf(
            &raw mut buffer as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                as size_t,
            b"unexpected end string: '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            name,
        );
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            214 as ::core::ffi::c_int,
            &raw mut buffer as *mut ::core::ffi::c_char,
        );
    }
    g_triplet_end_flag = XML_TRUE as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn overwrite_start_checker(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut storage: *mut CharData =
        userData as *mut CharData;
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"start \0".as_ptr() as *const XML_Char,
        6 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
    while !(*atts).is_null() {
        CharData_AppendXMLChars(
            storage as *mut CharData,
            b"\nattribute \0".as_ptr() as *const XML_Char,
            11 as ::core::ffi::c_int,
        );
        CharData_AppendXMLChars(
            storage as *mut CharData,
            *atts,
            -1 as ::core::ffi::c_int,
        );
        atts = atts.offset(2 as ::core::ffi::c_int as isize);
    }
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"\n\0".as_ptr() as *const XML_Char,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn overwrite_end_checker(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
) {
    let mut storage: *mut CharData =
        userData as *mut CharData;
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"end \0".as_ptr() as *const XML_Char,
        4 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"\n\0".as_ptr() as *const XML_Char,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_element_fail(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    _fail(
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        249 as ::core::ffi::c_int,
        b"should never reach start_element_fail()\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_ns_clearing_start_element(
    mut userData: *mut ::core::ffi::c_void,
    mut prefix: *const XML_Char,
    mut uri: *const XML_Char,
) {
    XML_SetStartElementHandler(
        userData as XML_Parser,
        None,
    );
}
#[no_mangle]

pub unsafe extern "C" fn start_element_issue_240(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let mut mydata: *mut crate::src::tests::handlers::DataIssue240 =
        userData as *mut crate::src::tests::handlers::DataIssue240;
    (*mydata).deep += 1;
}
#[no_mangle]

pub unsafe extern "C" fn end_element_issue_240(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
) {
    let mut mydata: *mut crate::src::tests::handlers::DataIssue240 =
        userData as *mut crate::src::tests::handlers::DataIssue240;
    (*mydata).deep -= 1;
    if (*mydata).deep == 0 as ::core::ffi::c_int {
        XML_StopParser((*mydata).parser, 0 as XML_Bool);
    }
}
#[no_mangle]

pub unsafe extern "C" fn UnknownEncodingHandler(
    mut data: *mut ::core::ffi::c_void,
    mut encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> ::core::ffi::c_int {
    if strcmp(
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
        (*info).data = NULL;
        (*info).convert = None;
        (*info).release = None;
        return XML_STATUS_OK as ::core::ffi::c_int;
    }
    return XML_STATUS_ERROR as ::core::ffi::c_int;
}

unsafe extern "C" fn dummy_release(mut data: *mut ::core::ffi::c_void) {}
#[no_mangle]

pub unsafe extern "C" fn UnrecognisedEncodingHandler(
    mut data: *mut ::core::ffi::c_void,
    mut encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> ::core::ffi::c_int {
    (*info).data = NULL;
    (*info).convert = None;
    (*info).release = Some(dummy_release as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
    return XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn unknown_released_encoding_handler(
    mut data: *mut ::core::ffi::c_void,
    mut encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> ::core::ffi::c_int {
    if strcmp(
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
        (*info).data = NULL;
        (*info).convert = None;
        (*info).release =
            Some(dummy_release as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ())
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
        return XML_STATUS_OK as ::core::ffi::c_int;
    }
    return XML_STATUS_ERROR as ::core::ffi::c_int;
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
    mut encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut high_map: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
    if strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-9\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            encoding as *const ::core::ffi::c_char,
            b"ascii-like\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        || strcmp(
            encoding as *const ::core::ffi::c_char,
            b"invalid-len\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        || strcmp(
            encoding as *const ::core::ffi::c_char,
            b"invalid-a\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        || strcmp(
            encoding as *const ::core::ffi::c_char,
            b"invalid-surrogate\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
        || strcmp(
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
    if strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-9\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[9 as ::core::ffi::c_int as usize] = 5 as ::core::ffi::c_int;
    }
    if strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-len\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[0x81 as ::core::ffi::c_int as usize] = -5 as ::core::ffi::c_int;
    }
    if strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-a\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[0x82 as ::core::ffi::c_int as usize] = 'a' as i32;
    }
    if strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-surrogate\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[0x83 as ::core::ffi::c_int as usize] = 0xd801 as ::core::ffi::c_int;
    }
    if strcmp(
        encoding as *const ::core::ffi::c_char,
        b"invalid-high\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        (*info).map[0x84 as ::core::ffi::c_int as usize] = 0x10101 as ::core::ffi::c_int;
    }
    (*info).data = data;
    (*info).release = None;
    if strcmp(
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
    } else if strcmp(
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
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn long_encoding_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        (*info).map[i as usize] = i;
        i += 1;
    }
    (*info).data = NULL;
    (*info).convert = None;
    (*info).release = None;
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn user_data_checking_unknown_encoding_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut encoding: *const XML_Char,
    mut info: *mut XML_Encoding,
) -> ::core::ffi::c_int {
    let number: intptr_t = userData as intptr_t;
    if !(number == 0xc0ffee as ::core::ffi::c_int as intptr_t) {
        _fail(
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
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut options: *mut crate::src::tests::handlers::ExtOption =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtOption;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    while !(*options).parse_text.is_null() {
        if strcmp(
            systemId as *const ::core::ffi::c_char,
            (*options).system_id as *const ::core::ffi::c_char,
        ) == 0
        {
            let mut rc: XML_Status = XML_STATUS_ERROR;
            ext_parser = XML_ExternalEntityParserCreate(
                parser,
                context,
                ::core::ptr::null::<XML_Char>(),
            );
            if ext_parser.is_null() {
                return XML_STATUS_ERROR as ::core::ffi::c_int;
            }
            rc = _XML_Parse_SINGLE_BYTES(
                ext_parser,
                (*options).parse_text,
                strlen((*options).parse_text) as ::core::ffi::c_int,
                XML_TRUE as ::core::ffi::c_int,
            );
            XML_ParserFree(ext_parser);
            return rc as ::core::ffi::c_int;
        }
        options = options.offset(1);
    }
    _fail(
        b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
            as *const ::core::ffi::c_char,
        444 as ::core::ffi::c_int,
        b"No suitable option found\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_loader(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut test_data: *mut ExtTest =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut ExtTest;
    let mut extparser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    extparser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if extparser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            460 as ::core::ffi::c_int,
            b"Could not create external entity parser.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(*test_data).encoding.is_null() {
        if XML_SetEncoding(extparser, (*test_data).encoding) as u64 == 0
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                463 as ::core::ffi::c_int,
                b"XML_SetEncoding() ignored for external entity\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if _XML_Parse_SINGLE_BYTES(
        extparser,
        (*test_data).parse_text,
        strlen((*test_data).parse_text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            extparser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            468 as ::core::ffi::c_int,
        );
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    XML_ParserFree(extparser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_faulter(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut fault: *mut crate::src::tests::handlers::ExtFaults =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtFaults;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            487 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(*fault).encoding.is_null() {
        if XML_SetEncoding(ext_parser, (*fault).encoding) as u64 == 0 {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                490 as ::core::ffi::c_int,
                b"XML_SetEncoding failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        (*fault).parse_text,
        strlen((*fault).parse_text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            495 as ::core::ffi::c_int,
            (*fault).fail_text,
        );
    }
    if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != (*fault).error as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            497 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_null_loader(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_resetter(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut status: XML_ParsingStatus = XML_ParsingStatus { parsing:  XML_INITIALIZED, finalBuffer:  0 };
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            528 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_GetParsingStatus(
        ext_parser,
        &raw mut status as *mut _ as *mut XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        != XML_INITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            531 as ::core::ffi::c_int,
            b"Parsing status is not INITIALIZED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            536 as ::core::ffi::c_int,
        );
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    XML_GetParsingStatus(
        ext_parser,
        &raw mut status as *mut _ as *mut XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        != XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            541 as ::core::ffi::c_int,
            b"Parsing status is not FINISHED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            547 as ::core::ffi::c_int,
            b"Parsing when finished not faulted\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != XML_ERROR_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            549 as ::core::ffi::c_int,
            b"Parsing when finished faulted with wrong code\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_ParserReset(
        ext_parser,
        ::core::ptr::null::<XML_Char>(),
    );
    XML_GetParsingStatus(
        ext_parser,
        &raw mut status as *mut _ as *mut XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        != XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            553 as ::core::ffi::c_int,
            b"Parsing status not still FINISHED\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn entity_suspending_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut model: *mut XML_Content,
) {
    let mut ext_parser: XML_Parser = userData as XML_Parser;
    if XML_StopParser(ext_parser, XML_TRUE)
        as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            567 as ::core::ffi::c_int,
            b"Attempting to suspend a subordinate parser not faulted\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != XML_ERROR_SUSPEND_PE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            569 as ::core::ffi::c_int,
            b"Suspending subordinate parser get wrong code\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_SetElementDeclHandler(
        ext_parser,
        None as ::std::option::Option<
            unsafe extern "C" fn(
                _: *mut ::std::ffi::c_void,
                _: *const i8,
                _: *mut XML_cp,
            ) -> (),
        >,
    );
    XML_FreeContentModel(
        g_parser,
        model as *mut XML_cp,
    );
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_suspender(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            586 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetElementDeclHandler(
        ext_parser,
        ::core::mem::transmute(Some(
            entity_suspending_decl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *mut XML_Content,
                ) -> (),
        )),
    );
    XML_SetUserData(ext_parser, ext_parser as *mut ::core::ffi::c_void);
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            591 as ::core::ffi::c_int,
        );
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn entity_suspending_xdecl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut version: *const XML_Char,
    mut encoding: *const XML_Char,
    mut standalone: ::core::ffi::c_int,
) {
    let mut ext_parser: XML_Parser = userData as XML_Parser;
    XML_StopParser(ext_parser, g_resumable);
    XML_SetXmlDeclHandler(ext_parser, None);
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_suspend_xmldecl(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='us-ascii'?>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut status: XML_ParsingStatus = XML_ParsingStatus { parsing:  XML_INITIALIZED, finalBuffer:  0 };
    let mut rc: XML_Status = XML_STATUS_ERROR;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            624 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetXmlDeclHandler(
        ext_parser,
        Some(
            entity_suspending_xdecl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(ext_parser, ext_parser as *mut ::core::ffi::c_void);
    rc = _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    );
    XML_GetParsingStatus(
        ext_parser,
        &raw mut status as *mut _ as *mut XML_ParsingStatus,
    );
    if g_resumable != 0 {
        if rc as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                631 as ::core::ffi::c_int,
            );
        }
        if status.parsing as ::core::ffi::c_uint
            != XML_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                633 as ::core::ffi::c_int,
                b"Ext Parsing status not SUSPENDED\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    } else {
        if rc as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                636 as ::core::ffi::c_int,
                b"Ext parsing not aborted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != XML_ERROR_ABORTED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                638 as ::core::ffi::c_int,
            );
        }
        if status.parsing as ::core::ffi::c_uint
            != XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                640 as ::core::ffi::c_int,
                b"Ext Parsing status not FINISHED\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_suspending_faulter(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut fault: *mut crate::src::tests::handlers::ExtFaults =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtFaults;
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut parse_len: ::core::ffi::c_int =
        strlen((*fault).parse_text) as ::core::ffi::c_int;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            662 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetXmlDeclHandler(
        ext_parser,
        Some(
            entity_suspending_xdecl_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    XML_SetUserData(ext_parser, ext_parser as *mut ::core::ffi::c_void);
    g_resumable = XML_TRUE;
    buffer = XML_GetBuffer(ext_parser, parse_len);
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            668 as ::core::ffi::c_int,
            b"Could not allocate parse buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            669 as ::core::ffi::c_uint,
            b"int external_entity_suspending_faulter(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    memcpy(
        buffer,
        (*fault).parse_text as *const ::core::ffi::c_void,
        parse_len as size_t,
    );
    if XML_ParseBuffer(
        ext_parser,
        parse_len,
        XML_FALSE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            672 as ::core::ffi::c_int,
            b"XML declaration did not suspend\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_ResumeParser(ext_parser) as ::core::ffi::c_uint
        != XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            674 as ::core::ffi::c_int,
        );
    }
    if XML_ParseBuffer(
        ext_parser,
        0 as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            676 as ::core::ffi::c_int,
            (*fault).fail_text,
        );
    }
    if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != (*fault).error as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            678 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_failer__if_not_xml_ge(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_cr_catcher(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = b"\r\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            714 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            718 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_bad_cr_catcher(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = b"<tag>\r\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            735 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            cr_cdata_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            739 as ::core::ffi::c_int,
            b"Async entity error not caught\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != XML_ERROR_ASYNC_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            741 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_rsqb_catcher(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = b"<tag>]\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            758 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetCharacterDataHandler(
        ext_parser,
        Some(
            rsqb_handler
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *const XML_Char,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            762 as ::core::ffi::c_int,
            b"Async entity error not caught\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
        != XML_ERROR_ASYNC_ENTITY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            764 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_good_cdata_ascii(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<a><![CDATA[<greeting>Hello, world!</greeting>]]></a>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut expected: *const XML_Char =
        b"<greeting>Hello, world!</greeting>\0".as_ptr()
            as *const XML_Char;
    let mut storage: CharData =
        CharData { count:  0, data:  [0; 2048] };
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    CharData_Init(
        &raw mut storage as *mut _ as *mut CharData,
    );
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            784 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetUserData(
        ext_parser,
        &raw mut storage as *mut ::core::ffi::c_void,
    );
    XML_SetCharacterDataHandler(
        ext_parser,
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
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            790 as ::core::ffi::c_int,
        );
    }
    CharData_CheckXMLChars(
        &raw mut storage as *mut _ as *mut CharData,
        expected,
    );
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_param_checker(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!-- Subordinate parser -->\n<!ELEMENT doc (#PCDATA)*>\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            810 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    g_handler_data = ext_parser as *const ::core::ffi::c_void;
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            814 as ::core::ffi::c_int,
        );
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    g_handler_data = parser as *const ::core::ffi::c_void;
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_ref_param_checker(
    mut parameter: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!ELEMENT doc (#PCDATA)*>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    if parameter as *mut ::core::ffi::c_void != g_handler_data as *mut ::core::ffi::c_void {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            834 as ::core::ffi::c_int,
            b"External entity ref handler parameter not correct\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    ext_parser = XML_ExternalEntityParserCreate(
        g_parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            839 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            842 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_param(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM '004-2.ent'>\n<!ENTITY % e2 '%e1;'>\n%e1;\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<!ELEMENT el EMPTY>\n<el/>\n\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    if systemId.is_null() {
        return XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            867 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-1.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if _XML_Parse_SINGLE_BYTES(
            ext_parser,
            text1,
            strlen(text1) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                872 as ::core::ffi::c_int,
                b"Inner DTD with invalid tag not rejected\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != XML_ERROR_EXTERNAL_ENTITY_HANDLING as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                874 as ::core::ffi::c_int,
            );
        }
    } else if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-2.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if _XML_Parse_SINGLE_BYTES(
            ext_parser,
            text2,
            strlen(text2) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                878 as ::core::ffi::c_int,
                b"Invalid tag in external param not rejected\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != XML_ERROR_SYNTAX as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                880 as ::core::ffi::c_int,
            );
        }
    } else {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            882 as ::core::ffi::c_int,
            b"Unknown system ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_load_ignore(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<![IGNORE[<!ELEMENT e (#PCDATA)*>]]>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            901 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            904 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_load_ignore_utf16(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let text: [::core::ffi::c_char; 73] = ::core::mem::transmute::<
        [u8; 73],
        [::core::ffi::c_char; 73],
    >(
        *b"<\0!\0[\0I\0G\0N\0O\0R\0E\0[\0<\0!\0E\0L\0E\0M\0E\0N\0T\0 \0e\0 \0(\0#\0P\0C\0D\0A\0T\0A\0)\0*\0>\0]\0]\0>\0\0",
    );
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            927 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 73]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            930 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_load_ignore_utf16_be(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let text: [::core::ffi::c_char; 73] = ::core::mem::transmute::<
        [u8; 73],
        [::core::ffi::c_char; 73],
    >(
        *b"\0<\0!\0[\0I\0G\0N\0O\0R\0E\0[\0<\0!\0E\0L\0E\0M\0E\0N\0T\0 \0e\0 \0(\0#\0P\0C\0D\0A\0T\0A\0)\0*\0>\0]\0]\0>\0",
    );
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            953 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        &raw const text as *const ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 73]>() as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            956 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_valuer(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM '004-2.ent'>\n<!ENTITY % e2 '%e1;'>\n%e1;\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    if systemId.is_null() {
        return XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            978 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-1.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if _XML_Parse_SINGLE_BYTES(
            ext_parser,
            text1,
            strlen(text1) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                982 as ::core::ffi::c_int,
            );
        }
    } else if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-2.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut fault: *mut crate::src::tests::handlers::ExtFaults = *(parser
            as *mut *mut ::core::ffi::c_void)
            as *mut crate::src::tests::handlers::ExtFaults;
        let mut status: XML_Status = XML_STATUS_ERROR;
        let mut error: XML_Error = XML_ERROR_NONE;
        status = _XML_Parse_SINGLE_BYTES(
            ext_parser,
            (*fault).parse_text,
            strlen((*fault).parse_text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
        if (*fault).error as ::core::ffi::c_uint
            == XML_ERROR_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if status as ::core::ffi::c_uint
                == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _xml_failure(
                    ext_parser,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    992 as ::core::ffi::c_int,
                );
            }
        } else {
            if status as ::core::ffi::c_uint
                != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                _fail(
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    995 as ::core::ffi::c_int,
                    (*fault).fail_text,
                );
            }
            error = XML_GetErrorCode(ext_parser);
            if error as ::core::ffi::c_uint != (*fault).error as ::core::ffi::c_uint
                && ((*fault).error as ::core::ffi::c_uint
                    != XML_ERROR_XML_DECL as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    || error as ::core::ffi::c_uint
                        != XML_ERROR_TEXT_DECL as ::core::ffi::c_int
                            as ::core::ffi::c_uint)
            {
                _xml_failure(
                    ext_parser,
                    b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    1000 as ::core::ffi::c_int,
                );
            }
        }
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_not_standalone(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM 'bar'>\n%e1;\n\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<!ATTLIST doc a1 CDATA 'value'>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    if systemId.is_null() {
        return XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1024 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"foo\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        XML_SetNotStandaloneHandler(
            ext_parser,
            Some(
                reject_not_standalone_handler
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            ),
        );
        if _XML_Parse_SINGLE_BYTES(
            ext_parser,
            text1,
            strlen(text1) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1029 as ::core::ffi::c_int,
                b"Expected not standalone rejection\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != XML_ERROR_NOT_STANDALONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1031 as ::core::ffi::c_int,
            );
        }
        XML_SetNotStandaloneHandler(ext_parser, None);
        XML_ParserFree(ext_parser);
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    } else if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"bar\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if _XML_Parse_SINGLE_BYTES(
            ext_parser,
            text2,
            strlen(text2) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1038 as ::core::ffi::c_int,
            );
        }
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_value_aborter(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM '004-2.ent'>\n<!ENTITY % e2 '%e1;'>\n%e1;\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<?xml version='1.0' encoding='utf-8'?>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    if systemId.is_null() {
        return XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1062 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-1.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        if _XML_Parse_SINGLE_BYTES(
            ext_parser,
            text1,
            strlen(text1) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1066 as ::core::ffi::c_int,
            );
        }
    }
    if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"004-2.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        XML_SetXmlDeclHandler(
            ext_parser,
            Some(
                entity_suspending_xdecl_handler
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const XML_Char,
                        *const XML_Char,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        XML_SetUserData(
            ext_parser,
            ext_parser as *mut ::core::ffi::c_void,
        );
        if _XML_Parse_SINGLE_BYTES(
            ext_parser,
            text2,
            strlen(text2) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1073 as ::core::ffi::c_int,
                b"Aborted parse not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(ext_parser) as ::core::ffi::c_uint
            != XML_ERROR_ABORTED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                ext_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1075 as ::core::ffi::c_int,
            );
        }
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_public(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text1: *const ::core::ffi::c_char =
        *(parser as *mut *mut ::core::ffi::c_void) as *const ::core::ffi::c_char;
    let mut text2: *const ::core::ffi::c_char =
        b"<!ATTLIST doc a CDATA 'value'>\0".as_ptr() as *const ::core::ffi::c_char;
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut parse_res: ::core::ffi::c_int = 0;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    if !systemId.is_null()
        && strcmp(
            systemId as *const ::core::ffi::c_char,
            b"http://example.org/\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        text = text1;
    } else if !publicId.is_null()
        && strcmp(
            publicId as *const ::core::ffi::c_char,
            b"foo\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        text = text2;
    } else {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1101 as ::core::ffi::c_int,
            b"Unexpected parameters to external entity parser\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if !text.is_null() {
    } else {
        __assert_fail(
            b"text != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1102 as ::core::ffi::c_uint,
            b"int external_entity_public(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    parse_res = _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    XML_ParserFree(ext_parser);
    return parse_res;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_devaluer(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<!ELEMENT doc EMPTY>\n<!ENTITY % e1 SYSTEM 'bar'>\n%e1;\n\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut clear_handler_flag: ::core::ffi::c_int =
        !(*(parser as *mut *mut ::core::ffi::c_void)).is_null() as ::core::ffi::c_int;
    if systemId.is_null()
        || strcmp(
            systemId as *const ::core::ffi::c_char,
            b"bar\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0
    {
        return XML_STATUS_OK as ::core::ffi::c_int;
    }
    if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"foo\0".as_ptr() as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1124 as ::core::ffi::c_int,
            b"Unexpected system ID\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1127 as ::core::ffi::c_int,
            b"Could note create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if clear_handler_flag != 0 {
        XML_SetExternalEntityRefHandler(ext_parser, None);
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1132 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_oneshot_loader(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut test_data: *mut crate::src::tests::handlers::ExtHdlrData =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtHdlrData;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1150 as ::core::ffi::c_int,
            b"Could not create external entity parser.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    XML_SetExternalEntityRefHandler(ext_parser, (*test_data).handler);
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        (*test_data).parse_text,
        strlen((*test_data).parse_text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1156 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_loader2(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut test_data: *mut crate::src::tests::handlers::ExtTest2 =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtTest2;
    let mut extparser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    extparser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if extparser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1175 as ::core::ffi::c_int,
            b"Coulr not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(*test_data).encoding.is_null() {
        if XML_SetEncoding(extparser, (*test_data).encoding) as u64 == 0
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1178 as ::core::ffi::c_int,
                b"XML_SetEncoding() ignored for external entity\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if _XML_Parse_SINGLE_BYTES(
        extparser,
        (*test_data).parse_text,
        (*test_data).parse_len,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            extparser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1183 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(extparser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_faulter2(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut test_data: *mut crate::src::tests::handlers::ExtFaults2 =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut crate::src::tests::handlers::ExtFaults2;
    let mut extparser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    extparser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if extparser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1202 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !(*test_data).encoding.is_null() {
        if XML_SetEncoding(extparser, (*test_data).encoding) as u64 == 0
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1205 as ::core::ffi::c_int,
                b"XML_SetEncoding() ignored for external entity\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if _XML_Parse_SINGLE_BYTES(
        extparser,
        (*test_data).parse_text,
        (*test_data).parse_len,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1210 as ::core::ffi::c_int,
            (*test_data).fail_text,
        );
    }
    if XML_GetErrorCode(extparser) as ::core::ffi::c_uint
        != (*test_data).error as ::core::ffi::c_uint
    {
        _xml_failure(
            extparser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1212 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(extparser);
    return XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_unfinished_attlist(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = b"<!ELEMENT barf ANY>\n<!ATTLIST barf my_attr (blah|%blah;a|foo) #REQUIRED>\n<!--COMMENT-->\n\0"
        .as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    if systemId.is_null() {
        return XML_STATUS_OK as ::core::ffi::c_int;
    }
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1235 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            ext_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1239 as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_handler(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut user_data: *mut ::core::ffi::c_void = *(parser as *mut *mut ::core::ffi::c_void);
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p2: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    if user_data.is_null() {
        text = b"<!ELEMENT doc (e+)>\n<!ATTLIST doc xmlns CDATA #IMPLIED>\n<!ELEMENT e EMPTY>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
    } else {
        text = b"<?xml version='1.0' encoding='us-ascii'?><e/>\0".as_ptr()
            as *const ::core::ffi::c_char;
    }
    XML_SetUserData(parser, parser as *mut ::core::ffi::c_void);
    p2 = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if _XML_Parse_SINGLE_BYTES(
        p2,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            p2,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1269 as ::core::ffi::c_int,
        );
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    XML_ParserFree(p2);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_duff_loader(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut new_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut i: ::core::ffi::c_uint = 0;
    let max_alloc_count: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while i < max_alloc_count {
        g_allocation_count = i as ::core::ffi::c_int;
        new_parser = XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<XML_Char>(),
        );
        if !new_parser.is_null() {
            XML_ParserFree(new_parser);
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if i == 0 as ::core::ffi::c_uint {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1297 as ::core::ffi::c_int,
            b"External parser creation ignored failing allocator\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else if i == max_alloc_count {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1299 as ::core::ffi::c_int,
            b"Extern parser not created with max allocation count\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    g_allocation_count = ALLOC_ALWAYS_SUCCEED;
    return XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_dbl_handler(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut pcallno: *mut ::core::ffi::c_int =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut ::core::ffi::c_int;
    let mut callno: ::core::ffi::c_int = *pcallno;
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut new_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut i: ::core::ffi::c_int = 0;
    let max_alloc_count: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    if callno == 0 as ::core::ffi::c_int {
        text = b"<!ELEMENT doc (e+)>\n<!ATTLIST doc xmlns CDATA #IMPLIED>\n<!ELEMENT e EMPTY>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        g_allocation_count = 10000 as ::core::ffi::c_int;
        new_parser = XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<XML_Char>(),
        );
        if new_parser.is_null() {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1330 as ::core::ffi::c_int,
                b"Unable to allocate first external parser\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        *pcallno = 10000 as ::core::ffi::c_int - g_allocation_count;
    } else {
        text = b"<?xml version='1.0' encoding='us-ascii'?><e/>\0".as_ptr()
            as *const ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < max_alloc_count {
            g_allocation_count = callno + i;
            new_parser = XML_ExternalEntityParserCreate(
                parser,
                context,
                ::core::ptr::null::<XML_Char>(),
            );
            if !new_parser.is_null() {
                break;
            }
            i += 1;
        }
        if i == 0 as ::core::ffi::c_int {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1346 as ::core::ffi::c_int,
                b"Second external parser unexpectedly created\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else if i == max_alloc_count {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1350 as ::core::ffi::c_int,
                b"Second external parser not created\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    g_allocation_count = ALLOC_ALWAYS_SUCCEED;
    if _XML_Parse_SINGLE_BYTES(
        new_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        _xml_failure(
            new_parser,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1358 as ::core::ffi::c_int,
        );
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    XML_ParserFree(new_parser);
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_dbl_handler_2(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut pcallno: *mut ::core::ffi::c_int =
        *(parser as *mut *mut ::core::ffi::c_void) as *mut ::core::ffi::c_int;
    let mut callno: ::core::ffi::c_int = *pcallno;
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut new_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut rv: XML_Status = XML_STATUS_ERROR;
    if callno == 0 as ::core::ffi::c_int {
        text = b"<!ELEMENT doc (e+)>\n<!ATTLIST doc xmlns CDATA #IMPLIED>\n<!ELEMENT e EMPTY>\n\0"
            .as_ptr() as *const ::core::ffi::c_char;
        *pcallno = 1 as ::core::ffi::c_int;
        new_parser = XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<XML_Char>(),
        );
        if new_parser.is_null() {
            return XML_STATUS_ERROR as ::core::ffi::c_int;
        }
        rv = _XML_Parse_SINGLE_BYTES(
            new_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
    } else {
        text = b"<?xml version='1.0' encoding='us-ascii'?><e/>\0".as_ptr()
            as *const ::core::ffi::c_char;
        new_parser = XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<XML_Char>(),
        );
        if new_parser.is_null() {
            return XML_STATUS_ERROR as ::core::ffi::c_int;
        }
        rv = _XML_Parse_SINGLE_BYTES(
            new_parser,
            text,
            strlen(text) as ::core::ffi::c_int,
            XML_TRUE as ::core::ffi::c_int,
        );
    }
    XML_ParserFree(new_parser);
    if rv as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_alloc_set_encoding(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        b"<?xml encoding='iso-8859-3'?>\xC3\xA9\0".as_ptr() as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut status: XML_Status = XML_STATUS_ERROR;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    if XML_SetEncoding(
        ext_parser,
        b"utf-8\0".as_ptr() as *const XML_Char,
    ) as u64
        == 0
    {
        XML_ParserFree(ext_parser);
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    status = _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    );
    XML_ParserFree(ext_parser);
    if status as ::core::ffi::c_uint
        == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_reallocator(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char = get_buffer_test_text;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut buffer: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut status: XML_Status = XML_STATUS_ERROR;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1446 as ::core::ffi::c_int,
            b"Could not create external entity parser\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    g_reallocation_count =
        *(*(parser as *mut *mut ::core::ffi::c_void) as *mut ::core::ffi::c_int);
    buffer = XML_GetBuffer(ext_parser, 1536 as ::core::ffi::c_int);
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1451 as ::core::ffi::c_int,
            b"Buffer allocation failed\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !buffer.is_null() {
    } else {
        __assert_fail(
            b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1452 as ::core::ffi::c_uint,
            b"int external_entity_reallocator(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    memcpy(
        buffer,
        text as *const ::core::ffi::c_void,
        strlen(text),
    );
    status = XML_ParseBuffer(
        ext_parser,
        strlen(text) as ::core::ffi::c_int,
        XML_FALSE as ::core::ffi::c_int,
    );
    g_reallocation_count = -1 as ::core::ffi::c_int;
    XML_ParserFree(ext_parser);
    return if status as ::core::ffi::c_uint
        == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        XML_STATUS_OK as ::core::ffi::c_int
    } else {
        XML_STATUS_ERROR as ::core::ffi::c_int
    };
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_alloc(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let mut text: *const ::core::ffi::c_char =
        *(parser as *mut *mut ::core::ffi::c_void) as *const ::core::ffi::c_char;
    let mut ext_parser: XML_Parser =
        ::core::ptr::null_mut::<XML_ParserStruct>();
    let mut parse_res: ::core::ffi::c_int = 0;
    ext_parser = XML_ExternalEntityParserCreate(
        parser,
        context,
        ::core::ptr::null::<XML_Char>(),
    );
    if ext_parser.is_null() {
        return XML_STATUS_ERROR as ::core::ffi::c_int;
    }
    parse_res = _XML_Parse_SINGLE_BYTES(
        ext_parser,
        text,
        strlen(text) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    XML_ParserFree(ext_parser);
    return parse_res;
}
#[no_mangle]

pub unsafe extern "C" fn external_entity_parser_create_alloc_fail_handler(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    if !context.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1491 as ::core::ffi::c_int,
            b"Unexpected non-NULL context\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    g_allocation_count = 3 as ::core::ffi::c_int;
    let encodingName: *const XML_Char =
        b"UTF-8\0".as_ptr() as *const XML_Char;
    let ext_parser: XML_Parser =
        XML_ExternalEntityParserCreate(parser, context, encodingName)
            as XML_Parser;
    if !ext_parser.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1503 as ::core::ffi::c_int,
            b"Call to XML_ExternalEntityParserCreate was expected to fail out-of-memory\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    g_allocation_count = ALLOC_ALWAYS_SUCCEED;
    return XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn accounting_external_entity_ref_handler(
    mut parser: XML_Parser,
    mut context: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) -> ::core::ffi::c_int {
    let testCase: *const crate::src::tests::handlers::AccountingTestCase = *(parser
        as *mut *mut ::core::ffi::c_void)
        as *const crate::src::tests::handlers::AccountingTestCase;
    let mut externalText: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if strcmp(
        systemId as *const ::core::ffi::c_char,
        b"first.ent\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        externalText = (*testCase).firstExternalText;
    } else if strcmp(
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
            __assert_fail(
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
        __assert_fail(
            b"externalText\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1530 as ::core::ffi::c_uint,
            b"int accounting_external_entity_ref_handler(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    let mut entParser: XML_Parser =
        XML_ExternalEntityParserCreate(
            parser,
            context,
            ::core::ptr::null::<XML_Char>(),
        );
    if !entParser.is_null() {
    } else {
        __assert_fail(
            b"entParser\0".as_ptr() as *const ::core::ffi::c_char,
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            1533 as ::core::ffi::c_uint,
            b"int accounting_external_entity_ref_handler(XML_Parser, const XML_Char *, const XML_Char *, const XML_Char *, const XML_Char *)\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    };
    let status: XML_Status = _XML_Parse_SINGLE_BYTES(
        entParser,
        externalText,
        strlen(externalText) as ::core::ffi::c_int,
        XML_TRUE as ::core::ffi::c_int,
    ) as XML_Status;
    XML_ParserFree(entParser);
    return status as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn reject_not_standalone_handler(
    mut userData: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return XML_STATUS_ERROR as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn accept_not_standalone_handler(
    mut userData: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return XML_STATUS_OK as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn verify_attlist_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut element_name: *const XML_Char,
    mut attr_name: *const XML_Char,
    mut attr_type: *const XML_Char,
    mut default_value: *const XML_Char,
    mut is_required: ::core::ffi::c_int,
) {
    let mut at: *mut crate::src::tests::handlers::AttTest =
        userData as *mut crate::src::tests::handlers::AttTest;
    if strcmp(
        element_name as *const ::core::ffi::c_char,
        (*at).element_name as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1566 as ::core::ffi::c_int,
            b"Unexpected element name in attribute declaration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        attr_name as *const ::core::ffi::c_char,
        (*at).attr_name as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1568 as ::core::ffi::c_int,
            b"Unexpected attribute name in attribute declaration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if strcmp(
        attr_type as *const ::core::ffi::c_char,
        (*at).attr_type as *const ::core::ffi::c_char,
    ) != 0 as ::core::ffi::c_int
    {
        _fail(
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
            && strcmp(
                default_value as *const ::core::ffi::c_char,
                (*at).default_value as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1575 as ::core::ffi::c_int,
            b"Unexpected default value in attribute declaration\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    if is_required != (*at).is_required {
        _fail(
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
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    XML_StopParser(
        g_parser,
        g_resumable,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        None,
    );
}
#[no_mangle]

pub unsafe extern "C" fn parser_stop_character_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut status: XML_ParsingStatus = XML_ParsingStatus { parsing:  XML_INITIALIZED, finalBuffer:  0 };
    XML_GetParsingStatus(
        g_parser,
        &raw mut status as *mut _ as *mut XML_ParsingStatus,
    );
    if status.parsing as ::core::ffi::c_uint
        == XML_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    XML_StopParser(
        g_parser,
        g_resumable,
    );
    XML_SetCharacterDataHandler(
        g_parser,
        None,
    );
    if g_resumable == 0 {
        if XML_StopParser(
            g_parser,
            XML_FALSE,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1607 as ::core::ffi::c_int,
                b"Aborting aborted parser not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser)
            as ::core::ffi::c_uint
            != XML_ERROR_FINISHED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1609 as ::core::ffi::c_int,
            );
        }
    } else if g_abortable != 0 {
        if XML_StopParser(
            g_parser,
            XML_FALSE,
        ) as ::core::ffi::c_uint
            == XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1613 as ::core::ffi::c_int,
            );
        }
    } else {
        if XML_StopParser(
            g_parser,
            XML_TRUE,
        ) as ::core::ffi::c_uint
            != XML_STATUS_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _fail(
                b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                1617 as ::core::ffi::c_int,
                b"Suspending suspended parser not faulted\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if XML_GetErrorCode(g_parser)
            as ::core::ffi::c_uint
            != XML_ERROR_SUSPENDED as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            _xml_failure(
                g_parser,
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
    mut s: *const XML_Char,
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
    mut s: *const XML_Char,
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
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut offset: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut buffer: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut data: *mut crate::src::tests::handlers::ByteTestData =
        userData as *mut crate::src::tests::handlers::ByteTestData;
    buffer = XML_GetInputContext(
        g_parser,
        &raw mut offset,
        &raw mut size,
    );
    if buffer.is_null() {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1652 as ::core::ffi::c_int,
            b"Failed to get context buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if offset != (*data).start_element_len {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1654 as ::core::ffi::c_int,
            b"Context offset in unexpected position\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if len != (*data).cdata_len {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1656 as ::core::ffi::c_int,
            b"CDATA length reported incorrectly\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if size != (*data).total_string_len {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1658 as ::core::ffi::c_int,
            b"Context size is not full buffer\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetCurrentByteIndex(g_parser)
        != offset as XML_Index
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1660 as ::core::ffi::c_int,
            b"Character byte index incorrect\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if XML_GetCurrentByteCount(g_parser)
        != len
    {
        _fail(
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
    mut s: *const XML_Char,
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
        _fail(
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
    mut s: *const XML_Char,
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
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    record_call(
        userData as *mut crate::src::tests::handlers::handler_record_list,
        b"record_cdata_handler\0".as_ptr() as *const ::core::ffi::c_char,
        len,
    );
    XML_DefaultCurrent(g_parser);
}
#[no_mangle]

pub unsafe extern "C" fn record_cdata_nodefault_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
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
    mut entityName: *const XML_Char,
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
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    CharData_AppendXMLChars(
        userData as *mut CharData
            as *mut CharData,
        name,
        strlen(name as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn record_element_end_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
) {
    let mut storage: *mut CharData =
        userData as *mut CharData;
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"/\0".as_ptr() as *const XML_Char,
        1 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
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
        _fail(
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

static mut entity_name_to_match: *const XML_Char =
    ::core::ptr::null::<XML_Char>();

static mut entity_value_to_match: *const XML_Char =
    ::core::ptr::null::<XML_Char>();

static mut entity_match_flag: ::core::ffi::c_int =
    crate::src::tests::handlers::ENTITY_MATCH_NOT_FOUND;
#[no_mangle]

pub unsafe extern "C" fn param_entity_match_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut entityName: *const XML_Char,
    mut is_parameter_entity: ::core::ffi::c_int,
    mut value: *const XML_Char,
    mut value_length: ::core::ffi::c_int,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
    mut notationName: *const XML_Char,
) {
    if is_parameter_entity == 0 || entity_name_to_match.is_null() || entity_value_to_match.is_null()
    {
        return;
    }
    if strcmp(
        entityName as *const ::core::ffi::c_char,
        entity_name_to_match as *const ::core::ffi::c_char,
    ) == 0
    {
        if value_length
            != strlen(entity_value_to_match as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
            || strncmp(
                value as *const ::core::ffi::c_char,
                entity_value_to_match as *const ::core::ffi::c_char,
                value_length as size_t,
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
    mut name: *const XML_Char,
    mut value: *const XML_Char,
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
    mut version: *const XML_Char,
    mut encoding: *const XML_Char,
    mut standalone: ::core::ffi::c_int,
) {
    if userData != g_handler_data as *mut ::core::ffi::c_void {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1794 as ::core::ffi::c_int,
            b"User data (xml decl) not correctly set\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if standalone != -1 as ::core::ffi::c_int {
        _fail(
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
    mut entityName: *const XML_Char,
    mut is_parameter_entity: ::core::ffi::c_int,
) {
    if userData != g_handler_data as *mut ::core::ffi::c_void {
        _fail(
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
    mut data: *const XML_Char,
) {
    if userData != g_handler_data as *mut ::core::ffi::c_void {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1815 as ::core::ffi::c_int,
            b"User data (parser) not correctly set\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if *(userData as *mut *mut ::core::ffi::c_void)
        != 1 as ::core::ffi::c_int as *mut ::core::ffi::c_void
    {
        _fail(
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
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let trigger_char: XML_Char =
        *(userData as *const XML_Char);
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
        XML_StopParser(
            g_parser,
            g_resumable,
        );
        XML_SetDefaultHandler(g_parser, None);
    }
}
#[no_mangle]

pub unsafe extern "C" fn suspending_comment_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut data: *const XML_Char,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    XML_StopParser(parser, XML_TRUE);
}
#[no_mangle]

pub unsafe extern "C" fn element_decl_suspender(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut model: *mut XML_Content,
) {
    XML_StopParser(
        g_parser,
        XML_TRUE,
    );
    XML_FreeContentModel(
        g_parser,
        model as *mut XML_cp,
    );
}
#[no_mangle]

pub unsafe extern "C" fn suspend_after_element_declaration(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut model: *mut XML_Content,
) {
    let mut parser: XML_Parser = userData as XML_Parser;
    if !(XML_StopParser(
        parser,
        1 as ::core::ffi::c_int as XML_Bool,
    ) as ::core::ffi::c_uint
        == XML_STATUS_OK as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        _fail(
            b"/mnt/ssd1/ahomescu/development/immunant/libexpat/expat/tests/handlers.c\0".as_ptr()
                as *const ::core::ffi::c_char,
            1861 as ::core::ffi::c_int,
            b"check failed: XML_StopParser(parser, XML_TRUE) == XML_STATUS_OK\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    XML_FreeContentModel(parser, model as *mut XML_cp);
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_pi_characters(
    mut userData: *mut ::core::ffi::c_void,
    mut target: *const XML_Char,
    mut data: *const XML_Char,
) {
    let mut storage: *mut CharData =
        userData as *mut CharData;
    CharData_AppendXMLChars(
        storage as *mut CharData,
        target,
        -1 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b": \0".as_ptr() as *const XML_Char,
        2 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
        data,
        -1 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"\n\0".as_ptr() as *const XML_Char,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_comment(
    mut userData: *mut ::core::ffi::c_void,
    mut data: *const XML_Char,
) {
    let mut storage: *mut CharData =
        userData as *mut CharData;
    CharData_AppendXMLChars(
        storage as *mut CharData,
        data,
        -1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_entity_decl(
    mut userData: *mut ::core::ffi::c_void,
    mut entityName: *const XML_Char,
    mut is_parameter_entity: ::core::ffi::c_int,
    mut value: *const XML_Char,
    mut value_length: ::core::ffi::c_int,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
    mut notationName: *const XML_Char,
) {
    let mut storage: *mut CharData =
        userData as *mut CharData;
    CharData_AppendXMLChars(
        storage as *mut CharData,
        entityName,
        -1 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"=\0".as_ptr() as *const XML_Char,
        1 as ::core::ffi::c_int,
    );
    if value.is_null() {
        CharData_AppendXMLChars(
            storage as *mut CharData,
            b"(null)\0".as_ptr() as *const XML_Char,
            -1 as ::core::ffi::c_int,
        );
    } else {
        CharData_AppendXMLChars(
            storage as *mut CharData,
            value,
            value_length,
        );
    }
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"\n\0".as_ptr() as *const XML_Char,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_char_data_and_suspend(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let parserPlusStorage: *mut crate::src::tests::handlers::ParserPlusStorage =
        userData as *mut crate::src::tests::handlers::ParserPlusStorage;
    CharData_AppendXMLChars(
        (*parserPlusStorage).storage as *mut CharData,
        s,
        len,
    );
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        if *s.offset(i as isize) as ::core::ffi::c_int == 'Z' as i32 {
            XML_StopParser(
                (*parserPlusStorage).parser,
                XML_TRUE,
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
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let storage: *mut CharData =
        userData as *mut CharData;
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b"(\0".as_ptr() as *const XML_Char,
        1 as ::core::ffi::c_int,
    );
    CharData_AppendXMLChars(
        storage as *mut CharData,
        name,
        -1 as ::core::ffi::c_int,
    );
    if !atts.is_null() && !(*atts.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        CharData_AppendXMLChars(
            storage as *mut CharData,
            b"(\0".as_ptr() as *const XML_Char,
            1 as ::core::ffi::c_int,
        );
        while !(*atts.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            CharData_AppendXMLChars(
                storage as *mut CharData,
                *atts.offset(0 as ::core::ffi::c_int as isize),
                -1 as ::core::ffi::c_int,
            );
            CharData_AppendXMLChars(
                storage as *mut CharData,
                b"=\0".as_ptr() as *const XML_Char,
                1 as ::core::ffi::c_int,
            );
            CharData_AppendXMLChars(
                storage as *mut CharData,
                *atts.offset(1 as ::core::ffi::c_int as isize),
                -1 as ::core::ffi::c_int,
            );
            atts = atts.offset(2 as ::core::ffi::c_int as isize);
            if !(*atts.offset(0 as ::core::ffi::c_int as isize)).is_null() {
                CharData_AppendXMLChars(
                    storage as *mut CharData,
                    b",\0".as_ptr() as *const XML_Char,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        CharData_AppendXMLChars(
            storage as *mut CharData,
            b")\0".as_ptr() as *const XML_Char,
            1 as ::core::ffi::c_int,
        );
    }
    CharData_AppendXMLChars(
        storage as *mut CharData,
        b")\n\0".as_ptr() as *const XML_Char,
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_characters(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let storage: *mut CharData =
        userData as *mut CharData;
    CharData_AppendXMLChars(
        storage as *mut CharData,
        s,
        len,
    );
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_attribute(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    let storage: *mut CharData =
        userData as *mut CharData;
    if atts.is_null() {
        return;
    }
    while (*storage).count < 0 as ::core::ffi::c_int
        && !(*atts.offset(0 as ::core::ffi::c_int as isize)).is_null()
    {
        CharData_AppendXMLChars(
            storage as *mut CharData,
            *atts.offset(1 as ::core::ffi::c_int as isize),
            -1 as ::core::ffi::c_int,
        );
        atts = atts.offset(2 as ::core::ffi::c_int as isize);
    }
}
#[no_mangle]

pub unsafe extern "C" fn ext_accumulate_characters(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let test_data: *mut ExtTest =
        userData as *mut ExtTest;
    accumulate_characters((*test_data).storage as *mut ::core::ffi::c_void, s, len);
}
#[no_mangle]

pub unsafe extern "C" fn checking_default_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
    let mut data: *mut crate::src::tests::handlers::DefaultCheck =
        userData as *mut crate::src::tests::handlers::DefaultCheck;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !(*data.offset(i as isize)).expected.is_null() {
        if (*data.offset(i as isize)).expectedLen == len
            && memcmp(
                (*data.offset(i as isize)).expected as *const ::core::ffi::c_void,
                s as *const ::core::ffi::c_void,
                (len as size_t)
                    .wrapping_mul(::core::mem::size_of::<XML_Char>()
                        as size_t),
            ) == 0
        {
            (*data.offset(i as isize)).seen = XML_TRUE;
            break;
        } else {
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn accumulate_and_suspend_comment_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut data: *const XML_Char,
) {
    let parserPlusStorage: *mut crate::src::tests::handlers::ParserPlusStorage =
        userData as *mut crate::src::tests::handlers::ParserPlusStorage;
    accumulate_comment(
        (*parserPlusStorage).storage as *mut ::core::ffi::c_void,
        data,
    );
    XML_StopParser(
        (*parserPlusStorage).parser,
        XML_TRUE,
    );
}
