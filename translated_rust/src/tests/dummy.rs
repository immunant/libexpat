use std::sync::atomic::{AtomicUsize, Ordering};

extern "C" {
    pub type XML_ParserStruct;
    fn XML_FreeContentModel(parser: XML_Parser, model: *mut XML_Content);
    static mut g_parser: XML_Parser;
}
pub type XML_Char = ::core::ffi::c_char;
pub type XML_Parser = *mut XML_ParserStruct;
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
pub const DUMMY_START_ELEMENT_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 8 as ::core::ffi::c_int;
pub const DUMMY_START_CDATA_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 9 as ::core::ffi::c_int;
pub const DUMMY_END_CDATA_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 10 as ::core::ffi::c_int;
pub const DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 11 as ::core::ffi::c_int;
pub const DUMMY_START_NS_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 12 as ::core::ffi::c_int;
pub const DUMMY_END_NS_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 13 as ::core::ffi::c_int;
pub const DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 14 as ::core::ffi::c_int;
pub const DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 15 as ::core::ffi::c_int;
pub const DUMMY_SKIP_HANDLER_FLAG: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 16 as ::core::ffi::c_int;

static DUMMY_HANDLER_FLAGS: AtomicUsize = AtomicUsize::new(0);

fn reset_dummy_handler_flags() {
    DUMMY_HANDLER_FLAGS.store(0, Ordering::Relaxed);
}

fn get_dummy_handler_flags_value() -> ::core::ffi::c_ulong {
    DUMMY_HANDLER_FLAGS.load(Ordering::Relaxed) as ::core::ffi::c_ulong
}

fn set_dummy_handler_flag(flag: ::core::ffi::c_ulong) {
    DUMMY_HANDLER_FLAGS.fetch_or(flag as usize, Ordering::Relaxed);
}

fn free_content_model(model: *mut XML_Content) {
    unsafe {
        XML_FreeContentModel(g_parser, model);
    }
}

#[no_mangle]
pub unsafe extern "C" fn init_dummy_handlers() {
    reset_dummy_handler_flags();
}

#[no_mangle]
pub unsafe extern "C" fn get_dummy_handler_flags() -> ::core::ffi::c_ulong {
    get_dummy_handler_flags_value()
}

#[no_mangle]
pub unsafe extern "C" fn dummy_xdecl_handler(
    _userData: *mut ::core::ffi::c_void,
    _version: *const XML_Char,
    _encoding: *const XML_Char,
    _standalone: ::core::ffi::c_int,
) {
}

#[no_mangle]
pub unsafe extern "C" fn dummy_start_doctype_handler(
    _userData: *mut ::core::ffi::c_void,
    _doctypeName: *const XML_Char,
    _sysid: *const XML_Char,
    _pubid: *const XML_Char,
    _has_internal_subset: ::core::ffi::c_int,
) {
    set_dummy_handler_flag(DUMMY_START_DOCTYPE_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_end_doctype_handler(_userData: *mut ::core::ffi::c_void) {
    set_dummy_handler_flag(DUMMY_END_DOCTYPE_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_entity_decl_handler(
    _userData: *mut ::core::ffi::c_void,
    _entityName: *const XML_Char,
    _is_parameter_entity: ::core::ffi::c_int,
    _value: *const XML_Char,
    _value_length: ::core::ffi::c_int,
    _base: *const XML_Char,
    _systemId: *const XML_Char,
    _publicId: *const XML_Char,
    _notationName: *const XML_Char,
) {
    set_dummy_handler_flag(DUMMY_ENTITY_DECL_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_notation_decl_handler(
    _userData: *mut ::core::ffi::c_void,
    _notationName: *const XML_Char,
    _base: *const XML_Char,
    _systemId: *const XML_Char,
    _publicId: *const XML_Char,
) {
    set_dummy_handler_flag(DUMMY_NOTATION_DECL_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_element_decl_handler(
    _userData: *mut ::core::ffi::c_void,
    _name: *const XML_Char,
    model: *mut XML_Content,
) {
    free_content_model(model);
    set_dummy_handler_flag(DUMMY_ELEMENT_DECL_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_attlist_decl_handler(
    _userData: *mut ::core::ffi::c_void,
    _elname: *const XML_Char,
    _attname: *const XML_Char,
    _att_type: *const XML_Char,
    _dflt: *const XML_Char,
    _isrequired: ::core::ffi::c_int,
) {
    set_dummy_handler_flag(DUMMY_ATTLIST_DECL_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_comment_handler(
    _userData: *mut ::core::ffi::c_void,
    _data: *const XML_Char,
) {
    set_dummy_handler_flag(DUMMY_COMMENT_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_pi_handler(
    _userData: *mut ::core::ffi::c_void,
    _target: *const XML_Char,
    _data: *const XML_Char,
) {
    set_dummy_handler_flag(DUMMY_PI_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_start_element(
    _userData: *mut ::core::ffi::c_void,
    _name: *const XML_Char,
    _atts: *mut *const XML_Char,
) {
    set_dummy_handler_flag(DUMMY_START_ELEMENT_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_end_element(
    _userData: *mut ::core::ffi::c_void,
    _name: *const XML_Char,
) {
}

#[no_mangle]
pub unsafe extern "C" fn dummy_start_cdata_handler(_userData: *mut ::core::ffi::c_void) {
    set_dummy_handler_flag(DUMMY_START_CDATA_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_end_cdata_handler(_userData: *mut ::core::ffi::c_void) {
    set_dummy_handler_flag(DUMMY_END_CDATA_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_cdata_handler(
    _userData: *mut ::core::ffi::c_void,
    _s: *const XML_Char,
    _len: ::core::ffi::c_int,
) {
}

#[no_mangle]
pub unsafe extern "C" fn dummy_start_namespace_decl_handler(
    _userData: *mut ::core::ffi::c_void,
    _prefix: *const XML_Char,
    _uri: *const XML_Char,
) {
    set_dummy_handler_flag(DUMMY_START_NS_DECL_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_end_namespace_decl_handler(
    _userData: *mut ::core::ffi::c_void,
    _prefix: *const XML_Char,
) {
    set_dummy_handler_flag(DUMMY_END_NS_DECL_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_unparsed_entity_decl_handler(
    _userData: *mut ::core::ffi::c_void,
    _entityName: *const XML_Char,
    _base: *const XML_Char,
    _systemId: *const XML_Char,
    _publicId: *const XML_Char,
    _notationName: *const XML_Char,
) {
    set_dummy_handler_flag(DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_default_handler(
    _userData: *mut ::core::ffi::c_void,
    _s: *const XML_Char,
    _len: ::core::ffi::c_int,
) {
}

#[no_mangle]
pub unsafe extern "C" fn dummy_start_doctype_decl_handler(
    _userData: *mut ::core::ffi::c_void,
    _doctypeName: *const XML_Char,
    _sysid: *const XML_Char,
    _pubid: *const XML_Char,
    _has_internal_subset: ::core::ffi::c_int,
) {
    set_dummy_handler_flag(DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_end_doctype_decl_handler(_userData: *mut ::core::ffi::c_void) {
    set_dummy_handler_flag(DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_skip_handler(
    _userData: *mut ::core::ffi::c_void,
    _entityName: *const XML_Char,
    _is_parameter_entity: ::core::ffi::c_int,
) {
    set_dummy_handler_flag(DUMMY_SKIP_HANDLER_FLAG);
}
