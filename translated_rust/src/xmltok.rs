// =============== BEGIN xmltok_h ================
pub const XML_TOK_TRAILING_RSQB: ::core::ffi::c_int = -5;

pub const XML_TOK_TRAILING_RSQB_1: ::core::ffi::c_int = -5 as ::core::ffi::c_int;

pub const XML_TOK_NONE: ::core::ffi::c_int = -4;

pub const XML_TOK_NONE_1: ::core::ffi::c_int = -4 as ::core::ffi::c_int;

pub const XML_TOK_TRAILING_CR: ::core::ffi::c_int = -3;

pub const XML_TOK_TRAILING_CR_1: ::core::ffi::c_int = -3 as ::core::ffi::c_int;

pub const XML_TOK_PARTIAL_CHAR: ::core::ffi::c_int = -2;

pub const XML_TOK_PARTIAL_CHAR_1: ::core::ffi::c_int = -2 as ::core::ffi::c_int;

pub const XML_TOK_PARTIAL: ::core::ffi::c_int = -1;

pub const XML_TOK_PARTIAL_1: ::core::ffi::c_int = -1 as ::core::ffi::c_int;

pub const XML_TOK_INVALID: ::core::ffi::c_int = 0;

pub const XML_TOK_INVALID_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const XML_TOK_START_TAG_WITH_ATTS: ::core::ffi::c_int = 1;

pub const XML_TOK_START_TAG_WITH_ATTS_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub const XML_TOK_START_TAG_NO_ATTS: ::core::ffi::c_int = 2;

pub const XML_TOK_START_TAG_NO_ATTS_1: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: ::core::ffi::c_int = 3;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: ::core::ffi::c_int = 4;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub const XML_TOK_END_TAG: ::core::ffi::c_int = 5;

pub const XML_TOK_END_TAG_1: ::core::ffi::c_int = 5 as ::core::ffi::c_int;

pub const XML_TOK_DATA_CHARS: ::core::ffi::c_int = 6;

pub const XML_TOK_DATA_CHARS_1: ::core::ffi::c_int = 6 as ::core::ffi::c_int;

pub const XML_TOK_DATA_NEWLINE: ::core::ffi::c_int = 7;

pub const XML_TOK_DATA_NEWLINE_1: ::core::ffi::c_int = 7 as ::core::ffi::c_int;

pub const XML_TOK_CDATA_SECT_OPEN: ::core::ffi::c_int = 8;

pub const XML_TOK_CDATA_SECT_OPEN_1: ::core::ffi::c_int = 8 as ::core::ffi::c_int;

pub const XML_TOK_ENTITY_REF: ::core::ffi::c_int = 9;

pub const XML_TOK_ENTITY_REF_1: ::core::ffi::c_int = 9 as ::core::ffi::c_int;

pub const XML_TOK_CHAR_REF: ::core::ffi::c_int = 10;

pub const XML_TOK_CHAR_REF_1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;

pub const XML_TOK_PI: ::core::ffi::c_int = 11;

pub const XML_TOK_PI_1: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub const XML_TOK_XML_DECL: ::core::ffi::c_int = 12;

pub const XML_TOK_XML_DECL_1: ::core::ffi::c_int = 12 as ::core::ffi::c_int;

pub const XML_TOK_COMMENT: ::core::ffi::c_int = 13;

pub const XML_TOK_COMMENT_1: ::core::ffi::c_int = 13 as ::core::ffi::c_int;

pub const XML_TOK_BOM: ::core::ffi::c_int = 14;

pub const XML_TOK_BOM_1: ::core::ffi::c_int = 14 as ::core::ffi::c_int;

pub const XML_TOK_PROLOG_S: ::core::ffi::c_int = 15;

pub const XML_TOK_PROLOG_S_1: ::core::ffi::c_int = 15 as ::core::ffi::c_int;

pub const XML_TOK_DECL_OPEN: ::core::ffi::c_int = 16;

pub const XML_TOK_DECL_OPEN_1: ::core::ffi::c_int = 16 as ::core::ffi::c_int;

pub const XML_TOK_DECL_CLOSE: ::core::ffi::c_int = 17;

pub const XML_TOK_DECL_CLOSE_1: ::core::ffi::c_int = 17 as ::core::ffi::c_int;

pub const XML_TOK_NAME: ::core::ffi::c_int = 18;

pub const XML_TOK_NMTOKEN: ::core::ffi::c_int = 19;

pub const XML_TOK_NMTOKEN_1: ::core::ffi::c_int = 19 as ::core::ffi::c_int;

pub const XML_TOK_POUND_NAME: ::core::ffi::c_int = 20;

pub const XML_TOK_POUND_NAME_1: ::core::ffi::c_int = 20 as ::core::ffi::c_int;

pub const XML_TOK_OR: ::core::ffi::c_int = 21;

pub const XML_TOK_OR_1: ::core::ffi::c_int = 21 as ::core::ffi::c_int;

pub const XML_TOK_PERCENT: ::core::ffi::c_int = 22;

pub const XML_TOK_PERCENT_1: ::core::ffi::c_int = 22 as ::core::ffi::c_int;

pub const XML_TOK_OPEN_PAREN: ::core::ffi::c_int = 23;

pub const XML_TOK_OPEN_PAREN_1: ::core::ffi::c_int = 23 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_PAREN: ::core::ffi::c_int = 24;

pub const XML_TOK_CLOSE_PAREN_1: ::core::ffi::c_int = 24 as ::core::ffi::c_int;

pub const XML_TOK_OPEN_BRACKET: ::core::ffi::c_int = 25;

pub const XML_TOK_OPEN_BRACKET_1: ::core::ffi::c_int = 25 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_BRACKET: ::core::ffi::c_int = 26;

pub const XML_TOK_CLOSE_BRACKET_1: ::core::ffi::c_int = 26 as ::core::ffi::c_int;

pub const XML_TOK_LITERAL: ::core::ffi::c_int = 27;

pub const XML_TOK_LITERAL_1: ::core::ffi::c_int = 27 as ::core::ffi::c_int;

pub const XML_TOK_PARAM_ENTITY_REF: ::core::ffi::c_int = 28;

pub const XML_TOK_PARAM_ENTITY_REF_1: ::core::ffi::c_int = 28 as ::core::ffi::c_int;

pub const XML_TOK_INSTANCE_START: ::core::ffi::c_int = 29 as ::core::ffi::c_int;

pub const XML_TOK_INSTANCE_START_1: ::core::ffi::c_int = 29;

pub const XML_TOK_NAME_QUESTION: ::core::ffi::c_int = 30;

pub const XML_TOK_NAME_QUESTION_1: ::core::ffi::c_int = 30 as ::core::ffi::c_int;

pub const XML_TOK_NAME_ASTERISK: ::core::ffi::c_int = 31;

pub const XML_TOK_NAME_ASTERISK_1: ::core::ffi::c_int = 31 as ::core::ffi::c_int;

pub const XML_TOK_NAME_PLUS: ::core::ffi::c_int = 32;

pub const XML_TOK_NAME_PLUS_1: ::core::ffi::c_int = 32 as ::core::ffi::c_int;

pub const XML_TOK_COND_SECT_OPEN: ::core::ffi::c_int = 33;

pub const XML_TOK_COND_SECT_OPEN_1: ::core::ffi::c_int = 33 as ::core::ffi::c_int;

pub const XML_TOK_COND_SECT_CLOSE: ::core::ffi::c_int = 34;

pub const XML_TOK_COND_SECT_CLOSE_1: ::core::ffi::c_int = 34 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_PAREN_QUESTION: ::core::ffi::c_int = 35;

pub const XML_TOK_CLOSE_PAREN_QUESTION_1: ::core::ffi::c_int = 35 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_PAREN_ASTERISK: ::core::ffi::c_int = 36;

pub const XML_TOK_CLOSE_PAREN_ASTERISK_1: ::core::ffi::c_int = 36 as ::core::ffi::c_int;

pub const XML_TOK_CLOSE_PAREN_PLUS: ::core::ffi::c_int = 37;

pub const XML_TOK_CLOSE_PAREN_PLUS_1: ::core::ffi::c_int = 37 as ::core::ffi::c_int;

pub const XML_TOK_COMMA: ::core::ffi::c_int = 38;

pub const XML_TOK_COMMA_1: ::core::ffi::c_int = 38 as ::core::ffi::c_int;

pub const XML_TOK_ATTRIBUTE_VALUE_S: ::core::ffi::c_int = 39;

pub const XML_TOK_ATTRIBUTE_VALUE_S_1: ::core::ffi::c_int = 39 as ::core::ffi::c_int;

pub const XML_TOK_CDATA_SECT_CLOSE: ::core::ffi::c_int = 40;

pub const XML_TOK_CDATA_SECT_CLOSE_1: ::core::ffi::c_int = 40 as ::core::ffi::c_int;

pub const XML_TOK_PREFIXED_NAME: ::core::ffi::c_int = 41;

pub const XML_TOK_IGNORE_SECT: ::core::ffi::c_int = 42;

pub const XML_TOK_IGNORE_SECT_1: ::core::ffi::c_int = 42 as ::core::ffi::c_int;

pub const XML_PROLOG_STATE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const XML_CONTENT_STATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub type POSITION = crate::src::xmltok::position;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct position {
    pub lineNumber: crate::expat_external_h::XML_Size,
    pub columnNumber: crate::expat_external_h::XML_Size,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ATTRIBUTE {
    pub name: *const ::core::ffi::c_char,
    pub valuePtr: *const ::core::ffi::c_char,
    pub valueEnd: *const ::core::ffi::c_char,
    pub normalized: ::core::ffi::c_char,
}

pub type ENCODING = crate::src::xmltok::encoding;

pub type SCANNER = Option<
    unsafe extern "C" fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

pub type XML_Convert_Result = ::core::ffi::c_uint;

pub const XML_CONVERT_COMPLETED: crate::src::xmltok::XML_Convert_Result = 0;

pub const XML_CONVERT_INPUT_INCOMPLETE: crate::src::xmltok::XML_Convert_Result = 1;

pub const XML_CONVERT_OUTPUT_EXHAUSTED: crate::src::xmltok::XML_Convert_Result = 2;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct encoding {
    pub scanners: [crate::src::xmltok::SCANNER; 4],
    pub literalScanners: [crate::src::xmltok::SCANNER; 2],
    pub nameMatchesAscii: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub nameLength: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub skipS: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
    pub getAtts: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut crate::src::xmltok::ATTRIBUTE,
        ) -> ::core::ffi::c_int,
    >,
    pub charRefNumber: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub predefinedEntityName: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub updatePosition: Option<
        extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            &mut crate::src::xmltok::POSITION,
        ) -> (),
    >,
    pub isPublicId: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub utf8Convert: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> crate::src::xmltok::XML_Convert_Result,
    >,
    pub utf16Convert: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_ushort,
            *const ::core::ffi::c_ushort,
        ) -> crate::src::xmltok::XML_Convert_Result,
    >,
    pub minBytesPerChar: ::core::ffi::c_int,
    pub isUtf8: ::core::ffi::c_char,
    pub isUtf16: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct INIT_ENCODING {
    pub initEnc: crate::src::xmltok::ENCODING,
    pub encPtr: *mut *const crate::src::xmltok::ENCODING,
}

pub type CONVERTER = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

pub mod xmltok_impl_c {

    pub extern "C" fn normal_scanComment(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_comment(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_ascii_byte,
            super::normal_encoded_byte_type,
            super::normal_invalid_char,
        )
    }

    pub extern "C" fn normal_scanDecl(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_decl(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
            normal_scanComment,
        )
    }

