// =============== BEGIN xmlrole_h ================
pub type C2RustUnnamed_0 = c_int;

pub const XML_ROLE_ERROR: C2RustUnnamed_0 = -1;

pub const XML_ROLE_NONE: C2RustUnnamed_0 = 0;

pub const XML_ROLE_XML_DECL: C2RustUnnamed_0 = 1;

pub const XML_ROLE_INSTANCE_START: C2RustUnnamed_0 = 2;

pub const XML_ROLE_DOCTYPE_NONE: C2RustUnnamed_0 = 3;

pub const XML_ROLE_DOCTYPE_NAME: C2RustUnnamed_0 = 4;

pub const XML_ROLE_DOCTYPE_SYSTEM_ID: C2RustUnnamed_0 = 5;

pub const XML_ROLE_DOCTYPE_PUBLIC_ID: C2RustUnnamed_0 = 6;

pub const XML_ROLE_DOCTYPE_INTERNAL_SUBSET: C2RustUnnamed_0 = 7;

pub const XML_ROLE_DOCTYPE_CLOSE: C2RustUnnamed_0 = 8;

pub const XML_ROLE_GENERAL_ENTITY_NAME: C2RustUnnamed_0 = 9;

pub const XML_ROLE_PARAM_ENTITY_NAME: C2RustUnnamed_0 = 10;

pub const XML_ROLE_ENTITY_NONE: C2RustUnnamed_0 = 11;

pub const XML_ROLE_ENTITY_VALUE: C2RustUnnamed_0 = 12;

pub const XML_ROLE_ENTITY_SYSTEM_ID: C2RustUnnamed_0 = 13;

pub const XML_ROLE_ENTITY_PUBLIC_ID: C2RustUnnamed_0 = 14;

pub const XML_ROLE_ENTITY_COMPLETE: C2RustUnnamed_0 = 15;

pub const XML_ROLE_ENTITY_NOTATION_NAME: C2RustUnnamed_0 = 16;

pub const XML_ROLE_NOTATION_NONE: C2RustUnnamed_0 = 17;

pub const XML_ROLE_NOTATION_NAME: C2RustUnnamed_0 = 18;

pub const XML_ROLE_NOTATION_SYSTEM_ID: C2RustUnnamed_0 = 19;

pub const XML_ROLE_NOTATION_NO_SYSTEM_ID: C2RustUnnamed_0 = 20;

pub const XML_ROLE_NOTATION_PUBLIC_ID: C2RustUnnamed_0 = 21;

pub const XML_ROLE_ATTRIBUTE_NAME: C2RustUnnamed_0 = 22;

pub const XML_ROLE_ATTRIBUTE_TYPE_CDATA: C2RustUnnamed_0 = 23;

pub const XML_ROLE_ATTRIBUTE_TYPE_ID: C2RustUnnamed_0 = 24;

pub const XML_ROLE_ATTRIBUTE_TYPE_IDREF: C2RustUnnamed_0 = 25;

pub const XML_ROLE_ATTRIBUTE_TYPE_IDREFS: C2RustUnnamed_0 = 26;

pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITY: C2RustUnnamed_0 = 27;

pub const XML_ROLE_ATTRIBUTE_TYPE_ENTITIES: C2RustUnnamed_0 = 28;

pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN: C2RustUnnamed_0 = 29;

pub const XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS: C2RustUnnamed_0 = 30;

pub const XML_ROLE_ATTRIBUTE_ENUM_VALUE: C2RustUnnamed_0 = 31;

pub const XML_ROLE_ATTRIBUTE_NOTATION_VALUE: C2RustUnnamed_0 = 32;

pub const XML_ROLE_ATTLIST_NONE: C2RustUnnamed_0 = 33;

pub const XML_ROLE_ATTLIST_ELEMENT_NAME: C2RustUnnamed_0 = 34;

pub const XML_ROLE_IMPLIED_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 35;

pub const XML_ROLE_REQUIRED_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 36;

pub const XML_ROLE_DEFAULT_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 37;

pub const XML_ROLE_FIXED_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 38;

pub const XML_ROLE_ELEMENT_NONE: C2RustUnnamed_0 = 39;

pub const XML_ROLE_ELEMENT_NAME: C2RustUnnamed_0 = 40;

pub const XML_ROLE_CONTENT_ANY: C2RustUnnamed_0 = 41;

pub const XML_ROLE_CONTENT_EMPTY: C2RustUnnamed_0 = 42;

pub const XML_ROLE_CONTENT_PCDATA: C2RustUnnamed_0 = 43;

pub const XML_ROLE_GROUP_OPEN: C2RustUnnamed_0 = 44;

pub const XML_ROLE_GROUP_CLOSE: C2RustUnnamed_0 = 45;

pub const XML_ROLE_GROUP_CLOSE_REP: C2RustUnnamed_0 = 46;

pub const XML_ROLE_GROUP_CLOSE_OPT: C2RustUnnamed_0 = 47;

pub const XML_ROLE_GROUP_CLOSE_PLUS: C2RustUnnamed_0 = 48;

pub const XML_ROLE_GROUP_CHOICE: C2RustUnnamed_0 = 49;

