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
    pub handler: Option<
        unsafe fn(*mut prolog_state, c_int, *const c_char, *const c_char, *const ENCODING) -> c_int,
    >,
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

pub type PROLOG_HANDLER =
    unsafe fn(*mut PROLOG_STATE, c_int, *const c_char, *const c_char, *const ENCODING) -> c_int;

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

unsafe fn prolog0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => {
            (*state).handler = Some(
                prolog1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_NONE;
        }
        XML_TOK_XML_DECL => {
            (*state).handler = Some(
                prolog1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_XML_DECL;
        }
        XML_TOK_PI => {
            (*state).handler = Some(
                prolog1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_PI;
        }
        XML_TOK_COMMENT => {
            (*state).handler = Some(
                prolog1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_COMMENT;
        }
        XML_TOK_BOM => return XML_ROLE_NONE,
        XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii(
                enc,
                ptr.offset((2i32 * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_DOCTYPE as *const c_char,
            ) == 0)
            {
                (*state).handler = Some(
                    doctype0
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_DOCTYPE_NONE;
            }
        }
        XML_TOK_INSTANCE_START_1 => {
            (*state).handler = Some(
                error
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_INSTANCE_START;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn prolog1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_PI => return XML_ROLE_PI,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT,
        XML_TOK_BOM => return XML_ROLE_NONE,
        XML_TOK_DECL_OPEN => {
            if !((*enc).nameMatchesAscii(
                enc,
                ptr.offset((2i32 * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_DOCTYPE as *const c_char,
            ) == 0)
            {
                (*state).handler = Some(
                    doctype0
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_DOCTYPE_NONE;
            }
        }
        XML_TOK_INSTANCE_START_1 => {
            (*state).handler = Some(
                error
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_INSTANCE_START;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn prolog2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_PI => return XML_ROLE_PI,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT,
        XML_TOK_INSTANCE_START_1 => {
            (*state).handler = Some(
                error
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_INSTANCE_START;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn doctype0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                doctype1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DOCTYPE_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn doctype1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                internalSubset
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET;
        }
        XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(
                prolog2
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DOCTYPE_CLOSE;
        }
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_SYSTEM as *const c_char) != 0 {
                (*state).handler = Some(
                    doctype3
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_DOCTYPE_NONE;
            }
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_PUBLIC as *const c_char) != 0 {
                (*state).handler = Some(
                    doctype2
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_DOCTYPE_NONE;
            }
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn doctype2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                doctype3
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DOCTYPE_PUBLIC_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn doctype3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                doctype4
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DOCTYPE_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn doctype4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                internalSubset
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DOCTYPE_INTERNAL_SUBSET;
        }
        XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(
                prolog2
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DOCTYPE_CLOSE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn doctype5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_DOCTYPE_NONE,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = Some(
                prolog2
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DOCTYPE_CLOSE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn internalSubset(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_DECL_OPEN => {
            if (*enc).nameMatchesAscii(
                enc,
                ptr.offset((2i32 * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_ENTITY as *const c_char,
            ) != 0
            {
                (*state).handler = Some(
                    entity0
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ENTITY_NONE;
            }
            if (*enc).nameMatchesAscii(
                enc,
                ptr.offset((2i32 * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_ATTLIST as *const c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist0
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ATTLIST_NONE;
            }
            if (*enc).nameMatchesAscii(
                enc,
                ptr.offset((2i32 * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_ELEMENT as *const c_char,
            ) != 0
            {
                (*state).handler = Some(
                    element0
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ELEMENT_NONE;
            }
            if (*enc).nameMatchesAscii(
                enc,
                ptr.offset((2i32 * (*enc).minBytesPerChar) as isize),
                end,
                &raw const KW_NOTATION as *const c_char,
            ) != 0
            {
                (*state).handler = Some(
                    notation0
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_NOTATION_NONE;
            }
        }
        XML_TOK_PI => return XML_ROLE_PI,
        XML_TOK_COMMENT => return XML_ROLE_COMMENT,
        XML_TOK_PARAM_ENTITY_REF_1 => {
            return XML_ROLE_PARAM_ENTITY_REF;
        }
        XML_TOK_CLOSE_BRACKET => {
            (*state).handler = Some(
                doctype5
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DOCTYPE_NONE;
        }
        XML_TOK_NONE => return XML_ROLE_NONE,
        _ => {}
    }
    return common(state, tok);
}

unsafe fn externalSubset0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    (*state).handler = Some(
        externalSubset1
            as unsafe fn(
                *mut PROLOG_STATE,
                c_int,
                *const c_char,
                *const c_char,
                *const ENCODING,
            ) -> c_int,
    );
    if tok == XML_TOK_XML_DECL {
        return XML_ROLE_TEXT_DECL;
    }
    return externalSubset1(state, tok, ptr, end, enc);
}

unsafe fn externalSubset1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_COND_SECT_OPEN => {
            (*state).handler = Some(
                condSect0
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_NONE;
        }
        XML_TOK_COND_SECT_CLOSE => {
            if !((*state).includeLevel == 0u32) {
                (*state).includeLevel = (*state).includeLevel.wrapping_sub(1u32);
                return XML_ROLE_NONE;
            }
        }
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_CLOSE_BRACKET => {}
        XML_TOK_NONE => {
            if !((*state).includeLevel != 0) {
                return XML_ROLE_NONE;
            }
        }
        _ => return internalSubset(state, tok, ptr, end, enc),
    }
    return common(state, tok);
}

unsafe fn entity0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_PERCENT => {
            (*state).handler = Some(
                entity1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ENTITY_NONE;
        }
        XML_TOK_NAME => {
            (*state).handler = Some(
                entity2
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_GENERAL_ENTITY_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_NAME => {
            (*state).handler = Some(
                entity7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_PARAM_ENTITY_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_SYSTEM as *const c_char) != 0 {
                (*state).handler = Some(
                    entity4
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ENTITY_NONE;
            }
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_PUBLIC as *const c_char) != 0 {
                (*state).handler = Some(
                    entity3
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ENTITY_NONE;
            }
        }
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).role_none = XML_ROLE_ENTITY_NONE;
            return XML_ROLE_ENTITY_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity4
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ENTITY_PUBLIC_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity5
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ENTITY_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            };
            return XML_ROLE_ENTITY_COMPLETE;
        }
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_NDATA as *const c_char) != 0 {
                (*state).handler = Some(
                    entity6
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ENTITY_NONE;
            }
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity6(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_NAME => {
            (*state).handler = Some(
                declClose
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).role_none = XML_ROLE_ENTITY_NONE;
            return XML_ROLE_ENTITY_NOTATION_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity7(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_SYSTEM as *const c_char) != 0 {
                (*state).handler = Some(
                    entity9
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ENTITY_NONE;
            }
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_PUBLIC as *const c_char) != 0 {
                (*state).handler = Some(
                    entity8
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ENTITY_NONE;
            }
        }
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).role_none = XML_ROLE_ENTITY_NONE;
            return XML_ROLE_ENTITY_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity8(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity9
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ENTITY_PUBLIC_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity9(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                entity10
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ENTITY_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn entity10(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ENTITY_NONE,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            };
            return XML_ROLE_ENTITY_COMPLETE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn notation0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_NAME => {
            (*state).handler = Some(
                notation1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_NOTATION_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn notation1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_SYSTEM as *const c_char) != 0 {
                (*state).handler = Some(
                    notation3
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_NOTATION_NONE;
            }
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_PUBLIC as *const c_char) != 0 {
                (*state).handler = Some(
                    notation2
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_NOTATION_NONE;
            }
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn notation2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                notation4
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_NOTATION_PUBLIC_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn notation3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).role_none = XML_ROLE_NOTATION_NONE;
            return XML_ROLE_NOTATION_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn notation4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NOTATION_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                declClose
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).role_none = XML_ROLE_NOTATION_NONE;
            return XML_ROLE_NOTATION_SYSTEM_ID;
        }
        XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            };
            return XML_ROLE_NOTATION_NO_SYSTEM_ID;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                attlist1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTLIST_ELEMENT_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            };
            return XML_ROLE_ATTLIST_NONE;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                attlist2
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTRIBUTE_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
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
            let mut i: c_int = 0;
            i = 0;
            while i
                < (::core::mem::size_of::<[*const c_char; 8]>())
                    .wrapping_div(::core::mem::size_of::<*const c_char>())
                    as c_int
            {
                if (*enc).nameMatchesAscii(enc, ptr, end, types[i as usize]) != 0 {
                    (*state).handler = Some(
                        attlist8
                            as unsafe fn(
                                *mut PROLOG_STATE,
                                c_int,
                                *const c_char,
                                *const c_char,
                                *const ENCODING,
                            ) -> c_int,
                    );
                    return XML_ROLE_ATTRIBUTE_TYPE_CDATA + i;
                }
                i += 1;
            }
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_NOTATION as *const c_char) != 0
            {
                (*state).handler = Some(
                    attlist5
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ATTLIST_NONE;
            }
        }
        XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(
                attlist3
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_NMTOKEN | XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                attlist4
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTRIBUTE_ENUM_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(
                attlist8
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTLIST_NONE;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                attlist3
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(
                attlist6
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist6(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_NAME => {
            (*state).handler = Some(
                attlist7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTRIBUTE_NOTATION_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist7(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(
                attlist8
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTLIST_NONE;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                attlist6
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ATTLIST_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist8(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_IMPLIED as *const c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist1
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_IMPLIED_ATTRIBUTE_VALUE;
            }
            if (*enc).nameMatchesAscii(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_REQUIRED as *const c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist1
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_REQUIRED_ATTRIBUTE_VALUE;
            }
            if (*enc).nameMatchesAscii(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_FIXED as *const c_char,
            ) != 0
            {
                (*state).handler = Some(
                    attlist9
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_ATTLIST_NONE;
            }
        }
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                attlist1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_DEFAULT_ATTRIBUTE_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn attlist9(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ATTLIST_NONE,
        XML_TOK_LITERAL => {
            (*state).handler = Some(
                attlist1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_FIXED_ATTRIBUTE_VALUE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn element0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ELEMENT_NAME;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn element1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_EMPTY as *const c_char) != 0 {
                (*state).handler = Some(
                    declClose
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                (*state).role_none = XML_ROLE_ELEMENT_NONE;
                return XML_ROLE_CONTENT_EMPTY;
            }
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_ANY as *const c_char) != 0 {
                (*state).handler = Some(
                    declClose
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                (*state).role_none = XML_ROLE_ELEMENT_NONE;
                return XML_ROLE_CONTENT_ANY;
            }
        }
        XML_TOK_OPEN_PAREN => {
            (*state).handler = Some(
                element2
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).level = 1u32;
            return XML_ROLE_GROUP_OPEN;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn element2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_POUND_NAME => {
            if (*enc).nameMatchesAscii(
                enc,
                ptr.offset((*enc).minBytesPerChar as isize),
                end,
                &raw const KW_PCDATA as *const c_char,
            ) != 0
            {
                (*state).handler = Some(
                    element3
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_CONTENT_PCDATA;
            }
        }
        XML_TOK_OPEN_PAREN => {
            (*state).level = 2u32;
            (*state).handler = Some(
                element6
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_GROUP_OPEN;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_CONTENT_ELEMENT;
        }
        XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(
                element7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_CONTENT_ELEMENT_OPT;
        }
        XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(
                element7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_CONTENT_ELEMENT_REP;
        }
        XML_TOK_NAME_PLUS => {
            (*state).handler = Some(
                element7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_CONTENT_ELEMENT_PLUS;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn element3(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_CLOSE_PAREN => {
            (*state).handler = Some(
                declClose
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).role_none = XML_ROLE_ELEMENT_NONE;
            return XML_ROLE_GROUP_CLOSE;
        }
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(
                declClose
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).role_none = XML_ROLE_ELEMENT_NONE;
            return XML_ROLE_GROUP_CLOSE_REP;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                element4
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ELEMENT_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn element4(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element5
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_CONTENT_ELEMENT;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn element5(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).handler = Some(
                declClose
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).role_none = XML_ROLE_ELEMENT_NONE;
            return XML_ROLE_GROUP_CLOSE_REP;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                element4
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_ELEMENT_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn element6(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_OPEN_PAREN => {
            (*state).level = (*state).level.wrapping_add(1u32);
            return XML_ROLE_GROUP_OPEN;
        }
        XML_TOK_NAME | XML_TOK_PREFIXED_NAME => {
            (*state).handler = Some(
                element7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_CONTENT_ELEMENT;
        }
        XML_TOK_NAME_QUESTION => {
            (*state).handler = Some(
                element7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_CONTENT_ELEMENT_OPT;
        }
        XML_TOK_NAME_ASTERISK => {
            (*state).handler = Some(
                element7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_CONTENT_ELEMENT_REP;
        }
        XML_TOK_NAME_PLUS => {
            (*state).handler = Some(
                element7
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_CONTENT_ELEMENT_PLUS;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn element7(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_ELEMENT_NONE,
        XML_TOK_CLOSE_PAREN => {
            (*state).level = (*state).level.wrapping_sub(1u32);
            if (*state).level == 0u32 {
                (*state).handler = Some(
                    declClose
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                (*state).role_none = XML_ROLE_ELEMENT_NONE;
            }
            return XML_ROLE_GROUP_CLOSE;
        }
        XML_TOK_CLOSE_PAREN_ASTERISK => {
            (*state).level = (*state).level.wrapping_sub(1u32);
            if (*state).level == 0u32 {
                (*state).handler = Some(
                    declClose
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                (*state).role_none = XML_ROLE_ELEMENT_NONE;
            }
            return XML_ROLE_GROUP_CLOSE_REP;
        }
        XML_TOK_CLOSE_PAREN_QUESTION => {
            (*state).level = (*state).level.wrapping_sub(1u32);
            if (*state).level == 0u32 {
                (*state).handler = Some(
                    declClose
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                (*state).role_none = XML_ROLE_ELEMENT_NONE;
            }
            return XML_ROLE_GROUP_CLOSE_OPT;
        }
        XML_TOK_CLOSE_PAREN_PLUS => {
            (*state).level = (*state).level.wrapping_sub(1u32);
            if (*state).level == 0u32 {
                (*state).handler = Some(
                    declClose
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                (*state).role_none = XML_ROLE_ELEMENT_NONE;
            }
            return XML_ROLE_GROUP_CLOSE_PLUS;
        }
        XML_TOK_COMMA => {
            (*state).handler = Some(
                element6
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_GROUP_SEQUENCE;
        }
        XML_TOK_OR => {
            (*state).handler = Some(
                element6
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_GROUP_CHOICE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn condSect0(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_NAME => {
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_INCLUDE as *const c_char) != 0 {
                (*state).handler = Some(
                    condSect1
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_NONE;
            }
            if (*enc).nameMatchesAscii(enc, ptr, end, &raw const KW_IGNORE as *const c_char) != 0 {
                (*state).handler = Some(
                    condSect2
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                );
                return XML_ROLE_NONE;
            }
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn condSect1(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                externalSubset1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            (*state).includeLevel = (*state).includeLevel.wrapping_add(1u32);
            return XML_ROLE_NONE;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn condSect2(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return XML_ROLE_NONE,
        XML_TOK_OPEN_BRACKET => {
            (*state).handler = Some(
                externalSubset1
                    as unsafe fn(
                        *mut PROLOG_STATE,
                        c_int,
                        *const c_char,
                        *const c_char,
                        *const ENCODING,
                    ) -> c_int,
            );
            return XML_ROLE_IGNORE_SECT;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn declClose(
    mut state: *mut PROLOG_STATE,
    mut tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    match tok {
        XML_TOK_PROLOG_S => return (*state).role_none,
        XML_TOK_DECL_CLOSE => {
            (*state).handler = if (*state).documentEntity != 0 {
                Some(
                    internalSubset
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            } else {
                Some(
                    externalSubset1
                        as unsafe fn(
                            *mut PROLOG_STATE,
                            c_int,
                            *const c_char,
                            *const c_char,
                            *const ENCODING,
                        ) -> c_int,
                )
            };
            return (*state).role_none;
        }
        _ => {}
    }
    return common(state, tok);
}

unsafe fn error(
    mut _state: *mut PROLOG_STATE,
    mut _tok: c_int,
    mut _ptr: *const c_char,
    mut _end: *const c_char,
    mut _enc: *const ENCODING,
) -> c_int {
    return XML_ROLE_NONE;
}

unsafe fn common(mut state: *mut PROLOG_STATE, mut tok: c_int) -> c_int {
    if (*state).documentEntity == 0 && tok == XML_TOK_PARAM_ENTITY_REF_1 {
        return XML_ROLE_INNER_PARAM_ENTITY_REF;
    }
    (*state).handler = Some(
        error
            as unsafe fn(
                *mut PROLOG_STATE,
                c_int,
                *const c_char,
                *const c_char,
                *const ENCODING,
            ) -> c_int,
    );
    return XML_ROLE_ERROR;
}
pub(crate) unsafe fn XmlPrologStateInit(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(
        prolog0
            as unsafe fn(
                *mut PROLOG_STATE,
                c_int,
                *const c_char,
                *const c_char,
                *const ENCODING,
            ) -> c_int,
    );
    (*state).documentEntity = 1;
    (*state).includeLevel = 0u32;
    (*state).inEntityValue = 0;
}
pub(crate) unsafe fn XmlPrologStateInitExternalEntity(mut state: *mut PROLOG_STATE) {
    (*state).handler = Some(
        externalSubset0
            as unsafe fn(
                *mut PROLOG_STATE,
                c_int,
                *const c_char,
                *const c_char,
                *const ENCODING,
            ) -> c_int,
    );
    (*state).documentEntity = 0;
    (*state).includeLevel = 0u32;
}