    pub fn normal_checkPiTarget(
        mut _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        tok: &mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        *tok = crate::src::xmltok::XML_TOK_PI_1;
        if super::byte_distance(ptr, end) != 3 as ::core::ffi::c_long {
            return 1 as ::core::ffi::c_int;
        }
        match super::normal_ascii_byte(ptr) {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.wrapping_add(1);
        match super::normal_ascii_byte(ptr) {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.wrapping_add(1);
        match super::normal_ascii_byte(ptr) {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        if upper != 0 {
            return 0 as ::core::ffi::c_int;
        }
        *tok = crate::src::xmltok::XML_TOK_XML_DECL_1;
        return 1 as ::core::ffi::c_int;
    }

    pub extern "C" fn normal_scanPi(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_pi(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_ascii_byte,
            super::normal_encoded_byte_type,
            normal_checkPiTarget,
            super::normal_nonascii_name_char,
            super::normal_nonascii_name_char,
            super::normal_name_start_char,
            super::normal_name_char,
            super::normal_pi_content_char,
        )
    }

    pub extern "C" fn normal_scanCdataSection(
        mut _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        static CDATA_LSQB: [::core::ffi::c_char; 6] = [
            crate::ascii_h::ASCII_C as ::core::ffi::c_char,
            crate::ascii_h::ASCII_D as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_T as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_LSQB as ::core::ffi::c_char,
        ];
        if super::byte_distance(ptr, end) < 6 as ::core::ffi::c_long {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        for expected in CDATA_LSQB {
            if super::normal_ascii_byte(ptr) != expected as ::core::ffi::c_int {
                super::set_next_tok_ptr(nextTokPtr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(1);
        }
        super::set_next_tok_ptr(nextTokPtr, ptr);
        return crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    }

    pub extern "C" fn normal_cdataSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return super::cdata_section_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_ascii_byte,
            super::normal_encoded_byte_type,
            super::normal_invalid_char,
        );
    }

    pub extern "C" fn normal_scanEndTag(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_end_tag(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
            super::normal_nonascii_name_start_char,
            super::normal_nonascii_name_char,
            super::normal_name_start_char,
            super::normal_name_char,
        )
    }

    pub extern "C" fn normal_scanHexCharRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_hex_char_ref(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
        )
    }

    pub extern "C" fn normal_scanCharRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_char_ref(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_ascii_byte,
            super::normal_encoded_byte_type,
        )
    }

    pub extern "C" fn normal_scanRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_ref(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
            super::normal_ascii_byte,
            super::normal_nonascii_name_start_char,
            super::normal_nonascii_name_char,
            super::normal_name_start_char,
            super::normal_name_char,
        )
    }

    pub extern "C" fn normal_scanAtts(
        enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while super::byte_distance(ptr, end) >= 1 {
            let mut scan_value = false;
            match super::normal_encoded_byte_type(enc, ptr) {
                29 => {
                    super::set_next_tok_ptr(nextTokPtr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                22 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.wrapping_add(1);
                }
                t @ (5 | 6 | 7) => match super::normal_scan_name_width(enc, ptr, end, t, false) {
                    Ok(width) => ptr = ptr.wrapping_add(width),
                    Err(result) => {
                        if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                        }
                        return result;
                    }
                },
                23 => {
                    if hadColon != 0 {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    hadColon = 1;
                    ptr = ptr.wrapping_add(1);
                    if super::byte_distance(ptr, end) < 1 {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    match super::normal_encoded_byte_type(enc, ptr) {
                        29 => {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                        22 | 24 => ptr = ptr.wrapping_add(1),
                        t @ (5 | 6 | 7) => {
                            match super::normal_scan_name_width(enc, ptr, end, t, true) {
                                Ok(width) => ptr = ptr.wrapping_add(width),
                                Err(result) => {
                                    if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                                        super::set_next_tok_ptr(nextTokPtr, ptr);
                                    }
                                    return result;
                                }
                            }
                        }
                        _ => {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                }
                21 | 9 | 10 => loop {
                    ptr = ptr.wrapping_add(1);
                    if super::byte_distance(ptr, end) < 1 {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    let t = super::normal_encoded_byte_type(enc, ptr);
                    if t == crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int {
                        scan_value = true;
                        break;
                    }
                    match t {
                        21 | 10 | 9 => {}
                        _ => {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                },
                14 => scan_value = true,
                _ => {
                    super::set_next_tok_ptr(nextTokPtr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
            if !scan_value {
                continue;
            }

            hadColon = 0;
            let open = loop {
                ptr = ptr.wrapping_add(1);
                if super::byte_distance(ptr, end) < 1 {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                let open = super::normal_encoded_byte_type(enc, ptr);
                if open == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                    || open == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                {
                    break open;
                }
                match open {
                    21 | 10 | 9 => {}
                    _ => {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
            };
            ptr = ptr.wrapping_add(1);

            loop {
                if super::byte_distance(ptr, end) < 1 {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                let t = super::normal_encoded_byte_type(enc, ptr);
                if t == open {
                    break;
                }
                match t {
                    5 | 6 | 7 => {
                        let width = super::lead_byte_width(t).expect("lead byte width");
                        if super::byte_distance(ptr, end) < width as ::core::ffi::c_long {
                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        if super::normal_invalid_char(enc, ptr, width) {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                        ptr = ptr.wrapping_add(width);
                    }
                    0 | 1 | 8 => {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    3 => {
                        let tok = normal_scanRef(enc, ptr.wrapping_add(1), end, &mut ptr);
                        if tok <= 0 {
                            if tok == crate::src::xmltok::XML_TOK_INVALID_1 {
                                super::set_next_tok_ptr(nextTokPtr, ptr);
                            }
                            return tok;
                        }
                    }
                    2 => {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    _ => {
                        ptr = ptr.wrapping_add(1);
                    }
                }
            }
            ptr = ptr.wrapping_add(1);
            if super::byte_distance(ptr, end) < 1 {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }

            let close_or_next = loop {
                match super::normal_encoded_byte_type(enc, ptr) {
                    21 | 9 | 10 => {
                        ptr = ptr.wrapping_add(1);
                        if super::byte_distance(ptr, end) < 1 {
                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                        }
                    }
                    29 => {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    22 | 24 => {
                        ptr = ptr.wrapping_add(1);
                        break None;
                    }
                    t @ (5 | 6 | 7) => {
                        match super::normal_scan_name_width(enc, ptr, end, t, true) {
                            Ok(width) => ptr = ptr.wrapping_add(width),
                            Err(result) => {
                                if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                                    super::set_next_tok_ptr(nextTokPtr, ptr);
                                }
                                return result;
                            }
                        }
                        break None;
                    }
                    11 => break Some(crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1),
                    17 => break Some(crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1),
                    _ => {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
            };

            if let Some(tok) = close_or_next {
                if tok == crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1 {
                    ptr = ptr.wrapping_add(1);
                    if super::byte_distance(ptr, end) < 1 {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if super::normal_ascii_byte(ptr) != crate::ascii_h::ASCII_GT {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    super::set_next_tok_ptr(nextTokPtr, ptr.wrapping_add(1));
                    return tok;
                }
                super::set_next_tok_ptr(nextTokPtr, ptr.wrapping_add(1));
                return tok;
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    pub extern "C" fn normal_scanLt(
        enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        if super::byte_distance(ptr, end) < 1 {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }

        match super::normal_encoded_byte_type(enc, ptr) {
            29 => {
                super::set_next_tok_ptr(nextTokPtr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            22 | 24 => ptr = ptr.wrapping_add(1),
            t @ (5 | 6 | 7) => match super::normal_scan_name_width(enc, ptr, end, t, true) {
                Ok(width) => ptr = ptr.wrapping_add(width),
                Err(result) => {
                    if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                    }
                    return result;
                }
            },
            16 => {
                ptr = ptr.wrapping_add(1);
                if super::byte_distance(ptr, end) < 1 {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                match super::normal_encoded_byte_type(enc, ptr) {
                    27 => {
                        return normal_scanComment(enc, ptr.wrapping_add(1), end, nextTokPtr);
                    }
                    20 => {
                        return normal_scanCdataSection(enc, ptr.wrapping_add(1), end, nextTokPtr);
                    }
                    _ => {}
                }
                super::set_next_tok_ptr(nextTokPtr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            15 => {
                return normal_scanPi(enc, ptr.wrapping_add(1), end, nextTokPtr);
            }
            17 => {
                return normal_scanEndTag(enc, ptr.wrapping_add(1), end, nextTokPtr);
            }
            _ => {
                super::set_next_tok_ptr(nextTokPtr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
        }

        hadColon = 0 as ::core::ffi::c_int;
        while super::byte_distance(ptr, end) >= 1 {
            match super::normal_encoded_byte_type(enc, ptr) {
                29 => {
                    super::set_next_tok_ptr(nextTokPtr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                22 | 24 | 25 | 26 | 27 => ptr = ptr.wrapping_add(1),
                t @ (5 | 6 | 7) => match super::normal_scan_name_width(enc, ptr, end, t, false) {
                    Ok(width) => ptr = ptr.wrapping_add(width),
                    Err(result) => {
                        if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                        }
                        return result;
                    }
                },
                23 => {
                    if hadColon != 0 {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    hadColon = 1 as ::core::ffi::c_int;
                    ptr = ptr.wrapping_add(1);
                    if super::byte_distance(ptr, end) < 1 {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    match super::normal_encoded_byte_type(enc, ptr) {
                        29 => {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                        22 | 24 => ptr = ptr.wrapping_add(1),
                        t @ (5 | 6 | 7) => {
                            match super::normal_scan_name_width(enc, ptr, end, t, true) {
                                Ok(width) => ptr = ptr.wrapping_add(width),
                                Err(result) => {
                                    if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                                        super::set_next_tok_ptr(nextTokPtr, ptr);
                                    }
                                    return result;
                                }
                            }
                        }
                        _ => {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                }
                21 | 9 | 10 => {
                    ptr = ptr.wrapping_add(1);
                    loop {
                        if super::byte_distance(ptr, end) < 1 {
                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                        }
                        match super::normal_encoded_byte_type(enc, ptr) {
                            29 => {
                                super::set_next_tok_ptr(nextTokPtr, ptr);
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            22 | 24 => {
                                ptr = ptr.wrapping_add(1);
                                return normal_scanAtts(enc, ptr, end, nextTokPtr);
                            }
                            t @ (5 | 6 | 7) => {
                                match super::normal_scan_name_width(enc, ptr, end, t, true) {
                                    Ok(width) => ptr = ptr.wrapping_add(width),
                                    Err(result) => {
                                        if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                                            super::set_next_tok_ptr(nextTokPtr, ptr);
                                        }
                                        return result;
                                    }
                                }
                                return normal_scanAtts(enc, ptr, end, nextTokPtr);
                            }
                            11 => {
                                super::set_next_tok_ptr(nextTokPtr, ptr.wrapping_add(1));
                                return crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS_1;
                            }
                            17 => {
                                ptr = ptr.wrapping_add(1);
                                if super::byte_distance(ptr, end) < 1 {
                                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                                }
                                if super::normal_ascii_byte(ptr) != crate::ascii_h::ASCII_GT {
                                    super::set_next_tok_ptr(nextTokPtr, ptr);
                                    return crate::src::xmltok::XML_TOK_INVALID_1;
                                }
                                super::set_next_tok_ptr(nextTokPtr, ptr.wrapping_add(1));
                                return crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.wrapping_add(1);
                            }
                            _ => {
                                super::set_next_tok_ptr(nextTokPtr, ptr);
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                    }
                }
                11 => {
                    super::set_next_tok_ptr(nextTokPtr, ptr.wrapping_add(1));
                    return crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS_1;
                }
                17 => {
                    ptr = ptr.wrapping_add(1);
                    if super::byte_distance(ptr, end) < 1 {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if super::normal_ascii_byte(ptr) != crate::ascii_h::ASCII_GT {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    super::set_next_tok_ptr(nextTokPtr, ptr.wrapping_add(1));
                    return crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                }
                _ => {
                    super::set_next_tok_ptr(nextTokPtr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
        }
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    pub extern "C" fn normal_contentTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::content_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
            super::normal_ascii_byte,
            super::normal_invalid_char,
            normal_scanLt,
            normal_scanRef,
        )
    }

    pub extern "C" fn normal_scanPercent(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_percent(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
            super::normal_nonascii_name_start_char,
            super::normal_nonascii_name_char,
            super::normal_name_start_char,
            super::normal_name_char,
        )
    }

    pub extern "C" fn normal_scanPoundName(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_pound_name(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
            super::normal_nonascii_name_start_char,
            super::normal_nonascii_name_char,
            super::normal_name_start_char,
            super::normal_name_char,
        )
    }

    pub extern "C" fn normal_scanLit(
        open: ::core::ffi::c_int,
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_lit(
            open,
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
            super::normal_invalid_char,
        )
    }

    pub extern "C" fn normal_prologTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::prolog_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_ascii_byte,
            super::normal_encoded_byte_type,
            super::normal_nonascii_name_start_char,
            super::normal_nonascii_name_char,
            super::normal_name_start_char,
            super::normal_name_char,
            normal_scanLit,
            normal_scanDecl,
            normal_scanPi,
            normal_scanPercent,
            normal_scanPoundName,
        )
    }

    pub extern "C" fn normal_attributeValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::literal_value_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
            normal_scanRef,
            None,
            true,
            true,
        )
    }

    pub extern "C" fn normal_entityValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::literal_value_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_encoded_byte_type,
            normal_scanRef,
            Some(normal_scanPercent),
            false,
            false,
        )
    }

    pub extern "C" fn normal_ignoreSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::ignore_section_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            1,
            super::normal_ascii_byte,
            super::normal_encoded_byte_type,
            super::normal_invalid_char,
        )
    }

    pub extern "C" fn normal_isPublicId(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::is_public_id(
            enc,
            ptr,
            end,
            badPtr,
            1,
            super::normal_ascii_byte,
            super::normal_encoded_byte_type,
        )
    }

    pub extern "C" fn normal_getAtts(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        attsMax: ::core::ffi::c_int,
        atts: *mut crate::src::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        super::get_atts(
            enc,
            ptr,
            attsMax,
            atts,
            1,
            super::normal_ascii_byte,
            super::normal_encoded_byte_type,
        )
    }

    pub extern "C" fn normal_charRefNumber(
        mut _enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::char_ref_number(ptr, 1, super::normal_ascii_byte)
    }

    pub extern "C" fn normal_predefinedEntityName(
        mut _enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::predefined_entity_name(ptr, end, 1, super::normal_ascii_byte)
    }

    pub extern "C" fn normal_nameMatchesAscii(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr1: *const ::core::ffi::c_char,
        mut end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while super::utf8_byte(ptr2, 0) != 0 {
            if super::byte_distance(ptr1, end1) < 1 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
            if !(super::utf8_byte(ptr1, 0) == super::utf8_byte(ptr2, 0)) {
                return 0 as ::core::ffi::c_int;
            }
            ptr1 = ptr1.wrapping_add(1);
            ptr2 = ptr2.wrapping_add(1);
        }
        return (ptr1 == end1) as ::core::ffi::c_int;
    }

    pub extern "C" fn normal_nameLength(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match super::normal_byte_type(enc, super::utf8_byte(ptr, 0)) {
                5 => {
                    ptr = ptr.wrapping_add(2);
                }
                6 => {
                    ptr = ptr.wrapping_add(3);
                }
                7 => {
                    ptr = ptr.wrapping_add(4);
                }
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.wrapping_add(1);
                }
                _ => {
                    return super::byte_distance(start, ptr) as ::core::ffi::c_int;
                }
            }
        }
    }

    pub extern "C" fn normal_skipS(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        loop {
            match super::normal_byte_type(enc, super::utf8_byte(ptr, 0)) {
                10 | 9 | 21 => {
                    ptr = ptr.wrapping_add(1);
                }
                _ => return ptr,
            }
        }
    }

    pub extern "C" fn normal_updatePosition(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        pos: &mut crate::src::xmltok::POSITION,
    ) {
        while super::byte_distance(ptr, end)
            >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match super::normal_byte_type(enc, super::utf8_byte(ptr, 0)) {
                5 => {
                    ptr = ptr.wrapping_add(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.wrapping_add(3);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.wrapping_add(4);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                10 => {
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_add(1);
                }
                9 => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_add(1);
                    if super::byte_distance(ptr, end)
                        >= (1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                        && super::normal_byte_type(enc, super::utf8_byte(ptr, 0))
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.wrapping_add(1);
                    }
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                }
                _ => {
                    ptr = ptr.wrapping_add(1);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub extern "C" fn little2_scanComment(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_comment(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_ascii_byte,
            super::little2_encoded_byte_type,
            super::never_invalid_char,
        )
    }

    pub extern "C" fn little2_scanDecl(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_decl(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
            little2_scanComment,
        )
    }

    pub fn little2_checkPiTarget(
        mut _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        tok: &mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        *tok = crate::src::xmltok::XML_TOK_PI_1;
        if super::byte_distance(ptr, end) != 6 as ::core::ffi::c_long {
            return 1 as ::core::ffi::c_int;
        }
        match super::little2_ascii_byte(ptr) {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.wrapping_add(2);
        match super::little2_ascii_byte(ptr) {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.wrapping_add(2);
        match super::little2_ascii_byte(ptr) {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        if upper != 0 {
            return 0 as ::core::ffi::c_int;
        }
        *tok = crate::src::xmltok::XML_TOK_XML_DECL_1;
        return 1 as ::core::ffi::c_int;
    }

    pub extern "C" fn little2_scanPi(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_pi(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_ascii_byte,
            super::little2_encoded_byte_type,
            little2_checkPiTarget,
            super::little2_nonascii_name_start_char,
            super::little2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
            super::always_lead_char,
        )
    }

    pub extern "C" fn little2_scanCdataSection(
        mut _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        static CDATA_LSQB: [::core::ffi::c_char; 6] = [
            crate::ascii_h::ASCII_C as ::core::ffi::c_char,
            crate::ascii_h::ASCII_D as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_T as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_LSQB as ::core::ffi::c_char,
        ];
        if super::byte_distance(ptr, end) < 12 as ::core::ffi::c_long {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        for expected in CDATA_LSQB {
            if super::little2_ascii_byte(ptr) != expected as ::core::ffi::c_int {
                super::set_next_tok_ptr(nextTokPtr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(2);
        }
        super::set_next_tok_ptr(nextTokPtr, ptr);
        return crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    }

    pub extern "C" fn little2_cdataSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return super::cdata_section_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_ascii_byte,
            super::little2_encoded_byte_type,
            super::never_invalid_char,
        );
    }

    pub extern "C" fn little2_scanEndTag(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_end_tag(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
            super::little2_nonascii_name_start_char,
            super::little2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
        )
    }

    pub extern "C" fn little2_scanHexCharRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_hex_char_ref(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
        )
    }

    pub extern "C" fn little2_scanCharRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_char_ref(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_ascii_byte,
            super::little2_encoded_byte_type,
        )
    }

    pub extern "C" fn little2_scanRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_ref(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
            super::little2_ascii_byte,
            super::little2_nonascii_name_start_char,
            super::little2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
        )
    }

    #[derive(Copy, Clone)]
    enum Utf16ByteOrder {
        Little,
        Big,
    }

    fn utf16_encoded_byte_type(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        order: Utf16ByteOrder,
    ) -> ::core::ffi::c_int {
        match order {
            Utf16ByteOrder::Little => super::little2_encoded_byte_type(enc, ptr),
            Utf16ByteOrder::Big => super::big2_encoded_byte_type(enc, ptr),
        }
    }

    fn utf16_low_high(
        ptr: *const ::core::ffi::c_char,
        order: Utf16ByteOrder,
    ) -> (::core::ffi::c_int, ::core::ffi::c_int) {
        match order {
            Utf16ByteOrder::Little => (super::utf8_byte(ptr, 0), super::utf8_byte(ptr, 1)),
            Utf16ByteOrder::Big => (super::utf8_byte(ptr, 1), super::utf8_byte(ptr, 0)),
        }
    }

    fn utf16_char_in_pages(
        ptr: *const ::core::ffi::c_char,
        pages: &[::core::ffi::c_uchar; 256],
        order: Utf16ByteOrder,
    ) -> bool {
        let (low, high) = utf16_low_high(ptr, order);
        let page = pages[high as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int;
        let low = low as ::core::ffi::c_uchar as ::core::ffi::c_int;
        namingBitmap[((page << 3) + (low >> 5)) as usize]
            & ((1 as ::core::ffi::c_uint) << (low & 0x1f))
            != 0
    }

    fn utf16_ascii_byte(
        ptr: *const ::core::ffi::c_char,
        order: Utf16ByteOrder,
    ) -> ::core::ffi::c_int {
        match order {
            Utf16ByteOrder::Little => super::little2_ascii_byte(ptr),
            Utf16ByteOrder::Big => super::big2_ascii_byte(ptr),
        }
    }

    fn utf16_invalid_lead_or_partial(
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
        width: ::core::ffi::c_long,
    ) -> ::core::ffi::c_int {
        if super::byte_distance(ptr, end) < width {
            crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1
        } else {
            super::set_next_tok_ptr(nextTokPtr, ptr);
            crate::src::xmltok::XML_TOK_INVALID_1
        }
    }

    fn utf16_scan_attr_name_start(
        ptr: &mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        enc: *const crate::src::xmltok::ENCODING,
        nextTokPtr: *mut *const ::core::ffi::c_char,
        order: Utf16ByteOrder,
    ) -> Result<(), ::core::ffi::c_int> {
        if super::byte_distance(*ptr, end) < 2 {
            return Err(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }
        match utf16_encoded_byte_type(enc, *ptr, order) {
            29 => {
                if !utf16_char_in_pages(*ptr, &nmstrtPages, order) {
                    super::set_next_tok_ptr(nextTokPtr, *ptr);
                    return Err(crate::src::xmltok::XML_TOK_INVALID_1);
                }
                *ptr = ptr.wrapping_add(2);
                Ok(())
            }
            22 | 24 => {
                *ptr = ptr.wrapping_add(2);
                Ok(())
            }
            5 => Err(utf16_invalid_lead_or_partial(*ptr, end, nextTokPtr, 2)),
            6 => Err(utf16_invalid_lead_or_partial(*ptr, end, nextTokPtr, 3)),
            7 => Err(utf16_invalid_lead_or_partial(*ptr, end, nextTokPtr, 4)),
            _ => {
                super::set_next_tok_ptr(nextTokPtr, *ptr);
                Err(crate::src::xmltok::XML_TOK_INVALID_1)
            }
        }
    }

    fn scan_utf16_after_attr_value(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: &mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
        order: Utf16ByteOrder,
    ) -> Result<Option<::core::ffi::c_int>, ::core::ffi::c_int> {
        if super::byte_distance(*ptr, end) < 2 {
            return Err(crate::src::xmltok::XML_TOK_PARTIAL_1);
        }
        let mut saw_space = false;
        loop {
            match utf16_encoded_byte_type(enc, *ptr, order) {
                21 | 9 | 10 => {
                    saw_space = true;
                    *ptr = ptr.wrapping_add(2);
                    if super::byte_distance(*ptr, end) < 2 {
                        return Err(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                }
                29 if saw_space => {
                    if !utf16_char_in_pages(*ptr, &nmstrtPages, order) {
                        super::set_next_tok_ptr(nextTokPtr, *ptr);
                        return Err(crate::src::xmltok::XML_TOK_INVALID_1);
                    }
                    *ptr = ptr.wrapping_add(2);
                    return Ok(None);
                }
                22 | 24 if saw_space => {
                    *ptr = ptr.wrapping_add(2);
                    return Ok(None);
                }
                5 if saw_space => {
                    return Err(utf16_invalid_lead_or_partial(*ptr, end, nextTokPtr, 2));
                }
                6 if saw_space => {
                    return Err(utf16_invalid_lead_or_partial(*ptr, end, nextTokPtr, 3));
                }
                7 if saw_space => {
                    return Err(utf16_invalid_lead_or_partial(*ptr, end, nextTokPtr, 4));
                }
                11 => return Ok(Some(crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1)),
                17 => return Ok(Some(crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1)),
                _ => {
                    super::set_next_tok_ptr(nextTokPtr, *ptr);
                    return Err(crate::src::xmltok::XML_TOK_INVALID_1);
                }
            }
        }
    }

    fn scan_utf16_atts(
        enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
        order: Utf16ByteOrder,
        scan_ref: extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut had_colon = false;
        while super::byte_distance(ptr, end) >= 2 {
            let mut scan_value = false;
            match utf16_encoded_byte_type(enc, ptr, order) {
                29 => {
                    if !utf16_char_in_pages(ptr, &namePages, order) {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.wrapping_add(2);
                }
                22 | 24 | 25 | 26 | 27 => ptr = ptr.wrapping_add(2),
                5 => return utf16_invalid_lead_or_partial(ptr, end, nextTokPtr, 2),
                6 => return utf16_invalid_lead_or_partial(ptr, end, nextTokPtr, 3),
                7 => return utf16_invalid_lead_or_partial(ptr, end, nextTokPtr, 4),
                23 => {
                    if had_colon {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    had_colon = true;
                    ptr = ptr.wrapping_add(2);
                    if let Err(tok) =
                        utf16_scan_attr_name_start(&mut ptr, end, enc, nextTokPtr, order)
                    {
                        return tok;
                    }
                }
                21 | 9 | 10 => loop {
                    ptr = ptr.wrapping_add(2);
                    if super::byte_distance(ptr, end) < 2 {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    let t = utf16_encoded_byte_type(enc, ptr, order);
                    if t == crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int {
                        scan_value = true;
                        break;
                    }
                    match t {
                        21 | 10 | 9 => {}
                        _ => {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                },
                14 => scan_value = true,
                _ => {
                    super::set_next_tok_ptr(nextTokPtr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
            if !scan_value {
                continue;
            }

            had_colon = false;
            let open = loop {
                ptr = ptr.wrapping_add(2);
                if super::byte_distance(ptr, end) < 2 {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                let open = utf16_encoded_byte_type(enc, ptr, order);
                if open == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                    || open == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                {
                    break open;
                }
                match open {
                    21 | 10 | 9 => {}
                    _ => {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
            };
            ptr = ptr.wrapping_add(2);

            loop {
                if super::byte_distance(ptr, end) < 2 {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                let t = utf16_encoded_byte_type(enc, ptr, order);
                if t == open {
                    break;
                }
                match t {
                    5 => {
                        if super::byte_distance(ptr, end) < 2 {
                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_add(2);
                    }
                    6 => {
                        if super::byte_distance(ptr, end) < 3 {
                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_add(3);
                    }
                    7 => {
                        if super::byte_distance(ptr, end) < 4 {
                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.wrapping_add(4);
                    }
                    0 | 1 | 8 => {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    3 => {
                        let tok = scan_ref(enc, ptr.wrapping_add(2), end, &mut ptr);
                        if tok <= 0 {
                            if tok == crate::src::xmltok::XML_TOK_INVALID_1 {
                                super::set_next_tok_ptr(nextTokPtr, ptr);
                            }
                            return tok;
                        }
                    }
                    2 => {
                        super::set_next_tok_ptr(nextTokPtr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    _ => ptr = ptr.wrapping_add(2),
                }
            }

            ptr = ptr.wrapping_add(2);
            match scan_utf16_after_attr_value(enc, &mut ptr, end, nextTokPtr, order) {
                Ok(None) => {}
                Ok(Some(tok)) => {
                    if tok == crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1 {
                        ptr = ptr.wrapping_add(2);
                        if super::byte_distance(ptr, end) < 2 {
                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if utf16_ascii_byte(ptr, order) != crate::ascii_h::ASCII_GT {
                            super::set_next_tok_ptr(nextTokPtr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    super::set_next_tok_ptr(nextTokPtr, ptr.wrapping_add(2));
                    return tok;
                }
                Err(tok) => return tok,
            }
        }
        crate::src::xmltok::XML_TOK_PARTIAL_1
    }

    pub extern "C" fn little2_scanAtts(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        scan_utf16_atts(
            enc,
            ptr,
            end,
            nextTokPtr,
            Utf16ByteOrder::Little,
            little2_scanRef,
        )
    }

    pub extern "C" fn little2_scanLt(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_lt(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_ascii_byte,
            super::little2_encoded_byte_type,
            super::little2_nonascii_name_start_char,
            super::little2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
            little2_scanAtts,
            little2_scanComment,
            little2_scanCdataSection,
            little2_scanPi,
            little2_scanEndTag,
        )
    }

    pub extern "C" fn little2_contentTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::content_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
            super::little2_ascii_byte,
            super::never_invalid_char,
            little2_scanLt,
            little2_scanRef,
        )
    }

    pub extern "C" fn little2_scanPercent(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_percent(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
            super::little2_nonascii_name_start_char,
            super::little2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
        )
    }

    pub extern "C" fn little2_scanPoundName(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_pound_name(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
            super::little2_nonascii_name_start_char,
            super::little2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
        )
    }

    pub extern "C" fn little2_scanLit(
        open: ::core::ffi::c_int,
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_lit(
            open,
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
            super::never_invalid_char,
        )
    }

    pub extern "C" fn little2_prologTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::prolog_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_ascii_byte,
            super::little2_encoded_byte_type,
            super::little2_nonascii_name_start_char,
            super::little2_nonascii_name_char,
            super::reject_lead_name_char,
            super::reject_lead_name_char,
            little2_scanLit,
            little2_scanDecl,
            little2_scanPi,
            little2_scanPercent,
            little2_scanPoundName,
        )
    }

    pub extern "C" fn little2_attributeValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::literal_value_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
            little2_scanRef,
            None,
            true,
            true,
        )
    }

    pub extern "C" fn little2_entityValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::literal_value_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_encoded_byte_type,
            little2_scanRef,
            Some(little2_scanPercent),
            false,
            false,
        )
    }

    pub extern "C" fn little2_ignoreSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::ignore_section_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::little2_ascii_byte,
            super::little2_encoded_byte_type,
            super::never_invalid_char,
        )
    }

    pub extern "C" fn little2_isPublicId(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::is_public_id(
            enc,
            ptr,
            end,
            badPtr,
            2,
            super::little2_ascii_byte,
            super::little2_encoded_byte_type,
        )
    }

    pub extern "C" fn little2_getAtts(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        attsMax: ::core::ffi::c_int,
        atts: *mut crate::src::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        super::get_atts(
            enc,
            ptr,
            attsMax,
            atts,
            2,
            super::little2_ascii_byte,
            super::little2_encoded_byte_type,
        )
    }

    pub extern "C" fn little2_charRefNumber(
        mut _enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::char_ref_number(ptr, 2, super::little2_ascii_byte)
    }

    pub extern "C" fn little2_predefinedEntityName(
        mut _enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::predefined_entity_name(ptr, end, 2, super::little2_ascii_byte)
    }

    pub extern "C" fn little2_nameMatchesAscii(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr1: *const ::core::ffi::c_char,
        mut end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while super::utf8_byte(ptr2, 0) != 0 {
            if super::byte_distance(ptr1, end1) < 2 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
            if !(super::utf8_byte(ptr1, 1) == 0 as ::core::ffi::c_int
                && super::utf8_byte(ptr1, 0) == super::utf8_byte(ptr2, 0))
            {
                return 0 as ::core::ffi::c_int;
            }
            ptr1 = ptr1.wrapping_add(2);
            ptr2 = ptr2.wrapping_add(1);
        }
        return (ptr1 == end1) as ::core::ffi::c_int;
    }

    pub extern "C" fn little2_nameLength(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match if super::utf8_byte(ptr, 1) == 0 as ::core::ffi::c_int {
                super::normal_byte_type(enc, super::utf8_byte(ptr, 0))
            } else {
                unicode_byte_type(
                    super::utf8_byte(ptr, 1) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                    super::utf8_byte(ptr, 0) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                )
            } {
                5 => {
                    ptr = ptr.wrapping_add(2);
                }
                6 => {
                    ptr = ptr.wrapping_add(3);
                }
                7 => {
                    ptr = ptr.wrapping_add(4);
                }
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.wrapping_add(2);
                }
                _ => {
                    return super::byte_distance(start, ptr) as ::core::ffi::c_int;
                }
            }
        }
    }

    pub extern "C" fn little2_skipS(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        loop {
            match if super::utf8_byte(ptr, 1) == 0 as ::core::ffi::c_int {
                super::normal_byte_type(enc, super::utf8_byte(ptr, 0))
            } else {
                unicode_byte_type(
                    super::utf8_byte(ptr, 1) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                    super::utf8_byte(ptr, 0) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                )
            } {
                10 | 9 | 21 => {
                    ptr = ptr.wrapping_add(2);
                }
                _ => return ptr,
            }
        }
    }

    pub extern "C" fn little2_updatePosition(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        pos: &mut crate::src::xmltok::POSITION,
    ) {
        while super::byte_distance(ptr, end)
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match if super::utf8_byte(ptr, 1) == 0 as ::core::ffi::c_int {
                super::normal_byte_type(enc, super::utf8_byte(ptr, 0))
            } else {
                unicode_byte_type(
                    super::utf8_byte(ptr, 1) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                    super::utf8_byte(ptr, 0) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                )
            } {
                5 => {
                    ptr = ptr.wrapping_add(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.wrapping_add(3);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.wrapping_add(4);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                10 => {
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_add(2);
                }
                9 => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_add(2);
                    if super::byte_distance(ptr, end)
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                        && (if super::utf8_byte(ptr, 1) == 0 as ::core::ffi::c_int {
                            super::normal_byte_type(enc, super::utf8_byte(ptr, 0))
                        } else {
                            unicode_byte_type(
                                super::utf8_byte(ptr, 1) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_char,
                                super::utf8_byte(ptr, 0) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_char,
                            )
                        }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.wrapping_add(2);
                    }
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                }
                _ => {
                    ptr = ptr.wrapping_add(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub extern "C" fn big2_scanComment(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_comment(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_ascii_byte,
            super::big2_encoded_byte_type,
            super::never_invalid_char,
        )
    }

    pub extern "C" fn big2_scanDecl(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_decl(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_encoded_byte_type,
            big2_scanComment,
        )
    }

    pub fn big2_checkPiTarget(
        mut _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        tok: &mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        *tok = crate::src::xmltok::XML_TOK_PI_1;
        if super::byte_distance(ptr, end) != 6 as ::core::ffi::c_long {
            return 1 as ::core::ffi::c_int;
        }
        match super::big2_ascii_byte(ptr) {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.wrapping_add(2);
        match super::big2_ascii_byte(ptr) {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        ptr = ptr.wrapping_add(2);
        match super::big2_ascii_byte(ptr) {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1 as ::core::ffi::c_int;
            }
            _ => return 1 as ::core::ffi::c_int,
        }
        if upper != 0 {
            return 0 as ::core::ffi::c_int;
        }
        *tok = crate::src::xmltok::XML_TOK_XML_DECL_1;
        return 1 as ::core::ffi::c_int;
    }

    pub extern "C" fn big2_scanPi(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_pi(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_ascii_byte,
            super::big2_encoded_byte_type,
            big2_checkPiTarget,
            super::big2_nonascii_name_start_char,
            super::big2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
            super::always_lead_char,
        )
    }

    pub extern "C" fn big2_scanCdataSection(
        mut _enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        static CDATA_LSQB: [::core::ffi::c_char; 6] = [
            crate::ascii_h::ASCII_C as ::core::ffi::c_char,
            crate::ascii_h::ASCII_D as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_T as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_LSQB as ::core::ffi::c_char,
        ];
        if super::byte_distance(ptr, end) < 12 as ::core::ffi::c_long {
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        }
        for expected in CDATA_LSQB {
            if super::big2_ascii_byte(ptr) != expected as ::core::ffi::c_int {
                super::set_next_tok_ptr(nextTokPtr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(2);
        }
        super::set_next_tok_ptr(nextTokPtr, ptr);
        return crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    }

    pub extern "C" fn big2_cdataSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return super::cdata_section_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_ascii_byte,
            super::big2_encoded_byte_type,
            super::never_invalid_char,
        );
    }

    pub extern "C" fn big2_scanEndTag(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_end_tag(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_encoded_byte_type,
            super::big2_nonascii_name_start_char,
            super::big2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
        )
    }

    pub extern "C" fn big2_scanHexCharRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_hex_char_ref(enc, ptr, end, nextTokPtr, 2, super::big2_encoded_byte_type)
    }

    pub extern "C" fn big2_scanCharRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_char_ref(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_ascii_byte,
            super::big2_encoded_byte_type,
        )
    }

    pub extern "C" fn big2_scanRef(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_ref(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_encoded_byte_type,
            super::big2_ascii_byte,
            super::big2_nonascii_name_start_char,
            super::big2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
        )
    }

    pub extern "C" fn big2_scanAtts(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        scan_utf16_atts(enc, ptr, end, nextTokPtr, Utf16ByteOrder::Big, big2_scanRef)
    }

    pub extern "C" fn big2_scanLt(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_lt(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_ascii_byte,
            super::big2_encoded_byte_type,
            super::big2_nonascii_name_start_char,
            super::big2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
            big2_scanAtts,
            big2_scanComment,
            big2_scanCdataSection,
            big2_scanPi,
            big2_scanEndTag,
        )
    }

    pub extern "C" fn big2_contentTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::content_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_encoded_byte_type,
            super::big2_ascii_byte,
            super::never_invalid_char,
            big2_scanLt,
            big2_scanRef,
        )
    }

    pub extern "C" fn big2_scanPercent(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_percent(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_encoded_byte_type,
            super::big2_nonascii_name_start_char,
            super::big2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
        )
    }

    pub extern "C" fn big2_scanPoundName(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_pound_name(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_encoded_byte_type,
            super::big2_nonascii_name_start_char,
            super::big2_nonascii_name_char,
            super::never_lead_char,
            super::never_lead_char,
        )
    }

    pub extern "C" fn big2_scanLit(
        open: ::core::ffi::c_int,
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::scan_lit(
            open,
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_encoded_byte_type,
            super::never_invalid_char,
        )
    }

    pub extern "C" fn big2_prologTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::prolog_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_ascii_byte,
            super::big2_encoded_byte_type,
            super::big2_nonascii_name_start_char,
            super::big2_nonascii_name_char,
            super::reject_lead_name_char,
            super::reject_lead_name_char,
            big2_scanLit,
            big2_scanDecl,
            big2_scanPi,
            big2_scanPercent,
            big2_scanPoundName,
        )
    }

    pub extern "C" fn big2_attributeValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::literal_value_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_encoded_byte_type,
            big2_scanRef,
            None,
            true,
            true,
        )
    }

    pub extern "C" fn big2_entityValueTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::literal_value_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_encoded_byte_type,
            big2_scanRef,
            Some(big2_scanPercent),
            false,
            false,
        )
    }

    pub extern "C" fn big2_ignoreSectionTok(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::ignore_section_tok(
            enc,
            ptr,
            end,
            nextTokPtr,
            2,
            super::big2_ascii_byte,
            super::big2_encoded_byte_type,
            super::never_invalid_char,
        )
    }

    pub extern "C" fn big2_isPublicId(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::is_public_id(
            enc,
            ptr,
            end,
            badPtr,
            2,
            super::big2_ascii_byte,
            super::big2_encoded_byte_type,
        )
    }

    pub extern "C" fn big2_getAtts(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        attsMax: ::core::ffi::c_int,
        atts: *mut crate::src::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        super::get_atts(
            enc,
            ptr,
            attsMax,
            atts,
            2,
            super::big2_ascii_byte,
            super::big2_encoded_byte_type,
        )
    }

    pub extern "C" fn big2_charRefNumber(
        mut _enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::char_ref_number(ptr, 2, super::big2_ascii_byte)
    }

    pub extern "C" fn big2_predefinedEntityName(
        mut _enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        super::predefined_entity_name(ptr, end, 2, super::big2_ascii_byte)
    }

    pub extern "C" fn big2_nameMatchesAscii(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr1: *const ::core::ffi::c_char,
        mut end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while super::utf8_byte(ptr2, 0) != 0 {
            if super::byte_distance(ptr1, end1) < 2 as ::core::ffi::c_long {
                return 0 as ::core::ffi::c_int;
            }
            if !(super::utf8_byte(ptr1, 0) == 0 as ::core::ffi::c_int
                && super::utf8_byte(ptr1, 1) == super::utf8_byte(ptr2, 0))
            {
                return 0 as ::core::ffi::c_int;
            }
            ptr1 = ptr1.wrapping_add(2);
            ptr2 = ptr2.wrapping_add(1);
        }
        return (ptr1 == end1) as ::core::ffi::c_int;
    }

    pub extern "C" fn big2_nameLength(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match if super::utf8_byte(ptr, 0) == 0 as ::core::ffi::c_int {
                super::normal_byte_type(enc, super::utf8_byte(ptr, 1))
            } else {
                unicode_byte_type(
                    super::utf8_byte(ptr, 0) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                    super::utf8_byte(ptr, 1) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                )
            } {
                5 => {
                    ptr = ptr.wrapping_add(2);
                }
                6 => {
                    ptr = ptr.wrapping_add(3);
                }
                7 => {
                    ptr = ptr.wrapping_add(4);
                }
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.wrapping_add(2);
                }
                _ => {
                    return super::byte_distance(start, ptr) as ::core::ffi::c_int;
                }
            }
        }
    }

    pub extern "C" fn big2_skipS(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        loop {
            match if super::utf8_byte(ptr, 0) == 0 as ::core::ffi::c_int {
                super::normal_byte_type(enc, super::utf8_byte(ptr, 1))
            } else {
                unicode_byte_type(
                    super::utf8_byte(ptr, 0) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                    super::utf8_byte(ptr, 1) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                )
            } {
                10 | 9 | 21 => {
                    ptr = ptr.wrapping_add(2);
                }
                _ => return ptr,
            }
        }
    }

    pub extern "C" fn big2_updatePosition(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        pos: &mut crate::src::xmltok::POSITION,
    ) {
        while super::byte_distance(ptr, end)
            >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            match if super::utf8_byte(ptr, 0) == 0 as ::core::ffi::c_int {
                super::normal_byte_type(enc, super::utf8_byte(ptr, 1))
            } else {
                unicode_byte_type(
                    super::utf8_byte(ptr, 0) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                    super::utf8_byte(ptr, 1) as ::core::ffi::c_uchar as ::core::ffi::c_char,
                )
            } {
                5 => {
                    ptr = ptr.wrapping_add(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.wrapping_add(3);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.wrapping_add(4);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
                10 => {
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_add(2);
                }
                9 => {
                    pos.lineNumber = pos.lineNumber.wrapping_add(1);
                    ptr = ptr.wrapping_add(2);
                    if super::byte_distance(ptr, end)
                        >= (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                        && (if super::utf8_byte(ptr, 0) == 0 as ::core::ffi::c_int {
                            super::normal_byte_type(enc, super::utf8_byte(ptr, 1))
                        } else {
                            unicode_byte_type(
                                super::utf8_byte(ptr, 0) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_char,
                                super::utf8_byte(ptr, 1) as ::core::ffi::c_uchar
                                    as ::core::ffi::c_char,
                            )
                        }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.wrapping_add(2);
                    }
                    pos.columnNumber = 0 as crate::expat_external_h::XML_Size;
                }
                _ => {
                    ptr = ptr.wrapping_add(2);
                    pos.columnNumber = pos.columnNumber.wrapping_add(1);
                }
            }
        }
    }
    use crate::__stddef_size_t_h::size_t;
    use crate::ascii_h::ASCII_a_1;
    use crate::ascii_h::ASCII_0;
    use crate::ascii_h::ASCII_A;
    use crate::ascii_h::ASCII_AMP;
    use crate::ascii_h::ASCII_APOS;
    use crate::ascii_h::ASCII_C;
    use crate::ascii_h::ASCII_D;
    use crate::ascii_h::ASCII_GT;
    use crate::ascii_h::ASCII_LSQB;
    use crate::ascii_h::ASCII_LT;
    use crate::ascii_h::ASCII_QUOT;
    use crate::ascii_h::ASCII_SPACE;
    use crate::ascii_h::ASCII_T;
    use crate::expat_external_h::XML_Size;

    use crate::src::xmltok::nametab_h::namePages;
    use crate::src::xmltok::nametab_h::namingBitmap;
    use crate::src::xmltok::nametab_h::nmstrtPages;
    use crate::src::xmltok::normal_encoding;
    use crate::src::xmltok::unicode_byte_type;
    use crate::src::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1;
    use crate::src::xmltok::XML_TOK_CDATA_SECT_CLOSE_1;
    use crate::src::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    use crate::src::xmltok::XML_TOK_CHAR_REF_1;
    use crate::src::xmltok::XML_TOK_CLOSE_BRACKET_1;
    use crate::src::xmltok::XML_TOK_CLOSE_PAREN_1;
    use crate::src::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK_1;
    use crate::src::xmltok::XML_TOK_CLOSE_PAREN_PLUS_1;
    use crate::src::xmltok::XML_TOK_CLOSE_PAREN_QUESTION_1;
    use crate::src::xmltok::XML_TOK_COMMA_1;
    use crate::src::xmltok::XML_TOK_COMMENT_1;
    use crate::src::xmltok::XML_TOK_COND_SECT_CLOSE_1;
    use crate::src::xmltok::XML_TOK_COND_SECT_OPEN_1;
    use crate::src::xmltok::XML_TOK_DATA_CHARS_1;
    use crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
    use crate::src::xmltok::XML_TOK_DECL_CLOSE_1;
    use crate::src::xmltok::XML_TOK_DECL_OPEN_1;
    use crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
    use crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
    use crate::src::xmltok::XML_TOK_END_TAG_1;
    use crate::src::xmltok::XML_TOK_ENTITY_REF_1;
    use crate::src::xmltok::XML_TOK_IGNORE_SECT_1;
    use crate::src::xmltok::XML_TOK_INSTANCE_START;
    use crate::src::xmltok::XML_TOK_INVALID_1;
    use crate::src::xmltok::XML_TOK_LITERAL_1;
    use crate::src::xmltok::XML_TOK_NAME;
    use crate::src::xmltok::XML_TOK_NAME_ASTERISK_1;
    use crate::src::xmltok::XML_TOK_NAME_PLUS_1;
    use crate::src::xmltok::XML_TOK_NAME_QUESTION_1;
    use crate::src::xmltok::XML_TOK_NMTOKEN_1;
    use crate::src::xmltok::XML_TOK_NONE_1;
    use crate::src::xmltok::XML_TOK_OPEN_BRACKET_1;
    use crate::src::xmltok::XML_TOK_OPEN_PAREN_1;
    use crate::src::xmltok::XML_TOK_OR_1;
    use crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF_1;
    use crate::src::xmltok::XML_TOK_PARTIAL_1;
    use crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
    use crate::src::xmltok::XML_TOK_PERCENT_1;
    use crate::src::xmltok::XML_TOK_PI_1;
    use crate::src::xmltok::XML_TOK_POUND_NAME_1;
    use crate::src::xmltok::XML_TOK_PREFIXED_NAME;
    use crate::src::xmltok::XML_TOK_PROLOG_S_1;
    use crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS_1;
    use crate::src::xmltok::XML_TOK_START_TAG_WITH_ATTS_1;
    use crate::src::xmltok::XML_TOK_TRAILING_CR_1;
    use crate::src::xmltok::XML_TOK_TRAILING_RSQB_1;
    use crate::src::xmltok::XML_TOK_XML_DECL_1;
    use crate::xmltok_impl_h::BT_APOS;
    use crate::xmltok_impl_h::BT_EQUALS;
    use crate::xmltok_impl_h::BT_LF;
    use crate::xmltok_impl_h::BT_QUOT;
}

pub mod xmltok_ns_c {
    pub extern "C" fn XmlGetUtf8InternalEncoding() -> *const crate::src::xmltok::ENCODING {
        return &raw const internal_utf8_encoding.enc;
    }
    #[export_name = "XmlGetUtf8InternalEncoding"]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncoding_ffi() -> *const crate::src::xmltok::ENCODING
    {
        XmlGetUtf8InternalEncoding()
    }
    pub extern "C" fn XmlGetUtf16InternalEncoding() -> *const crate::src::xmltok::ENCODING {
        return &raw const internal_little2_encoding.enc;
    }
    #[export_name = "XmlGetUtf16InternalEncoding"]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncoding_ffi() -> *const crate::src::xmltok::ENCODING
    {
        XmlGetUtf16InternalEncoding()
    }
    fn encodings() -> [*const crate::src::xmltok::ENCODING; 7] {
        [
            &raw const latin1_encoding.enc,
            &raw const ascii_encoding.enc,
            &raw const utf8_encoding.enc,
            &raw const big2_encoding.enc,
            &raw const big2_encoding.enc,
            &raw const little2_encoding.enc,
            &raw const utf8_encoding.enc,
        ]
    }

    pub extern "C" fn initScanProlog(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let encodings = encodings();
        return initScan(
            encodings.as_ptr(),
            enc as *const crate::src::xmltok::INIT_ENCODING,
            crate::src::xmltok::XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub extern "C" fn initScanContent(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let encodings = encodings();
        return initScan(
            encodings.as_ptr(),
            enc as *const crate::src::xmltok::INIT_ENCODING,
            crate::src::xmltok::XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }
    pub fn XmlInitEncoding(
        p: &mut crate::src::xmltok::INIT_ENCODING,
        encPtr: &mut *const crate::src::xmltok::ENCODING,
        name: Option<&[::core::ffi::c_char]>,
    ) -> ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = getEncodingIndex(name);
        if i == UNKNOWN_ENC as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        p.initEnc.isUtf16 = i as ::core::ffi::c_char;
        p.initEnc.scanners[crate::src::xmltok::XML_PROLOG_STATE as usize] = Some(
            initScanProlog
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::xmltok::SCANNER;
        p.initEnc.scanners[crate::src::xmltok::XML_CONTENT_STATE as usize] = Some(
            initScanContent
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::xmltok::SCANNER;
        p.initEnc.updatePosition = Some(
            initUpdatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        )
            as Option<
                extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
            >;
        p.encPtr = encPtr;
        *encPtr = &raw mut p.initEnc;
        return 1 as ::core::ffi::c_int;
    }
    #[export_name = "XmlInitEncoding"]

    pub unsafe extern "C" fn XmlInitEncoding_ffi(
        mut p: *mut crate::src::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let name = if name.is_null() {
            None
        } else {
            let len = std::ffi::CStr::from_ptr(name).to_bytes_with_nul().len();
            Some(::core::slice::from_raw_parts(name, len))
        };
        XmlInitEncoding(&mut *p, &mut *encPtr, name)
    }
    pub extern "C" fn findEncoding(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *const crate::src::xmltok::ENCODING {
        let Some(buf) = xml_decl_convert_to_utf8_name(enc, ptr, end) else {
            return ::core::ptr::null::<crate::src::xmltok::ENCODING>();
        };
        let encodings = encodings();
        find_encoding_from_converted_name(enc, &buf, &encodings)
    }
    pub fn XmlParseXmlDecl(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        badPtr: &mut *const ::core::ffi::c_char,
        versionPtr: Option<&mut *const ::core::ffi::c_char>,
        versionEndPtr: Option<&mut *const ::core::ffi::c_char>,
        encodingName: Option<&mut *const ::core::ffi::c_char>,
        encoding: Option<&mut *const crate::src::xmltok::ENCODING>,
        standalone: Option<&mut ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int {
        return doParseXmlDecl(
            Some(
                findEncoding
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> *const crate::src::xmltok::ENCODING,
            ),
            isGeneralTextEntity,
            enc,
            ptr,
            end,
            badPtr,
            versionPtr,
            versionEndPtr,
            encodingName,
            encoding,
            standalone,
        );
    }
    #[export_name = "XmlParseXmlDecl"]

    pub unsafe extern "C" fn XmlParseXmlDecl_ffi(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const crate::src::xmltok::ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        XmlParseXmlDecl(
            isGeneralTextEntity,
            enc,
            ptr,
            end,
            &mut *badPtr,
            if versionPtr.is_null() {
                None
            } else {
                Some(&mut *versionPtr)
            },
            if versionEndPtr.is_null() {
                None
            } else {
                Some(&mut *versionEndPtr)
            },
            if encodingName.is_null() {
                None
            } else {
                Some(&mut *encodingName)
            },
            if encoding.is_null() {
                None
            } else {
                Some(&mut *encoding)
            },
            if standalone.is_null() {
                None
            } else {
                Some(&mut *standalone)
            },
        )
    }
    pub extern "C" fn XmlGetUtf8InternalEncodingNS() -> *const crate::src::xmltok::ENCODING {
        return &raw const internal_utf8_encoding_ns.enc;
    }
    #[export_name = "XmlGetUtf8InternalEncodingNS"]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncodingNS_ffi(
    ) -> *const crate::src::xmltok::ENCODING {
        XmlGetUtf8InternalEncodingNS()
    }
    pub extern "C" fn XmlGetUtf16InternalEncodingNS() -> *const crate::src::xmltok::ENCODING {
        return &raw const internal_little2_encoding_ns.enc;
    }
    #[export_name = "XmlGetUtf16InternalEncodingNS"]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncodingNS_ffi(
    ) -> *const crate::src::xmltok::ENCODING {
        XmlGetUtf16InternalEncodingNS()
    }
    fn encodings_ns() -> [*const crate::src::xmltok::ENCODING; 7] {
        [
            &raw const latin1_encoding_ns.enc,
            &raw const ascii_encoding_ns.enc,
            &raw const utf8_encoding_ns.enc,
            &raw const big2_encoding_ns.enc,
            &raw const big2_encoding_ns.enc,
            &raw const little2_encoding_ns.enc,
            &raw const utf8_encoding_ns.enc,
        ]
    }

    pub extern "C" fn initScanPrologNS(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let encodings_ns = encodings_ns();
        return initScan(
            encodings_ns.as_ptr(),
            enc as *const crate::src::xmltok::INIT_ENCODING,
            crate::src::xmltok::XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub extern "C" fn initScanContentNS(
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let encodings_ns = encodings_ns();
        return initScan(
            encodings_ns.as_ptr(),
            enc as *const crate::src::xmltok::INIT_ENCODING,
            crate::src::xmltok::XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }
    pub fn XmlInitEncodingNS(
        p: &mut crate::src::xmltok::INIT_ENCODING,
        encPtr: &mut *const crate::src::xmltok::ENCODING,
        name: Option<&[::core::ffi::c_char]>,
    ) -> ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = getEncodingIndex(name);
        if i == UNKNOWN_ENC as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        p.initEnc.isUtf16 = i as ::core::ffi::c_char;
        p.initEnc.scanners[crate::src::xmltok::XML_PROLOG_STATE as usize] = Some(
            initScanPrologNS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::xmltok::SCANNER;
        p.initEnc.scanners[crate::src::xmltok::XML_CONTENT_STATE as usize] = Some(
            initScanContentNS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::xmltok::SCANNER;
        p.initEnc.updatePosition = Some(
            initUpdatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        )
            as Option<
                extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
            >;
        p.encPtr = encPtr;
        *encPtr = &raw mut p.initEnc;
        return 1 as ::core::ffi::c_int;
    }
    #[export_name = "XmlInitEncodingNS"]

    pub unsafe extern "C" fn XmlInitEncodingNS_ffi(
        mut p: *mut crate::src::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let name = if name.is_null() {
            None
        } else {
            let len = std::ffi::CStr::from_ptr(name).to_bytes_with_nul().len();
            Some(::core::slice::from_raw_parts(name, len))
        };
        XmlInitEncodingNS(&mut *p, &mut *encPtr, name)
    }
    pub extern "C" fn findEncodingNS(
        enc: *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *const crate::src::xmltok::ENCODING {
        let Some(buf) = xml_decl_convert_to_utf8_name(enc, ptr, end) else {
            return ::core::ptr::null::<crate::src::xmltok::ENCODING>();
        };
        let encodings_ns = encodings_ns();
        find_encoding_from_converted_name(enc, &buf, &encodings_ns)
    }
    pub fn XmlParseXmlDeclNS(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        badPtr: &mut *const ::core::ffi::c_char,
        versionPtr: Option<&mut *const ::core::ffi::c_char>,
        versionEndPtr: Option<&mut *const ::core::ffi::c_char>,
        encodingName: Option<&mut *const ::core::ffi::c_char>,
        encoding: Option<&mut *const crate::src::xmltok::ENCODING>,
        standalone: Option<&mut ::core::ffi::c_int>,
    ) -> ::core::ffi::c_int {
        return doParseXmlDecl(
            Some(
                findEncodingNS
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> *const crate::src::xmltok::ENCODING,
            ),
            isGeneralTextEntity,
            enc,
            ptr,
            end,
            badPtr,
            versionPtr,
            versionEndPtr,
            encodingName,
            encoding,
            standalone,
        );
    }
    #[export_name = "XmlParseXmlDeclNS"]

    pub unsafe extern "C" fn XmlParseXmlDeclNS_ffi(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const crate::src::xmltok::ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        XmlParseXmlDeclNS(
            isGeneralTextEntity,
            enc,
            ptr,
            end,
            &mut *badPtr,
            if versionPtr.is_null() {
                None
            } else {
                Some(&mut *versionPtr)
            },
            if versionEndPtr.is_null() {
                None
            } else {
                Some(&mut *versionEndPtr)
            },
            if encodingName.is_null() {
                None
            } else {
                Some(&mut *encodingName)
            },
            if encoding.is_null() {
                None
            } else {
                Some(&mut *encoding)
            },
            if standalone.is_null() {
                None
            } else {
                Some(&mut *standalone)
            },
        )
    }
    use crate::src::xmltok::ascii_encoding;
    use crate::src::xmltok::ascii_encoding_ns;
    use crate::src::xmltok::big2_encoding;
    use crate::src::xmltok::big2_encoding_ns;
    use crate::src::xmltok::doParseXmlDecl;
    use crate::src::xmltok::find_encoding_from_converted_name;
    use crate::src::xmltok::getEncodingIndex;
    use crate::src::xmltok::initScan;
    use crate::src::xmltok::initUpdatePosition;
    use crate::src::xmltok::internal_little2_encoding;
    use crate::src::xmltok::internal_little2_encoding_ns;
    use crate::src::xmltok::internal_utf8_encoding;
    use crate::src::xmltok::internal_utf8_encoding_ns;
    use crate::src::xmltok::latin1_encoding;
    use crate::src::xmltok::latin1_encoding_ns;
    use crate::src::xmltok::little2_encoding;
    use crate::src::xmltok::little2_encoding_ns;
    use crate::src::xmltok::utf8_encoding;
    use crate::src::xmltok::utf8_encoding_ns;
    use crate::src::xmltok::xml_decl_convert_to_utf8_name;
    use crate::src::xmltok::ENCODING;
    use crate::src::xmltok::INIT_ENCODING;
    use crate::src::xmltok::POSITION;
    use crate::src::xmltok::SCANNER;
    use crate::src::xmltok::UNKNOWN_ENC;
    use crate::src::xmltok::XML_CONTENT_STATE;
    use crate::src::xmltok::XML_PROLOG_STATE;
}

pub mod nametab_h {

    pub static namingBitmap: [::core::ffi::c_uint; 320] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x4000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x87fffffe as ::core::ffi::c_uint,
        0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xff7fffff as ::core::ffi::c_uint,
        0xff7fffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x7ff3ffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffdfe as ::core::ffi::c_uint,
        0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffe00f as ::core::ffi::c_uint,
        0xfc31ffff as ::core::ffi::c_uint,
        0xffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffff0000 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xf80001ff as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffd740 as ::core::ffi::c_uint,
        0xfffffffb as ::core::ffi::c_uint,
        0x547f7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffd as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffdffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xdffeffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffff0003 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffff199f as ::core::ffi::c_uint,
        0x33fcfff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffe0000 as ::core::ffi::c_uint,
        0x27fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0x7f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffff0000 as ::core::ffi::c_uint,
        0x707ff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7fe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffe0000 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x7cffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2f7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x60 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffe0 as ::core::ffi::c_uint,
        0x23ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xff000000 as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfff99fe0 as ::core::ffi::c_uint,
        0x3c5fdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xb0000000 as ::core::ffi::c_uint,
        0x30003 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfff987e0 as ::core::ffi::c_uint,
        0x36dfdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x5e000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1c0000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffbafe0 as ::core::ffi::c_uint,
        0x23edfdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfff99fe0 as ::core::ffi::c_uint,
        0x23cdfdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xb0000000 as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xd63dc7e0 as ::core::ffi::c_uint,
        0x3bfc718 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffddfe0 as ::core::ffi::c_uint,
        0x3effdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffddfe0 as ::core::ffi::c_uint,
        0x3effdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x40000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffddfe0 as ::core::ffi::c_uint,
        0x3fffdff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xd7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfef02596 as ::core::ffi::c_uint,
        0x200d6cae as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffeff as ::core::ffi::c_uint,
        0x3ff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffff003f as ::core::ffi::c_uint,
        0x7fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7daed as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x50000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x82315001 as ::core::ffi::c_uint,
        0x2c62ab as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x40000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xf580c900 as ::core::ffi::c_uint,
        0x7 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2010800 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xfffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x3ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3f3fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xaaff3f3f as ::core::ffi::c_uint,
        0x3fffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x5fdfffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfcf1fdc as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1fdc1fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x4c40 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x80 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3fe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x1fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x7ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffe0 as ::core::ffi::c_uint,
        0x1fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x3f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7ff6000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x87fffffe as ::core::ffi::c_uint,
        0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x800000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xff7fffff as ::core::ffi::c_uint,
        0xff7fffff as ::core::ffi::c_uint,
        0xffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffff0000 as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xf80001ff as ::core::ffi::c_uint,
        0x30003 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x3f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffd7c0 as ::core::ffi::c_uint,
        0xfffffffb as ::core::ffi::c_uint,
        0x547f7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffd as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffdffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xdffeffff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffff007b as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0xffff199f as ::core::ffi::c_uint,
        0x33fcfff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffe0000 as ::core::ffi::c_uint,
        0x27fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xfffe007f as ::core::ffi::c_uint,
        0xbbfffffb as ::core::ffi::c_uint,
        0xffff0016 as ::core::ffi::c_uint,
        0x707ff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7fffffe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x7ffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffff03ff as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x7cffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffef7fff as ::core::ffi::c_uint,
        0x3ff3dff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffffffee as ::core::ffi::c_uint,
        0xf3ffffff as ::core::ffi::c_uint,
        0xff1e3fff as ::core::ffi::c_uint,
        0xffcf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfff99fee as ::core::ffi::c_uint,
        0xd3c5fdff as ::core::ffi::c_uint,
        0xb080399f as ::core::ffi::c_uint,
        0x3ffcf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfff987e4 as ::core::ffi::c_uint,
        0xd36dfdff as ::core::ffi::c_uint,
        0x5e003987 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1fffc0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffbafee as ::core::ffi::c_uint,
        0xf3edfdff as ::core::ffi::c_uint,
        0x3bbf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffc1 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfff99fee as ::core::ffi::c_uint,
        0xf3cdfdff as ::core::ffi::c_uint,
        0xb0c0398f as ::core::ffi::c_uint,
        0xffc3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xd63dc7ec as ::core::ffi::c_uint,
        0xc3bfc718 as ::core::ffi::c_uint,
        0x803dc7 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xff80 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffddfee as ::core::ffi::c_uint,
        0xc3effdff as ::core::ffi::c_uint,
        0x603ddf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffc3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffddfec as ::core::ffi::c_uint,
        0xc3effdff as ::core::ffi::c_uint,
        0x40603ddf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffc3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffddfec as ::core::ffi::c_uint,
        0xc3fffdff as ::core::ffi::c_uint,
        0x803dcf as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xffc3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0x7ff7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3ff7fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfef02596 as ::core::ffi::c_uint,
        0x3bff6cae as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3ff3f5f as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3000000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xc2a003ff as ::core::ffi::c_uint,
        0xfffffeff as ::core::ffi::c_uint,
        0xfffe03ff as ::core::ffi::c_uint,
        0xfebf0fdf as ::core::ffi::c_uint,
        0x2fe3fff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x1fff0000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x2 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xa0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        0x3efffe as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x661fffff as ::core::ffi::c_int as ::core::ffi::c_uint,
        0xfffffffe as ::core::ffi::c_uint,
        0xffffffff as ::core::ffi::c_uint,
        0x77ffffff as ::core::ffi::c_int as ::core::ffi::c_uint,
    ];

    pub static nmstrtPages: [::core::ffi::c_uchar; 256] = [
        0x2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x5 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x7 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xb as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xe as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xf as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x15 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];

    pub static namePages: [::core::ffi::c_uchar; 256] = [
        0x19 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1b as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1c as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1d as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1e as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x20 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x21 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x22 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x23 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x24 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x25 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x10 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x11 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x12 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x13 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x26 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x14 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x27 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x16 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x17 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x18 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
}
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::ascii_h::ASCII_a_1;
pub use crate::ascii_h::ASCII_b;
pub use crate::ascii_h::ASCII_c_1;
pub use crate::ascii_h::ASCII_d;
pub use crate::ascii_h::ASCII_e_1;
pub use crate::ascii_h::ASCII_f;
pub use crate::ascii_h::ASCII_g_1;
pub use crate::ascii_h::ASCII_i;
pub use crate::ascii_h::ASCII_l_1;
pub use crate::ascii_h::ASCII_m_1;
pub use crate::ascii_h::ASCII_n;
pub use crate::ascii_h::ASCII_o;
pub use crate::ascii_h::ASCII_q;
pub use crate::ascii_h::ASCII_r;
pub use crate::ascii_h::ASCII_s;
pub use crate::ascii_h::ASCII_t;
pub use crate::ascii_h::ASCII_v;
pub use crate::ascii_h::ASCII_x_1;
pub use crate::ascii_h::ASCII_y;
pub use crate::ascii_h::ASCII_z;
pub use crate::ascii_h::ASCII_0;
pub use crate::ascii_h::ASCII_1_1;
pub use crate::ascii_h::ASCII_2_1;
pub use crate::ascii_h::ASCII_3_1;
pub use crate::ascii_h::ASCII_4;
pub use crate::ascii_h::ASCII_5;
pub use crate::ascii_h::ASCII_6;
pub use crate::ascii_h::ASCII_7;
pub use crate::ascii_h::ASCII_8_1;
pub use crate::ascii_h::ASCII_9_1;
pub use crate::ascii_h::ASCII_A;
pub use crate::ascii_h::ASCII_AMP;
pub use crate::ascii_h::ASCII_APOS;
pub use crate::ascii_h::ASCII_B_1;
pub use crate::ascii_h::ASCII_C;
pub use crate::ascii_h::ASCII_COLON;
pub use crate::ascii_h::ASCII_D;
pub use crate::ascii_h::ASCII_EQUALS;
pub use crate::ascii_h::ASCII_E_1;
pub use crate::ascii_h::ASCII_F_1;
pub use crate::ascii_h::ASCII_GT;
pub use crate::ascii_h::ASCII_I;
pub use crate::ascii_h::ASCII_LSQB;
pub use crate::ascii_h::ASCII_LT;
pub use crate::ascii_h::ASCII_L_1;
pub use crate::ascii_h::ASCII_MINUS;
pub use crate::ascii_h::ASCII_M_1;
pub use crate::ascii_h::ASCII_O;
pub use crate::ascii_h::ASCII_PERIOD;
pub use crate::ascii_h::ASCII_QUOT;
pub use crate::ascii_h::ASCII_S;
pub use crate::ascii_h::ASCII_SPACE;
pub use crate::ascii_h::ASCII_T;
pub use crate::ascii_h::ASCII_U;
pub use crate::ascii_h::ASCII_UNDERSCORE;
pub use crate::ascii_h::ASCII_X_1;
pub use crate::ascii_h::ASCII_Z;
pub use crate::expat_external_h::XML_Size;

pub use crate::src::xmltok::nametab_h::namePages;
pub use crate::src::xmltok::nametab_h::namingBitmap;
pub use crate::src::xmltok::nametab_h::nmstrtPages;
pub use crate::stdbool_h::false_0;
pub use crate::stdbool_h::true_0;

pub use crate::src::xmltok::xmltok_impl_c::big2_attributeValueTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_cdataSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_charRefNumber;
pub use crate::src::xmltok::xmltok_impl_c::big2_checkPiTarget;
pub use crate::src::xmltok::xmltok_impl_c::big2_contentTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_entityValueTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_getAtts;
pub use crate::src::xmltok::xmltok_impl_c::big2_ignoreSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_isPublicId;
pub use crate::src::xmltok::xmltok_impl_c::big2_nameLength;
pub use crate::src::xmltok::xmltok_impl_c::big2_nameMatchesAscii;
pub use crate::src::xmltok::xmltok_impl_c::big2_predefinedEntityName;
pub use crate::src::xmltok::xmltok_impl_c::big2_prologTok;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanAtts;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanCdataSection;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanCharRef;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanComment;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanDecl;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanEndTag;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanHexCharRef;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanLit;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanLt;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanPercent;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanPi;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanPoundName;
pub use crate::src::xmltok::xmltok_impl_c::big2_scanRef;
pub use crate::src::xmltok::xmltok_impl_c::big2_skipS;
pub use crate::src::xmltok::xmltok_impl_c::big2_updatePosition;
pub use crate::src::xmltok::xmltok_impl_c::little2_attributeValueTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_cdataSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_charRefNumber;
pub use crate::src::xmltok::xmltok_impl_c::little2_checkPiTarget;
pub use crate::src::xmltok::xmltok_impl_c::little2_contentTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_entityValueTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_getAtts;
pub use crate::src::xmltok::xmltok_impl_c::little2_ignoreSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_isPublicId;
pub use crate::src::xmltok::xmltok_impl_c::little2_nameLength;
pub use crate::src::xmltok::xmltok_impl_c::little2_nameMatchesAscii;
pub use crate::src::xmltok::xmltok_impl_c::little2_predefinedEntityName;
pub use crate::src::xmltok::xmltok_impl_c::little2_prologTok;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanAtts;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanCdataSection;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanCharRef;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanComment;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanDecl;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanEndTag;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanHexCharRef;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanLit;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanLt;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanPercent;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanPi;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanPoundName;
pub use crate::src::xmltok::xmltok_impl_c::little2_scanRef;
pub use crate::src::xmltok::xmltok_impl_c::little2_skipS;
pub use crate::src::xmltok::xmltok_impl_c::little2_updatePosition;
pub use crate::src::xmltok::xmltok_impl_c::normal_attributeValueTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_cdataSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_charRefNumber;
pub use crate::src::xmltok::xmltok_impl_c::normal_checkPiTarget;
pub use crate::src::xmltok::xmltok_impl_c::normal_contentTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_entityValueTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_getAtts;
pub use crate::src::xmltok::xmltok_impl_c::normal_ignoreSectionTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_isPublicId;
pub use crate::src::xmltok::xmltok_impl_c::normal_nameLength;
pub use crate::src::xmltok::xmltok_impl_c::normal_nameMatchesAscii;
pub use crate::src::xmltok::xmltok_impl_c::normal_predefinedEntityName;
pub use crate::src::xmltok::xmltok_impl_c::normal_prologTok;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanAtts;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanCdataSection;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanCharRef;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanComment;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanDecl;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanEndTag;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanHexCharRef;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanLit;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanLt;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanPercent;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanPi;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanPoundName;
pub use crate::src::xmltok::xmltok_impl_c::normal_scanRef;
pub use crate::src::xmltok::xmltok_impl_c::normal_skipS;
pub use crate::src::xmltok::xmltok_impl_c::normal_updatePosition;
pub use crate::src::xmltok::xmltok_ns_c::findEncoding;
pub use crate::src::xmltok::xmltok_ns_c::findEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::initScanContent;
pub use crate::src::xmltok::xmltok_ns_c::initScanContentNS;
pub use crate::src::xmltok::xmltok_ns_c::initScanProlog;
pub use crate::src::xmltok::xmltok_ns_c::initScanPrologNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf16InternalEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf16InternalEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlInitEncoding;
pub use crate::src::xmltok::xmltok_ns_c::XmlInitEncodingNS;
pub use crate::src::xmltok::xmltok_ns_c::XmlParseXmlDecl;
pub use crate::src::xmltok::xmltok_ns_c::XmlParseXmlDeclNS;
pub use crate::xmltok_impl_c::inName;
pub use crate::xmltok_impl_c::inName_0;
pub use crate::xmltok_impl_c::inName_1;
pub use crate::xmltok_impl_c::inValue;
pub use crate::xmltok_impl_c::inValue_0;
pub use crate::xmltok_impl_c::inValue_1;
pub use crate::xmltok_impl_c::other;
pub use crate::xmltok_impl_c::other_0;
pub use crate::xmltok_impl_c::other_1;
pub use crate::xmltok_impl_h::C2Rust_Unnamed_3;
pub use crate::xmltok_impl_h::BT_AMP;
pub use crate::xmltok_impl_h::BT_APOS;
pub use crate::xmltok_impl_h::BT_AST;
pub use crate::xmltok_impl_h::BT_COLON_0;
pub use crate::xmltok_impl_h::BT_COMMA;
pub use crate::xmltok_impl_h::BT_CR;
pub use crate::xmltok_impl_h::BT_DIGIT;
pub use crate::xmltok_impl_h::BT_EQUALS;
pub use crate::xmltok_impl_h::BT_EXCL;
pub use crate::xmltok_impl_h::BT_GT;
pub use crate::xmltok_impl_h::BT_HEX;
pub use crate::xmltok_impl_h::BT_LEAD2;
pub use crate::xmltok_impl_h::BT_LEAD3;
pub use crate::xmltok_impl_h::BT_LEAD4;
pub use crate::xmltok_impl_h::BT_LF;
pub use crate::xmltok_impl_h::BT_LPAR;
pub use crate::xmltok_impl_h::BT_LSQB;
pub use crate::xmltok_impl_h::BT_LT;
pub use crate::xmltok_impl_h::BT_MALFORM;
pub use crate::xmltok_impl_h::BT_MINUS;
pub use crate::xmltok_impl_h::BT_NAME;
pub use crate::xmltok_impl_h::BT_NMSTRT;
pub use crate::xmltok_impl_h::BT_NONASCII;
pub use crate::xmltok_impl_h::BT_NONXML;
pub use crate::xmltok_impl_h::BT_NUM;
pub use crate::xmltok_impl_h::BT_OTHER;
pub use crate::xmltok_impl_h::BT_PERCNT;
pub use crate::xmltok_impl_h::BT_PLUS;
pub use crate::xmltok_impl_h::BT_QUEST;
pub use crate::xmltok_impl_h::BT_QUOT;
pub use crate::xmltok_impl_h::BT_RPAR;
pub use crate::xmltok_impl_h::BT_RSQB;
pub use crate::xmltok_impl_h::BT_S;
pub use crate::xmltok_impl_h::BT_SEMI;
pub use crate::xmltok_impl_h::BT_SOL;
pub use crate::xmltok_impl_h::BT_TRAIL;
pub use crate::xmltok_impl_h::BT_VERBAR;

pub type NORMAL_ENCODING_CHAR_CHECK = extern "C" fn(
    *const crate::src::xmltok::ENCODING,
    *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;

#[derive(Copy, Clone)]
#[repr(C)]

pub struct normal_encoding {
    pub enc: crate::src::xmltok::ENCODING,
    pub type_0: [::core::ffi::c_uchar; 256],
    pub isName2: Option<NORMAL_ENCODING_CHAR_CHECK>,
    pub isName3: Option<NORMAL_ENCODING_CHAR_CHECK>,
    pub isName4: Option<NORMAL_ENCODING_CHAR_CHECK>,
    pub isNmstrt2: Option<NORMAL_ENCODING_CHAR_CHECK>,
    pub isNmstrt3: Option<NORMAL_ENCODING_CHAR_CHECK>,
    pub isNmstrt4: Option<NORMAL_ENCODING_CHAR_CHECK>,
    pub isInvalid2: Option<NORMAL_ENCODING_CHAR_CHECK>,
    pub isInvalid3: Option<NORMAL_ENCODING_CHAR_CHECK>,
    pub isInvalid4: Option<NORMAL_ENCODING_CHAR_CHECK>,
}

pub const UTF8_cval2: C2Rust_Unnamed_8 = 192;

pub const UTF8_cval4: C2Rust_Unnamed_8 = 240;

pub const UTF8_cval3: C2Rust_Unnamed_8 = 224;

pub const UNKNOWN_ENC: C2Rust_Unnamed_9 = -1;

pub const NO_ENC: C2Rust_Unnamed_9 = 6;

pub const UTF_16LE_ENC: C2Rust_Unnamed_9 = 5;

pub const UTF_16BE_ENC: C2Rust_Unnamed_9 = 4;

pub const UTF_8_ENC: C2Rust_Unnamed_9 = 2;

pub const UTF_16_ENC: C2Rust_Unnamed_9 = 3;

pub const ISO_8859_1_ENC: C2Rust_Unnamed_9 = 0;

pub const min4: C2Rust_Unnamed_7 = 65536;

pub const min3: C2Rust_Unnamed_7 = 2048;

pub const UTF8_cval1: C2Rust_Unnamed_8 = 0;

pub const min2: C2Rust_Unnamed_7 = 128;

pub type C2Rust_Unnamed_7 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unknown_encoding {
    pub normal: normal_encoding,
    pub convert: crate::src::xmltok::CONVERTER,
    pub userData: *mut ::core::ffi::c_void,
    pub utf16: [::core::ffi::c_ushort; 256],
    pub utf8: [[::core::ffi::c_char; 4]; 256],
}

pub type C2Rust_Unnamed_8 = ::core::ffi::c_uint;

pub type C2Rust_Unnamed_9 = ::core::ffi::c_int;

pub const US_ASCII_ENC: C2Rust_Unnamed_9 = 1;

extern "C" fn isNever(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}

fn normal_ascii_byte(p: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    utf8_byte(p, 0)
}

fn little2_ascii_byte(p: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if utf8_byte(p, 1) == 0 {
        utf8_byte(p, 0)
    } else {
        -1
    }
}

fn big2_ascii_byte(p: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if utf8_byte(p, 0) == 0 {
        utf8_byte(p, 1)
    } else {
        -1
    }
}

fn normal_encoded_byte_type(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    normal_byte_type(enc, utf8_byte(p, 0))
}

fn little2_encoded_byte_type(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let lo = utf8_byte(p, 0);
    let hi = utf8_byte(p, 1);
    if hi == 0 {
        normal_byte_type(enc, lo)
    } else {
        unicode_byte_type(
            hi as ::core::ffi::c_uchar as ::core::ffi::c_char,
            lo as ::core::ffi::c_uchar as ::core::ffi::c_char,
        )
    }
}

fn big2_encoded_byte_type(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let hi = utf8_byte(p, 0);
    let lo = utf8_byte(p, 1);
    if hi == 0 {
        normal_byte_type(enc, lo)
    } else {
        unicode_byte_type(
            hi as ::core::ffi::c_uchar as ::core::ffi::c_char,
            lo as ::core::ffi::c_uchar as ::core::ffi::c_char,
        )
    }
}

enum NormalCharCheck {
    Invalid,
    Name,
    NameStart,
}

enum XmlDeclEncodingAction<'a> {
    MinBytes,
    ToAscii {
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    },
    NameMatchesAscii {
        name: *const ::core::ffi::c_char,
        name_end: *const ::core::ffi::c_char,
        ascii: *const ::core::ffi::c_char,
    },
    ConvertToUtf8Name {
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    },
    FindEncoding {
        finder: unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *const crate::src::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    },
    UpdatePosition {
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        pos: &'a mut crate::src::xmltok::POSITION,
    },
}

enum XmlDeclEncodingResult {
    Int(::core::ffi::c_int),
    Encoding(*const crate::src::xmltok::ENCODING),
    EncodingName(Option<[::core::ffi::c_char; 128]>),
}

enum EncodingDataLookup<'a> {
    RawByte {
        p: *const ::core::ffi::c_char,
        offset: usize,
    },
    WriteNextTokPtr {
        dst: *mut *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    },
    WriteAttribute {
        dst: *mut crate::src::xmltok::ATTRIBUTE,
        value: crate::src::xmltok::ATTRIBUTE,
    },
    NormalByteType(::core::ffi::c_int),
    NormalCharCheck {
        p: *const ::core::ffi::c_char,
        width: usize,
        check: NormalCharCheck,
    },
    Unknown(UnknownEncodingLookup),
    InitScan {
        encoding_table: *const *const crate::src::xmltok::ENCODING,
        init_encoding: *const crate::src::xmltok::INIT_ENCODING,
        state: ::core::ffi::c_int,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        next_tok_ptr: *mut *const ::core::ffi::c_char,
    },
    XmlDecl(XmlDeclEncodingAction<'a>),
}

enum EncodingDataValue {
    Int(::core::ffi::c_int),
    Bool(bool),
    Unknown(UnknownEncodingValue),
    XmlDecl(XmlDeclEncodingResult),
    Unit,
}

fn encoding_data_lookup(
    enc: *const crate::src::xmltok::ENCODING,
    lookup: EncodingDataLookup<'_>,
) -> EncodingDataValue {
    unsafe {
        match lookup {
            EncodingDataLookup::RawByte { p, offset } => EncodingDataValue::Int(
                *(p as *const ::core::ffi::c_uchar).add(offset) as ::core::ffi::c_int,
            ),
            EncodingDataLookup::WriteNextTokPtr { dst, value } => {
                *dst = value;
                EncodingDataValue::Unit
            }
            EncodingDataLookup::WriteAttribute { dst, value } => {
                *dst = value;
                EncodingDataValue::Unit
            }
            EncodingDataLookup::NormalByteType(byte) => {
                let normal = &*(enc as *const normal_encoding);
                EncodingDataValue::Int(
                    normal.type_0[byte as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int,
                )
            }
            EncodingDataLookup::NormalCharCheck { p, width, check } => {
                let normal = &*(enc as *const normal_encoding);
                let callback = match check {
                    NormalCharCheck::Invalid => match width {
                        2 => normal.isInvalid2,
                        3 => normal.isInvalid3,
                        4 => normal.isInvalid4,
                        _ => None,
                    },
                    NormalCharCheck::Name => match width {
                        2 => normal.isName2,
                        3 => normal.isName3,
                        4 => normal.isName4,
                        _ => None,
                    },
                    NormalCharCheck::NameStart => match width {
                        2 => normal.isNmstrt2,
                        3 => normal.isNmstrt3,
                        4 => normal.isNmstrt4,
                        _ => None,
                    },
                };
                EncodingDataValue::Bool(match callback {
                    Some(callback) => callback(enc, p) != 0,
                    None => false,
                })
            }
            EncodingDataLookup::Unknown(lookup) => {
                let uenc = &*(enc as *const unknown_encoding);
                EncodingDataValue::Unknown(match lookup {
                    UnknownEncodingLookup::Convert(p) => {
                        let convert = uenc.convert.expect("non-null function pointer");
                        UnknownEncodingValue::Code(convert(uenc.userData, p))
                    }
                    UnknownEncodingLookup::Utf8(byte) => {
                        UnknownEncodingValue::Utf8(uenc.utf8[byte])
                    }
                    UnknownEncodingLookup::Utf16(byte) => {
                        UnknownEncodingValue::Utf16(uenc.utf16[byte])
                    }
                })
            }
            EncodingDataLookup::InitScan {
                encoding_table,
                init_encoding,
                state,
                ptr,
                end,
                next_tok_ptr,
            } => {
                let enc_ptr = (*init_encoding).encPtr;
                if ptr >= end {
                    return EncodingDataValue::Int(crate::src::xmltok::XML_TOK_NONE_1);
                }
                if ptr.offset(1) == end {
                    match (*init_encoding).initEnc.isUtf16 as ::core::ffi::c_int {
                        3 | 5 | 4 => {
                            return EncodingDataValue::Int(crate::src::xmltok::XML_TOK_PARTIAL_1)
                        }
                        _ => {}
                    }
                    let partial = match *ptr as ::core::ffi::c_uchar as ::core::ffi::c_int {
                        254 | 255 | 239 => {
                            !((*init_encoding).initEnc.isUtf16 as ::core::ffi::c_int
                                == ISO_8859_1_ENC as ::core::ffi::c_int
                                && state == crate::src::xmltok::XML_CONTENT_STATE)
                        }
                        0 | 60 => true,
                        _ => false,
                    };
                    if partial {
                        return EncodingDataValue::Int(crate::src::xmltok::XML_TOK_PARTIAL_1);
                    }
                } else {
                    match (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int) << 8
                        | *ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int
                    {
                        65279 => {
                            if !((*init_encoding).initEnc.isUtf16 as ::core::ffi::c_int
                                == ISO_8859_1_ENC as ::core::ffi::c_int
                                && state == crate::src::xmltok::XML_CONTENT_STATE)
                            {
                                *next_tok_ptr = ptr.offset(2);
                                *enc_ptr = *encoding_table
                                    .offset(UTF_16BE_ENC as ::core::ffi::c_int as isize);
                                return EncodingDataValue::Int(crate::src::xmltok::XML_TOK_BOM_1);
                            }
                        }
                        15360 => {
                            if !(((*init_encoding).initEnc.isUtf16 as ::core::ffi::c_int
                                == UTF_16BE_ENC as ::core::ffi::c_int
                                || (*init_encoding).initEnc.isUtf16 as ::core::ffi::c_int
                                    == UTF_16_ENC as ::core::ffi::c_int)
                                && state == crate::src::xmltok::XML_CONTENT_STATE)
                            {
                                *enc_ptr = *encoding_table
                                    .offset(UTF_16LE_ENC as ::core::ffi::c_int as isize);
                                return EncodingDataValue::Int((**enc_ptr).scanners
                                    [state as usize]
                                    .expect("non-null function pointer")(
                                    *enc_ptr,
                                    ptr,
                                    end,
                                    next_tok_ptr,
                                ));
                            }
                        }
                        65534 => {
                            if !((*init_encoding).initEnc.isUtf16 as ::core::ffi::c_int
                                == ISO_8859_1_ENC as ::core::ffi::c_int
                                && state == crate::src::xmltok::XML_CONTENT_STATE)
                            {
                                *next_tok_ptr = ptr.offset(2);
                                *enc_ptr = *encoding_table
                                    .offset(UTF_16LE_ENC as ::core::ffi::c_int as isize);
                                return EncodingDataValue::Int(crate::src::xmltok::XML_TOK_BOM_1);
                            }
                        }
                        61371 => {
                            let skip_utf8_bom = if state == crate::src::xmltok::XML_CONTENT_STATE {
                                let e = (*init_encoding).initEnc.isUtf16 as ::core::ffi::c_int;
                                e == ISO_8859_1_ENC as ::core::ffi::c_int
                                    || e == UTF_16BE_ENC as ::core::ffi::c_int
                                    || e == UTF_16LE_ENC as ::core::ffi::c_int
                                    || e == UTF_16_ENC as ::core::ffi::c_int
                            } else {
                                false
                            };
                            if !skip_utf8_bom {
                                if ptr.offset(2) == end {
                                    return EncodingDataValue::Int(
                                        crate::src::xmltok::XML_TOK_PARTIAL_1,
                                    );
                                }
                                if *ptr.offset(2) as ::core::ffi::c_uchar as ::core::ffi::c_int
                                    == 0xbf
                                {
                                    *next_tok_ptr = ptr.offset(3);
                                    *enc_ptr = *encoding_table
                                        .offset(UTF_8_ENC as ::core::ffi::c_int as isize);
                                    return EncodingDataValue::Int(
                                        crate::src::xmltok::XML_TOK_BOM_1,
                                    );
                                }
                            }
                        }
                        _ => {
                            if *ptr.offset(0) as ::core::ffi::c_int == '\0' as i32 {
                                if !(state == crate::src::xmltok::XML_CONTENT_STATE
                                    && (*init_encoding).initEnc.isUtf16 as ::core::ffi::c_int
                                        == UTF_16LE_ENC as ::core::ffi::c_int)
                                {
                                    *enc_ptr = *encoding_table
                                        .offset(UTF_16BE_ENC as ::core::ffi::c_int as isize);
                                    return EncodingDataValue::Int((**enc_ptr).scanners
                                        [state as usize]
                                        .expect("non-null function pointer")(
                                        *enc_ptr,
                                        ptr,
                                        end,
                                        next_tok_ptr,
                                    ));
                                }
                            } else if *ptr.offset(1) as ::core::ffi::c_int == '\0' as i32
                                && state != crate::src::xmltok::XML_CONTENT_STATE
                            {
                                *enc_ptr = *encoding_table
                                    .offset(UTF_16LE_ENC as ::core::ffi::c_int as isize);
                                return EncodingDataValue::Int((**enc_ptr).scanners
                                    [state as usize]
                                    .expect("non-null function pointer")(
                                    *enc_ptr,
                                    ptr,
                                    end,
                                    next_tok_ptr,
                                ));
                            }
                        }
                    }
                }
                *enc_ptr = *encoding_table
                    .offset((*init_encoding).initEnc.isUtf16 as ::core::ffi::c_int as isize);
                EncodingDataValue::Int((**enc_ptr).scanners[state as usize]
                    .expect("non-null function pointer")(
                    *enc_ptr, ptr, end, next_tok_ptr
                ))
            }
            EncodingDataLookup::XmlDecl(action) => EncodingDataValue::XmlDecl(match action {
                XmlDeclEncodingAction::MinBytes => {
                    XmlDeclEncodingResult::Int((*enc).minBytesPerChar)
                }
                XmlDeclEncodingAction::ToAscii { mut ptr, end } => {
                    let mut buf: [::core::ffi::c_char; 1] = [0; 1];
                    let mut p = buf.as_mut_ptr();
                    let to_lim = p.wrapping_add(1);
                    (*enc).utf8Convert.expect("non-null function pointer")(
                        enc,
                        &raw mut ptr,
                        end,
                        &raw mut p,
                        to_lim,
                    );
                    XmlDeclEncodingResult::Int(if p == buf.as_mut_ptr() {
                        -1 as ::core::ffi::c_int
                    } else {
                        buf[0] as ::core::ffi::c_int
                    })
                }
                XmlDeclEncodingAction::NameMatchesAscii {
                    name,
                    name_end,
                    ascii,
                } => XmlDeclEncodingResult::Int((*enc)
                    .nameMatchesAscii
                    .expect("non-null function pointer")(
                    enc, name, name_end, ascii
                )),
                XmlDeclEncodingAction::ConvertToUtf8Name { mut ptr, end } => {
                    let mut buf: [::core::ffi::c_char; 128] = [0; 128];
                    let mut p = buf.as_mut_ptr();
                    let to_lim = p.wrapping_add(buf.len() - 1);
                    (*enc).utf8Convert.expect("non-null function pointer")(
                        enc,
                        &raw mut ptr,
                        end,
                        &raw mut p,
                        to_lim,
                    );
                    if ptr != end {
                        XmlDeclEncodingResult::EncodingName(None)
                    } else {
                        *p = 0 as ::core::ffi::c_char;
                        XmlDeclEncodingResult::EncodingName(Some(buf))
                    }
                }
                XmlDeclEncodingAction::FindEncoding { finder, ptr, end } => {
                    XmlDeclEncodingResult::Encoding(finder(enc, ptr, end))
                }
                XmlDeclEncodingAction::UpdatePosition { ptr, end, pos } => {
                    (*enc).updatePosition.expect("non-null function pointer")(enc, ptr, end, pos);
                    XmlDeclEncodingResult::Int(0)
                }
            }),
        }
    }
}

fn utf8_byte(p: *const ::core::ffi::c_char, offset: usize) -> ::core::ffi::c_int {
    match encoding_data_lookup(
        ::core::ptr::null::<crate::src::xmltok::ENCODING>(),
        EncodingDataLookup::RawByte { p, offset },
    ) {
        EncodingDataValue::Int(value) => value,
        _ => unreachable!(),
    }
}

fn normal_char_check(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
    width: usize,
    check: NormalCharCheck,
) -> bool {
    match encoding_data_lookup(enc, EncodingDataLookup::NormalCharCheck { p, width, check }) {
        EncodingDataValue::Bool(matches) => matches,
        _ => unreachable!(),
    }
}

fn normal_invalid_char(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
    width: usize,
) -> bool {
    normal_char_check(enc, p, width, NormalCharCheck::Invalid)
}

fn normal_name_char(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
    width: usize,
) -> bool {
    !normal_invalid_char(enc, p, width) && normal_char_check(enc, p, width, NormalCharCheck::Name)
}

fn normal_name_start_char(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
    width: usize,
) -> bool {
    !normal_invalid_char(enc, p, width)
        && normal_char_check(enc, p, width, NormalCharCheck::NameStart)
}

fn normal_scan_name_width(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    t: ::core::ffi::c_int,
    start: bool,
) -> Result<usize, ::core::ffi::c_int> {
    let Some(width) = lead_byte_width(t) else {
        return Err(crate::src::xmltok::XML_TOK_INVALID_1);
    };
    if byte_distance(p, end) < width as ::core::ffi::c_long {
        return Err(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1);
    }
    let valid = if start {
        normal_name_start_char(enc, p, width)
    } else {
        normal_name_char(enc, p, width)
    };
    if valid {
        Ok(width)
    } else {
        Err(crate::src::xmltok::XML_TOK_INVALID_1)
    }
}

fn normal_pi_content_char(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
    width: usize,
) -> bool {
    !normal_invalid_char(enc, p, width)
}

fn never_invalid_char(
    _enc: *const crate::src::xmltok::ENCODING,
    _p: *const ::core::ffi::c_char,
    _width: usize,
) -> bool {
    false
}

fn never_lead_char(
    _enc: *const crate::src::xmltok::ENCODING,
    _p: *const ::core::ffi::c_char,
    _width: usize,
) -> bool {
    false
}

fn always_lead_char(
    _enc: *const crate::src::xmltok::ENCODING,
    _p: *const ::core::ffi::c_char,
    _width: usize,
) -> bool {
    true
}

type TokScanner = extern "C" fn(
    *const crate::src::xmltok::ENCODING,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;

fn scan_lt(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    nonascii_name_start: fn(*const ::core::ffi::c_char) -> bool,
    nonascii_name: fn(*const ::core::ffi::c_char) -> bool,
    lead_name_start: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
    lead_name: fn(*const crate::src::xmltok::ENCODING, *const ::core::ffi::c_char, usize) -> bool,
    scan_atts: TokScanner,
    scan_comment: TokScanner,
    scan_cdata_section: TokScanner,
    scan_pi: TokScanner,
    scan_end_tag: TokScanner,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) < unit {
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    match byte_type(enc, ptr) {
        t if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int => {
            if !nonascii_name_start(ptr) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(width);
        }
        t if is_pi_name_start_type(t) => {
            ptr = ptr.wrapping_add(width);
        }
        t if lead_byte_width(t).is_some() => {
            let lead_width = lead_byte_width(t).expect("checked above");
            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if !lead_name_start(enc, ptr, lead_width) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(lead_width);
        }
        t if t == crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int => {
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < unit {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }
            match byte_type(enc, ptr) {
                t if t == crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int => {
                    return scan_comment(enc, ptr.wrapping_add(width), end, next_tok_ptr);
                }
                t if t == crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int => {
                    return scan_cdata_section(enc, ptr.wrapping_add(width), end, next_tok_ptr);
                }
                _ => {}
            }
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        t if t == crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int => {
            return scan_pi(enc, ptr.wrapping_add(width), end, next_tok_ptr);
        }
        t if t == crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int => {
            return scan_end_tag(enc, ptr.wrapping_add(width), end, next_tok_ptr);
        }
        _ => {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
    }

    let mut had_colon = false;
    while byte_distance(ptr, end) >= unit {
        match byte_type(enc, ptr) {
            t if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int => {
                if !nonascii_name(ptr) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.wrapping_add(width);
            }
            t if is_pi_name_type(t) => {
                ptr = ptr.wrapping_add(width);
            }
            t if lead_byte_width(t).is_some() => {
                let lead_width = lead_byte_width(t).expect("checked above");
                if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                    return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if !lead_name(enc, ptr, lead_width) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.wrapping_add(lead_width);
            }
            t if t == crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int => {
                if had_colon {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                had_colon = true;
                ptr = ptr.wrapping_add(width);
                if byte_distance(ptr, end) < unit {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                match byte_type(enc, ptr) {
                    t if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int => {
                        if !nonascii_name_start(ptr) {
                            set_next_tok_ptr(next_tok_ptr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                        ptr = ptr.wrapping_add(width);
                    }
                    t if is_pi_name_start_type(t) => {
                        ptr = ptr.wrapping_add(width);
                    }
                    t if lead_byte_width(t).is_some() => {
                        let lead_width = lead_byte_width(t).expect("checked above");
                        if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        if !lead_name_start(enc, ptr, lead_width) {
                            set_next_tok_ptr(next_tok_ptr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                        ptr = ptr.wrapping_add(lead_width);
                    }
                    _ => {
                        set_next_tok_ptr(next_tok_ptr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
            }
            t if is_pi_target_end_type(t) => {
                ptr = ptr.wrapping_add(width);
                loop {
                    if byte_distance(ptr, end) < unit {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    match byte_type(enc, ptr) {
                        t if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int => {
                            if !nonascii_name_start(ptr) {
                                set_next_tok_ptr(next_tok_ptr, ptr);
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.wrapping_add(width);
                            return scan_atts(enc, ptr, end, next_tok_ptr);
                        }
                        t if is_pi_name_start_type(t) => {
                            ptr = ptr.wrapping_add(width);
                            return scan_atts(enc, ptr, end, next_tok_ptr);
                        }
                        t if lead_byte_width(t).is_some() => {
                            let lead_width = lead_byte_width(t).expect("checked above");
                            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if !lead_name_start(enc, ptr, lead_width) {
                                set_next_tok_ptr(next_tok_ptr, ptr);
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.wrapping_add(lead_width);
                            return scan_atts(enc, ptr, end, next_tok_ptr);
                        }
                        t if t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int => {
                            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                            return crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS_1;
                        }
                        t if t == crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int => {
                            ptr = ptr.wrapping_add(width);
                            if byte_distance(ptr, end) < unit {
                                return crate::src::xmltok::XML_TOK_PARTIAL_1;
                            }
                            if ascii_byte(ptr) != crate::ascii_h::ASCII_GT {
                                set_next_tok_ptr(next_tok_ptr, ptr);
                                return crate::src::xmltok::XML_TOK_INVALID_1;
                            }
                            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                            return crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                        }
                        t if is_pi_target_end_type(t) => {
                            ptr = ptr.wrapping_add(width);
                        }
                        _ => {
                            set_next_tok_ptr(next_tok_ptr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                }
            }
            t if t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int => {
                set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                return crate::src::xmltok::XML_TOK_START_TAG_NO_ATTS_1;
            }
            t if t == crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int => {
                ptr = ptr.wrapping_add(width);
                if byte_distance(ptr, end) < unit {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                if ascii_byte(ptr) != crate::ascii_h::ASCII_GT {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                return crate::src::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
            }
            _ => {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
        }
    }

    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn content_tok(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    invalid_char: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
    scan_lt: TokScanner,
    scan_ref: TokScanner,
) -> ::core::ffi::c_int {
    if ptr >= end {
        return crate::src::xmltok::XML_TOK_NONE_1;
    }
    if width > 1 {
        let mut n = byte_distance(ptr, end) as crate::__stddef_size_t_h::size_t;
        if n & (width - 1) as crate::__stddef_size_t_h::size_t != 0 {
            n &= !(width - 1) as crate::__stddef_size_t_h::size_t;
            if n == 0 {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }
            end = ptr.wrapping_add(n);
        }
    }

    match byte_type(enc, ptr) {
        t if t == crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int => {
            return scan_lt(enc, ptr.wrapping_add(width), end, next_tok_ptr);
        }
        t if t == crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int => {
            return scan_ref(enc, ptr.wrapping_add(width), end, next_tok_ptr);
        }
        t if t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int => {
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_TRAILING_CR_1;
            }
            if byte_type(enc, ptr) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int {
                ptr = ptr.wrapping_add(width);
            }
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
        }
        t if t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int => {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
        }
        t if t == crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int => {
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_TRAILING_RSQB_1;
            }
            if ascii_byte(ptr) == 0x5d {
                ptr = ptr.wrapping_add(width);
                if byte_distance(ptr, end) < width as ::core::ffi::c_long {
                    return crate::src::xmltok::XML_TOK_TRAILING_RSQB_1;
                }
                if ascii_byte(ptr) == crate::ascii_h::ASCII_GT {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.wrapping_sub(width);
            }
        }
        t if t == crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int => {
            if byte_distance(ptr, end) < 2 {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if invalid_char(enc, ptr, 2) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(2);
        }
        t if t == crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int => {
            if byte_distance(ptr, end) < 3 {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if invalid_char(enc, ptr, 3) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(3);
        }
        t if t == crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int => {
            if byte_distance(ptr, end) < 4 {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if invalid_char(enc, ptr, 4) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(4);
        }
        t if t == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int =>
        {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        _ => {
            ptr = ptr.wrapping_add(width);
        }
    }

    while byte_distance(ptr, end) >= width as ::core::ffi::c_long {
        match byte_type(enc, ptr) {
            t if t == crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int => {
                if byte_distance(ptr, end) < 2 || invalid_char(enc, ptr, 2) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                ptr = ptr.wrapping_add(2);
            }
            t if t == crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int => {
                if byte_distance(ptr, end) < 3 || invalid_char(enc, ptr, 3) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                ptr = ptr.wrapping_add(3);
            }
            t if t == crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int => {
                if byte_distance(ptr, end) < 4 || invalid_char(enc, ptr, 4) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                ptr = ptr.wrapping_add(4);
            }
            t if t == crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int => {
                if byte_distance(ptr, end) >= (2 * width) as ::core::ffi::c_long {
                    if ascii_byte(ptr.wrapping_add(width)) != 0x5d {
                        ptr = ptr.wrapping_add(width);
                        continue;
                    }
                    if byte_distance(ptr, end) >= (3 * width) as ::core::ffi::c_long {
                        if ascii_byte(ptr.wrapping_add(2 * width)) != crate::ascii_h::ASCII_GT {
                            ptr = ptr.wrapping_add(width);
                            continue;
                        }
                        set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(2 * width));
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                }
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
            }
            t if t == crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int =>
            {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
            }
            _ => {
                ptr = ptr.wrapping_add(width);
            }
        }
    }
    set_next_tok_ptr(next_tok_ptr, ptr);
    crate::src::xmltok::XML_TOK_DATA_CHARS_1
}

fn literal_value_tok(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    scan_ref: TokScanner,
    scan_percent: Option<TokScanner>,
    quote_is_invalid: bool,
    report_attribute_space: bool,
) -> ::core::ffi::c_int {
    if ptr >= end {
        return crate::src::xmltok::XML_TOK_NONE_1;
    } else if byte_distance(ptr, end) < width as ::core::ffi::c_long {
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    let start = ptr;
    while byte_distance(ptr, end) >= width as ::core::ffi::c_long {
        match byte_type(enc, ptr) {
            5 => {
                ptr = ptr.wrapping_add(2);
            }
            6 => {
                ptr = ptr.wrapping_add(3);
            }
            7 => {
                ptr = ptr.wrapping_add(4);
            }
            3 => {
                if ptr == start {
                    return scan_ref(enc, ptr.wrapping_add(width), end, next_tok_ptr);
                }
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
            }
            2 if quote_is_invalid => {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            30 if scan_percent.is_some() => {
                if ptr == start {
                    let tok = scan_percent.expect("checked above")(
                        enc,
                        ptr.wrapping_add(width),
                        end,
                        next_tok_ptr,
                    );
                    return if tok == crate::src::xmltok::XML_TOK_PERCENT_1 {
                        crate::src::xmltok::XML_TOK_INVALID_1
                    } else {
                        tok
                    };
                }
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
            }
            10 => {
                if ptr == start {
                    set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                    return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
                }
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
            }
            9 => {
                if ptr == start {
                    ptr = ptr.wrapping_add(width);
                    if byte_distance(ptr, end) < width as ::core::ffi::c_long {
                        return crate::src::xmltok::XML_TOK_TRAILING_CR_1;
                    }
                    if byte_type(enc, ptr) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int {
                        ptr = ptr.wrapping_add(width);
                    }
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
                }
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
            }
            21 if report_attribute_space => {
                if ptr == start {
                    set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                    return crate::src::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1;
                }
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
            }
            _ => {
                ptr = ptr.wrapping_add(width);
            }
        }
    }
    set_next_tok_ptr(next_tok_ptr, ptr);
    crate::src::xmltok::XML_TOK_DATA_CHARS_1
}

fn scan_comment(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    invalid_char: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) >= unit {
        if ascii_byte(ptr) != crate::ascii_h::ASCII_MINUS {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(width);
        while byte_distance(ptr, end) >= unit {
            match byte_type(enc, ptr) {
                t if t == crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int => {
                    if byte_distance(ptr, end) < 2 {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if invalid_char(enc, ptr, 2) {
                        set_next_tok_ptr(next_tok_ptr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.wrapping_add(2);
                }
                t if t == crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int => {
                    if byte_distance(ptr, end) < 3 {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if invalid_char(enc, ptr, 3) {
                        set_next_tok_ptr(next_tok_ptr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.wrapping_add(3);
                }
                t if t == crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int => {
                    if byte_distance(ptr, end) < 4 {
                        return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if invalid_char(enc, ptr, 4) {
                        set_next_tok_ptr(next_tok_ptr, ptr);
                        return crate::src::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.wrapping_add(4);
                }
                t if matches!(
                    t,
                    t if t == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
                        || t == crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int
                        || t == crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int
                ) =>
                {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                t if t == crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int => {
                    ptr = ptr.wrapping_add(width);
                    if byte_distance(ptr, end) < unit {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if ascii_byte(ptr) == crate::ascii_h::ASCII_MINUS {
                        ptr = ptr.wrapping_add(width);
                        if byte_distance(ptr, end) < unit {
                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if ascii_byte(ptr) != crate::ascii_h::ASCII_GT {
                            set_next_tok_ptr(next_tok_ptr, ptr);
                            return crate::src::xmltok::XML_TOK_INVALID_1;
                        }
                        set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                        return crate::src::xmltok::XML_TOK_COMMENT_1;
                    }
                }
                _ => {
                    ptr = ptr.wrapping_add(width);
                }
            }
        }
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn scan_decl(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    scan_comment: extern "C" fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) < unit {
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }
    match byte_type(enc, ptr) {
        t if t == crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int => {
            return scan_comment(enc, ptr.wrapping_add(width), end, next_tok_ptr);
        }
        t if t == crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int => {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_COND_SECT_OPEN_1;
        }
        t if is_decl_name_type(t) => {
            ptr = ptr.wrapping_add(width);
        }
        _ => {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
    }

    while byte_distance(ptr, end) >= unit {
        match byte_type(enc, ptr) {
            t if t == crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int => {
                if byte_distance(ptr, end) < 2 * unit {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                let next = ptr.wrapping_add(width);
                if is_decl_percent_invalid_follower(byte_type(enc, next)) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
            t if is_decl_open_type(t) => {}
            t if is_decl_name_type(t) => {
                ptr = ptr.wrapping_add(width);
                continue;
            }
            _ => {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
        }
        set_next_tok_ptr(next_tok_ptr, ptr);
        return crate::src::xmltok::XML_TOK_DECL_OPEN_1;
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn is_decl_name_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if t == crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int
    )
}

fn is_decl_open_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
    )
}

fn is_decl_percent_invalid_follower(t: ::core::ffi::c_int) -> bool {
    is_decl_open_type(t) || t == crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int
}

fn lead_byte_width(t: ::core::ffi::c_int) -> Option<usize> {
    match t {
        t if t == crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int => Some(2),
        t if t == crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int => Some(3),
        t if t == crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int => Some(4),
        _ => None,
    }
}

fn is_pi_name_start_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if t == crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int
    )
}

fn is_pi_name_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if is_pi_name_start_type(t)
            || t == crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int
    )
}

fn is_pi_target_end_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
    )
}

fn is_pi_content_invalid_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if t == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int
    )
}

fn utf16_name_bitmap_char(
    high: ::core::ffi::c_int,
    low: ::core::ffi::c_int,
    name_start: bool,
) -> bool {
    let pages = if name_start { &nmstrtPages } else { &namePages };
    let high = high as ::core::ffi::c_uchar as usize;
    let low = low as ::core::ffi::c_uchar as ::core::ffi::c_int;
    let index = ((pages[high] as ::core::ffi::c_int) << 3) + (low >> 5);
    namingBitmap[index as usize] & ((1 as ::core::ffi::c_uint) << (low & 0x1f)) != 0
}

fn normal_nonascii_name_char(_p: *const ::core::ffi::c_char) -> bool {
    false
}

fn normal_nonascii_name_start_char(_p: *const ::core::ffi::c_char) -> bool {
    false
}

fn little2_nonascii_name_start_char(p: *const ::core::ffi::c_char) -> bool {
    utf16_name_bitmap_char(utf8_byte(p, 1), utf8_byte(p, 0), true)
}

fn little2_nonascii_name_char(p: *const ::core::ffi::c_char) -> bool {
    utf16_name_bitmap_char(utf8_byte(p, 1), utf8_byte(p, 0), false)
}

fn big2_nonascii_name_start_char(p: *const ::core::ffi::c_char) -> bool {
    utf16_name_bitmap_char(utf8_byte(p, 0), utf8_byte(p, 1), true)
}

fn big2_nonascii_name_char(p: *const ::core::ffi::c_char) -> bool {
    utf16_name_bitmap_char(utf8_byte(p, 0), utf8_byte(p, 1), false)
}

type ScanFn = extern "C" fn(
    *const crate::src::xmltok::ENCODING,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;

type ScanLitFn = extern "C" fn(
    ::core::ffi::c_int,
    *const crate::src::xmltok::ENCODING,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;

fn reject_lead_name_char(
    _enc: *const crate::src::xmltok::ENCODING,
    _p: *const ::core::ffi::c_char,
    _width: usize,
) -> bool {
    false
}

fn lead_name_result(
    enc: *const crate::src::xmltok::ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    t: ::core::ffi::c_int,
    name_start_char: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
    name_char: fn(*const crate::src::xmltok::ENCODING, *const ::core::ffi::c_char, usize) -> bool,
) -> Result<(::core::ffi::c_int, usize), ::core::ffi::c_int> {
    let width = lead_byte_width(t).expect("lead byte type");
    if byte_distance(ptr, end) < width as ::core::ffi::c_long {
        return Err(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1);
    }
    if name_start_char(enc, ptr, width) {
        Ok((crate::src::xmltok::XML_TOK_NAME, width))
    } else if name_char(enc, ptr, width) {
        Ok((crate::src::xmltok::XML_TOK_NMTOKEN_1, width))
    } else {
        Err(crate::src::xmltok::XML_TOK_INVALID_1)
    }
}

fn lead_name_continue(
    enc: *const crate::src::xmltok::ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    t: ::core::ffi::c_int,
    name_char: fn(*const crate::src::xmltok::ENCODING, *const ::core::ffi::c_char, usize) -> bool,
) -> Result<usize, ::core::ffi::c_int> {
    let width = lead_byte_width(t).expect("lead byte type");
    if byte_distance(ptr, end) < width as ::core::ffi::c_long {
        return Err(crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1);
    }
    if name_char(enc, ptr, width) {
        Ok(width)
    } else {
        Err(crate::src::xmltok::XML_TOK_INVALID_1)
    }
}

fn finish_prolog_whitespace(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    loop {
        ptr = ptr.wrapping_add(width);
        if byte_distance(ptr, end) < unit {
            break;
        }
        match byte_type(enc, ptr) {
            t if t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int =>
            {
                continue;
            }
            t if t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
                && ptr.wrapping_add(width) != end =>
            {
                continue;
            }
            _ => {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_PROLOG_S_1;
            }
        }
    }
    set_next_tok_ptr(next_tok_ptr, ptr);
    crate::src::xmltok::XML_TOK_PROLOG_S_1
}

fn prolog_tok(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    nonascii_name_start_char: fn(*const ::core::ffi::c_char) -> bool,
    nonascii_name_char: fn(*const ::core::ffi::c_char) -> bool,
    name_start_char: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
    name_char: fn(*const crate::src::xmltok::ENCODING, *const ::core::ffi::c_char, usize) -> bool,
    scan_lit: ScanLitFn,
    scan_decl: ScanFn,
    scan_pi: ScanFn,
    scan_percent: ScanFn,
    scan_pound_name: ScanFn,
) -> ::core::ffi::c_int {
    if ptr >= end {
        return crate::src::xmltok::XML_TOK_NONE_1;
    }
    if width > 1 {
        let mut n = byte_distance(ptr, end) as crate::__stddef_size_t_h::size_t;
        if n & (width - 1) as crate::__stddef_size_t_h::size_t != 0 {
            n &= !((width - 1) as crate::__stddef_size_t_h::size_t);
            if n == 0 {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }
            end = ptr.wrapping_add(n as usize);
        }
    }

    let unit = width as ::core::ffi::c_long;
    let mut tok: ::core::ffi::c_int;
    match byte_type(enc, ptr) {
        t if t == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int => {
            return scan_lit(
                crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int,
                enc,
                ptr.wrapping_add(width),
                end,
                next_tok_ptr,
            );
        }
        t if t == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int => {
            return scan_lit(
                crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int,
                enc,
                ptr.wrapping_add(width),
                end,
                next_tok_ptr,
            );
        }
        t if t == crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int => {
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < unit {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }
            match byte_type(enc, ptr) {
                t if t == crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int => {
                    return scan_decl(enc, ptr.wrapping_add(width), end, next_tok_ptr);
                }
                t if t == crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int => {
                    return scan_pi(enc, ptr.wrapping_add(width), end, next_tok_ptr);
                }
                t if t == crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int
                    || lead_byte_width(t).is_some() =>
                {
                    set_next_tok_ptr(next_tok_ptr, ptr.wrapping_sub(width));
                    return crate::src::xmltok::XML_TOK_INSTANCE_START;
                }
                _ => {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
        }
        t if t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int => {
            if ptr.wrapping_add(width) == end {
                set_next_tok_ptr(next_tok_ptr, end);
                return -(crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int);
            }
            return finish_prolog_whitespace(enc, ptr, end, next_tok_ptr, width, byte_type);
        }
        t if t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int =>
        {
            return finish_prolog_whitespace(enc, ptr, end, next_tok_ptr, width, byte_type);
        }
        t if t == crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int => {
            return scan_percent(enc, ptr.wrapping_add(width), end, next_tok_ptr);
        }
        t if t == crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int => {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_COMMA_1;
        }
        t if t == crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int => {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_OPEN_BRACKET_1;
        }
        t if t == crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int => {
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < unit {
                return -26;
            }
            if ascii_byte(ptr) == 0x5d {
                if byte_distance(ptr, end) < 2 * unit {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                if ascii_byte(ptr.wrapping_add(width)) == crate::ascii_h::ASCII_GT {
                    set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(2 * width));
                    return crate::src::xmltok::XML_TOK_COND_SECT_CLOSE_1;
                }
            }
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_CLOSE_BRACKET_1;
        }
        t if t == crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int => {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_OPEN_PAREN_1;
        }
        t if t == crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int => {
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < unit {
                return -24;
            }
            match byte_type(enc, ptr) {
                t if t == crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int => {
                    set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                    return crate::src::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK_1;
                }
                t if t == crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int => {
                    set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                    return crate::src::xmltok::XML_TOK_CLOSE_PAREN_QUESTION_1;
                }
                t if t == crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int => {
                    set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                    return crate::src::xmltok::XML_TOK_CLOSE_PAREN_PLUS_1;
                }
                t if t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int =>
                {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_CLOSE_PAREN_1;
                }
                _ => {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
        }
        t if t == crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int => {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_OR_1;
        }
        t if t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int => {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_DECL_CLOSE_1;
        }
        t if t == crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int => {
            return scan_pound_name(enc, ptr.wrapping_add(width), end, next_tok_ptr);
        }
        t if lead_byte_width(t).is_some() => {
            match lead_name_result(enc, ptr, end, t, name_start_char, name_char) {
                Ok((token, consumed)) => {
                    tok = token;
                    ptr = ptr.wrapping_add(consumed);
                }
                Err(result) => {
                    if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                        set_next_tok_ptr(next_tok_ptr, ptr);
                    }
                    return result;
                }
            }
        }
        t if t == crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int =>
        {
            tok = crate::src::xmltok::XML_TOK_NAME;
            ptr = ptr.wrapping_add(width);
        }
        t if t == crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int =>
        {
            tok = crate::src::xmltok::XML_TOK_NMTOKEN_1;
            ptr = ptr.wrapping_add(width);
        }
        t if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int => {
            if nonascii_name_start_char(ptr) {
                tok = crate::src::xmltok::XML_TOK_NAME;
                ptr = ptr.wrapping_add(width);
            } else if nonascii_name_char(ptr) {
                tok = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                ptr = ptr.wrapping_add(width);
            } else {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
        }
        _ => {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
    }

    while byte_distance(ptr, end) >= unit {
        match byte_type(enc, ptr) {
            t if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int => {
                if !nonascii_name_char(ptr) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.wrapping_add(width);
            }
            t if t == crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int =>
            {
                ptr = ptr.wrapping_add(width);
            }
            t if lead_byte_width(t).is_some() => {
                match lead_name_continue(enc, ptr, end, t, name_char) {
                    Ok(consumed) => ptr = ptr.wrapping_add(consumed),
                    Err(result) => {
                        if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                            set_next_tok_ptr(next_tok_ptr, ptr);
                        }
                        return result;
                    }
                }
            }
            t if t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int =>
            {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return tok;
            }
            t if t == crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int => {
                ptr = ptr.wrapping_add(width);
                match tok {
                    crate::src::xmltok::XML_TOK_NAME => {
                        if byte_distance(ptr, end) < unit {
                            return crate::src::xmltok::XML_TOK_PARTIAL_1;
                        }
                        tok = crate::src::xmltok::XML_TOK_PREFIXED_NAME;
                        match byte_type(enc, ptr) {
                            t if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int => {
                                if !nonascii_name_char(ptr) {
                                    set_next_tok_ptr(next_tok_ptr, ptr);
                                    return crate::src::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.wrapping_add(width);
                            }
                            t if t == crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int
                                || t == crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int
                                || t == crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int
                                || t == crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int
                                || t == crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int =>
                            {
                                ptr = ptr.wrapping_add(width);
                            }
                            t if lead_byte_width(t).is_some() => {
                                match lead_name_continue(enc, ptr, end, t, name_char) {
                                    Ok(consumed) => ptr = ptr.wrapping_add(consumed),
                                    Err(result) => {
                                        if result == crate::src::xmltok::XML_TOK_INVALID_1 {
                                            set_next_tok_ptr(next_tok_ptr, ptr);
                                        }
                                        return result;
                                    }
                                }
                            }
                            _ => tok = crate::src::xmltok::XML_TOK_NMTOKEN_1,
                        }
                    }
                    crate::src::xmltok::XML_TOK_PREFIXED_NAME => {
                        tok = crate::src::xmltok::XML_TOK_NMTOKEN_1;
                    }
                    _ => {}
                }
            }
            t if t == crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int => {
                if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                return crate::src::xmltok::XML_TOK_NAME_PLUS_1;
            }
            t if t == crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int => {
                if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                return crate::src::xmltok::XML_TOK_NAME_ASTERISK_1;
            }
            t if t == crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int => {
                if tok == crate::src::xmltok::XML_TOK_NMTOKEN_1 {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
                set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                return crate::src::xmltok::XML_TOK_NAME_QUESTION_1;
            }
            _ => {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
        }
    }
    -tok
}

fn scan_hex_char_ref(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) >= unit {
        if !is_char_ref_hex_type(byte_type(enc, ptr)) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(width);
        while byte_distance(ptr, end) >= unit {
            let t = byte_type(enc, ptr);
            if is_char_ref_hex_type(t) {
                ptr = ptr.wrapping_add(width);
            } else if t == crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int {
                set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                return crate::src::xmltok::XML_TOK_CHAR_REF_1;
            } else {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
        }
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn scan_char_ref(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) >= unit {
        if ascii_byte(ptr) == crate::ascii_h::ASCII_x_1 {
            return scan_hex_char_ref(
                enc,
                ptr.wrapping_add(width),
                end,
                next_tok_ptr,
                width,
                byte_type,
            );
        }
        if byte_type(enc, ptr) != crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(width);
        while byte_distance(ptr, end) >= unit {
            let t = byte_type(enc, ptr);
            if t == crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int {
                ptr = ptr.wrapping_add(width);
            } else if t == crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int {
                set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                return crate::src::xmltok::XML_TOK_CHAR_REF_1;
            } else {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
        }
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn scan_percent(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    nonascii_name_start: fn(*const ::core::ffi::c_char) -> bool,
    nonascii_name: fn(*const ::core::ffi::c_char) -> bool,
    lead_name_start: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
    lead_name: fn(*const crate::src::xmltok::ENCODING, *const ::core::ffi::c_char, usize) -> bool,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) < unit {
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    let t = byte_type(enc, ptr);
    if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
        if !nonascii_name_start(ptr) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(width);
    } else if is_ref_name_start_type(t) {
        ptr = ptr.wrapping_add(width);
    } else if let Some(lead_width) = lead_byte_width(t) {
        if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
        }
        if !lead_name_start(enc, ptr, lead_width) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(lead_width);
    } else if is_percent_literal_type(t) {
        set_next_tok_ptr(next_tok_ptr, ptr);
        return crate::src::xmltok::XML_TOK_PERCENT_1;
    } else {
        set_next_tok_ptr(next_tok_ptr, ptr);
        return crate::src::xmltok::XML_TOK_INVALID_1;
    }

    while byte_distance(ptr, end) >= unit {
        let t = byte_type(enc, ptr);
        if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
            if !nonascii_name(ptr) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(width);
        } else if is_ref_name_type(t) {
            ptr = ptr.wrapping_add(width);
        } else if let Some(lead_width) = lead_byte_width(t) {
            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if !lead_name(enc, ptr, lead_width) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(lead_width);
        } else if t == crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_PARAM_ENTITY_REF_1;
        } else {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn scan_pound_name(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    nonascii_name_start: fn(*const ::core::ffi::c_char) -> bool,
    nonascii_name: fn(*const ::core::ffi::c_char) -> bool,
    lead_name_start: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
    lead_name: fn(*const crate::src::xmltok::ENCODING, *const ::core::ffi::c_char, usize) -> bool,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) < unit {
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    let t = byte_type(enc, ptr);
    if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
        if !nonascii_name_start(ptr) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(width);
    } else if is_ref_name_start_type(t) {
        ptr = ptr.wrapping_add(width);
    } else if let Some(lead_width) = lead_byte_width(t) {
        if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
        }
        if !lead_name_start(enc, ptr, lead_width) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(lead_width);
    } else {
        set_next_tok_ptr(next_tok_ptr, ptr);
        return crate::src::xmltok::XML_TOK_INVALID_1;
    }

    while byte_distance(ptr, end) >= unit {
        let t = byte_type(enc, ptr);
        if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
            if !nonascii_name(ptr) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(width);
        } else if is_ref_name_type(t) {
            ptr = ptr.wrapping_add(width);
        } else if let Some(lead_width) = lead_byte_width(t) {
            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if !lead_name(enc, ptr, lead_width) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(lead_width);
        } else if is_pound_name_end_type(t) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_POUND_NAME_1;
        } else {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
    }
    -(crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int)
}

fn is_percent_literal_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int
    )
}

fn is_pound_name_end_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int
    )
}

fn scan_lit(
    open: ::core::ffi::c_int,
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    invalid_char: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    while byte_distance(ptr, end) >= unit {
        let t = byte_type(enc, ptr);
        if let Some(lead_width) = lead_byte_width(t) {
            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if invalid_char(enc, ptr, lead_width) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(lead_width);
        } else if is_lit_invalid_type(t) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        } else if is_lit_delimiter_type(t) {
            ptr = ptr.wrapping_add(width);
            if t == open {
                if byte_distance(ptr, end) < unit {
                    return -crate::src::xmltok::XML_TOK_LITERAL_1;
                }
                set_next_tok_ptr(next_tok_ptr, ptr);
                return if is_lit_end_follower_type(byte_type(enc, ptr)) {
                    crate::src::xmltok::XML_TOK_LITERAL_1
                } else {
                    crate::src::xmltok::XML_TOK_INVALID_1
                };
            }
        } else {
            ptr = ptr.wrapping_add(width);
        }
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn is_lit_invalid_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if t == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int
    )
}

fn is_lit_delimiter_type(t: ::core::ffi::c_int) -> bool {
    t == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
        || t == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
}

fn is_lit_end_follower_type(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        t if t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int
            || t == crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int
    )
}

fn scan_ref(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    nonascii_name_start: fn(*const ::core::ffi::c_char) -> bool,
    nonascii_name: fn(*const ::core::ffi::c_char) -> bool,
    lead_name_start: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
    lead_name: fn(*const crate::src::xmltok::ENCODING, *const ::core::ffi::c_char, usize) -> bool,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) < unit {
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    let t = byte_type(enc, ptr);
    if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
        if !nonascii_name_start(ptr) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(width);
    } else if is_ref_name_start_type(t) {
        ptr = ptr.wrapping_add(width);
    } else if let Some(lead_width) = lead_byte_width(t) {
        if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
        }
        if !lead_name_start(enc, ptr, lead_width) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(lead_width);
    } else if t == crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int {
        return scan_char_ref(
            enc,
            ptr.wrapping_add(width),
            end,
            next_tok_ptr,
            width,
            ascii_byte,
            byte_type,
        );
    } else {
        set_next_tok_ptr(next_tok_ptr, ptr);
        return crate::src::xmltok::XML_TOK_INVALID_1;
    }

    while byte_distance(ptr, end) >= unit {
        let t = byte_type(enc, ptr);
        if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
            if !nonascii_name(ptr) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(width);
        } else if is_ref_name_type(t) {
            ptr = ptr.wrapping_add(width);
        } else if let Some(lead_width) = lead_byte_width(t) {
            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if !lead_name(enc, ptr, lead_width) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(lead_width);
        } else if t == crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_ENTITY_REF_1;
        } else {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn is_char_ref_hex_type(t: ::core::ffi::c_int) -> bool {
    t == crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int
        || t == crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int
}

fn is_ref_name_start_type(t: ::core::ffi::c_int) -> bool {
    t == crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int
        || t == crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int
}

fn is_ref_name_type(t: ::core::ffi::c_int) -> bool {
    is_ref_name_start_type(t)
        || t == crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int
        || t == crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int
        || t == crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int
}

fn scan_end_tag(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    nonascii_name_start: fn(*const ::core::ffi::c_char) -> bool,
    nonascii_name: fn(*const ::core::ffi::c_char) -> bool,
    lead_name_start: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
    lead_name: fn(*const crate::src::xmltok::ENCODING, *const ::core::ffi::c_char, usize) -> bool,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) < unit {
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    let t = byte_type(enc, ptr);
    if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
        if !nonascii_name_start(ptr) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(width);
    } else if is_pi_name_start_type(t) {
        ptr = ptr.wrapping_add(width);
    } else if let Some(lead_width) = lead_byte_width(t) {
        if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
        }
        if !lead_name_start(enc, ptr, lead_width) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(lead_width);
    } else {
        set_next_tok_ptr(next_tok_ptr, ptr);
        return crate::src::xmltok::XML_TOK_INVALID_1;
    }

    while byte_distance(ptr, end) >= unit {
        let t = byte_type(enc, ptr);
        if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
            if !nonascii_name(ptr) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(width);
        } else if is_pi_name_type(t) || t == crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int
        {
            ptr = ptr.wrapping_add(width);
        } else if let Some(lead_width) = lead_byte_width(t) {
            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if !lead_name(enc, ptr, lead_width) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(lead_width);
        } else if is_pi_target_end_type(t) {
            ptr = ptr.wrapping_add(width);
            while byte_distance(ptr, end) >= unit {
                let t = byte_type(enc, ptr);
                if is_pi_target_end_type(t) {
                    ptr = ptr.wrapping_add(width);
                } else if t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int {
                    set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                    return crate::src::xmltok::XML_TOK_END_TAG_1;
                } else {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_INVALID_1;
                }
            }
            return crate::src::xmltok::XML_TOK_PARTIAL_1;
        } else if t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_END_TAG_1;
        } else {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn scan_pi_content(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    tok: ::core::ffi::c_int,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    lead_content: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
) -> ::core::ffi::c_int {
    let unit = width as ::core::ffi::c_long;
    while byte_distance(ptr, end) >= unit {
        let t = byte_type(enc, ptr);
        if let Some(lead_width) = lead_byte_width(t) {
            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if !lead_content(enc, ptr, lead_width) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(lead_width);
        } else if is_pi_content_invalid_type(t) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        } else if t == crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int {
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < unit {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }
            if ascii_byte(ptr) == crate::ascii_h::ASCII_GT {
                set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                return tok;
            }
        } else {
            ptr = ptr.wrapping_add(width);
        }
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn scan_pi(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    check_target: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        &mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
    nonascii_name_start: fn(*const ::core::ffi::c_char) -> bool,
    nonascii_name: fn(*const ::core::ffi::c_char) -> bool,
    lead_name_start: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
    lead_name: fn(*const crate::src::xmltok::ENCODING, *const ::core::ffi::c_char, usize) -> bool,
    lead_content: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
) -> ::core::ffi::c_int {
    let mut tok = 0 as ::core::ffi::c_int;
    let target = ptr;
    let unit = width as ::core::ffi::c_long;
    if byte_distance(ptr, end) < unit {
        return crate::src::xmltok::XML_TOK_PARTIAL_1;
    }

    let t = byte_type(enc, ptr);
    if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
        if !nonascii_name_start(ptr) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(width);
    } else if is_pi_name_start_type(t) {
        ptr = ptr.wrapping_add(width);
    } else if let Some(lead_width) = lead_byte_width(t) {
        if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
            return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
        }
        if !lead_name_start(enc, ptr, lead_width) {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        ptr = ptr.wrapping_add(lead_width);
    } else {
        set_next_tok_ptr(next_tok_ptr, ptr);
        return crate::src::xmltok::XML_TOK_INVALID_1;
    }

    while byte_distance(ptr, end) >= unit {
        let t = byte_type(enc, ptr);
        if t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int {
            if !nonascii_name(ptr) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(width);
        } else if is_pi_name_type(t) {
            ptr = ptr.wrapping_add(width);
        } else if let Some(lead_width) = lead_byte_width(t) {
            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if !lead_name(enc, ptr, lead_width) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(lead_width);
        } else if is_pi_target_end_type(t) {
            if check_target(enc, target, ptr, &mut tok) == 0 {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            return scan_pi_content(
                enc,
                ptr.wrapping_add(width),
                end,
                next_tok_ptr,
                tok,
                width,
                ascii_byte,
                byte_type,
                lead_content,
            );
        } else if t == crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int {
            if check_target(enc, target, ptr, &mut tok) == 0 {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < unit {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }
            if ascii_byte(ptr) == crate::ascii_h::ASCII_GT {
                set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                return tok;
            }
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        } else {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
    }
    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn cdata_section_tok(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    invalid_char: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
) -> ::core::ffi::c_int {
    if ptr >= end {
        return crate::src::xmltok::XML_TOK_NONE_1;
    }
    if width > 1 {
        let mut n = byte_distance(ptr, end) as usize;
        if n & (width - 1) != 0 {
            n &= !(width - 1);
            if n == 0 {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }
            end = ptr.wrapping_add(n);
        }
    }

    let unit = width as ::core::ffi::c_long;
    match byte_type(enc, ptr) {
        t if t == crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int => {
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < unit {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }
            if ascii_byte(ptr) == 0x5d {
                ptr = ptr.wrapping_add(width);
                if byte_distance(ptr, end) < unit {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                if ascii_byte(ptr) == crate::ascii_h::ASCII_GT {
                    set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
                    return crate::src::xmltok::XML_TOK_CDATA_SECT_CLOSE_1;
                }
                ptr = ptr.wrapping_sub(width);
            }
        }
        t if t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int => {
            ptr = ptr.wrapping_add(width);
            if byte_distance(ptr, end) < unit {
                return crate::src::xmltok::XML_TOK_PARTIAL_1;
            }
            if byte_type(enc, ptr) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int {
                ptr = ptr.wrapping_add(width);
            }
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
        }
        t if t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int => {
            set_next_tok_ptr(next_tok_ptr, ptr.wrapping_add(width));
            return crate::src::xmltok::XML_TOK_DATA_NEWLINE_1;
        }
        t if t == crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int => {
            if byte_distance(ptr, end) < 2 {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if invalid_char(enc, ptr, 2) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(2);
        }
        t if t == crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int => {
            if byte_distance(ptr, end) < 3 {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if invalid_char(enc, ptr, 3) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(3);
        }
        t if t == crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int => {
            if byte_distance(ptr, end) < 4 {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if invalid_char(enc, ptr, 4) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(4);
        }
        t if matches!(
            t,
            t if t == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int
        ) =>
        {
            set_next_tok_ptr(next_tok_ptr, ptr);
            return crate::src::xmltok::XML_TOK_INVALID_1;
        }
        _ => {
            ptr = ptr.wrapping_add(width);
        }
    }

    while byte_distance(ptr, end) >= unit {
        match byte_type(enc, ptr) {
            t if t == crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int => {
                if byte_distance(ptr, end) < 2 || invalid_char(enc, ptr, 2) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                ptr = ptr.wrapping_add(2);
            }
            t if t == crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int => {
                if byte_distance(ptr, end) < 3 || invalid_char(enc, ptr, 3) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                ptr = ptr.wrapping_add(3);
            }
            t if t == crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int => {
                if byte_distance(ptr, end) < 4 || invalid_char(enc, ptr, 4) {
                    set_next_tok_ptr(next_tok_ptr, ptr);
                    return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
                }
                ptr = ptr.wrapping_add(4);
            }
            t if matches!(
                t,
                t if t == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int
            ) =>
            {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_DATA_CHARS_1;
            }
            _ => {
                ptr = ptr.wrapping_add(width);
            }
        }
    }
    set_next_tok_ptr(next_tok_ptr, ptr);
    crate::src::xmltok::XML_TOK_DATA_CHARS_1
}

fn public_id_type_is_allowed(t: ::core::ffi::c_int) -> bool {
    matches!(
        t,
        25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33 | 30 | 19 | 23
    )
}

fn set_next_tok_ptr(
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    ptr: *const ::core::ffi::c_char,
) {
    match encoding_data_lookup(
        ::core::ptr::null::<crate::src::xmltok::ENCODING>(),
        EncodingDataLookup::WriteNextTokPtr {
            dst: next_tok_ptr,
            value: ptr,
        },
    ) {
        EncodingDataValue::Unit => {}
        _ => unreachable!(),
    }
}

fn current_attribute(
    atts: *mut crate::src::xmltok::ATTRIBUTE,
    index: ::core::ffi::c_int,
) -> *mut crate::src::xmltok::ATTRIBUTE {
    atts.wrapping_add(index as usize)
}

fn write_attribute(dst: *mut crate::src::xmltok::ATTRIBUTE, value: crate::src::xmltok::ATTRIBUTE) {
    match encoding_data_lookup(
        ::core::ptr::null::<crate::src::xmltok::ENCODING>(),
        EncodingDataLookup::WriteAttribute { dst, value },
    ) {
        EncodingDataValue::Unit => {}
        _ => unreachable!(),
    }
}

fn ignore_section_tok(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    next_tok_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    invalid_char: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        usize,
    ) -> bool,
) -> ::core::ffi::c_int {
    let mut level = 0;
    if width > 1 {
        let mut n = byte_distance(ptr, end) as usize;
        if n & (width - 1) != 0 {
            n &= !(width - 1);
            end = ptr.wrapping_add(n);
        }
    }

    while byte_distance(ptr, end) >= width as ::core::ffi::c_long {
        let t = byte_type(enc, ptr);
        if let Some(lead_width) = lead_byte_width(t) {
            if byte_distance(ptr, end) < lead_width as ::core::ffi::c_long {
                return crate::src::xmltok::XML_TOK_PARTIAL_CHAR_1;
            }
            if invalid_char(enc, ptr, lead_width) {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.wrapping_add(lead_width);
            continue;
        }

        match t {
            t if t == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int =>
            {
                set_next_tok_ptr(next_tok_ptr, ptr);
                return crate::src::xmltok::XML_TOK_INVALID_1;
            }
            t if t == crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int => {
                ptr = ptr.wrapping_add(width);
                if byte_distance(ptr, end) < width as ::core::ffi::c_long {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                if ascii_byte(ptr) == crate::ascii_h::ASCII_EXCL {
                    ptr = ptr.wrapping_add(width);
                    if byte_distance(ptr, end) < width as ::core::ffi::c_long {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if ascii_byte(ptr) == crate::ascii_h::ASCII_LSQB {
                        level += 1;
                        ptr = ptr.wrapping_add(width);
                    }
                }
            }
            t if t == crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int => {
                ptr = ptr.wrapping_add(width);
                if byte_distance(ptr, end) < width as ::core::ffi::c_long {
                    return crate::src::xmltok::XML_TOK_PARTIAL_1;
                }
                if ascii_byte(ptr) == 0x5d {
                    ptr = ptr.wrapping_add(width);
                    if byte_distance(ptr, end) < width as ::core::ffi::c_long {
                        return crate::src::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if ascii_byte(ptr) == crate::ascii_h::ASCII_GT {
                        ptr = ptr.wrapping_add(width);
                        if level == 0 {
                            set_next_tok_ptr(next_tok_ptr, ptr);
                            return crate::src::xmltok::XML_TOK_IGNORE_SECT_1;
                        }
                        level -= 1;
                    }
                }
            }
            _ => {
                ptr = ptr.wrapping_add(width);
            }
        }
    }

    crate::src::xmltok::XML_TOK_PARTIAL_1
}

fn is_public_id(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    bad_ptr: *mut *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    ptr = ptr.wrapping_add(width);
    end = end.wrapping_sub(width);
    while byte_distance(ptr, end) >= width as ::core::ffi::c_long {
        let c = ascii_byte(ptr);
        match byte_type(enc, ptr) {
            t if public_id_type_is_allowed(t) => {}
            21 => {
                if c == 0x9 {
                    set_next_tok_ptr(bad_ptr, ptr);
                    return 0;
                }
            }
            26 | 22 => {
                if c & !0x7f != 0 && !matches!(c, 36 | 64) {
                    set_next_tok_ptr(bad_ptr, ptr);
                    return 0;
                }
            }
            _ if matches!(c, 36 | 64) => {}
            _ => {
                set_next_tok_ptr(bad_ptr, ptr);
                return 0;
            }
        }
        ptr = ptr.wrapping_add(width);
    }
    1
}

fn get_atts(
    enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    atts_max: ::core::ffi::c_int,
    atts: *mut crate::src::xmltok::ATTRIBUTE,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    byte_type: fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state = crate::xmltok_impl_c::inName;
    let mut n_atts: ::core::ffi::c_int = 0;
    let mut open: ::core::ffi::c_int = 0;
    let mut current_att = crate::src::xmltok::ATTRIBUTE {
        name: ::core::ptr::null(),
        valuePtr: ::core::ptr::null(),
        valueEnd: ::core::ptr::null(),
        normalized: 1,
    };
    ptr = ptr.wrapping_add(width);
    loop {
        match byte_type(enc, ptr) {
            t if matches!(
                t,
                t if t == crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int
                    || t == crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int
            ) =>
            {
                if state == crate::xmltok_impl_c::other {
                    if n_atts < atts_max {
                        current_att = crate::src::xmltok::ATTRIBUTE {
                            name: ptr,
                            valuePtr: ::core::ptr::null(),
                            valueEnd: ::core::ptr::null(),
                            normalized: 1,
                        };
                        write_attribute(current_attribute(atts, n_atts), current_att);
                    }
                    state = crate::xmltok_impl_c::inName;
                }
                if let Some(lead_width) = lead_byte_width(t) {
                    ptr = ptr.wrapping_add(lead_width.saturating_sub(width));
                }
            }
            t if t == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int =>
            {
                if state != crate::xmltok_impl_c::inValue {
                    if n_atts < atts_max {
                        current_att.valuePtr = ptr.wrapping_add(width);
                        write_attribute(current_attribute(atts, n_atts), current_att);
                    }
                    state = crate::xmltok_impl_c::inValue;
                    open = t;
                } else if open == t {
                    state = crate::xmltok_impl_c::other;
                    if n_atts < atts_max {
                        current_att.valueEnd = ptr;
                        write_attribute(current_attribute(atts, n_atts), current_att);
                    }
                    n_atts += 1;
                }
            }
            t if t == crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int => {
                if n_atts < atts_max {
                    current_att.normalized = 0;
                    write_attribute(current_attribute(atts, n_atts), current_att);
                }
            }
            t if t == crate::xmltok_impl_h::BT_S as ::core::ffi::c_int => {
                if state == crate::xmltok_impl_c::inName {
                    state = crate::xmltok_impl_c::other;
                } else if state == crate::xmltok_impl_c::inValue && n_atts < atts_max {
                    let next = ptr.wrapping_add(width);
                    if current_att.normalized != 0
                        && (ptr == current_att.valuePtr
                            || ascii_byte(ptr) != crate::ascii_h::ASCII_SPACE
                            || ascii_byte(next) == crate::ascii_h::ASCII_SPACE
                            || byte_type(enc, next) == open)
                    {
                        current_att.normalized = 0;
                        write_attribute(current_attribute(atts, n_atts), current_att);
                    }
                }
            }
            t if t == crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int =>
            {
                if state == crate::xmltok_impl_c::inName {
                    state = crate::xmltok_impl_c::other;
                } else if state == crate::xmltok_impl_c::inValue && n_atts < atts_max {
                    current_att.normalized = 0;
                    write_attribute(current_attribute(atts, n_atts), current_att);
                }
            }
            t if t == crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int
                || t == crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int =>
            {
                if state != crate::xmltok_impl_c::inValue {
                    return n_atts;
                }
            }
            _ => {}
        }
        ptr = ptr.wrapping_add(width);
    }
}

fn add_char_ref_hex_digit(result: &mut ::core::ffi::c_int, c: ::core::ffi::c_int) {
    match c {
        crate::ascii_h::ASCII_0
        | crate::ascii_h::ASCII_1_1
        | crate::ascii_h::ASCII_2_1
        | crate::ascii_h::ASCII_3_1
        | crate::ascii_h::ASCII_4
        | crate::ascii_h::ASCII_5
        | crate::ascii_h::ASCII_6
        | crate::ascii_h::ASCII_7
        | crate::ascii_h::ASCII_8_1
        | crate::ascii_h::ASCII_9_1 => {
            *result <<= 4;
            *result |= c - crate::ascii_h::ASCII_0;
        }
        crate::ascii_h::ASCII_A
        | crate::ascii_h::ASCII_B_1
        | crate::ascii_h::ASCII_C
        | crate::ascii_h::ASCII_D
        | crate::ascii_h::ASCII_E_1
        | crate::ascii_h::ASCII_F_1 => {
            *result <<= 4;
            *result += 10 + (c - crate::ascii_h::ASCII_A);
        }
        crate::ascii_h::ASCII_a_1
        | crate::ascii_h::ASCII_b
        | crate::ascii_h::ASCII_c_1
        | crate::ascii_h::ASCII_d
        | crate::ascii_h::ASCII_e_1
        | crate::ascii_h::ASCII_f => {
            *result <<= 4;
            *result += 10 + (c - crate::ascii_h::ASCII_a_1);
        }
        _ => {}
    }
}

fn char_ref_number(
    mut ptr: *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut result = 0 as ::core::ffi::c_int;
    ptr = ptr.wrapping_add(2 * width);
    if ascii_byte(ptr) == crate::ascii_h::ASCII_x_1 {
        ptr = ptr.wrapping_add(width);
        while ascii_byte(ptr) != 0x3b {
            add_char_ref_hex_digit(&mut result, ascii_byte(ptr));
            if result >= 0x110000 {
                return -1;
            }
            ptr = ptr.wrapping_add(width);
        }
    } else {
        while ascii_byte(ptr) != 0x3b {
            result *= 10;
            result += ascii_byte(ptr) - crate::ascii_h::ASCII_0;
            if result >= 0x110000 {
                return -1;
            }
            ptr = ptr.wrapping_add(width);
        }
    }
    checkCharRefNumber(result)
}

fn predefined_entity_name(
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    width: usize,
    ascii_byte: fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match byte_distance(ptr, end) / width as ::core::ffi::c_long {
        2 if ascii_byte(ptr.wrapping_add(width)) == crate::ascii_h::ASCII_t => {
            match ascii_byte(ptr) {
                crate::ascii_h::ASCII_l_1 => crate::ascii_h::ASCII_LT,
                crate::ascii_h::ASCII_g_1 => crate::ascii_h::ASCII_GT,
                _ => 0,
            }
        }
        3 if ascii_byte(ptr) == crate::ascii_h::ASCII_a_1
            && ascii_byte(ptr.wrapping_add(width)) == crate::ascii_h::ASCII_m_1
            && ascii_byte(ptr.wrapping_add(2 * width)) == crate::ascii_h::ASCII_p =>
        {
            crate::ascii_h::ASCII_AMP
        }
        4 if ascii_byte(ptr) == crate::ascii_h::ASCII_q
            && ascii_byte(ptr.wrapping_add(width)) == crate::ascii_h::ASCII_U + 0x20
            && ascii_byte(ptr.wrapping_add(2 * width)) == crate::ascii_h::ASCII_o
            && ascii_byte(ptr.wrapping_add(3 * width)) == crate::ascii_h::ASCII_t =>
        {
            crate::ascii_h::ASCII_QUOT
        }
        4 if ascii_byte(ptr) == crate::ascii_h::ASCII_a_1
            && ascii_byte(ptr.wrapping_add(width)) == crate::ascii_h::ASCII_p
            && ascii_byte(ptr.wrapping_add(2 * width)) == crate::ascii_h::ASCII_o
            && ascii_byte(ptr.wrapping_add(3 * width)) == crate::ascii_h::ASCII_s =>
        {
            crate::ascii_h::ASCII_APOS
        }
        _ => 0,
    }
}

fn byte_distance(
    from: *const ::core::ffi::c_char,
    to: *const ::core::ffi::c_char,
) -> ::core::ffi::c_long {
    (to as isize).wrapping_sub(from as isize) as ::core::ffi::c_long
}

fn normal_byte_type(
    enc: *const crate::src::xmltok::ENCODING,
    byte: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match encoding_data_lookup(enc, EncodingDataLookup::NormalByteType(byte)) {
        EncodingDataValue::Int(value) => value,
        _ => unreachable!(),
    }
}

fn convert_raw_from_bytes<T, F>(
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut T,
    toLim: *const T,
    convert: F,
) -> crate::src::xmltok::XML_Convert_Result
where
    F: FnOnce(
        &[::core::ffi::c_uchar],
        &mut [T],
    ) -> (
        crate::__stddef_size_t_h::size_t,
        crate::__stddef_size_t_h::size_t,
        crate::src::xmltok::XML_Convert_Result,
    ),
{
    unsafe {
        let from = *fromP as *const ::core::ffi::c_uchar;
        let to = *toP;
        let input_len = (fromLim as usize).wrapping_sub(from as usize);
        let output_len = (toLim as usize).wrapping_sub(to as usize) / ::core::mem::size_of::<T>();
        let input = if input_len == 0 {
            &[]
        } else {
            ::core::slice::from_raw_parts(from, input_len)
        };
        let output: &mut [T] = if output_len == 0 {
            &mut []
        } else {
            ::core::slice::from_raw_parts_mut(to, output_len)
        };
        let (input_consumed, output_written, result) = convert(input, output);
        *fromP = from.wrapping_add(input_consumed) as *const ::core::ffi::c_char;
        *toP = to.wrapping_add(output_written);
        result
    }
}

extern "C" fn utf8_isName2(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let b0 = utf8_byte(p, 0);
    let b1 = utf8_byte(p, 1);
    let bitmap_index = ((namePages[((b0 >> 2) & 7) as usize] as ::core::ffi::c_int) << 3)
        + ((b0 & 3) << 1)
        + ((b1 >> 5) & 1);
    (namingBitmap[bitmap_index as usize] & (1 as ::core::ffi::c_uint) << (b1 & 0x1f))
        as ::core::ffi::c_int
}

extern "C" fn utf8_isName3(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let b0 = utf8_byte(p, 0);
    let b1 = utf8_byte(p, 1);
    let b2 = utf8_byte(p, 2);
    let page_index = ((b0 & 0xf) << 4) + ((b1 >> 2) & 0xf);
    let bitmap_index = ((namePages[page_index as usize] as ::core::ffi::c_int) << 3)
        + ((b1 & 3) << 1)
        + ((b2 >> 5) & 1);
    (namingBitmap[bitmap_index as usize] & (1 as ::core::ffi::c_uint) << (b2 & 0x1f))
        as ::core::ffi::c_int
}

extern "C" fn utf8_isNmstrt2(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let b0 = utf8_byte(p, 0);
    let b1 = utf8_byte(p, 1);
    let bitmap_index = ((nmstrtPages[((b0 >> 2) & 7) as usize] as ::core::ffi::c_int) << 3)
        + ((b0 & 3) << 1)
        + ((b1 >> 5) & 1);
    (namingBitmap[bitmap_index as usize] & (1 as ::core::ffi::c_uint) << (b1 & 0x1f))
        as ::core::ffi::c_int
}

extern "C" fn utf8_isNmstrt3(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let b0 = utf8_byte(p, 0);
    let b1 = utf8_byte(p, 1);
    let b2 = utf8_byte(p, 2);
    let page_index = ((b0 & 0xf) << 4) + ((b1 >> 2) & 0xf);
    let bitmap_index = ((nmstrtPages[page_index as usize] as ::core::ffi::c_int) << 3)
        + ((b1 & 3) << 1)
        + ((b2 >> 5) & 1);
    (namingBitmap[bitmap_index as usize] & (1 as ::core::ffi::c_uint) << (b2 & 0x1f))
        as ::core::ffi::c_int
}

extern "C" fn utf8_isInvalid2(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let b0 = utf8_byte(p, 0);
    let b1 = utf8_byte(p, 1);
    (b0 < 0xc2 || b1 & 0x80 == 0 || b1 & 0xc0 == 0xc0) as ::core::ffi::c_int
}

extern "C" fn utf8_isInvalid3(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let b0 = utf8_byte(p, 0);
    let b1 = utf8_byte(p, 1);
    let b2 = utf8_byte(p, 2);
    let invalid_third = b2 & 0x80 == 0
        || if b0 == 0xef && b1 == 0xbf {
            b2 > 0xbd
        } else {
            b2 & 0xc0 == 0xc0
        };
    let invalid_second = if b0 == 0xe0 {
        b1 < 0xa0 || b1 & 0xc0 == 0xc0
    } else {
        b1 & 0x80 == 0
            || if b0 == 0xed {
                b1 > 0x9f
            } else {
                b1 & 0xc0 == 0xc0
            }
    };
    (invalid_third || invalid_second) as ::core::ffi::c_int
}

extern "C" fn utf8_isInvalid4(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let b0 = utf8_byte(p, 0);
    let b1 = utf8_byte(p, 1);
    let b2 = utf8_byte(p, 2);
    let b3 = utf8_byte(p, 3);
    let invalid_fourth = b3 & 0x80 == 0 || b3 & 0xc0 == 0xc0;
    let invalid_third = b2 & 0x80 == 0 || b2 & 0xc0 == 0xc0;
    let invalid_second = if b0 == 0xf0 {
        b1 < 0x90 || b1 & 0xc0 == 0xc0
    } else {
        b1 & 0x80 == 0
            || if b0 == 0xf4 {
                b1 > 0x8f
            } else {
                b1 & 0xc0 == 0xc0
            }
    };
    (invalid_fourth || invalid_third || invalid_second) as ::core::ffi::c_int
}
pub fn _INTERNAL_trim_to_complete_utf8_characters(
    from: &[::core::ffi::c_uchar],
) -> crate::__stddef_size_t_h::size_t {
    let mut fromLim: crate::__stddef_size_t_h::size_t = from.len();
    let mut walked: crate::__stddef_size_t_h::size_t = 0 as crate::__stddef_size_t_h::size_t;
    while fromLim > 0 {
        let prev: ::core::ffi::c_uchar = from[fromLim - 1];
        if prev as ::core::ffi::c_uint & 0xf8 as ::core::ffi::c_uint == 0xf0 as ::core::ffi::c_uint
        {
            if walked.wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                >= 4 as crate::__stddef_size_t_h::size_t
            {
                fromLim = fromLim.wrapping_add(
                    (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as crate::__stddef_size_t_h::size_t,
                );
                break;
            } else {
                walked = 0 as crate::__stddef_size_t_h::size_t;
            }
        } else if prev as ::core::ffi::c_uint & 0xf0 as ::core::ffi::c_uint
            == 0xe0 as ::core::ffi::c_uint
        {
            if walked.wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                >= 3 as crate::__stddef_size_t_h::size_t
            {
                fromLim = fromLim.wrapping_add(
                    (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as crate::__stddef_size_t_h::size_t,
                );
                break;
            } else {
                walked = 0 as crate::__stddef_size_t_h::size_t;
            }
        } else if prev as ::core::ffi::c_uint & 0xe0 as ::core::ffi::c_uint
            == 0xc0 as ::core::ffi::c_uint
        {
            if walked.wrapping_add(1 as crate::__stddef_size_t_h::size_t)
                >= 2 as crate::__stddef_size_t_h::size_t
            {
                fromLim = fromLim.wrapping_add(
                    (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as crate::__stddef_size_t_h::size_t,
                );
                break;
            } else {
                walked = 0 as crate::__stddef_size_t_h::size_t;
            }
        } else if prev as ::core::ffi::c_uint & 0x80 as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint
        {
            break;
        }
        fromLim = fromLim.wrapping_sub(1);
        walked = walked.wrapping_add(1);
    }
    fromLim
}
#[export_name = "_INTERNAL_trim_to_complete_utf8_characters"]

pub unsafe extern "C" fn _INTERNAL_trim_to_complete_utf8_characters_ffi(
    mut from: *const ::core::ffi::c_char,
    mut fromLimRef: *mut *const ::core::ffi::c_char,
) {
    let fromLim = *fromLimRef;
    if fromLim <= from {
        return;
    }
    let bytes = ::core::slice::from_raw_parts(
        from as *const ::core::ffi::c_uchar,
        fromLim.offset_from(from) as crate::__stddef_size_t_h::size_t,
    );
    *fromLimRef = from.add(_INTERNAL_trim_to_complete_utf8_characters(bytes));
}
extern "C" fn utf8_toUtf8(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        let output_exhausted = input.len() > output.len();
        let available = input.len().min(output.len());
        let bytes_to_copy = _INTERNAL_trim_to_complete_utf8_characters(&input[..available]);
        for (dst, src) in output[..bytes_to_copy]
            .iter_mut()
            .zip(input[..bytes_to_copy].iter())
        {
            *dst = *src as ::core::ffi::c_char;
        }
        let input_incomplete = bytes_to_copy < available;
        let result = if output_exhausted {
            crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED
        } else if input_incomplete {
            crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE
        } else {
            crate::src::xmltok::XML_CONVERT_COMPLETED
        };
        (bytes_to_copy, bytes_to_copy, result)
    })
}

extern "C" fn utf8_toUtf16(
    mut enc: *const crate::src::xmltok::ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_ushort,
    toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        let mut input_consumed = 0;
        let mut output_written = 0;
        let mut result = crate::src::xmltok::XML_CONVERT_COMPLETED;
        while input_consumed < input.len() && output_written < output.len() {
            match normal_byte_type(enc, input[input_consumed] as ::core::ffi::c_int) {
                5 => {
                    if input.len() - input_consumed < 2 {
                        result = crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
                        break;
                    }
                    output[output_written] = ((input[input_consumed] as ::core::ffi::c_int
                        & 0x1f as ::core::ffi::c_int)
                        << 6 as ::core::ffi::c_int
                        | input[input_consumed + 1] as ::core::ffi::c_int
                            & 0x3f as ::core::ffi::c_int)
                        as ::core::ffi::c_ushort;
                    input_consumed += 2;
                    output_written += 1;
                }
                6 => {
                    if input.len() - input_consumed < 3 {
                        result = crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
                        break;
                    }
                    output[output_written] = ((input[input_consumed] as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int)
                        << 12 as ::core::ffi::c_int
                        | (input[input_consumed + 1] as ::core::ffi::c_int
                            & 0x3f as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int
                        | input[input_consumed + 2] as ::core::ffi::c_int
                            & 0x3f as ::core::ffi::c_int)
                        as ::core::ffi::c_ushort;
                    input_consumed += 3;
                    output_written += 1;
                }
                7 => {
                    if output.len() - output_written < 2 {
                        result = crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                        break;
                    }
                    if input.len() - input_consumed < 4 {
                        result = crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
                        break;
                    }
                    let mut n = ((input[input_consumed] as ::core::ffi::c_int
                        & 0x7 as ::core::ffi::c_int)
                        << 18 as ::core::ffi::c_int
                        | (input[input_consumed + 1] as ::core::ffi::c_int
                            & 0x3f as ::core::ffi::c_int)
                            << 12 as ::core::ffi::c_int
                        | (input[input_consumed + 2] as ::core::ffi::c_int
                            & 0x3f as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int
                        | input[input_consumed + 3] as ::core::ffi::c_int
                            & 0x3f as ::core::ffi::c_int)
                        as ::core::ffi::c_ulong;
                    n = n.wrapping_sub(0x10000 as ::core::ffi::c_ulong);
                    output[output_written] = (n >> 10 as ::core::ffi::c_int
                        | 0xd800 as ::core::ffi::c_ulong)
                        as ::core::ffi::c_ushort;
                    output[output_written + 1] = (n & 0x3ff as ::core::ffi::c_ulong
                        | 0xdc00 as ::core::ffi::c_ulong)
                        as ::core::ffi::c_ushort;
                    input_consumed += 4;
                    output_written += 2;
                }
                _ => {
                    output[output_written] =
                        input[input_consumed] as ::core::ffi::c_char as ::core::ffi::c_ushort;
                    input_consumed += 1;
                    output_written += 1;
                }
            }
        }
        if result == crate::src::xmltok::XML_CONVERT_COMPLETED && input_consumed < input.len() {
            result = crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
        }
        (input_consumed, output_written, result)
    })
}

static utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                normal_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                normal_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            normal_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            normal_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            normal_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            normal_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            normal_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            normal_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            normal_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            normal_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            utf8_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            utf8_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Some(utf8_isName2 as NORMAL_ENCODING_CHAR_CHECK),
    isName3: Some(utf8_isName3 as NORMAL_ENCODING_CHAR_CHECK),
    isName4: Some(isNever as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt2: Some(utf8_isNmstrt2 as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt3: Some(utf8_isNmstrt3 as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt4: Some(isNever as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid2: Some(utf8_isInvalid2 as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid3: Some(utf8_isInvalid3 as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid4: Some(utf8_isInvalid4 as NORMAL_ENCODING_CHAR_CHECK),
};

static utf8_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                normal_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                normal_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            normal_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            normal_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            normal_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            normal_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            normal_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            normal_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            normal_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            normal_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            utf8_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            utf8_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Some(utf8_isName2 as NORMAL_ENCODING_CHAR_CHECK),
    isName3: Some(utf8_isName3 as NORMAL_ENCODING_CHAR_CHECK),
    isName4: Some(isNever as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt2: Some(utf8_isNmstrt2 as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt3: Some(utf8_isNmstrt3 as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt4: Some(isNever as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid2: Some(utf8_isInvalid2 as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid3: Some(utf8_isInvalid3 as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid4: Some(utf8_isInvalid4 as NORMAL_ENCODING_CHAR_CHECK),
};

static internal_utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                normal_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                normal_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            normal_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            normal_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            normal_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            normal_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            normal_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            normal_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            normal_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            normal_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            utf8_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            utf8_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Some(utf8_isName2 as NORMAL_ENCODING_CHAR_CHECK),
    isName3: Some(utf8_isName3 as NORMAL_ENCODING_CHAR_CHECK),
    isName4: Some(isNever as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt2: Some(utf8_isNmstrt2 as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt3: Some(utf8_isNmstrt3 as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt4: Some(isNever as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid2: Some(utf8_isInvalid2 as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid3: Some(utf8_isInvalid3 as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid4: Some(utf8_isInvalid4 as NORMAL_ENCODING_CHAR_CHECK),
};

static internal_utf8_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                normal_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                normal_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            normal_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            normal_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            normal_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            normal_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            normal_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            normal_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            normal_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            normal_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            utf8_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            utf8_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD3 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: Some(utf8_isName2 as NORMAL_ENCODING_CHAR_CHECK),
    isName3: Some(utf8_isName3 as NORMAL_ENCODING_CHAR_CHECK),
    isName4: Some(isNever as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt2: Some(utf8_isNmstrt2 as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt3: Some(utf8_isNmstrt3 as NORMAL_ENCODING_CHAR_CHECK),
    isNmstrt4: Some(isNever as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid2: Some(utf8_isInvalid2 as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid3: Some(utf8_isInvalid3 as NORMAL_ENCODING_CHAR_CHECK),
    isInvalid4: Some(utf8_isInvalid4 as NORMAL_ENCODING_CHAR_CHECK),
};

extern "C" fn latin1_toUtf8(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        let mut input_consumed = 0;
        let mut output_written = 0;
        for &c in input {
            if c as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
                if output.len().saturating_sub(output_written) < 2 {
                    return (
                        input_consumed,
                        output_written,
                        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                    );
                }
                output[output_written] = (c as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                    | UTF8_cval2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                output[output_written + 1] = (c as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
                    | 0x80 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                output_written += 2;
            } else {
                if output_written == output.len() {
                    return (
                        input_consumed,
                        output_written,
                        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                    );
                }
                output[output_written] = c as ::core::ffi::c_char;
                output_written += 1;
            }
            input_consumed += 1;
        }
        (
            input_consumed,
            output_written,
            crate::src::xmltok::XML_CONVERT_COMPLETED,
        )
    })
}

extern "C" fn latin1_toUtf16(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        let chars_to_copy = input.len().min(output.len());
        for (dst, src) in output[..chars_to_copy]
            .iter_mut()
            .zip(input[..chars_to_copy].iter())
        {
            *dst = *src as ::core::ffi::c_ushort;
        }
        let result = if chars_to_copy < input.len() {
            crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED
        } else {
            crate::src::xmltok::XML_CONVERT_COMPLETED
        };
        (chars_to_copy, chars_to_copy, result)
    })
}

static latin1_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                normal_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                normal_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            normal_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            normal_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            normal_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            normal_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            normal_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            normal_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            normal_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            normal_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            latin1_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            latin1_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

static latin1_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                normal_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                normal_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            normal_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            normal_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            normal_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            normal_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            normal_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            normal_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            normal_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            normal_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            latin1_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            latin1_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

extern "C" fn ascii_toUtf8(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        let bytes_to_copy = input.len().min(output.len());
        for (dst, src) in output[..bytes_to_copy]
            .iter_mut()
            .zip(input[..bytes_to_copy].iter())
        {
            *dst = *src as ::core::ffi::c_char;
        }
        let result = if bytes_to_copy < input.len() {
            crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED
        } else {
            crate::src::xmltok::XML_CONVERT_COMPLETED
        };
        (bytes_to_copy, bytes_to_copy, result)
    })
}

static ascii_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                normal_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                normal_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            normal_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            normal_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            normal_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            normal_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            normal_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            normal_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            normal_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            normal_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            ascii_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            latin1_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

static ascii_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                normal_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                normal_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                normal_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            normal_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            normal_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            normal_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            normal_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            normal_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            normal_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            normal_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            normal_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            ascii_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            latin1_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 1 as ::core::ffi::c_int,
        isUtf8: 1 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

extern "C" fn unicode_byte_type(
    mut hi: ::core::ffi::c_char,
    mut lo: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match hi as ::core::ffi::c_uchar as ::core::ffi::c_int {
        216 | 217 | 218 | 219 => return crate::xmltok_impl_h::BT_LEAD4 as ::core::ffi::c_int,
        220 | 221 | 222 | 223 => return crate::xmltok_impl_h::BT_TRAIL as ::core::ffi::c_int,
        255 => match lo as ::core::ffi::c_uchar as ::core::ffi::c_int {
            255 | 254 => return crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int,
            _ => {}
        },
        _ => {}
    }
    return crate::xmltok_impl_h::BT_NONASCII as ::core::ffi::c_int;
}

fn utf16_to_utf8(
    input: &[::core::ffi::c_uchar],
    output: &mut [::core::ffi::c_char],
    little_endian: bool,
) -> (
    crate::__stddef_size_t_h::size_t,
    crate::__stddef_size_t_h::size_t,
    crate::src::xmltok::XML_Convert_Result,
) {
    let input_end = input.len() & !1;
    let mut input_pos = 0;
    let mut output_pos = 0;

    while input_pos < input_end {
        let (hi, lo) = if little_endian {
            (input[input_pos + 1], input[input_pos])
        } else {
            (input[input_pos], input[input_pos + 1])
        };

        match hi as ::core::ffi::c_int {
            0 if (lo as ::core::ffi::c_int) < 0x80 => {
                if output_pos == output.len() {
                    return (
                        input_pos,
                        output_pos,
                        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                    );
                }
                output[output_pos] = lo as ::core::ffi::c_char;
                output_pos += 1;
                input_pos += 2;
            }
            0..=7 => {
                if output.len() - output_pos < 2 {
                    return (
                        input_pos,
                        output_pos,
                        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                    );
                }
                output[output_pos] = (lo as ::core::ffi::c_int >> 6
                    | (hi as ::core::ffi::c_int) << 2
                    | UTF8_cval2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                output[output_pos + 1] =
                    (lo as ::core::ffi::c_int & 0x3f | 0x80) as ::core::ffi::c_char;
                output_pos += 2;
                input_pos += 2;
            }
            216..=219 => {
                if output.len() - output_pos < 4 {
                    return (
                        input_pos,
                        output_pos,
                        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                    );
                }
                if input_end - input_pos < 4 {
                    return (
                        input_pos,
                        output_pos,
                        crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE,
                    );
                }
                let (hi2, lo2) = if little_endian {
                    (input[input_pos + 3], input[input_pos + 2])
                } else {
                    (input[input_pos + 2], input[input_pos + 3])
                };
                let plane = (((hi as ::core::ffi::c_int & 0x3) << 2)
                    | ((lo as ::core::ffi::c_int >> 6) & 0x3))
                    + 1;
                output[output_pos] =
                    (plane >> 2 | UTF8_cval4 as ::core::ffi::c_int) as ::core::ffi::c_char;
                output[output_pos + 1] = ((lo as ::core::ffi::c_int >> 2 & 0xf)
                    | (plane & 0x3) << 4
                    | 0x80) as ::core::ffi::c_char;
                output[output_pos + 2] = ((lo as ::core::ffi::c_int & 0x3) << 4
                    | (hi2 as ::core::ffi::c_int & 0x3) << 2
                    | lo2 as ::core::ffi::c_int >> 6
                    | 0x80) as ::core::ffi::c_char;
                output[output_pos + 3] =
                    (lo2 as ::core::ffi::c_int & 0x3f | 0x80) as ::core::ffi::c_char;
                output_pos += 4;
                input_pos += 4;
            }
            _ => {
                if output.len() - output_pos < 3 {
                    return (
                        input_pos,
                        output_pos,
                        crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                    );
                }
                output[output_pos] = (hi as ::core::ffi::c_int >> 4
                    | UTF8_cval3 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                output[output_pos + 1] = ((hi as ::core::ffi::c_int & 0xf) << 2
                    | lo as ::core::ffi::c_int >> 6
                    | 0x80) as ::core::ffi::c_char;
                output[output_pos + 2] =
                    (lo as ::core::ffi::c_int & 0x3f | 0x80) as ::core::ffi::c_char;
                output_pos += 3;
                input_pos += 2;
            }
        }
    }

    (
        input_pos,
        output_pos,
        crate::src::xmltok::XML_CONVERT_COMPLETED,
    )
}

extern "C" fn little2_toUtf8(
    mut enc: *const crate::src::xmltok::ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        utf16_to_utf8(input, output, true)
    })
}

extern "C" fn little2_toUtf16(
    mut enc: *const crate::src::xmltok::ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_ushort,
    toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        let mut input_end = input.len() & !1;
        let output_bytes = output.len().saturating_mul(2);
        let mut result = crate::src::xmltok::XML_CONVERT_COMPLETED;
        if input_end > output_bytes
            && input_end >= 2
            && input[input_end - 1] as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                == 0xd8 as ::core::ffi::c_int
        {
            input_end -= 2;
            result = crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
        }

        let chars_to_copy = (input_end / 2).min(output.len());
        for (dst, bytes) in output[..chars_to_copy]
            .iter_mut()
            .zip(input[..chars_to_copy * 2].chunks_exact(2))
        {
            *dst = ((bytes[1] as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | bytes[0] as ::core::ffi::c_int) as ::core::ffi::c_ushort;
        }

        let input_consumed = chars_to_copy * 2;
        if input_consumed < input_end {
            result = crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
        }
        (input_consumed, chars_to_copy, result)
    })
}

extern "C" fn big2_toUtf8(
    mut enc: *const crate::src::xmltok::ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_char,
    toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        utf16_to_utf8(input, output, false)
    })
}

extern "C" fn big2_toUtf16(
    mut enc: *const crate::src::xmltok::ENCODING,
    fromP: *mut *const ::core::ffi::c_char,
    fromLim: *const ::core::ffi::c_char,
    toP: *mut *mut ::core::ffi::c_ushort,
    toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        let mut input_end = input.len() & !1;
        let output_bytes = output.len().saturating_mul(2);
        let mut result = crate::src::xmltok::XML_CONVERT_COMPLETED;
        if input_end > output_bytes
            && input_end >= 2
            && input[input_end - 2] as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                == 0xd8 as ::core::ffi::c_int
        {
            input_end -= 2;
            result = crate::src::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
        }

        let chars_to_copy = (input_end / 2).min(output.len());
        for (dst, bytes) in output[..chars_to_copy]
            .iter_mut()
            .zip(input[..chars_to_copy * 2].chunks_exact(2))
        {
            *dst = ((bytes[0] as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | bytes[1] as ::core::ffi::c_int) as ::core::ffi::c_ushort;
        }

        let input_consumed = chars_to_copy * 2;
        if input_consumed < input_end {
            result = crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
        }
        (input_consumed, chars_to_copy, result)
    })
}

static little2_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                little2_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                little2_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            little2_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            little2_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            little2_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            little2_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            little2_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            little2_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            little2_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            little2_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            little2_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            little2_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 1 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

static little2_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                little2_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                little2_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            little2_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            little2_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            little2_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            little2_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            little2_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            little2_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            little2_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            little2_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            little2_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            little2_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 1 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

static internal_little2_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                little2_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                little2_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            little2_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            little2_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            little2_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            little2_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            little2_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            little2_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            little2_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            little2_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            little2_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            little2_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 1 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

static internal_little2_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                little2_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                little2_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                little2_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            little2_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            little2_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            little2_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            little2_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            little2_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            little2_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            little2_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            little2_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            little2_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            little2_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 1 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

static big2_encoding_ns: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                big2_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                big2_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                big2_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                big2_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                big2_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                big2_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            big2_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            big2_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            big2_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            big2_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            big2_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            big2_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            big2_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            big2_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            big2_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            big2_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

static big2_encoding: normal_encoding = normal_encoding {
    enc: crate::src::xmltok::encoding {
        scanners: [
            Some(
                big2_prologTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                big2_contentTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                big2_cdataSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                big2_ignoreSectionTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        literalScanners: [
            Some(
                big2_attributeValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                big2_entityValueTok
                    as unsafe extern "C" fn(
                        *const crate::src::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
        ],
        nameMatchesAscii: Some(
            big2_nameMatchesAscii
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        nameLength: Some(
            big2_nameLength
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        skipS: Some(
            big2_skipS
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> *const ::core::ffi::c_char,
        ),
        getAtts: Some(
            big2_getAtts
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                    *mut crate::src::xmltok::ATTRIBUTE,
                ) -> ::core::ffi::c_int,
        ),
        charRefNumber: Some(
            big2_charRefNumber
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        predefinedEntityName: Some(
            big2_predefinedEntityName
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        updatePosition: Some(
            big2_updatePosition
                as extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    &mut crate::src::xmltok::POSITION,
                ) -> (),
        ),
        isPublicId: Some(
            big2_isPublicId
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        utf8Convert: Some(
            big2_toUtf8
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        utf16Convert: Some(
            big2_toUtf16
                as unsafe extern "C" fn(
                    *const crate::src::xmltok::ENCODING,
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_ushort,
                    *const ::core::ffi::c_ushort,
                ) -> crate::src::xmltok::XML_Convert_Result,
        ),
        minBytesPerChar: 2 as ::core::ffi::c_int,
        isUtf8: 0 as ::core::ffi::c_char,
        isUtf16: 0 as ::core::ffi::c_char,
    },
    type_0: [
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_CR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_S as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EXCL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NUM as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PERCNT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AMP as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RPAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_AST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_PLUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_COMMA as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_MINUS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SOL as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_DIGIT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_SEMI as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_GT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_QUEST as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_LSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_RSQB as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_HEX as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_VERBAR as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
        crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ],
    isName2: None,
    isName3: None,
    isName4: None,
    isNmstrt2: None,
    isNmstrt3: None,
    isNmstrt4: None,
    isInvalid2: None,
    isInvalid3: None,
    isInvalid4: None,
};

fn streqci(
    s1: impl IntoIterator<Item = ::core::ffi::c_char>,
    s2: impl IntoIterator<Item = ::core::ffi::c_char>,
) -> ::core::ffi::c_int {
    let mut s1 = s1.into_iter();
    let mut s2 = s2.into_iter();
    loop {
        let Some(mut c1) = s1.next() else {
            return 0 as ::core::ffi::c_int;
        };
        let Some(mut c2) = s2.next() else {
            return 0 as ::core::ffi::c_int;
        };
        if crate::ascii_h::ASCII_a_1 <= c1 as ::core::ffi::c_int
            && c1 as ::core::ffi::c_int <= crate::ascii_h::ASCII_z
        {
            c1 = (c1 as ::core::ffi::c_int + (crate::ascii_h::ASCII_A - crate::ascii_h::ASCII_a_1))
                as ::core::ffi::c_char;
        }
        if crate::ascii_h::ASCII_a_1 <= c2 as ::core::ffi::c_int
            && c2 as ::core::ffi::c_int <= crate::ascii_h::ASCII_z
        {
            c2 = (c2 as ::core::ffi::c_int + (crate::ascii_h::ASCII_A - crate::ascii_h::ASCII_a_1))
                as ::core::ffi::c_char;
        }
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if c1 == 0 {
            break;
        }
    }
    return 1 as ::core::ffi::c_int;
}

extern "C" fn initUpdatePosition(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    pos: &mut crate::src::xmltok::POSITION,
) {
    normal_updatePosition(&raw const utf8_encoding.enc, ptr, end, pos);
}

fn xml_decl_encoding_action(
    enc: *const crate::src::xmltok::ENCODING,
    action: XmlDeclEncodingAction<'_>,
) -> XmlDeclEncodingResult {
    match encoding_data_lookup(enc, EncodingDataLookup::XmlDecl(action)) {
        EncodingDataValue::XmlDecl(result) => result,
        _ => unreachable!(),
    }
}

pub(crate) fn encoding_min_bytes(enc: *const crate::src::xmltok::ENCODING) -> ::core::ffi::c_int {
    match xml_decl_encoding_action(enc, XmlDeclEncodingAction::MinBytes) {
        XmlDeclEncodingResult::Int(value) => value,
        XmlDeclEncodingResult::Encoding(_) => unreachable!(),
        XmlDeclEncodingResult::EncodingName(_) => unreachable!(),
    }
}

fn toAscii(
    enc: *const crate::src::xmltok::ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match xml_decl_encoding_action(enc, XmlDeclEncodingAction::ToAscii { ptr, end }) {
        XmlDeclEncodingResult::Int(value) => value,
        XmlDeclEncodingResult::Encoding(_) => unreachable!(),
        XmlDeclEncodingResult::EncodingName(_) => unreachable!(),
    }
}

pub(crate) fn encoding_name_matches_ascii(
    enc: *const crate::src::xmltok::ENCODING,
    name: *const ::core::ffi::c_char,
    name_end: *const ::core::ffi::c_char,
    ascii: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match xml_decl_encoding_action(
        enc,
        XmlDeclEncodingAction::NameMatchesAscii {
            name,
            name_end,
            ascii,
        },
    ) {
        XmlDeclEncodingResult::Int(value) => value,
        XmlDeclEncodingResult::Encoding(_) => unreachable!(),
        XmlDeclEncodingResult::EncodingName(_) => unreachable!(),
    }
}

pub(crate) fn encoding_update_position(
    enc: *const crate::src::xmltok::ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
    pos: &mut crate::src::xmltok::POSITION,
) {
    match xml_decl_encoding_action(enc, XmlDeclEncodingAction::UpdatePosition { ptr, end, pos }) {
        XmlDeclEncodingResult::Int(_) => {}
        XmlDeclEncodingResult::Encoding(_) => unreachable!(),
        XmlDeclEncodingResult::EncodingName(_) => unreachable!(),
    }
}

fn xml_decl_find_encoding(
    enc: *const crate::src::xmltok::ENCODING,
    finder: unsafe extern "C" fn(
        *const crate::src::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> *const crate::src::xmltok::ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> *const crate::src::xmltok::ENCODING {
    match xml_decl_encoding_action(
        enc,
        XmlDeclEncodingAction::FindEncoding { finder, ptr, end },
    ) {
        XmlDeclEncodingResult::Encoding(encoding) => encoding,
        XmlDeclEncodingResult::Int(_) => unreachable!(),
        XmlDeclEncodingResult::EncodingName(_) => unreachable!(),
    }
}

fn xml_decl_convert_to_utf8_name(
    enc: *const crate::src::xmltok::ENCODING,
    ptr: *const ::core::ffi::c_char,
    end: *const ::core::ffi::c_char,
) -> Option<[::core::ffi::c_char; 128]> {
    match xml_decl_encoding_action(enc, XmlDeclEncodingAction::ConvertToUtf8Name { ptr, end }) {
        XmlDeclEncodingResult::EncodingName(name) => name,
        XmlDeclEncodingResult::Int(_) | XmlDeclEncodingResult::Encoding(_) => unreachable!(),
    }
}

fn find_encoding_from_converted_name(
    enc: *const crate::src::xmltok::ENCODING,
    name: &[::core::ffi::c_char; 128],
    encodings: &[*const crate::src::xmltok::ENCODING; 7],
) -> *const crate::src::xmltok::ENCODING {
    if streqci(name.iter().copied(), KW_UTF_16.iter().copied()) != 0
        && encoding_min_bytes(enc) == 2 as ::core::ffi::c_int
    {
        return enc;
    }
    let i = getEncodingIndex(Some(name));
    if i == UNKNOWN_ENC as ::core::ffi::c_int {
        return ::core::ptr::null::<crate::src::xmltok::ENCODING>();
    }
    encodings[i as usize]
}

extern "C" fn isSpace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match c {
        32 | 13 | 10 | 9 => return 1 as ::core::ffi::c_int,
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}

fn parsePseudoAttribute(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    namePtr: &mut *const ::core::ffi::c_char,
    nameEndPtr: &mut *const ::core::ffi::c_char,
    valPtr: &mut *const ::core::ffi::c_char,
    nextTokPtr: &mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut open: ::core::ffi::c_char = 0;
    let min_bytes = encoding_min_bytes(enc) as isize;
    if ptr == end {
        *namePtr = ::core::ptr::null::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    if isSpace(toAscii(enc, ptr, end)) == 0 {
        *nextTokPtr = ptr;
        return 0 as ::core::ffi::c_int;
    }
    loop {
        ptr = ptr.wrapping_offset(min_bytes);
        if !(isSpace(toAscii(enc, ptr, end)) != 0) {
            break;
        }
    }
    if ptr == end {
        *namePtr = ::core::ptr::null::<::core::ffi::c_char>();
        return 1 as ::core::ffi::c_int;
    }
    *namePtr = ptr;
    loop {
        c = toAscii(enc, ptr, end);
        if c == -1 as ::core::ffi::c_int {
            *nextTokPtr = ptr;
            return 0 as ::core::ffi::c_int;
        }
        if c == crate::ascii_h::ASCII_EQUALS {
            *nameEndPtr = ptr;
            break;
        } else if isSpace(c) != 0 {
            *nameEndPtr = ptr;
            loop {
                ptr = ptr.wrapping_offset(min_bytes);
                c = toAscii(enc, ptr, end);
                if !(isSpace(c) != 0) {
                    break;
                }
            }
            if c != crate::ascii_h::ASCII_EQUALS {
                *nextTokPtr = ptr;
                return 0 as ::core::ffi::c_int;
            }
            break;
        } else {
            ptr = ptr.wrapping_offset(min_bytes);
        }
    }
    if ptr == *namePtr {
        *nextTokPtr = ptr;
        return 0 as ::core::ffi::c_int;
    }
    ptr = ptr.wrapping_offset(min_bytes);
    c = toAscii(enc, ptr, end);
    while isSpace(c) != 0 {
        ptr = ptr.wrapping_offset(min_bytes);
        c = toAscii(enc, ptr, end);
    }
    if c != crate::ascii_h::ASCII_QUOT && c != crate::ascii_h::ASCII_APOS {
        *nextTokPtr = ptr;
        return 0 as ::core::ffi::c_int;
    }
    open = c as ::core::ffi::c_char;
    ptr = ptr.wrapping_offset(min_bytes);
    *valPtr = ptr;
    loop {
        c = toAscii(enc, ptr, end);
        if c == open as ::core::ffi::c_int {
            break;
        }
        if !(crate::ascii_h::ASCII_a_1 <= c && c <= crate::ascii_h::ASCII_z)
            && !(crate::ascii_h::ASCII_A <= c && c <= crate::ascii_h::ASCII_Z)
            && !(crate::ascii_h::ASCII_0 <= c && c <= crate::ascii_h::ASCII_9_1)
            && c != crate::ascii_h::ASCII_PERIOD
            && c != crate::ascii_h::ASCII_MINUS
            && c != crate::ascii_h::ASCII_UNDERSCORE
        {
            *nextTokPtr = ptr;
            return 0 as ::core::ffi::c_int;
        }
        ptr = ptr.wrapping_offset(min_bytes);
    }
    *nextTokPtr = ptr.wrapping_offset(min_bytes);
    return 1 as ::core::ffi::c_int;
}

static KW_version: [::core::ffi::c_char; 8] = [
    crate::ascii_h::ASCII_v as ::core::ffi::c_char,
    crate::ascii_h::ASCII_e_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_r as ::core::ffi::c_char,
    crate::ascii_h::ASCII_s as ::core::ffi::c_char,
    crate::ascii_h::ASCII_i as ::core::ffi::c_char,
    crate::ascii_h::ASCII_o as ::core::ffi::c_char,
    crate::ascii_h::ASCII_n as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

static KW_encoding: [::core::ffi::c_char; 9] = [
    crate::ascii_h::ASCII_e_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_n as ::core::ffi::c_char,
    crate::ascii_h::ASCII_c_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_o as ::core::ffi::c_char,
    crate::ascii_h::ASCII_d as ::core::ffi::c_char,
    crate::ascii_h::ASCII_i as ::core::ffi::c_char,
    crate::ascii_h::ASCII_n as ::core::ffi::c_char,
    crate::ascii_h::ASCII_g_1 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

static KW_standalone: [::core::ffi::c_char; 11] = [
    crate::ascii_h::ASCII_s as ::core::ffi::c_char,
    crate::ascii_h::ASCII_t as ::core::ffi::c_char,
    crate::ascii_h::ASCII_a_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_n as ::core::ffi::c_char,
    crate::ascii_h::ASCII_d as ::core::ffi::c_char,
    crate::ascii_h::ASCII_a_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_l_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_o as ::core::ffi::c_char,
    crate::ascii_h::ASCII_n as ::core::ffi::c_char,
    crate::ascii_h::ASCII_e_1 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

static KW_yes: [::core::ffi::c_char; 4] = [
    crate::ascii_h::ASCII_y as ::core::ffi::c_char,
    crate::ascii_h::ASCII_e_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_s as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

static KW_no: [::core::ffi::c_char; 3] = [
    crate::ascii_h::ASCII_n as ::core::ffi::c_char,
    crate::ascii_h::ASCII_o as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

fn doParseXmlDecl(
    mut encodingFinder: Option<
        unsafe extern "C" fn(
            *const crate::src::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *const crate::src::xmltok::ENCODING,
    >,
    mut isGeneralTextEntity: ::core::ffi::c_int,
    mut enc: *const crate::src::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    badPtr: &mut *const ::core::ffi::c_char,
    versionPtr: Option<&mut *const ::core::ffi::c_char>,
    versionEndPtr: Option<&mut *const ::core::ffi::c_char>,
    encodingName: Option<&mut *const ::core::ffi::c_char>,
    encoding: Option<&mut *const crate::src::xmltok::ENCODING>,
    mut standalone: Option<&mut ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let mut val: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nameEnd: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let min_bytes = encoding_min_bytes(enc);
    ptr = ptr.wrapping_offset((5 as ::core::ffi::c_int * min_bytes) as isize);
    end = end.wrapping_offset(-((2 as ::core::ffi::c_int * min_bytes) as isize));
    if parsePseudoAttribute(enc, ptr, end, &mut name, &mut nameEnd, &mut val, &mut ptr) == 0
        || name.is_null()
    {
        *badPtr = ptr;
        return 0 as ::core::ffi::c_int;
    }
    if encoding_name_matches_ascii(enc, name, nameEnd, KW_version.as_ptr()) == 0 {
        if isGeneralTextEntity == 0 {
            *badPtr = name;
            return 0 as ::core::ffi::c_int;
        }
    } else {
        if let Some(versionPtr) = versionPtr {
            *versionPtr = val;
        }
        if let Some(versionEndPtr) = versionEndPtr {
            *versionEndPtr = ptr;
        }
        if parsePseudoAttribute(enc, ptr, end, &mut name, &mut nameEnd, &mut val, &mut ptr) == 0 {
            *badPtr = ptr;
            return 0 as ::core::ffi::c_int;
        }
        if name.is_null() {
            if isGeneralTextEntity != 0 {
                *badPtr = ptr;
                return 0 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
    }
    if encoding_name_matches_ascii(enc, name, nameEnd, KW_encoding.as_ptr()) != 0 {
        let mut c: ::core::ffi::c_int = toAscii(enc, val, end);
        if !(crate::ascii_h::ASCII_a_1 <= c && c <= crate::ascii_h::ASCII_z)
            && !(crate::ascii_h::ASCII_A <= c && c <= crate::ascii_h::ASCII_Z)
        {
            *badPtr = val;
            return 0 as ::core::ffi::c_int;
        }
        if let Some(encodingName) = encodingName {
            *encodingName = val;
        }
        if let Some(encoding) = encoding {
            *encoding = xml_decl_find_encoding(
                enc,
                encodingFinder.expect("non-null function pointer"),
                val,
                ptr.wrapping_offset(-(min_bytes as isize)),
            );
        }
        if parsePseudoAttribute(enc, ptr, end, &mut name, &mut nameEnd, &mut val, &mut ptr) == 0 {
            *badPtr = ptr;
            return 0 as ::core::ffi::c_int;
        }
        if name.is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    if encoding_name_matches_ascii(enc, name, nameEnd, KW_standalone.as_ptr()) == 0
        || isGeneralTextEntity != 0
    {
        *badPtr = name;
        return 0 as ::core::ffi::c_int;
    }
    if encoding_name_matches_ascii(
        enc,
        val,
        ptr.wrapping_offset(-(min_bytes as isize)),
        KW_yes.as_ptr(),
    ) != 0
    {
        if let Some(standalone) = standalone.as_mut() {
            **standalone = 1 as ::core::ffi::c_int;
        }
    } else if encoding_name_matches_ascii(
        enc,
        val,
        ptr.wrapping_offset(-(min_bytes as isize)),
        KW_no.as_ptr(),
    ) != 0
    {
        if let Some(standalone) = standalone.as_mut() {
            **standalone = 0 as ::core::ffi::c_int;
        }
    } else {
        *badPtr = val;
        return 0 as ::core::ffi::c_int;
    }
    while isSpace(toAscii(enc, ptr, end)) != 0 {
        ptr = ptr.wrapping_offset(min_bytes as isize);
    }
    if ptr != end {
        *badPtr = ptr;
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}

extern "C" fn checkCharRefNumber(mut result: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match result >> 8 as ::core::ffi::c_int {
        216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 => return -1 as ::core::ffi::c_int,
        0 => {
            if latin1_encoding.type_0[result as usize] as ::core::ffi::c_int
                == crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
            {
                return -1 as ::core::ffi::c_int;
            }
        }
        255 => {
            if result == 0xfffe as ::core::ffi::c_int || result == 0xffff as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return result;
}
pub fn XmlUtf8Encode(
    mut c: ::core::ffi::c_int,
    buf: &mut [::core::ffi::c_char],
) -> ::core::ffi::c_int {
    if c < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if c < min2 as ::core::ffi::c_int {
        buf[0] = (c | UTF8_cval1 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 1 as ::core::ffi::c_int;
    }
    if c < min3 as ::core::ffi::c_int {
        buf[0] = (c >> 6 as ::core::ffi::c_int | UTF8_cval2 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        buf[1] =
            (c & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 2 as ::core::ffi::c_int;
    }
    if c < min4 as ::core::ffi::c_int {
        buf[0] = (c >> 12 as ::core::ffi::c_int | UTF8_cval3 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        buf[1] = (c >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        buf[2] =
            (c & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 3 as ::core::ffi::c_int;
    }
    if c < 0x110000 as ::core::ffi::c_int {
        buf[0] = (c >> 18 as ::core::ffi::c_int | UTF8_cval4 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        buf[1] = (c >> 12 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        buf[2] = (c >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        buf[3] =
            (c & 0x3f as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 4 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name = "XmlUtf8Encode"]

pub unsafe extern "C" fn XmlUtf8Encode_ffi(
    mut c: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    XmlUtf8Encode(c, ::core::slice::from_raw_parts_mut(buf, 4))
}
pub fn XmlUtf16Encode(
    mut charNum: ::core::ffi::c_int,
    buf: &mut [::core::ffi::c_ushort],
) -> ::core::ffi::c_int {
    if charNum < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if charNum < 0x10000 as ::core::ffi::c_int {
        buf[0] = charNum as ::core::ffi::c_ushort;
        return 1 as ::core::ffi::c_int;
    }
    if charNum < 0x110000 as ::core::ffi::c_int {
        charNum -= 0x10000 as ::core::ffi::c_int;
        buf[0] = ((charNum >> 10 as ::core::ffi::c_int) + 0xd800 as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        buf[1] = ((charNum & 0x3ff as ::core::ffi::c_int) + 0xdc00 as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        return 2 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name = "XmlUtf16Encode"]

pub unsafe extern "C" fn XmlUtf16Encode_ffi(
    mut charNum: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    XmlUtf16Encode(charNum, ::core::slice::from_raw_parts_mut(buf, 2))
}
pub extern "C" fn XmlSizeOfUnknownEncoding() -> ::core::ffi::c_int {
    return ::core::mem::size_of::<unknown_encoding>() as ::core::ffi::c_int;
}
#[export_name = "XmlSizeOfUnknownEncoding"]

pub unsafe extern "C" fn XmlSizeOfUnknownEncoding_ffi() -> ::core::ffi::c_int {
    XmlSizeOfUnknownEncoding()
}
enum UnknownEncodingLookup {
    Convert(*const ::core::ffi::c_char),
    Utf8(usize),
    Utf16(usize),
}

enum UnknownEncodingValue {
    Code(::core::ffi::c_int),
    Utf8([::core::ffi::c_char; 4]),
    Utf16(::core::ffi::c_ushort),
}

fn unknown_encoding_lookup(
    enc: *const crate::src::xmltok::ENCODING,
    lookup: UnknownEncodingLookup,
) -> UnknownEncodingValue {
    match encoding_data_lookup(enc, EncodingDataLookup::Unknown(lookup)) {
        EncodingDataValue::Unknown(value) => value,
        _ => unreachable!(),
    }
}

fn unknown_convert_code(
    enc: *const crate::src::xmltok::ENCODING,
    p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match unknown_encoding_lookup(enc, UnknownEncodingLookup::Convert(p)) {
        UnknownEncodingValue::Code(code) => code,
        _ => unreachable!(),
    }
}

fn unknown_utf8_entry(
    enc: *const crate::src::xmltok::ENCODING,
    byte: usize,
) -> [::core::ffi::c_char; 4] {
    match unknown_encoding_lookup(enc, UnknownEncodingLookup::Utf8(byte)) {
        UnknownEncodingValue::Utf8(entry) => entry,
        _ => unreachable!(),
    }
}

fn unknown_utf16_entry(
    enc: *const crate::src::xmltok::ENCODING,
    byte: usize,
) -> ::core::ffi::c_ushort {
    match unknown_encoding_lookup(enc, UnknownEncodingLookup::Utf16(byte)) {
        UnknownEncodingValue::Utf16(entry) => entry,
        _ => unreachable!(),
    }
}

extern "C" fn unknown_isName(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = unknown_convert_code(enc, p);
    if c & !(0xffff as ::core::ffi::c_int) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return (namingBitmap[(((namePages[(c >> 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int)
        + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
        as usize]
        & (1 as ::core::ffi::c_uint)
            << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
        as ::core::ffi::c_int;
}

extern "C" fn unknown_isNmstrt(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = unknown_convert_code(enc, p);
    if c & !(0xffff as ::core::ffi::c_int) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    return (namingBitmap[(((nmstrtPages[(c >> 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int)
        << 3 as ::core::ffi::c_int)
        + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
        as usize]
        & (1 as ::core::ffi::c_uint)
            << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int))
        as ::core::ffi::c_int;
}

extern "C" fn unknown_isInvalid(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = unknown_convert_code(enc, p);
    return (c & !(0xffff as ::core::ffi::c_int) != 0
        || checkCharRefNumber(c) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}

extern "C" fn unknown_toUtf8(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        let mut input_consumed = 0;
        let mut output_written = 0;

        while input_consumed < input.len() {
            let byte = input[input_consumed] as usize;
            let entry = unknown_utf8_entry(enc, byte);
            let mut utf8: [::core::ffi::c_char; 4] = [0; 4];
            let (n, bytes_consumed) = if entry[0 as ::core::ffi::c_int as usize]
                == 0 as ::core::ffi::c_char
            {
                let c = unknown_convert_code(
                    enc,
                    input[input_consumed..].as_ptr() as *const ::core::ffi::c_char,
                );
                let n = XmlUtf8Encode(c, &mut utf8) as usize;
                let bytes_consumed = (normal_byte_type(enc, input[input_consumed] as _)
                    - (crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int)) as usize;
                (n, bytes_consumed)
            } else {
                let n = entry[0 as ::core::ffi::c_int as usize] as usize;
                utf8[..n].copy_from_slice(&entry[1..1 + n]);
                (n, 1)
            };

            if n > output.len() - output_written {
                return (
                    input_consumed,
                    output_written,
                    crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED,
                );
            }

            output[output_written..output_written + n].copy_from_slice(&utf8[..n]);
            input_consumed += bytes_consumed;
            output_written += n;
        }

        (
            input_consumed,
            output_written,
            crate::src::xmltok::XML_CONVERT_COMPLETED,
        )
    })
}

extern "C" fn unknown_toUtf16(
    mut enc: *const crate::src::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::xmltok::XML_Convert_Result {
    convert_raw_from_bytes(fromP, fromLim, toP, toLim, |input, output| {
        let mut input_consumed = 0;
        let mut output_written = 0;

        while input_consumed < input.len() && output_written < output.len() {
            let byte = input[input_consumed] as usize;
            let mut c = unknown_utf16_entry(enc, byte);
            let bytes_consumed = if c == 0 as ::core::ffi::c_ushort {
                c = unknown_convert_code(
                    enc,
                    input[input_consumed..].as_ptr() as *const ::core::ffi::c_char,
                ) as ::core::ffi::c_ushort;
                (normal_byte_type(enc, input[input_consumed] as _)
                    - (crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int)) as usize
            } else {
                1
            };

            output[output_written] = c;
            input_consumed += bytes_consumed;
            output_written += 1;
        }

        let result = if input_consumed < input.len() {
            crate::src::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED
        } else {
            crate::src::xmltok::XML_CONVERT_COMPLETED
        };

        (input_consumed, output_written, result)
    })
}
pub fn XmlInitUnknownEncoding(
    table: &[::core::ffi::c_int; 256],
    mut convert: crate::src::xmltok::CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> Option<unknown_encoding> {
    let mut e = unknown_encoding {
        normal: latin1_encoding,
        convert: None,
        userData: ::core::ptr::null_mut(),
        utf16: [0; 256],
        utf8: [[0; 4]; 256],
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 128 as ::core::ffi::c_int {
        if latin1_encoding.type_0[i as usize] as ::core::ffi::c_int
            != crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int
            && latin1_encoding.type_0[i as usize] as ::core::ffi::c_int
                != crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
            && table[i as usize] != i
        {
            return None;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_int = table[i as usize];
        if c == -1 as ::core::ffi::c_int {
            e.normal.type_0[i as usize] =
                crate::xmltok_impl_h::BT_MALFORM as ::core::ffi::c_int as ::core::ffi::c_uchar;
            e.utf16[i as usize] = 0xffff as ::core::ffi::c_ushort;
            e.utf8[i as usize][0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_char;
            e.utf8[i as usize][1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        } else if c < 0 as ::core::ffi::c_int {
            if c < -4 as ::core::ffi::c_int {
                return None;
            }
            if convert.is_none() {
                return None;
            }
            e.normal.type_0[i as usize] = (crate::xmltok_impl_h::BT_LEAD2 as ::core::ffi::c_int
                - (c + 2 as ::core::ffi::c_int))
                as ::core::ffi::c_uchar;
            e.utf8[i as usize][0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            e.utf16[i as usize] = 0 as ::core::ffi::c_ushort;
        } else if c < 0x80 as ::core::ffi::c_int {
            if latin1_encoding.type_0[c as usize] as ::core::ffi::c_int
                != crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int
                && latin1_encoding.type_0[c as usize] as ::core::ffi::c_int
                    != crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int
                && c != i
            {
                return None;
            }
            e.normal.type_0[i as usize] = latin1_encoding.type_0[c as usize];
            e.utf8[i as usize][0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_char;
            e.utf8[i as usize][1 as ::core::ffi::c_int as usize] = c as ::core::ffi::c_char;
            e.utf16[i as usize] = (if c == 0 as ::core::ffi::c_int {
                0xffff as ::core::ffi::c_int
            } else {
                c
            }) as ::core::ffi::c_ushort;
        } else if checkCharRefNumber(c) < 0 as ::core::ffi::c_int {
            e.normal.type_0[i as usize] =
                crate::xmltok_impl_h::BT_NONXML as ::core::ffi::c_int as ::core::ffi::c_uchar;
            e.utf16[i as usize] = 0xffff as ::core::ffi::c_ushort;
            e.utf8[i as usize][0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_char;
            e.utf8[i as usize][1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        } else {
            if c > 0xffff as ::core::ffi::c_int {
                return None;
            }
            if namingBitmap[(((nmstrtPages[(c >> 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int)
                + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
                as usize]
                & (1 as ::core::ffi::c_uint)
                    << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                != 0
            {
                e.normal.type_0[i as usize] =
                    crate::xmltok_impl_h::BT_NMSTRT as ::core::ffi::c_int as ::core::ffi::c_uchar;
            } else if namingBitmap[(((namePages[(c >> 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int)
                + ((c & 0xff as ::core::ffi::c_int) >> 5 as ::core::ffi::c_int))
                as usize]
                & (1 as ::core::ffi::c_uint)
                    << (c & 0xff as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                != 0
            {
                e.normal.type_0[i as usize] =
                    crate::xmltok_impl_h::BT_NAME as ::core::ffi::c_int as ::core::ffi::c_uchar;
            } else {
                e.normal.type_0[i as usize] =
                    crate::xmltok_impl_h::BT_OTHER as ::core::ffi::c_int as ::core::ffi::c_uchar;
            }
            e.utf8[i as usize][0 as ::core::ffi::c_int as usize] =
                XmlUtf8Encode(c, &mut e.utf8[i as usize][1..]) as ::core::ffi::c_char;
            e.utf16[i as usize] = c as ::core::ffi::c_ushort;
        }
        i += 1;
    }
    e.userData = userData;
    e.convert = convert;
    if convert.is_some() {
        e.normal.isName2 = Some(unknown_isName as NORMAL_ENCODING_CHAR_CHECK);
        e.normal.isName3 = Some(unknown_isName as NORMAL_ENCODING_CHAR_CHECK);
        e.normal.isName4 = Some(unknown_isName as NORMAL_ENCODING_CHAR_CHECK);
        e.normal.isNmstrt2 = Some(unknown_isNmstrt as NORMAL_ENCODING_CHAR_CHECK);
        e.normal.isNmstrt3 = Some(unknown_isNmstrt as NORMAL_ENCODING_CHAR_CHECK);
        e.normal.isNmstrt4 = Some(unknown_isNmstrt as NORMAL_ENCODING_CHAR_CHECK);
        e.normal.isInvalid2 = Some(unknown_isInvalid as NORMAL_ENCODING_CHAR_CHECK);
        e.normal.isInvalid3 = Some(unknown_isInvalid as NORMAL_ENCODING_CHAR_CHECK);
        e.normal.isInvalid4 = Some(unknown_isInvalid as NORMAL_ENCODING_CHAR_CHECK);
    }
    e.normal.enc.utf8Convert = Some(
        unknown_toUtf8
            as unsafe extern "C" fn(
                *const crate::src::xmltok::ENCODING,
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> crate::src::xmltok::XML_Convert_Result,
    )
        as Option<
            unsafe extern "C" fn(
                *const crate::src::xmltok::ENCODING,
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> crate::src::xmltok::XML_Convert_Result,
        >;
    e.normal.enc.utf16Convert = Some(
        unknown_toUtf16
            as unsafe extern "C" fn(
                *const crate::src::xmltok::ENCODING,
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_ushort,
                *const ::core::ffi::c_ushort,
            ) -> crate::src::xmltok::XML_Convert_Result,
    )
        as Option<
            unsafe extern "C" fn(
                *const crate::src::xmltok::ENCODING,
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_ushort,
                *const ::core::ffi::c_ushort,
            ) -> crate::src::xmltok::XML_Convert_Result,
        >;
    Some(e)
}
#[export_name = "XmlInitUnknownEncoding"]

pub unsafe extern "C" fn XmlInitUnknownEncoding_ffi(
    mut mem: *mut ::core::ffi::c_void,
    mut table: *const ::core::ffi::c_int,
    mut convert: crate::src::xmltok::CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> *mut crate::src::xmltok::ENCODING {
    let Some(encoding) =
        XmlInitUnknownEncoding(&*(table as *const [::core::ffi::c_int; 256]), convert, userData)
    else {
        return ::core::ptr::null_mut::<crate::src::xmltok::ENCODING>();
    };
    let e = mem as *mut unknown_encoding;
    e.write(encoding);
    &raw mut (*e).normal.enc
}
static KW_ISO_8859_1: [::core::ffi::c_char; 11] = [
    crate::ascii_h::ASCII_I as ::core::ffi::c_char,
    crate::ascii_h::ASCII_S as ::core::ffi::c_char,
    crate::ascii_h::ASCII_O as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_8_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_8_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_5 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_9_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_1_1 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

static KW_US_ASCII: [::core::ffi::c_char; 9] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_S as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_A as ::core::ffi::c_char,
    crate::ascii_h::ASCII_S as ::core::ffi::c_char,
    crate::ascii_h::ASCII_C as ::core::ffi::c_char,
    crate::ascii_h::ASCII_I as ::core::ffi::c_char,
    crate::ascii_h::ASCII_I as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

static KW_UTF_8: [::core::ffi::c_char; 6] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_T as ::core::ffi::c_char,
    crate::ascii_h::ASCII_F_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_8_1 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

static KW_UTF_16: [::core::ffi::c_char; 7] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_T as ::core::ffi::c_char,
    crate::ascii_h::ASCII_F_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_1_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_6 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

static KW_UTF_16BE: [::core::ffi::c_char; 9] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_T as ::core::ffi::c_char,
    crate::ascii_h::ASCII_F_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_1_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_6 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_B_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_E_1 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

static KW_UTF_16LE: [::core::ffi::c_char; 9] = [
    crate::ascii_h::ASCII_U as ::core::ffi::c_char,
    crate::ascii_h::ASCII_T as ::core::ffi::c_char,
    crate::ascii_h::ASCII_F_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_MINUS as ::core::ffi::c_char,
    crate::ascii_h::ASCII_1_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_6 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_L_1 as ::core::ffi::c_char,
    crate::ascii_h::ASCII_E_1 as ::core::ffi::c_char,
    '\0' as i32 as ::core::ffi::c_char,
];

fn getEncodingIndex(name: Option<&[::core::ffi::c_char]>) -> ::core::ffi::c_int {
    let Some(name) = name else {
        return NO_ENC as ::core::ffi::c_int;
    };
    let encoding_names: [&[::core::ffi::c_char]; 6] = [
        &KW_ISO_8859_1,
        &KW_US_ASCII,
        &KW_UTF_8,
        &KW_UTF_16,
        &KW_UTF_16BE,
        &KW_UTF_16LE,
    ];
    for (i, encoding_name) in encoding_names.iter().enumerate() {
        if streqci(name.iter().copied(), encoding_name.iter().copied()) != 0 {
            return i as ::core::ffi::c_int;
        }
    }
    return UNKNOWN_ENC as ::core::ffi::c_int;
}

extern "C" fn initScan(
    mut encodingTable: *const *const crate::src::xmltok::ENCODING,
    mut enc: *const crate::src::xmltok::INIT_ENCODING,
    mut state: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match encoding_data_lookup(
        ::core::ptr::null::<crate::src::xmltok::ENCODING>(),
        EncodingDataLookup::InitScan {
            encoding_table: encodingTable,
            init_encoding: enc,
            state,
            ptr,
            end,
            next_tok_ptr: nextTokPtr,
        },
    ) {
        EncodingDataValue::Int(value) => value,
        _ => unreachable!(),
    }
}
pub fn XmlInitUnknownEncodingNS(
    table: &[::core::ffi::c_int; 256],
    mut convert: crate::src::xmltok::CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> Option<unknown_encoding> {
    let mut encoding = XmlInitUnknownEncoding(table, convert, userData)?;
    encoding.normal.type_0[crate::ascii_h::ASCII_COLON as usize] =
        crate::xmltok_impl_h::BT_COLON_0 as ::core::ffi::c_int as ::core::ffi::c_uchar;
    Some(encoding)
}
#[export_name = "XmlInitUnknownEncodingNS"]

pub unsafe extern "C" fn XmlInitUnknownEncodingNS_ffi(
    mut mem: *mut ::core::ffi::c_void,
    mut table: *const ::core::ffi::c_int,
    mut convert: crate::src::xmltok::CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> *mut crate::src::xmltok::ENCODING {
    let Some(encoding) =
        XmlInitUnknownEncodingNS(&*(table as *const [::core::ffi::c_int; 256]), convert, userData)
    else {
        return ::core::ptr::null_mut::<crate::src::xmltok::ENCODING>();
    };
    let e = mem as *mut unknown_encoding;
    e.write(encoding);
    &raw mut (*e).normal.enc
}