pub const XML_ROLE_GROUP_SEQUENCE: C2RustUnnamed_0 = 50;

pub const XML_ROLE_CONTENT_ELEMENT: C2RustUnnamed_0 = 51;

pub const XML_ROLE_CONTENT_ELEMENT_REP: C2RustUnnamed_0 = 52;

pub const XML_ROLE_CONTENT_ELEMENT_OPT: C2RustUnnamed_0 = 53;

pub const XML_ROLE_CONTENT_ELEMENT_PLUS: C2RustUnnamed_0 = 54;

pub const XML_ROLE_PI: C2RustUnnamed_0 = 55;

pub const XML_ROLE_COMMENT: C2RustUnnamed_0 = 56;

pub const XML_ROLE_TEXT_DECL: C2RustUnnamed_0 = 57;

pub const XML_ROLE_IGNORE_SECT: C2RustUnnamed_0 = 58;

pub const XML_ROLE_INNER_PARAM_ENTITY_REF: C2RustUnnamed_0 = 59;

pub const XML_ROLE_PARAM_ENTITY_REF: C2RustUnnamed_0 = 60;

pub type PROLOG_STATE = prolog_state;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct prolog_state {
    pub handler: Option<fn(*mut prolog_state, c_int, &[c_char], *const ENCODING) -> c_int>,
    pub level: ::core::ffi::c_uint,
    pub role_none: c_int,
    pub includeLevel: ::core::ffi::c_uint,
    pub documentEntity: c_int,
    pub inEntityValue: c_int,
}
pub use crate::ascii_h::ASCII_A;
pub use crate::ascii_h::ASCII_B;
pub use crate::ascii_h::ASCII_C;
pub use crate::ascii_h::ASCII_D;
pub use crate::ascii_h::ASCII_E;
pub use crate::ascii_h::ASCII_F;
pub use crate::ascii_h::ASCII_G;
pub use crate::ascii_h::ASCII_I;
pub use crate::ascii_h::ASCII_K;
pub use crate::ascii_h::ASCII_L;
pub use crate::ascii_h::ASCII_M;
pub use crate::ascii_h::ASCII_N;
pub use crate::ascii_h::ASCII_O;
pub use crate::ascii_h::ASCII_P;
pub use crate::ascii_h::ASCII_Q;
pub use crate::ascii_h::ASCII_R;
pub use crate::ascii_h::ASCII_S;
pub use crate::ascii_h::ASCII_T;
pub use crate::ascii_h::ASCII_U;
pub use crate::ascii_h::ASCII_X;
pub use crate::ascii_h::ASCII_Y;
pub use crate::expat_external_h::XML_Size;

pub use crate::src::lib::xmltok::encoding;
pub use crate::src::lib::xmltok::position;
pub use crate::src::lib::xmltok::XML_Convert_Result;
pub use crate::src::lib::xmltok::ATTRIBUTE;
pub use crate::src::lib::xmltok::ENCODING;
pub use crate::src::lib::xmltok::POSITION;
pub use crate::src::lib::xmltok::SCANNER;
pub use crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
pub use crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
pub use crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
pub use crate::src::lib::xmltok::XML_TOK_BOM;
pub use crate::src::lib::xmltok::XML_TOK_CLOSE_BRACKET;
pub use crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN;
pub use crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK;
pub use crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_PLUS;
pub use crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_QUESTION;
pub use crate::src::lib::xmltok::XML_TOK_COMMA;
pub use crate::src::lib::xmltok::XML_TOK_COMMENT;
pub use crate::src::lib::xmltok::XML_TOK_COND_SECT_CLOSE;
pub use crate::src::lib::xmltok::XML_TOK_COND_SECT_OPEN;
pub use crate::src::lib::xmltok::XML_TOK_DECL_CLOSE;
pub use crate::src::lib::xmltok::XML_TOK_DECL_OPEN;
pub use crate::src::lib::xmltok::XML_TOK_INSTANCE_START_1;
pub use crate::src::lib::xmltok::XML_TOK_LITERAL;
pub use crate::src::lib::xmltok::XML_TOK_NAME;
pub use crate::src::lib::xmltok::XML_TOK_NAME_ASTERISK;
pub use crate::src::lib::xmltok::XML_TOK_NAME_PLUS;
pub use crate::src::lib::xmltok::XML_TOK_NAME_QUESTION;
pub use crate::src::lib::xmltok::XML_TOK_NMTOKEN;
pub use crate::src::lib::xmltok::XML_TOK_NONE;
pub use crate::src::lib::xmltok::XML_TOK_OPEN_BRACKET;
pub use crate::src::lib::xmltok::XML_TOK_OPEN_PAREN;
pub use crate::src::lib::xmltok::XML_TOK_OR;
pub use crate::src::lib::xmltok::XML_TOK_PARAM_ENTITY_REF_1;
pub use crate::src::lib::xmltok::XML_TOK_PERCENT;
pub use crate::src::lib::xmltok::XML_TOK_PI;
pub use crate::src::lib::xmltok::XML_TOK_POUND_NAME;
pub use crate::src::lib::xmltok::XML_TOK_PREFIXED_NAME;
pub use crate::src::lib::xmltok::XML_TOK_PROLOG_S;
pub use crate::src::lib::xmltok::XML_TOK_XML_DECL;
use core::ffi::c_char;
use core::ffi::c_int;

