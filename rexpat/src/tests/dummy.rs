// =============== BEGIN dummy_h ================
pub const DUMMY_START_DOCTYPE_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 0;

pub const DUMMY_END_DOCTYPE_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 1;

pub const DUMMY_ENTITY_DECL_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 2;

pub const DUMMY_NOTATION_DECL_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 3;

pub const DUMMY_ELEMENT_DECL_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 4;

pub const DUMMY_ATTLIST_DECL_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 5;

pub const DUMMY_COMMENT_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 6;

pub const DUMMY_PI_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 7;

pub const DUMMY_START_ELEMENT_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 8;

pub const DUMMY_START_CDATA_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 9;

pub const DUMMY_END_CDATA_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 10;

pub const DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 11;

pub const DUMMY_START_NS_DECL_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 12;

pub const DUMMY_END_NS_DECL_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 13;

pub const DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 14;

pub const DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 15;

pub const DUMMY_SKIP_HANDLER_FLAG: ::core::ffi::c_ulong = (1) << 16;
use crate::src::tests::common::g_parser;

pub use crate::expat_external_h::XML_Char;
pub use crate::expat_h::XML_Content;
pub use crate::expat_h::XML_Content_Quant;
pub use crate::expat_h::XML_Content_Type;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_ParserStruct;
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
pub use crate::src::lib::xmlparse::XML_FreeContentModel;

static mut dummy_handler_flags: ::core::ffi::c_ulong = 0;
#[no_mangle]

pub unsafe extern "C" fn init_dummy_handlers() {
    dummy_handler_flags = 0;
}
#[no_mangle]

pub unsafe extern "C" fn get_dummy_handler_flags() -> ::core::ffi::c_ulong {
    return dummy_handler_flags;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_xdecl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut version: *const XML_Char,
    mut encoding: *const XML_Char,
    mut standalone: ::core::ffi::c_int,
) {
}
#[no_mangle]

pub unsafe extern "C" fn dummy_start_doctype_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut doctypeName: *const XML_Char,
    mut sysid: *const XML_Char,
    mut pubid: *const XML_Char,
    mut has_internal_subset: ::core::ffi::c_int,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_START_DOCTYPE_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_end_doctype_handler(mut userData: *mut ::core::ffi::c_void) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_END_DOCTYPE_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_entity_decl_handler(
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
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_ENTITY_DECL_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_notation_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut notationName: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_NOTATION_DECL_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_element_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut model: *mut XML_Content,
) {
    XML_FreeContentModel(g_parser, model);
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_ELEMENT_DECL_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_attlist_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut elname: *const XML_Char,
    mut attname: *const XML_Char,
    mut att_type: *const XML_Char,
    mut dflt: *const XML_Char,
    mut isrequired: ::core::ffi::c_int,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_ATTLIST_DECL_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_comment_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut data: *const XML_Char,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_COMMENT_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_pi_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut target: *const XML_Char,
    mut data: *const XML_Char,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_PI_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_start_element(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
    mut atts: *mut *const XML_Char,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_START_ELEMENT_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_end_element(
    mut userData: *mut ::core::ffi::c_void,
    mut name: *const XML_Char,
) {
}
#[no_mangle]

pub unsafe extern "C" fn dummy_start_cdata_handler(mut userData: *mut ::core::ffi::c_void) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_START_CDATA_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_end_cdata_handler(mut userData: *mut ::core::ffi::c_void) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_END_CDATA_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_cdata_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
}
#[no_mangle]

pub unsafe extern "C" fn dummy_start_namespace_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut prefix: *const XML_Char,
    mut uri: *const XML_Char,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_START_NS_DECL_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_end_namespace_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut prefix: *const XML_Char,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_END_NS_DECL_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_unparsed_entity_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut entityName: *const XML_Char,
    mut base: *const XML_Char,
    mut systemId: *const XML_Char,
    mut publicId: *const XML_Char,
    mut notationName: *const XML_Char,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_UNPARSED_ENTITY_DECL_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_default_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut s: *const XML_Char,
    mut len: ::core::ffi::c_int,
) {
}
#[no_mangle]

pub unsafe extern "C" fn dummy_start_doctype_decl_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut doctypeName: *const XML_Char,
    mut sysid: *const XML_Char,
    mut pubid: *const XML_Char,
    mut has_internal_subset: ::core::ffi::c_int,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_START_DOCTYPE_DECL_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_end_doctype_decl_handler(mut userData: *mut ::core::ffi::c_void) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_END_DOCTYPE_DECL_HANDLER_FLAG;
}
#[no_mangle]

pub unsafe extern "C" fn dummy_skip_handler(
    mut userData: *mut ::core::ffi::c_void,
    mut entityName: *const XML_Char,
    mut is_parameter_entity: ::core::ffi::c_int,
) {
    dummy_handler_flags |= crate::src::tests::dummy::DUMMY_SKIP_HANDLER_FLAG;
}