pub type PROLOG_HANDLER = fn(*mut PROLOG_STATE, c_int, &[c_char], *const ENCODING) -> c_int;

#[inline]
fn c_char_slice_from_ptr_end<'a>(ptr: *const c_char, end: *const c_char) -> &'a [c_char] {
    unsafe { core::slice::from_raw_parts(ptr, end.offset_from(ptr) as usize) }
}

#[inline]
fn input_ptr_and_end(input: &[c_char]) -> (*const c_char, *const c_char) {
    let ptr = input.as_ptr();
    let end = ptr.wrapping_add(input.len());
    (ptr, end)
}

#[inline]
fn set_handler(state: *mut PROLOG_STATE, handler: PROLOG_HANDLER) {
    unsafe { (*state).handler = Some(handler) }
}

#[inline]
fn role_none(state: *mut PROLOG_STATE) -> c_int {
    unsafe { (*state).role_none }
}

#[inline]
fn document_entity(state: *mut PROLOG_STATE) -> c_int {
    unsafe { (*state).documentEntity }
}

#[inline]
fn set_document_entity(state: *mut PROLOG_STATE, value: c_int) {
    unsafe { (*state).documentEntity = value }
}

#[inline]
fn include_level(state: *mut PROLOG_STATE) -> core::ffi::c_uint {
    unsafe { (*state).includeLevel }
}

#[inline]
fn set_include_level(state: *mut PROLOG_STATE, value: core::ffi::c_uint) {
    unsafe { (*state).includeLevel = value }
}

#[inline]
fn inc_include_level(state: *mut PROLOG_STATE) {
    unsafe { (*state).includeLevel = (*state).includeLevel.wrapping_add(1u32) }
}

#[inline]
fn dec_include_level(state: *mut PROLOG_STATE) {
    unsafe { (*state).includeLevel = (*state).includeLevel.wrapping_sub(1u32) }
}

#[inline]
fn set_in_entity_value(state: *mut PROLOG_STATE, value: c_int) {
    unsafe { (*state).inEntityValue = value }
}

#[inline]
fn set_role_none(state: *mut PROLOG_STATE, value: c_int) {
    unsafe { (*state).role_none = value }
}

#[inline]
fn level(state: *mut PROLOG_STATE) -> core::ffi::c_uint {
    unsafe { (*state).level }
}

#[inline]
fn set_level(state: *mut PROLOG_STATE, value: core::ffi::c_uint) {
    unsafe { (*state).level = value }
}

#[inline]
fn inc_level(state: *mut PROLOG_STATE) {
    unsafe { (*state).level = (*state).level.wrapping_add(1u32) }
}

#[inline]
fn dec_level(state: *mut PROLOG_STATE) {
    unsafe { (*state).level = (*state).level.wrapping_sub(1u32) }
}

#[inline]
fn set_handler_for_subset(state: *mut PROLOG_STATE) {
    if document_entity(state) != 0 {
        set_handler(state, internalSubset);
    } else {
        set_handler(state, externalSubset1);
    }
}

#[inline]
fn set_decl_close_role_none(state: *mut PROLOG_STATE, role: c_int) {
    set_handler(state, declClose);
    set_role_none(state, role);
}

#[inline]
fn close_element_group(state: *mut PROLOG_STATE, role: c_int) -> c_int {
    dec_level(state);
    if level(state) == 0u32 {
        set_decl_close_role_none(state, XML_ROLE_ELEMENT_NONE);
    }
    role
}

#[inline]
fn enc_min_bytes_per_char(enc: *const ENCODING) -> isize {
    unsafe { (*enc).minBytesPerChar as isize }
}

#[inline]
fn decl_open_keyword_slice<'a>(
    ptr: *const c_char,
    end: *const c_char,
    enc: *const ENCODING,
) -> &'a [c_char] {
    c_char_slice_from_ptr_end(ptr.wrapping_offset(2isize * enc_min_bytes_per_char(enc)), end)
}

#[inline]
fn enc_name_matches_ascii(enc: *const ENCODING, input: &[c_char], keyword: *const c_char) -> bool {
    unsafe { (*enc).nameMatchesAscii(&*enc, input, keyword) != 0 }
}

static KW_ANY: [c_char; 4] = [
    ASCII_A as c_char,
    ASCII_N as c_char,
    ASCII_Y as c_char,
    '\0' as c_char,
];

static KW_ATTLIST: [c_char; 8] = [
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_T as c_char,
    ASCII_L as c_char,
    ASCII_I as c_char,
    ASCII_S as c_char,
    ASCII_T as c_char,
    '\0' as c_char,
];

static KW_CDATA: [c_char; 6] = [
    ASCII_C as c_char,
    ASCII_D as c_char,
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_A as c_char,
    '\0' as c_char,
];

static KW_DOCTYPE: [c_char; 8] = [
    ASCII_D as c_char,
    ASCII_O as c_char,
    ASCII_C as c_char,
    ASCII_T as c_char,
    ASCII_Y as c_char,
    ASCII_P as c_char,
    ASCII_E as c_char,
    '\0' as c_char,
];

static KW_ELEMENT: [c_char; 8] = [
    ASCII_E as c_char,
    ASCII_L as c_char,
    ASCII_E as c_char,
    ASCII_M as c_char,
    ASCII_E as c_char,
    ASCII_N as c_char,
    ASCII_T as c_char,
    '\0' as c_char,
];

static KW_EMPTY: [c_char; 6] = [
    ASCII_E as c_char,
    ASCII_M as c_char,
    ASCII_P as c_char,
    ASCII_T as c_char,
    ASCII_Y as c_char,
    '\0' as c_char,
];

static KW_ENTITIES: [c_char; 9] = [
    ASCII_E as c_char,
    ASCII_N as c_char,
    ASCII_T as c_char,
    ASCII_I as c_char,
    ASCII_T as c_char,
    ASCII_I as c_char,
    ASCII_E as c_char,
    ASCII_S as c_char,
    '\0' as c_char,
];

static KW_ENTITY: [c_char; 7] = [
    ASCII_E as c_char,
    ASCII_N as c_char,
    ASCII_T as c_char,
    ASCII_I as c_char,
    ASCII_T as c_char,
    ASCII_Y as c_char,
    '\0' as c_char,
];

static KW_FIXED: [c_char; 6] = [
    ASCII_F as c_char,
    ASCII_I as c_char,
    ASCII_X as c_char,
    ASCII_E as c_char,
    ASCII_D as c_char,
    '\0' as c_char,
];

static KW_ID: [c_char; 3] = [ASCII_I as c_char, ASCII_D as c_char, '\0' as c_char];

static KW_IDREF: [c_char; 6] = [
    ASCII_I as c_char,
    ASCII_D as c_char,
    ASCII_R as c_char,
    ASCII_E as c_char,
    ASCII_F as c_char,
    '\0' as c_char,
];

static KW_IDREFS: [c_char; 7] = [
    ASCII_I as c_char,
    ASCII_D as c_char,
    ASCII_R as c_char,
    ASCII_E as c_char,
    ASCII_F as c_char,
    ASCII_S as c_char,
    '\0' as c_char,
];

static KW_IGNORE: [c_char; 7] = [
    ASCII_I as c_char,
    ASCII_G as c_char,
    ASCII_N as c_char,
    ASCII_O as c_char,
    ASCII_R as c_char,
    ASCII_E as c_char,
    '\0' as c_char,
];

static KW_IMPLIED: [c_char; 8] = [
    ASCII_I as c_char,
    ASCII_M as c_char,
    ASCII_P as c_char,
    ASCII_L as c_char,
    ASCII_I as c_char,
    ASCII_E as c_char,
    ASCII_D as c_char,
    '\0' as c_char,
];

static KW_INCLUDE: [c_char; 8] = [
    ASCII_I as c_char,
    ASCII_N as c_char,
    ASCII_C as c_char,
    ASCII_L as c_char,
    ASCII_U as c_char,
    ASCII_D as c_char,
    ASCII_E as c_char,
    '\0' as c_char,
];

static KW_NDATA: [c_char; 6] = [
    ASCII_N as c_char,
    ASCII_D as c_char,
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_A as c_char,
    '\0' as c_char,
];

static KW_NMTOKEN: [c_char; 8] = [
    ASCII_N as c_char,
    ASCII_M as c_char,
    ASCII_T as c_char,
    ASCII_O as c_char,
    ASCII_K as c_char,
    ASCII_E as c_char,
    ASCII_N as c_char,
    '\0' as c_char,
];

static KW_NMTOKENS: [c_char; 9] = [
    ASCII_N as c_char,
    ASCII_M as c_char,
    ASCII_T as c_char,
    ASCII_O as c_char,
    ASCII_K as c_char,
    ASCII_E as c_char,
    ASCII_N as c_char,
    ASCII_S as c_char,
    '\0' as c_char,
];

static KW_NOTATION: [c_char; 9] = [
    ASCII_N as c_char,
    ASCII_O as c_char,
    ASCII_T as c_char,
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_I as c_char,
    ASCII_O as c_char,
    ASCII_N as c_char,
    '\0' as c_char,
];

static KW_PCDATA: [c_char; 7] = [
    ASCII_P as c_char,
    ASCII_C as c_char,
    ASCII_D as c_char,
    ASCII_A as c_char,
    ASCII_T as c_char,
    ASCII_A as c_char,
    '\0' as c_char,
];

static KW_PUBLIC: [c_char; 7] = [
    ASCII_P as c_char,
    ASCII_U as c_char,
    ASCII_B as c_char,
    ASCII_L as c_char,
    ASCII_I as c_char,
    ASCII_C as c_char,
    '\0' as c_char,
];

static KW_REQUIRED: [c_char; 9] = [
    ASCII_R as c_char,
    ASCII_E as c_char,
    ASCII_Q as c_char,
    ASCII_U as c_char,
    ASCII_I as c_char,
    ASCII_R as c_char,
    ASCII_E as c_char,
    ASCII_D as c_char,
    '\0' as c_char,
];

static KW_SYSTEM: [c_char; 7] = [
    ASCII_S as c_char,
    ASCII_Y as c_char,
    ASCII_S as c_char,
    ASCII_T as c_char,
    ASCII_E as c_char,
    ASCII_M as c_char,
    '\0' as c_char,
];

fn prolog0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => {
            set_handler(state, prolog1);
            return XML_ROLE_NONE;
        }
        XML_TOK_XML_DECL => {
            set_handler(state, prolog1);
            return XML_ROLE_XML_DECL;
        }
        XML_TOK_PI => {
            set_handler(state, prolog1);
            return XML_ROLE_PI;
        }
        XML_TOK_COMMENT => {
            set_handler(state, prolog1);
            return XML_ROLE_COMMENT;
        }
        XML_TOK_BOM => return XML_ROLE_NONE,
        XML_TOK_DECL_OPEN => {
            if enc_name_matches_ascii(enc, decl_open_keyword_slice(ptr, end, enc), KW_DOCTYPE.as_ptr()) {
                set_handler(state, doctype0);
                return XML_ROLE_DOCTYPE_NONE;
            }
        }
        XML_TOK_INSTANCE_START_1 => {
            set_handler(state, error);
            return XML_ROLE_INSTANCE_START;
        }
        _ => {}
    }
    common(state, tok)
}

fn prolog1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_PI => return XML_ROLE_PI,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT,
        XML_TOK_BOM => return XML_ROLE_NONE,
        XML_TOK_DECL_OPEN => {
            if enc_name_matches_ascii(enc, decl_open_keyword_slice(ptr, end, enc), KW_DOCTYPE.as_ptr()) {
                set_handler(state, doctype0);
                return XML_ROLE_DOCTYPE_NONE;
            }
        }
        XML_TOK_INSTANCE_START_1 => {
            set_handler(state, error);
            return XML_ROLE_INSTANCE_START;
        }
        _ => {}
    }
    common(state, tok)
}

fn prolog2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_PI => return XML_ROLE_PI,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT,
        XML_TOK_INSTANCE_START_1 => {
            set_handler(state, error);
            return XML_ROLE_INSTANCE_START;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, doctype1);
            return XML_ROLE_DOCTYPE_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_OPEN_BRACKET => {
            set_handler(state, internalSubset);
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET;
        }
        XML_TOK_DECL_CLOSE => {
            set_handler(state, prolog2);
            return XML_ROLE_DOCTYPE_CLOSE;
        }
        XML_TOK_NAME => {
            if enc_name_matches_ascii(enc, c_char_slice_from_ptr_end(ptr, end), KW_SYSTEM.as_ptr()) {
                set_handler(state, doctype3);
                return XML_ROLE_DOCTYPE_NONE;
            }
            if enc_name_matches_ascii(enc, c_char_slice_from_ptr_end(ptr, end), KW_PUBLIC.as_ptr()) {
                set_handler(state, doctype2);
                return XML_ROLE_DOCTYPE_NONE;
            }
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_LITERAL => {
            set_handler(state, doctype3);
            return XML_ROLE_DOCTYPE_PUBLIC_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_LITERAL => {
            set_handler(state, doctype4);
            return XML_ROLE_DOCTYPE_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_OPEN_BRACKET => {
            set_handler(state, internalSubset);
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET;
        }
        XML_TOK_DECL_CLOSE => {
            set_handler(state, prolog2);
            return XML_ROLE_DOCTYPE_CLOSE;
        }
        _ => {}
    }
    common(state, tok)
}

fn doctype5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_DECL_CLOSE => {
            set_handler(state, prolog2);
            return XML_ROLE_DOCTYPE_CLOSE;
        }
        _ => {}
    }
    common(state, tok)
}

fn internalSubset(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_DECL_OPEN => {
            let decl_name = decl_open_keyword_slice(ptr, end, enc);
            if enc_name_matches_ascii(enc, decl_name, KW_ENTITY.as_ptr()) {
                set_handler(state, entity0);
                return XML_ROLE_ENTITY_NONE;
            }
            if enc_name_matches_ascii(enc, decl_name, KW_ATTLIST.as_ptr()) {
                set_handler(state, attlist0);
                return XML_ROLE_ATTLIST_NONE;
            }
            if enc_name_matches_ascii(enc, decl_name, KW_ELEMENT.as_ptr()) {
                set_handler(state, element0);
                return XML_ROLE_ELEMENT_NONE;
            }
            if enc_name_matches_ascii(enc, decl_name, KW_NOTATION.as_ptr()) {
                set_handler(state, notation0);
                return XML_ROLE_NOTATION_NONE;
            }
        }
        XML_TOK_PI => return XML_ROLE_PI,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT,
        XML_TOK_PARAM_ENTITY_REF_1 => {
            return XML_ROLE_PARAM_ENTITY_REF;
        }
        XML_TOK_CLOSE_BRACKET => {
            set_handler(state, doctype5);
            return XML_ROLE_DOCTYPE_NONE;
        }
        XML_TOK_NONE => return XML_ROLE_NONE,
        _ => {}
    }
    common(state, tok)
}

fn externalSubset0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    set_handler(state, externalSubset1);
    if tok == XML_TOK_XML_DECL {
        return XML_ROLE_TEXT_DECL;
    }
    externalSubset1(state, tok, c_char_slice_from_ptr_end(ptr, end), enc)
}

fn externalSubset1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_COND_SECT_OPEN => {
            set_handler(state, condSect0);
            return XML_ROLE_NONE;
        }
        XML_TOK_COND_SECT_CLOSE => {
            if include_level(state) != 0u32 {
                dec_include_level(state);
                return XML_ROLE_NONE;
            }
        }
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_CLOSE_BRACKET => {}
        XML_TOK_NONE => {
            if include_level(state) == 0 {
                return XML_ROLE_NONE;
            }
        }
        _ => {
            return internalSubset(state, tok, c_char_slice_from_ptr_end(ptr, end), enc)
        }
    }
    common(state, tok)
}

fn entity0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_PERCENT => {
            set_handler(state, entity1);
            return XML_ROLE_ENTITY_NONE;
        }
        XML_TOK_NAME => {
            set_handler(state, entity2);
            return XML_ROLE_GENERAL_ENTITY_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_NAME => {
            set_handler(state, entity7);
            return XML_ROLE_PARAM_ENTITY_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_NAME => {
            let name = c_char_slice_from_ptr_end(ptr, end);
            if enc_name_matches_ascii(enc, name, KW_SYSTEM.as_ptr()) {
                set_handler(state, entity4);
                return XML_ROLE_ENTITY_NONE;
            }
            if enc_name_matches_ascii(enc, name, KW_PUBLIC.as_ptr()) {
                set_handler(state, entity3);
                return XML_ROLE_ENTITY_NONE;
            }
        }
        XML_TOK_LITERAL => {
            set_handler(state, declClose);
            set_role_none(state, XML_ROLE_ENTITY_NONE);
            return XML_ROLE_ENTITY_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_LITERAL => {
            set_handler(state, entity4);
            return XML_ROLE_ENTITY_PUBLIC_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_LITERAL => {
            set_handler(state, entity5);
            return XML_ROLE_ENTITY_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_DECL_CLOSE => {
            set_handler_for_subset(state);
            return XML_ROLE_ENTITY_COMPLETE;
        }
        XML_TOK_NAME => {
            if enc_name_matches_ascii(enc, c_char_slice_from_ptr_end(ptr, end), KW_NDATA.as_ptr()) {
                set_handler(state, entity6);
                return XML_ROLE_ENTITY_NONE;
            }
        }
        _ => {}
    }
    common(state, tok)
}

fn entity6(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_NAME => {
            set_handler(state, declClose);
            set_role_none(state, XML_ROLE_ENTITY_NONE);
            return XML_ROLE_ENTITY_NOTATION_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity7(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_NAME => {
            let name = c_char_slice_from_ptr_end(ptr, end);
            if enc_name_matches_ascii(enc, name, KW_SYSTEM.as_ptr()) {
                set_handler(state, entity9);
                return XML_ROLE_ENTITY_NONE;
            }
            if enc_name_matches_ascii(enc, name, KW_PUBLIC.as_ptr()) {
                set_handler(state, entity8);
                return XML_ROLE_ENTITY_NONE;
            }
        }
        XML_TOK_LITERAL => {
            set_handler(state, declClose);
            set_role_none(state, XML_ROLE_ENTITY_NONE);
            return XML_ROLE_ENTITY_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity8(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_LITERAL => {
            set_handler(state, entity9);
            return XML_ROLE_ENTITY_PUBLIC_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity9(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_LITERAL => {
            set_handler(state, entity10);
            return XML_ROLE_ENTITY_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn entity10(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_DECL_CLOSE => {
            set_handler_for_subset(state);
            return XML_ROLE_ENTITY_COMPLETE;
        }
        _ => {}
    }
    common(state, tok)
}

fn notation0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_NAME => {
            set_handler(state, notation1);
            return XML_ROLE_NOTATION_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn notation1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_NAME => {
            let name_input = c_char_slice_from_ptr_end(ptr, end);
            if enc_name_matches_ascii(enc, name_input, &raw const KW_SYSTEM as *const c_char) {
                set_handler(state, notation3);
                return XML_ROLE_NOTATION_NONE;
            }
            if enc_name_matches_ascii(enc, name_input, &raw const KW_PUBLIC as *const c_char) {
                set_handler(state, notation2);
                return XML_ROLE_NOTATION_NONE;
            }
        }
        _ => {}
    }
    common(state, tok)
}

fn notation2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_LITERAL => {
            set_handler(state, notation4);
            return XML_ROLE_NOTATION_PUBLIC_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn notation3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_LITERAL => {
            set_decl_close_role_none(state, XML_ROLE_NOTATION_NONE);
            return XML_ROLE_NOTATION_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn notation4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_LITERAL => {
            set_decl_close_role_none(state, XML_ROLE_NOTATION_NONE);
            return XML_ROLE_NOTATION_SYSTEM_ID;
        }
        XML_TOK_DECL_CLOSE => {
            set_handler_for_subset(state);
            return XML_ROLE_NOTATION_NO_SYSTEM_ID;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, attlist1);
            return XML_ROLE_ATTLIST_ELEMENT_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_DECL_CLOSE => {
            set_handler_for_subset(state);
            return XML_ROLE_ATTLIST_NONE;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, attlist2);
            return XML_ROLE_ATTRIBUTE_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_NAME => {
            let types: [*const c_char; 8] = [
                &raw const KW_CDATA as *const c_char,
                &raw const KW_ID as *const c_char,
                &raw const KW_IDREF as *const c_char,
                &raw const KW_IDREFS as *const c_char,
                &raw const KW_ENTITY as *const c_char,
                &raw const KW_ENTITIES as *const c_char,
                &raw const KW_NMTOKEN as *const c_char,
                &raw const KW_NMTOKENS as *const c_char,
            ];
            let name_input = c_char_slice_from_ptr_end(ptr, end);
            for (i, ty) in types.iter().enumerate() {
                if enc_name_matches_ascii(enc, name_input, *ty) {
                    set_handler(state, attlist8);
                    return XML_ROLE_ATTRIBUTE_TYPE_CDATA + i as c_int;
                }
            }
            if enc_name_matches_ascii(enc, name_input, &raw const KW_NOTATION as *const c_char) {
                set_handler(state, attlist5);
                return XML_ROLE_ATTLIST_NONE;
            }
        }
        XML_TOK_OPEN_PAREN => {
            set_handler(state, attlist3);
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_NMTOKEN | XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, attlist4);
            return XML_ROLE_ATTRIBUTE_ENUM_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_CLOSE_PAREN => {
            set_handler(state, attlist8);
            return XML_ROLE_ATTLIST_NONE;
        }
        XML_TOK_OR => {
            set_handler(state, attlist3);
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_OPEN_PAREN => {
            set_handler(state, attlist6);
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist6(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_NAME => {
            set_handler(state, attlist7);
            return XML_ROLE_ATTRIBUTE_NOTATION_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist7(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_CLOSE_PAREN => {
            set_handler(state, attlist8);
            return XML_ROLE_ATTLIST_NONE;
        }
        XML_TOK_OR => {
            set_handler(state, attlist6);
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist8(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_POUND_NAME => {
            let pound_name_input =
                c_char_slice_from_ptr_end(ptr.wrapping_offset(enc_min_bytes_per_char(enc)), end);
            if enc_name_matches_ascii(enc, pound_name_input, &raw const KW_IMPLIED as *const c_char)
            {
                set_handler(state, attlist1);
                return XML_ROLE_IMPLIED_ATTRIBUTE_VALUE;
            }
            if enc_name_matches_ascii(enc, pound_name_input, &raw const KW_REQUIRED as *const c_char)
            {
                set_handler(state, attlist1);
                return XML_ROLE_REQUIRED_ATTRIBUTE_VALUE;
            }
            if enc_name_matches_ascii(enc, pound_name_input, &raw const KW_FIXED as *const c_char) {
                set_handler(state, attlist9);
                return XML_ROLE_ATTLIST_NONE;
            }
        }
        XML_TOK_LITERAL => {
            set_handler(state, attlist1);
            return XML_ROLE_DEFAULT_ATTRIBUTE_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn attlist9(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_LITERAL => {
            set_handler(state, attlist1);
            return XML_ROLE_FIXED_ATTRIBUTE_VALUE;
        }
        _ => {}
    }
    common(state, tok)
}

fn element0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, element1);
            return XML_ROLE_ELEMENT_NAME;
        }
        _ => {}
    }
    common(state, tok)
}

fn element1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_NAME => {
            let name_input = c_char_slice_from_ptr_end(ptr, end);
            if enc_name_matches_ascii(enc, name_input, &raw const KW_EMPTY as *const c_char) {
                set_decl_close_role_none(state, XML_ROLE_ELEMENT_NONE);
                return XML_ROLE_CONTENT_EMPTY;
            }
            if enc_name_matches_ascii(enc, name_input, &raw const KW_ANY as *const c_char) {
                set_decl_close_role_none(state, XML_ROLE_ELEMENT_NONE);
                return XML_ROLE_CONTENT_ANY;
            }
        }
        XML_TOK_OPEN_PAREN => {
            set_handler(state, element2);
            set_level(state, 1u32);
            return XML_ROLE_GROUP_OPEN;
        }
        _ => {}
    }
    common(state, tok)
}

fn element2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_POUND_NAME => {
            let pound_name_input =
                c_char_slice_from_ptr_end(ptr.wrapping_offset(enc_min_bytes_per_char(enc)), end);
            if enc_name_matches_ascii(enc, pound_name_input, &raw const KW_PCDATA as *const c_char)
            {
                set_handler(state, element3);
                return XML_ROLE_CONTENT_PCDATA;
            }
        }
        XML_TOK_OPEN_PAREN => {
            set_level(state, 2u32);
            set_handler(state, element6);
            return XML_ROLE_GROUP_OPEN;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, element7);
            return XML_ROLE_CONTENT_ELEMENT;
        }
        XML_TOK_NAME_QUESTION => {
            set_handler(state, element7);
            return XML_ROLE_CONTENT_ELEMENT_OPT;
        }
        XML_TOK_NAME_ASTERISK => {
            set_handler(state, element7);
            return XML_ROLE_CONTENT_ELEMENT_REP;
        }
        XML_TOK_NAME_PLUS => {
            set_handler(state, element7);
            return XML_ROLE_CONTENT_ELEMENT_PLUS;
        }
        _ => {}
    }
    common(state, tok)
}

fn element3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_CLOSE_PAREN => {
            set_decl_close_role_none(state, XML_ROLE_ELEMENT_NONE);
            return XML_ROLE_GROUP_CLOSE;
        }
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            set_decl_close_role_none(state, XML_ROLE_ELEMENT_NONE);
            return XML_ROLE_GROUP_CLOSE_REP;
        }
        XML_TOK_OR => {
            set_handler(state, element4);
            return XML_ROLE_ELEMENT_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn element4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, element5);
            return XML_ROLE_CONTENT_ELEMENT;
        }
        _ => {}
    }
    common(state, tok)
}

fn element5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            set_decl_close_role_none(state, XML_ROLE_ELEMENT_NONE);
            return XML_ROLE_GROUP_CLOSE_REP;
        }
        XML_TOK_OR => {
            set_handler(state, element4);
            return XML_ROLE_ELEMENT_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn element6(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_OPEN_PAREN => {
            inc_level(state);
            return XML_ROLE_GROUP_OPEN;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            set_handler(state, element7);
            return XML_ROLE_CONTENT_ELEMENT;
        }
        XML_TOK_NAME_QUESTION => {
            set_handler(state, element7);
            return XML_ROLE_CONTENT_ELEMENT_OPT;
        }
        XML_TOK_NAME_ASTERISK => {
            set_handler(state, element7);
            return XML_ROLE_CONTENT_ELEMENT_REP;
        }
        XML_TOK_NAME_PLUS => {
            set_handler(state, element7);
            return XML_ROLE_CONTENT_ELEMENT_PLUS;
        }
        _ => {}
    }
    common(state, tok)
}

fn element7(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_CLOSE_PAREN => return close_element_group(state, XML_ROLE_GROUP_CLOSE),
        XML_TOK_CLOSE_PAREN_ASTERISK => return close_element_group(state, XML_ROLE_GROUP_CLOSE_REP),
        XML_TOK_CLOSE_PAREN_QUESTION => return close_element_group(state, XML_ROLE_GROUP_CLOSE_OPT),
        XML_TOK_CLOSE_PAREN_PLUS => return close_element_group(state, XML_ROLE_GROUP_CLOSE_PLUS),
        XML_TOK_COMMA => {
            set_handler(state, element6);
            return XML_ROLE_GROUP_SEQUENCE;
        }
        XML_TOK_OR => {
            set_handler(state, element6);
            return XML_ROLE_GROUP_CHOICE;
        }
        _ => {}
    }
    common(state, tok)
}

fn condSect0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut enc: *const ENCODING,
) -> c_int {
    let (ptr, end) = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_NAME => {
            let name_input = c_char_slice_from_ptr_end(ptr, end);
            if enc_name_matches_ascii(enc, name_input, &raw const KW_INCLUDE as *const c_char) {
                set_handler(state, condSect1);
                return XML_ROLE_NONE;
            }
            if enc_name_matches_ascii(enc, name_input, &raw const KW_IGNORE as *const c_char) {
                set_handler(state, condSect2);
                return XML_ROLE_NONE;
            }
        }
        _ => {}
    }
    common(state, tok)
}

fn condSect1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_OPEN_BRACKET => {
            set_handler(state, externalSubset1);
            inc_include_level(state);
            return XML_ROLE_NONE;
        }
        _ => {}
    }
    common(state, tok)
}

fn condSect2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_OPEN_BRACKET => {
            set_handler(state, externalSubset1);
            return XML_ROLE_IGNORE_SECT;
        }
        _ => {}
    }
    common(state, tok)
}

fn declClose(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input_ptr_and_end(input);
    match tok {
        XML_TOK_PROLOG_S => return role_none(state),
        XML_TOK_DECL_CLOSE => {
            set_handler_for_subset(state);
            return role_none(state);
        }
        _ => {}
    }
    common(state, tok)
}

fn error(
    mut _state: *mut PROLOG_STATE,
    mut _tok: c_int,
    input: &[c_char],
    mut _enc: *const ENCODING,
) -> c_int {
    let _ = input;
    XML_ROLE_NONE
}

fn common(mut state: *mut PROLOG_STATE, mut tok: c_int) -> c_int {
    if document_entity(state) == 0 && tok == XML_TOK_PARAM_ENTITY_REF_1 {
        return XML_ROLE_INNER_PARAM_ENTITY_REF;
    }
    set_handler(state, error);
    XML_ROLE_ERROR
}
pub(crate) fn XmlPrologStateInit(mut state: *mut PROLOG_STATE) {
    set_handler(state, prolog0);
    set_document_entity(state, 1);
    set_include_level(state, 0u32);
    set_in_entity_value(state, 0);
}
pub(crate) fn XmlPrologStateInitExternalEntity(mut state: *mut PROLOG_STATE) {
    set_handler(state, externalSubset0);
    set_document_entity(state, 0);
    set_include_level(state, 0u32);
}
