// =============== BEGIN xmltok_h ================
pub const XML_TOK_TRAILING_RSQB: ::core::ffi::c_int = -5;

pub const XML_TOK_TRAILING_RSQB_1: ::core::ffi::c_int = -(5);

pub const XML_TOK_NONE: ::core::ffi::c_int = -4;

pub const XML_TOK_NONE_1: ::core::ffi::c_int = -(4);

pub const XML_TOK_TRAILING_CR: ::core::ffi::c_int = -3;

pub const XML_TOK_TRAILING_CR_1: ::core::ffi::c_int = -(3);

pub const XML_TOK_PARTIAL_CHAR: ::core::ffi::c_int = -2;

pub const XML_TOK_PARTIAL_CHAR_1: ::core::ffi::c_int = -(2);

pub const XML_TOK_PARTIAL: ::core::ffi::c_int = -1;

pub const XML_TOK_PARTIAL_1: ::core::ffi::c_int = -(1);

pub const XML_TOK_INVALID: ::core::ffi::c_int = 0;

pub const XML_TOK_INVALID_1: ::core::ffi::c_int = 0;

pub const XML_TOK_START_TAG_WITH_ATTS: ::core::ffi::c_int = 1;

pub const XML_TOK_START_TAG_WITH_ATTS_1: ::core::ffi::c_int = 1;

pub const XML_TOK_START_TAG_NO_ATTS: ::core::ffi::c_int = 2;

pub const XML_TOK_START_TAG_NO_ATTS_1: ::core::ffi::c_int = 2;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: ::core::ffi::c_int = 3;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1: ::core::ffi::c_int = 3;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: ::core::ffi::c_int = 4;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS_1: ::core::ffi::c_int = 4;

pub const XML_TOK_END_TAG: ::core::ffi::c_int = 5;

pub const XML_TOK_END_TAG_1: ::core::ffi::c_int = 5;

pub const XML_TOK_DATA_CHARS: ::core::ffi::c_int = 6;

pub const XML_TOK_DATA_CHARS_1: ::core::ffi::c_int = 6;

pub const XML_TOK_DATA_NEWLINE: ::core::ffi::c_int = 7;

pub const XML_TOK_DATA_NEWLINE_1: ::core::ffi::c_int = 7;

pub const XML_TOK_CDATA_SECT_OPEN: ::core::ffi::c_int = 8;

pub const XML_TOK_CDATA_SECT_OPEN_1: ::core::ffi::c_int = 8;

pub const XML_TOK_ENTITY_REF: ::core::ffi::c_int = 9;

pub const XML_TOK_ENTITY_REF_1: ::core::ffi::c_int = 9;

pub const XML_TOK_CHAR_REF: ::core::ffi::c_int = 10;

pub const XML_TOK_CHAR_REF_1: ::core::ffi::c_int = 10;

pub const XML_TOK_PI: ::core::ffi::c_int = 11;

pub const XML_TOK_PI_1: ::core::ffi::c_int = 11;

pub const XML_TOK_XML_DECL: ::core::ffi::c_int = 12;

pub const XML_TOK_XML_DECL_1: ::core::ffi::c_int = 12;

pub const XML_TOK_COMMENT: ::core::ffi::c_int = 13;

pub const XML_TOK_COMMENT_1: ::core::ffi::c_int = 13;

pub const XML_TOK_BOM: ::core::ffi::c_int = 14;

pub const XML_TOK_BOM_1: ::core::ffi::c_int = 14;

pub const XML_TOK_PROLOG_S: ::core::ffi::c_int = 15;

pub const XML_TOK_PROLOG_S_1: ::core::ffi::c_int = 15;

pub const XML_TOK_DECL_OPEN: ::core::ffi::c_int = 16;

pub const XML_TOK_DECL_OPEN_1: ::core::ffi::c_int = 16;

pub const XML_TOK_DECL_CLOSE: ::core::ffi::c_int = 17;

pub const XML_TOK_DECL_CLOSE_1: ::core::ffi::c_int = 17;

pub const XML_TOK_NAME: ::core::ffi::c_int = 18;

pub const XML_TOK_NMTOKEN: ::core::ffi::c_int = 19;

pub const XML_TOK_NMTOKEN_1: ::core::ffi::c_int = 19;

pub const XML_TOK_POUND_NAME: ::core::ffi::c_int = 20;

pub const XML_TOK_POUND_NAME_1: ::core::ffi::c_int = 20;

pub const XML_TOK_OR: ::core::ffi::c_int = 21;

pub const XML_TOK_OR_1: ::core::ffi::c_int = 21;

pub const XML_TOK_PERCENT: ::core::ffi::c_int = 22;

pub const XML_TOK_PERCENT_1: ::core::ffi::c_int = 22;

pub const XML_TOK_OPEN_PAREN: ::core::ffi::c_int = 23;

pub const XML_TOK_OPEN_PAREN_1: ::core::ffi::c_int = 23;

pub const XML_TOK_CLOSE_PAREN: ::core::ffi::c_int = 24;

pub const XML_TOK_CLOSE_PAREN_1: ::core::ffi::c_int = 24;

pub const XML_TOK_OPEN_BRACKET: ::core::ffi::c_int = 25;

pub const XML_TOK_OPEN_BRACKET_1: ::core::ffi::c_int = 25;

pub const XML_TOK_CLOSE_BRACKET: ::core::ffi::c_int = 26;

pub const XML_TOK_CLOSE_BRACKET_1: ::core::ffi::c_int = 26;

pub const XML_TOK_LITERAL: ::core::ffi::c_int = 27;

pub const XML_TOK_LITERAL_1: ::core::ffi::c_int = 27;

pub const XML_TOK_PARAM_ENTITY_REF: ::core::ffi::c_int = 28;

pub const XML_TOK_PARAM_ENTITY_REF_1: ::core::ffi::c_int = 28;

pub const XML_TOK_INSTANCE_START: ::core::ffi::c_int = 29;

pub const XML_TOK_INSTANCE_START_1: ::core::ffi::c_int = 29;

pub const XML_TOK_NAME_QUESTION: ::core::ffi::c_int = 30;

pub const XML_TOK_NAME_QUESTION_1: ::core::ffi::c_int = 30;

pub const XML_TOK_NAME_ASTERISK: ::core::ffi::c_int = 31;

pub const XML_TOK_NAME_ASTERISK_1: ::core::ffi::c_int = 31;

pub const XML_TOK_NAME_PLUS: ::core::ffi::c_int = 32;

pub const XML_TOK_NAME_PLUS_1: ::core::ffi::c_int = 32;

pub const XML_TOK_COND_SECT_OPEN: ::core::ffi::c_int = 33;

pub const XML_TOK_COND_SECT_OPEN_1: ::core::ffi::c_int = 33;

pub const XML_TOK_COND_SECT_CLOSE: ::core::ffi::c_int = 34;

pub const XML_TOK_COND_SECT_CLOSE_1: ::core::ffi::c_int = 34;

pub const XML_TOK_CLOSE_PAREN_QUESTION: ::core::ffi::c_int = 35;

pub const XML_TOK_CLOSE_PAREN_QUESTION_1: ::core::ffi::c_int = 35;

pub const XML_TOK_CLOSE_PAREN_ASTERISK: ::core::ffi::c_int = 36;

pub const XML_TOK_CLOSE_PAREN_ASTERISK_1: ::core::ffi::c_int = 36;

pub const XML_TOK_CLOSE_PAREN_PLUS: ::core::ffi::c_int = 37;

pub const XML_TOK_CLOSE_PAREN_PLUS_1: ::core::ffi::c_int = 37;

pub const XML_TOK_COMMA: ::core::ffi::c_int = 38;

pub const XML_TOK_COMMA_1: ::core::ffi::c_int = 38;

pub const XML_TOK_ATTRIBUTE_VALUE_S: ::core::ffi::c_int = 39;

pub const XML_TOK_ATTRIBUTE_VALUE_S_1: ::core::ffi::c_int = 39;

pub const XML_TOK_CDATA_SECT_CLOSE: ::core::ffi::c_int = 40;

pub const XML_TOK_CDATA_SECT_CLOSE_1: ::core::ffi::c_int = 40;

pub const XML_TOK_PREFIXED_NAME: ::core::ffi::c_int = 41;

pub const XML_TOK_IGNORE_SECT: ::core::ffi::c_int = 42;

pub const XML_TOK_IGNORE_SECT_1: ::core::ffi::c_int = 42;

pub const XML_PROLOG_STATE: ::core::ffi::c_int = 0;

pub const XML_CONTENT_STATE: ::core::ffi::c_int = 1;

pub type POSITION = crate::src::lib::xmltok::position;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct position {
    pub lineNumber: XML_Size,
    pub columnNumber: XML_Size,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ATTRIBUTE {
    pub name: *const ::core::ffi::c_char,
    pub valuePtr: *const ::core::ffi::c_char,
    pub valueEnd: *const ::core::ffi::c_char,
    pub normalized: ::core::ffi::c_char,
}

pub type ENCODING = crate::src::lib::xmltok::encoding;

pub type SCANNER = unsafe fn(
    *const crate::src::lib::xmltok::ENCODING,
    *const ::core::ffi::c_char,
    *const ::core::ffi::c_char,
    *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int;

pub type XML_Convert_Result = ::core::ffi::c_uint;

pub const XML_CONVERT_COMPLETED: crate::src::lib::xmltok::XML_Convert_Result = 0;

pub const XML_CONVERT_INPUT_INCOMPLETE: crate::src::lib::xmltok::XML_Convert_Result = 1;

pub const XML_CONVERT_OUTPUT_EXHAUSTED: crate::src::lib::xmltok::XML_Convert_Result = 2;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct encoding {
    pub scanners: [crate::src::lib::xmltok::SCANNER; 4],
    pub literalScanners: [crate::src::lib::xmltok::SCANNER; 2],
    pub nameMatchesAscii: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    pub nameLength: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    pub skipS: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char,
    pub getAtts: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
        *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int,
    pub charRefNumber: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    pub predefinedEntityName: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    pub updatePosition: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut crate::src::lib::xmltok::POSITION,
    ) -> (),
    pub isPublicId: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
    pub utf8Convert: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *mut *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *mut ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> crate::src::lib::xmltok::XML_Convert_Result,
    pub utf16Convert: unsafe fn(
        *const crate::src::lib::xmltok::ENCODING,
        *mut *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *mut ::core::ffi::c_ushort,
        *const ::core::ffi::c_ushort,
    ) -> crate::src::lib::xmltok::XML_Convert_Result,
    pub minBytesPerChar: ::core::ffi::c_int,
    pub isUtf8: ::core::ffi::c_char,
    pub isUtf16: ::core::ffi::c_char,
}

impl encoding {
    pub(crate) unsafe fn nameMatchesAscii(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        kw: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        (self.nameMatchesAscii)(enc, ptr, end, kw)
    }

    pub(crate) unsafe fn nameLength(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        (self.nameLength)(enc, ptr)
    }

    pub(crate) unsafe fn skipS(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        (self.skipS)(enc, ptr)
    }

    pub(crate) unsafe fn getAtts(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        n: ::core::ffi::c_int,
        atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        (self.getAtts)(enc, ptr, n, atts)
    }

    pub(crate) unsafe fn charRefNumber(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        (self.charRefNumber)(enc, ptr)
    }

    pub(crate) unsafe fn predefinedEntityName(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        (self.predefinedEntityName)(enc, ptr, end)
    }

    pub(crate) unsafe fn updatePosition(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        pos: *mut crate::src::lib::xmltok::POSITION,
    ) {
        (self.updatePosition)(enc, ptr, end, pos)
    }

    pub(crate) unsafe fn isPublicId(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        ptr: *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
        event_pp: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        (self.isPublicId)(enc, ptr, end, event_pp)
    }

    pub(crate) unsafe fn utf8Convert(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        from_p: *mut *const ::core::ffi::c_char,
        from_lim: *const ::core::ffi::c_char,
        to_p: *mut *mut ::core::ffi::c_char,
        to_lim: *const ::core::ffi::c_char,
    ) -> crate::src::lib::xmltok::XML_Convert_Result {
        (self.utf8Convert)(enc, from_p, from_lim, to_p, to_lim)
    }

    pub(crate) unsafe fn utf16Convert(
        &self,
        enc: *const crate::src::lib::xmltok::ENCODING,
        from_p: *mut *const ::core::ffi::c_char,
        from_lim: *const ::core::ffi::c_char,
        to_p: *mut *mut ::core::ffi::c_ushort,
        to_lim: *const ::core::ffi::c_ushort,
    ) -> crate::src::lib::xmltok::XML_Convert_Result {
        (self.utf16Convert)(enc, from_p, from_lim, to_p, to_lim)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct INIT_ENCODING {
    pub initEnc: crate::src::lib::xmltok::ENCODING,
    pub encPtr: *mut *const crate::src::lib::xmltok::ENCODING,
}

pub type CONVERTER = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

pub mod xmltok_impl_c {

    pub(crate) unsafe fn normal_scanComment(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            if !(*ptr as ::core::ffi::c_int == 0x2d) {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.offset(1);
            while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    5 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        if (*(enc as *const normal_encoding))
                            .isInvalid2
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                        {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    6 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        if (*(enc as *const normal_encoding))
                            .isInvalid3
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                        {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    7 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        if (*(enc as *const normal_encoding))
                            .isInvalid4
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                        {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    27 => {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr as ::core::ffi::c_int == 0x2d {
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 1) as ::core::ffi::c_long)
                            {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr as ::core::ffi::c_int == 0x3e) {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(1);
                            return crate::src::lib::xmltok::XML_TOK_COMMENT_1;
                        }
                    }
                    _ => {
                        ptr = ptr.offset(1isize);
                    }
                }
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_scanDecl(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            27 => {
                return normal_scanComment(enc, ptr.offset(1isize), end, nextTokPtr);
            }
            20 => {
                *nextTokPtr = ptr.offset(1);
                return crate::src::lib::xmltok::XML_TOK_COND_SECT_OPEN_1;
            }
            22 | 24 => {
                ptr = ptr.offset(1isize);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            's_129: {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    30 => {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (2i32 * 1) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(1);
                        break 's_129;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_DECL_OPEN_1;
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_checkPiTarget(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut tokPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0;
        *tokPtr = crate::src::lib::xmltok::XML_TOK_PI_1;
        if end.offset_from(ptr) as ::core::ffi::c_long != (1i32 * 3) as ::core::ffi::c_long {
            return 1i32;
        }
        match *ptr as ::core::ffi::c_int {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1i32;
            }
            _ => return 1,
        }
        ptr = ptr.offset(1);
        match *ptr as ::core::ffi::c_int {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1i32;
            }
            _ => return 1,
        }
        ptr = ptr.offset(1);
        match *ptr as ::core::ffi::c_int {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1i32;
            }
            _ => return 1,
        }
        if upper != 0 {
            return 0i32;
        }
        *tokPtr = crate::src::lib::xmltok::XML_TOK_XML_DECL_1;
        return 1;
    }

    pub(crate) unsafe fn normal_scanPi(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        let mut target: *const ::core::ffi::c_char = ptr;
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_32: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_32 = 11470911313929454839;
            }
            22 | 24 => {
                current_block_32 = 11470911313929454839;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_32 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_32 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_32 = 14763689060501151050;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_32 {
            11470911313929454839 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_118: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_118 = 8485341570193076947;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_118 = 8485341570193076947;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_118 = 13349765058737954042;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_118 = 13349765058737954042;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_118 = 13349765058737954042;
                }
                21 | 9 | 10 => {
                    if normal_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(1);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long
                    {
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            15 => {
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 1) as ::core::ffi::c_long)
                                {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                                }
                                if *ptr as ::core::ffi::c_int == 0x3e {
                                    *nextTokPtr = ptr.offset(1);
                                    return tok;
                                }
                            }
                            _ => {
                                ptr = ptr.offset(1isize);
                            }
                        }
                    }
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                15 => {
                    if normal_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr as ::core::ffi::c_int == 0x3e {
                        *nextTokPtr = ptr.offset(1);
                        return tok;
                    }
                    current_block_118 = 11310415194689177606;
                }
                _ => {
                    current_block_118 = 11310415194689177606;
                }
            }
            match current_block_118 {
                11310415194689177606 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                8485341570193076947 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_scanCdataSection(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        pub static CDATA_LSQB: [::core::ffi::c_char; 6] = [
            crate::ascii_h::ASCII_C as ::core::ffi::c_char,
            crate::ascii_h::ASCII_D as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_T as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_LSQB as ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (6i32 * 1) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        i = 0;
        while i < 6 {
            if !(*ptr as ::core::ffi::c_int == CDATA_LSQB[i as usize] as ::core::ffi::c_int) {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            i += 1;
            ptr = ptr.offset(1);
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    }

    pub(crate) unsafe fn normal_cdataSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        }
        if 1 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (1i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(1i32 - 1) as crate::__stddef_size_t_h::size_t;
                if n == 0 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            4 => {
                ptr = ptr.offset(1);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                if *ptr as ::core::ffi::c_int == 0x5d {
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr as ::core::ffi::c_int == 0x3e) {
                        ptr = ptr.offset(-(1isize));
                    } else {
                        *nextTokPtr = ptr.offset(1);
                        return crate::src::lib::xmltok::XML_TOK_CDATA_SECT_CLOSE_1;
                    }
                }
            }
            9 => {
                ptr = ptr.offset(1);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                if (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                    == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1isize);
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(1);
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(1isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2
                        || (*(enc as *const normal_encoding))
                            .isInvalid2
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3
                        || (*(enc as *const normal_encoding))
                            .isInvalid3
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4
                        || (*(enc as *const normal_encoding))
                            .isInvalid4
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(1isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn normal_scanEndTag(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_32: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_32 = 4324628675098861213;
            }
            22 | 24 => {
                current_block_32 = 4324628675098861213;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_32 {
            4324628675098861213 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_73: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_73 = 14883924698754021420;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_73 = 14883924698754021420;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_73 = 981995395831942902;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_73 = 981995395831942902;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_73 = 981995395831942902;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(1);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long
                    {
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(1);
                                return crate::src::lib::xmltok::XML_TOK_END_TAG_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(1);
                    }
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                23 => {
                    ptr = ptr.offset(1);
                    current_block_73 = 981995395831942902;
                }
                11 => {
                    *nextTokPtr = ptr.offset(1);
                    return crate::src::lib::xmltok::XML_TOK_END_TAG_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_73 {
                14883924698754021420 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_scanHexCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(1);
            while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(1);
                        return crate::src::lib::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(1);
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_scanCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            if *ptr as ::core::ffi::c_int == 0x78 {
                return normal_scanHexCharRef(enc, ptr.offset(1isize), end, nextTokPtr);
            }
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(1);
            while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(1);
                        return crate::src::lib::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(1);
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_scanRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_33: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_33 = 8911980980495988282;
            }
            22 | 24 => {
                current_block_33 = 8911980980495988282;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_33 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_33 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_33 = 14763689060501151050;
            }
            19 => {
                return normal_scanCharRef(enc, ptr.offset(1isize), end, nextTokPtr);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_33 {
            8911980980495988282 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_64: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_64 = 11948064939145634034;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_64 = 11948064939145634034;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_64 = 10930818133215224067;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_64 = 10930818133215224067;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_64 = 10930818133215224067;
                }
                18 => {
                    *nextTokPtr = ptr.offset(1);
                    return crate::src::lib::xmltok::XML_TOK_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_64 {
                11948064939145634034 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_scanAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_186: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_186 = 3818392175876617014;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_186 = 3818392175876617014;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_186 = 1634947208139838470;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_186 = 1634947208139838470;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_186 = 1634947208139838470;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    hadColon = 1;
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    let mut current_block_64: u64;
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        29 => {
                            if 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            current_block_64 = 7083593080606520045;
                        }
                        22 | 24 => {
                            current_block_64 = 7083593080606520045;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid2
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_64 = 10930818133215224067;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid3
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_64 = 10930818133215224067;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid4
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_64 = 10930818133215224067;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_64 {
                        7083593080606520045 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                    current_block_186 = 1634947208139838470;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: ::core::ffi::c_int = 0;
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        t = (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int;
                        if t == crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                    }
                    current_block_186 = 10853015579903106591;
                }
                14 => {
                    current_block_186 = 10853015579903106591;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_186 {
                10853015579903106591 => {
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0;
                    loop {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        open = (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int;
                        if open == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                            || open == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                    }
                    ptr = ptr.offset(1);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        t_0 = (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int;
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int =
                                    normal_scanRef(enc, ptr.offset(1), end, &raw mut ptr);
                                if tok <= 0 {
                                    if tok == crate::src::lib::xmltok::XML_TOK_INVALID_1 {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            _ => {
                                ptr = ptr.offset(1isize);
                            }
                        }
                    }
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 1) as ::core::ffi::c_long)
                                {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                                }
                                match (*(enc as *const normal_encoding)).type_0
                                    [*ptr as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int
                                {
                                    29 => {
                                        if 0 == 0 {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        current_block_186 = 11210999262882855128;
                                        break;
                                    }
                                    22 | 24 => {
                                        current_block_186 = 11210999262882855128;
                                        break;
                                    }
                                    5 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if (*(enc as *const normal_encoding))
                                            .isInvalid2
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) != 0
                                            || (*(enc as *const normal_encoding))
                                                .isNmstrt2
                                                .expect("non-null function pointer")(
                                                enc, ptr
                                            ) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2);
                                        current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if (*(enc as *const normal_encoding))
                                            .isInvalid3
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) != 0
                                            || (*(enc as *const normal_encoding))
                                                .isNmstrt3
                                                .expect("non-null function pointer")(
                                                enc, ptr
                                            ) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3);
                                        current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if (*(enc as *const normal_encoding))
                                            .isInvalid4
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) != 0
                                            || (*(enc as *const normal_encoding))
                                                .isNmstrt4
                                                .expect("non-null function pointer")(
                                                enc, ptr
                                            ) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4);
                                        current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        current_block_186 = 2944436519209994553;
                                        break;
                                    }
                                    17 => {
                                        current_block_186 = 398073151373002430;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            match current_block_186 {
                                2944436519209994553 => {}
                                398073151373002430 => {}
                                1634947208139838470 => {}
                                _ => {
                                    ptr = ptr.offset(1);
                                    current_block_186 = 1634947208139838470;
                                }
                            }
                        }
                        17 => {
                            current_block_186 = 398073151373002430;
                        }
                        11 => {
                            current_block_186 = 2944436519209994553;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_186 {
                        1634947208139838470 => {}
                        _ => match current_block_186 {
                            398073151373002430 => {
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 1) as ::core::ffi::c_long)
                                {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                                }
                                if !(*ptr as ::core::ffi::c_int == 0x3e) {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                *nextTokPtr = ptr.offset(1);
                                return crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(1);
                                return crate::src::lib::xmltok::XML_TOK_START_TAG_WITH_ATTS_1;
                            }
                        },
                    }
                }
                3818392175876617014 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_scanLt(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_45: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_45 = 2165477741955893522;
            }
            22 | 24 => {
                current_block_45 = 2165477741955893522;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_45 = 8180496224585318153;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_45 = 8180496224585318153;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_45 = 8180496224585318153;
            }
            16 => {
                ptr = ptr.offset(1);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    27 => {
                        return normal_scanComment(enc, ptr.offset(1isize), end, nextTokPtr);
                    }
                    20 => {
                        return normal_scanCdataSection(enc, ptr.offset(1isize), end, nextTokPtr);
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            15 => {
                return normal_scanPi(enc, ptr.offset(1isize), end, nextTokPtr);
            }
            17 => {
                return normal_scanEndTag(enc, ptr.offset(1isize), end, nextTokPtr);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_45 {
            2165477741955893522 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        hadColon = 0;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_161: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_161 = 6701753098489376273;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_161 = 6701753098489376273;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_161 = 14714495436747744489;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_161 = 14714495436747744489;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_161 = 14714495436747744489;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    hadColon = 1;
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    let mut current_block_112: u64;
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        29 => {
                            if 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            current_block_112 = 9169466483824547789;
                        }
                        22 | 24 => {
                            current_block_112 = 9169466483824547789;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid2
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_112 = 2616667235040759262;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid3
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_112 = 2616667235040759262;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if (*(enc as *const normal_encoding))
                                .isInvalid4
                                .expect("non-null function pointer")(
                                enc, ptr
                            ) != 0
                                || (*(enc as *const normal_encoding))
                                    .isNmstrt4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_112 = 2616667235040759262;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_112 {
                        9169466483824547789 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                    current_block_161 = 14714495436747744489;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(1);
                    loop {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            current_block_161 = 13215501469961642988;
                            break;
                        }
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            29 => {
                                if 0 == 0 {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                current_block_161 = 7939927167482451446;
                            }
                            22 | 24 => {
                                current_block_161 = 7939927167482451446;
                            }
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                    || (*(enc as *const normal_encoding))
                                        .isNmstrt2
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) == 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2);
                                current_block_161 = 16314074004867283505;
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                    || (*(enc as *const normal_encoding))
                                        .isNmstrt3
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) == 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3);
                                current_block_161 = 16314074004867283505;
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                    || (*(enc as *const normal_encoding))
                                        .isNmstrt4
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) == 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4);
                                current_block_161 = 16314074004867283505;
                            }
                            11 => {
                                current_block_161 = 5640065479517572396;
                                break;
                            }
                            17 => {
                                current_block_161 = 12549409781983877175;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(1);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                        match current_block_161 {
                            7939927167482451446 => {
                                ptr = ptr.offset(1isize);
                            }
                            _ => {}
                        }
                        return normal_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match current_block_161 {
                        5640065479517572396 => {}
                        12549409781983877175 => {}
                        _ => return crate::src::lib::xmltok::XML_TOK_PARTIAL_1,
                    }
                }
                11 => {
                    current_block_161 = 5640065479517572396;
                }
                17 => {
                    current_block_161 = 12549409781983877175;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_161 {
                12549409781983877175 => {
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr as ::core::ffi::c_int == 0x3e) {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(1);
                    return crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                }
                5640065479517572396 => {
                    *nextTokPtr = ptr.offset(1);
                    return crate::src::lib::xmltok::XML_TOK_START_TAG_NO_ATTS_1;
                }
                6701753098489376273 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_contentTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        }
        if 1 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (1i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(1i32 - 1) as crate::__stddef_size_t_h::size_t;
                if n == 0 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            2 => {
                return normal_scanLt(enc, ptr.offset(1isize), end, nextTokPtr);
            }
            3 => {
                return normal_scanRef(enc, ptr.offset(1isize), end, nextTokPtr);
            }
            9 => {
                ptr = ptr.offset(1);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
                }
                if (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                    == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1isize);
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(1);
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            4 => {
                ptr = ptr.offset(1);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_TRAILING_RSQB_1;
                }
                if *ptr as ::core::ffi::c_int == 0x5d {
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_TRAILING_RSQB_1;
                    }
                    if !(*ptr as ::core::ffi::c_int == 0x3e) {
                        ptr = ptr.offset(-(1isize));
                    } else {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(1isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_76: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2
                        || (*(enc as *const normal_encoding))
                            .isInvalid2
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_76 = 7158658067966855297;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3
                        || (*(enc as *const normal_encoding))
                            .isInvalid3
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_76 = 7158658067966855297;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4
                        || (*(enc as *const normal_encoding))
                            .isInvalid4
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_76 = 7158658067966855297;
                }
                4 => {
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 1) as ::core::ffi::c_long
                    {
                        if !(*ptr.offset(1) as ::core::ffi::c_int == 0x5d) {
                            ptr = ptr.offset(1);
                            current_block_76 = 7158658067966855297;
                        } else if end.offset_from(ptr) as ::core::ffi::c_long
                            >= (3i32 * 1) as ::core::ffi::c_long
                        {
                            if !(*ptr.offset((2i32 * 1) as isize) as ::core::ffi::c_int == 0x3e) {
                                ptr = ptr.offset(1isize);
                            } else {
                                *nextTokPtr = ptr.offset((2i32 * 1) as isize);
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            current_block_76 = 7158658067966855297;
                        } else {
                            current_block_76 = 1999360611754201214;
                        }
                    } else {
                        current_block_76 = 1999360611754201214;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    current_block_76 = 1999360611754201214;
                }
                _ => {
                    ptr = ptr.offset(1);
                    current_block_76 = 7158658067966855297;
                }
            }
            match current_block_76 {
                7158658067966855297 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn normal_scanPercent(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_34: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_34 = 12478441211659886388;
            }
            22 | 24 => {
                current_block_34 = 12478441211659886388;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_34 = 4761528863920922185;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_34 = 4761528863920922185;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_34 = 4761528863920922185;
            }
            21 | 10 | 9 | 30 => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_PERCENT_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_34 {
            12478441211659886388 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_65: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_65 = 7770117754142564343;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_65 = 7770117754142564343;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_65 = 16415152177862271243;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_65 = 16415152177862271243;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_65 = 16415152177862271243;
                }
                18 => {
                    *nextTokPtr = ptr.offset(1);
                    return crate::src::lib::xmltok::XML_TOK_PARAM_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_65 {
                7770117754142564343 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_scanPoundName(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_32: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_32 = 1867613116081924762;
            }
            22 | 24 => {
                current_block_32 = 1867613116081924762;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt2
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt3
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                    || (*(enc as *const normal_encoding))
                        .isNmstrt4
                        .expect("non-null function pointer")(enc, ptr)
                        == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_32 {
            1867613116081924762 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_63: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_63 = 226587729178875444;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_63 = 226587729178875444;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_63 = 10380409671385728102;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_63 = 10380409671385728102;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_63 = 10380409671385728102;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_POUND_NAME_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_63 {
                226587729178875444 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return -crate::src::lib::xmltok::XML_TOK_POUND_NAME_1;
    }

    pub(crate) unsafe fn normal_scanLit(
        mut open: ::core::ffi::c_int,
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut t: ::core::ffi::c_int = (*(enc as *const normal_encoding)).type_0
                [*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int;
            match t {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                12 | 13 => {
                    ptr = ptr.offset(1);
                    if !(t != open) {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            return -crate::src::lib::xmltok::XML_TOK_LITERAL_1;
                        }
                        *nextTokPtr = ptr;
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return crate::src::lib::xmltok::XML_TOK_LITERAL_1
                            }
                            _ => return crate::src::lib::xmltok::XML_TOK_INVALID_1,
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(1isize);
                }
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_prologTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        }
        if 1 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (1i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(1i32 - 1) as crate::__stddef_size_t_h::size_t;
                if n == 0 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        let mut current_block_124: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            12 => {
                return normal_scanLit(
                    crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int,
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            13 => {
                return normal_scanLit(
                    crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int,
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            2 => {
                ptr = ptr.offset(1);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    16 => {
                        return normal_scanDecl(enc, ptr.offset(1isize), end, nextTokPtr);
                    }
                    15 => {
                        return normal_scanPi(enc, ptr.offset(1isize), end, nextTokPtr);
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(1));
                        return crate::src::lib::xmltok::XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            9 => {
                if ptr.offset(1) == end {
                    *nextTokPtr = end;
                    return -crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
                }
                current_block_124 = 6405334113228567422;
            }
            21 | 10 => {
                current_block_124 = 6405334113228567422;
            }
            30 => {
                return normal_scanPercent(enc, ptr.offset(1isize), end, nextTokPtr);
            }
            35 => {
                *nextTokPtr = ptr.offset(1);
                return crate::src::lib::xmltok::XML_TOK_COMMA_1;
            }
            20 => {
                *nextTokPtr = ptr.offset(1);
                return crate::src::lib::xmltok::XML_TOK_OPEN_BRACKET_1;
            }
            4 => {
                ptr = ptr.offset(1);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1) as ::core::ffi::c_long)
                {
                    return -crate::src::lib::xmltok::XML_TOK_CLOSE_BRACKET_1;
                }
                if *ptr as ::core::ffi::c_int == 0x5d {
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(1) as ::core::ffi::c_int == 0x3e {
                        *nextTokPtr = ptr.offset((2i32 * 1) as isize);
                        return crate::src::lib::xmltok::XML_TOK_COND_SECT_CLOSE_1;
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_CLOSE_BRACKET_1;
            }
            31 => {
                *nextTokPtr = ptr.offset(1);
                return crate::src::lib::xmltok::XML_TOK_OPEN_PAREN_1;
            }
            32 => {
                ptr = ptr.offset(1);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1) as ::core::ffi::c_long)
                {
                    return -crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_1;
                }
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    33 => {
                        *nextTokPtr = ptr.offset(1);
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK_1;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(1);
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_QUESTION_1;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(1);
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_PLUS_1;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_1;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            36 => {
                *nextTokPtr = ptr.offset(1);
                return crate::src::lib::xmltok::XML_TOK_OR_1;
            }
            11 => {
                *nextTokPtr = ptr.offset(1);
                return crate::src::lib::xmltok::XML_TOK_DECL_CLOSE_1;
            }
            19 => {
                return normal_scanPoundName(enc, ptr.offset(1isize), end, nextTokPtr);
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(2);
                    tok = crate::src::lib::xmltok::XML_TOK_NAME;
                } else if (*(enc as *const normal_encoding))
                    .isName2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(2);
                    tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                } else {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_124 = 2956972668325154207;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(3);
                    tok = crate::src::lib::xmltok::XML_TOK_NAME;
                } else if (*(enc as *const normal_encoding))
                    .isName3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(3);
                    tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                } else {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_124 = 2956972668325154207;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(4);
                    tok = crate::src::lib::xmltok::XML_TOK_NAME;
                } else if (*(enc as *const normal_encoding))
                    .isName4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(4);
                    tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                } else {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_124 = 2956972668325154207;
            }
            22 | 24 => {
                tok = crate::src::lib::xmltok::XML_TOK_NAME;
                ptr = ptr.offset(1);
                current_block_124 = 2956972668325154207;
            }
            25 | 26 | 27 | 23 => {
                tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                ptr = ptr.offset(1);
                current_block_124 = 2956972668325154207;
            }
            29 | _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_124 {
            2956972668325154207 => {}
            _ => {
                loop {
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        break;
                    }
                    let mut current_block_32: u64;
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        21 | 10 => {
                            current_block_32 = 17500079516916021833;
                        }
                        9 => {
                            if ptr.offset(1) != end {
                                current_block_32 = 17500079516916021833;
                            } else {
                                current_block_32 = 3687018382384043009;
                            }
                        }
                        _ => {
                            current_block_32 = 3687018382384043009;
                        }
                    }
                    match current_block_32 {
                        17500079516916021833 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_210: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_210 = 17210391895989911948;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_210 = 17210391895989911948;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName2
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_210 = 14244298717249035578;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName3
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_210 = 14244298717249035578;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                        || (*(enc as *const normal_encoding))
                            .isName4
                            .expect("non-null function pointer")(enc, ptr)
                            == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_210 = 14244298717249035578;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(1);
                    match tok {
                        crate::src::lib::xmltok::XML_TOK_NAME => {
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 1) as ::core::ffi::c_long)
                            {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                            }
                            tok = crate::src::lib::xmltok::XML_TOK_PREFIXED_NAME;
                            let mut current_block_187: u64;
                            match (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                            {
                                29 => {
                                    if 0 == 0 {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    current_block_187 = 2692573546887820791;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    current_block_187 = 2692573546887820791;
                                }
                                5 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if (*(enc as *const normal_encoding))
                                        .isInvalid2
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) != 0
                                        || (*(enc as *const normal_encoding))
                                            .isName2
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(2);
                                    current_block_187 = 9812798724717783973;
                                }
                                6 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if (*(enc as *const normal_encoding))
                                        .isInvalid3
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) != 0
                                        || (*(enc as *const normal_encoding))
                                            .isName3
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(3);
                                    current_block_187 = 9812798724717783973;
                                }
                                7 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if (*(enc as *const normal_encoding))
                                        .isInvalid4
                                        .expect("non-null function pointer")(
                                        enc, ptr
                                    ) != 0
                                        || (*(enc as *const normal_encoding))
                                            .isName4
                                            .expect("non-null function pointer")(
                                            enc, ptr
                                        ) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(4);
                                    current_block_187 = 9812798724717783973;
                                }
                                _ => {
                                    tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                                    current_block_187 = 9812798724717783973;
                                }
                            }
                            match current_block_187 {
                                2692573546887820791 => {
                                    ptr = ptr.offset(1isize);
                                }
                                _ => {}
                            }
                        }
                        crate::src::lib::xmltok::XML_TOK_PREFIXED_NAME => {
                            tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                        }
                        _ => {}
                    }
                    current_block_210 = 14244298717249035578;
                }
                34 => {
                    if tok == crate::src::lib::xmltok::XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(1);
                    return crate::src::lib::xmltok::XML_TOK_NAME_PLUS_1;
                }
                33 => {
                    if tok == crate::src::lib::xmltok::XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(1);
                    return crate::src::lib::xmltok::XML_TOK_NAME_ASTERISK_1;
                }
                15 => {
                    if tok == crate::src::lib::xmltok::XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(1);
                    return crate::src::lib::xmltok::XML_TOK_NAME_QUESTION_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_210 {
                17210391895989911948 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return -tok;
    }

    pub(crate) unsafe fn normal_attributeValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1) as ::core::ffi::c_long)
        {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                }
                3 => {
                    if ptr == start {
                        return normal_scanRef(enc, ptr.offset(1isize), end, nextTokPtr);
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1);
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
                        }
                        if (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(1isize);
                        }
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1);
                        return crate::src::lib::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(1isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn normal_entityValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1) as ::core::ffi::c_long)
        {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                }
                3 => {
                    if ptr == start {
                        return normal_scanRef(enc, ptr.offset(1isize), end, nextTokPtr);
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int =
                            normal_scanPercent(enc, ptr.offset(1), end, nextTokPtr);
                        return if tok == crate::src::lib::xmltok::XML_TOK_PERCENT_1 {
                            crate::src::lib::xmltok::XML_TOK_INVALID_1
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1);
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
                        }
                        if (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(1isize);
                        }
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(1isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn normal_ignoreSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut level: ::core::ffi::c_int = 0;
        if 1 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (1i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(1i32 - 1) as crate::__stddef_size_t_h::size_t;
                end = ptr.offset(n as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                2 => {
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr as ::core::ffi::c_int == 0x21 {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr as ::core::ffi::c_int == 0x5b {
                            level += 1;
                            ptr = ptr.offset(1isize);
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(1);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr as ::core::ffi::c_int == 0x5d {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr as ::core::ffi::c_int == 0x3e {
                            ptr = ptr.offset(1);
                            if level == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_IGNORE_SECT_1;
                            }
                            level -= 1;
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(1isize);
                }
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn normal_isPublicId(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        ptr = ptr.offset(1);
        end = end.offset(-(1));
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            let mut current_block_8: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 5143058163439228106;
                }
                21 => {
                    if *ptr as ::core::ffi::c_int == 0x9 {
                        *badPtr = ptr;
                        return 0i32;
                    }
                    current_block_8 = 5143058163439228106;
                }
                26 | 22 => {
                    if *ptr as ::core::ffi::c_int & !(0x7f) == 0 {
                        current_block_8 = 5143058163439228106;
                    } else {
                        current_block_8 = 10293610489198370764;
                    }
                }
                _ => {
                    current_block_8 = 10293610489198370764;
                }
            }
            match current_block_8 {
                10293610489198370764 => match *ptr as ::core::ffi::c_int {
                    36 | 64 => {}
                    _ => {
                        *badPtr = ptr;
                        return 0i32;
                    }
                },
                _ => {}
            }
            ptr = ptr.offset(1);
        }
        return 1;
    }

    pub(crate) unsafe fn normal_getAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut attsMax: ::core::ffi::c_int,
        mut atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        let mut state: crate::xmltok_impl_h::C2RustUnnamed_3 = crate::xmltok_impl_c::inName;
        let mut nAtts: ::core::ffi::c_int = 0;
        let mut open: ::core::ffi::c_int = 0;
        ptr = ptr.offset(1);
        loop {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    if state == crate::xmltok_impl_c::other {
                        if nAtts < attsMax {
                            let ref mut fresh10 = (*atts.offset(nAtts as isize)).name;
                            *fresh10 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName;
                    }
                    ptr = ptr.offset((2i32 - 1i32) as isize);
                }
                6 => {
                    if state == crate::xmltok_impl_c::other {
                        if nAtts < attsMax {
                            let ref mut fresh11 = (*atts.offset(nAtts as isize)).name;
                            *fresh11 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName;
                    }
                    ptr = ptr.offset((3i32 - 1i32) as isize);
                }
                7 => {
                    if state == crate::xmltok_impl_c::other {
                        if nAtts < attsMax {
                            let ref mut fresh12 = (*atts.offset(nAtts as isize)).name;
                            *fresh12 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName;
                    }
                    ptr = ptr.offset((4i32 - 1i32) as isize);
                }
                29 | 22 | 24 => {
                    if state == crate::xmltok_impl_c::other {
                        if nAtts < attsMax {
                            let ref mut fresh13 = (*atts.offset(nAtts as isize)).name;
                            *fresh13 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName;
                    }
                }
                12 => {
                    if state != crate::xmltok_impl_c::inValue {
                        if nAtts < attsMax {
                            let ref mut fresh14 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh14 = ptr.offset(1isize);
                        }
                        state = crate::xmltok_impl_c::inValue;
                        open = crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int;
                    } else if open == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other;
                        if nAtts < attsMax {
                            let ref mut fresh15 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh15 = ptr;
                        }
                        nAtts += 1;
                    }
                }
                13 => {
                    if state != crate::xmltok_impl_c::inValue {
                        if nAtts < attsMax {
                            let ref mut fresh16 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh16 = ptr.offset(1isize);
                        }
                        state = crate::xmltok_impl_c::inValue;
                        open = crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int;
                    } else if open == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other;
                        if nAtts < attsMax {
                            let ref mut fresh17 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh17 = ptr;
                        }
                        nAtts += 1;
                    }
                }
                3 => {
                    if nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                21 => {
                    if state == crate::xmltok_impl_c::inName {
                        state = crate::xmltok_impl_c::other;
                    } else if state == crate::xmltok_impl_c::inValue
                        && nAtts < attsMax
                        && (*atts.offset(nAtts as isize)).normalized as ::core::ffi::c_int != 0
                        && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                            || *ptr as ::core::ffi::c_int != crate::ascii_h::ASCII_SPACE
                            || *ptr.offset(1) as ::core::ffi::c_int == crate::ascii_h::ASCII_SPACE
                            || (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                                == open)
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                9 | 10 => {
                    if state == crate::xmltok_impl_c::inName {
                        state = crate::xmltok_impl_c::other;
                    } else if state == crate::xmltok_impl_c::inValue && nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                11 | 17 => {
                    if state != crate::xmltok_impl_c::inValue {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(1);
        }
    }

    pub(crate) unsafe fn normal_charRefNumber(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0;
        ptr = ptr.offset((2i32 * 1) as isize);
        if *ptr as ::core::ffi::c_int == 0x78 {
            ptr = ptr.offset(1);
            while !(*ptr as ::core::ffi::c_int == 0x3b) {
                let mut c: ::core::ffi::c_int = *ptr as ::core::ffi::c_int;
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
                        result <<= 4;
                        result |= c - crate::ascii_h::ASCII_0;
                    }
                    crate::ascii_h::ASCII_A
                    | crate::ascii_h::ASCII_B_1
                    | crate::ascii_h::ASCII_C
                    | crate::ascii_h::ASCII_D
                    | crate::ascii_h::ASCII_E_1
                    | crate::ascii_h::ASCII_F_1 => {
                        result <<= 4;
                        result += 10i32 + (c - crate::ascii_h::ASCII_A);
                    }
                    crate::ascii_h::ASCII_a_1
                    | crate::ascii_h::ASCII_b
                    | crate::ascii_h::ASCII_c_1
                    | crate::ascii_h::ASCII_d
                    | crate::ascii_h::ASCII_e_1
                    | crate::ascii_h::ASCII_f => {
                        result <<= 4;
                        result += 10i32 + (c - crate::ascii_h::ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.offset(1);
            }
        } else {
            while !(*ptr as ::core::ffi::c_int == 0x3b) {
                let mut c_0: ::core::ffi::c_int = *ptr as ::core::ffi::c_int;
                result *= 10;
                result += c_0 - crate::ascii_h::ASCII_0;
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.offset(1);
            }
        }
        return checkCharRefNumber(result);
    }

    pub(crate) unsafe fn normal_predefinedEntityName(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        match end.offset_from(ptr) as ::core::ffi::c_long / 1 {
            2 => {
                if *ptr.offset(1) as ::core::ffi::c_int == 0x74 {
                    match *ptr as ::core::ffi::c_int {
                        crate::ascii_h::ASCII_l_1 => return crate::ascii_h::ASCII_LT,
                        crate::ascii_h::ASCII_g_1 => return crate::ascii_h::ASCII_GT,
                        _ => {}
                    }
                }
            }
            3 => {
                if *ptr as ::core::ffi::c_int == 0x61 {
                    ptr = ptr.offset(1);
                    if *ptr as ::core::ffi::c_int == 0x6d {
                        ptr = ptr.offset(1);
                        if *ptr as ::core::ffi::c_int == 0x70 {
                            return crate::ascii_h::ASCII_AMP;
                        }
                    }
                }
            }
            4 => match *ptr as ::core::ffi::c_int {
                crate::ascii_h::ASCII_q => {
                    ptr = ptr.offset(1);
                    if *ptr as ::core::ffi::c_int == 0x75 {
                        ptr = ptr.offset(1);
                        if *ptr as ::core::ffi::c_int == 0x6f {
                            ptr = ptr.offset(1);
                            if *ptr as ::core::ffi::c_int == 0x74 {
                                return crate::ascii_h::ASCII_QUOT;
                            }
                        }
                    }
                }
                crate::ascii_h::ASCII_a_1 => {
                    ptr = ptr.offset(1);
                    if *ptr as ::core::ffi::c_int == 0x70 {
                        ptr = ptr.offset(1);
                        if *ptr as ::core::ffi::c_int == 0x6f {
                            ptr = ptr.offset(1);
                            if *ptr as ::core::ffi::c_int == 0x73 {
                                return crate::ascii_h::ASCII_APOS;
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
        return 0;
    }

    pub(crate) unsafe fn normal_nameMatchesAscii(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr1: *const ::core::ffi::c_char,
        mut end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while *ptr2 != 0 {
            if (end1.offset_from(ptr1) as ::core::ffi::c_long) < 1 {
                return 0i32;
            }
            if !(*ptr1 as ::core::ffi::c_int == *ptr2 as ::core::ffi::c_int) {
                return 0i32;
            }
            ptr1 = ptr1.offset(1);
            ptr2 = ptr2.offset(1);
        }
        return (ptr1 == end1) as ::core::ffi::c_int;
    }

    pub(crate) unsafe fn normal_nameLength(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                }
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {
                    return ptr.offset_from(start) as ::core::ffi::c_int;
                }
            }
        }
    }

    pub(crate) unsafe fn normal_skipS(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        loop {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                10 | 9 | 21 => {
                    ptr = ptr.offset(1isize);
                }
                _ => return ptr,
            }
        }
    }

    pub(crate) unsafe fn normal_updatePosition(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut pos: *mut crate::src::lib::xmltok::POSITION,
    ) {
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 1) as ::core::ffi::c_long {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    ptr = ptr.offset(2);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.offset(3);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.offset(4);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                10 => {
                    (*pos).columnNumber = 0u64;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(1isize);
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(1);
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1) as ::core::ffi::c_long
                        && (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(1isize);
                    }
                    (*pos).columnNumber = 0u64;
                }
                _ => {
                    ptr = ptr.offset(1);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub(crate) unsafe fn little2_scanComment(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            if !(*ptr.offset(1) as ::core::ffi::c_int == 0
                && *ptr.offset(0) as ::core::ffi::c_int == 0x2d)
            {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.offset(2);
            while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                } {
                    5 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    6 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    7 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    27 => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(1) as ::core::ffi::c_int == 0
                            && *ptr.offset(0) as ::core::ffi::c_int == 0x2d
                        {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 2) as ::core::ffi::c_long)
                            {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr.offset(1) as ::core::ffi::c_int == 0
                                && *ptr.offset(0) as ::core::ffi::c_int == 0x3e)
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            return crate::src::lib::xmltok::XML_TOK_COMMENT_1;
                        }
                    }
                    _ => {
                        ptr = ptr.offset(2isize);
                    }
                }
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_scanDecl(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            27 => {
                return little2_scanComment(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            20 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_COND_SECT_OPEN_1;
            }
            22 | 24 => {
                ptr = ptr.offset(2isize);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            's_129: {
                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                } {
                    30 => {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (2i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        match if *ptr.offset(2).offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(2) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(2).offset(1), *ptr.offset(2).offset(0))
                        } {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(2);
                        break 's_129;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_DECL_OPEN_1;
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_checkPiTarget(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut tokPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0;
        *tokPtr = crate::src::lib::xmltok::XML_TOK_PI_1;
        if end.offset_from(ptr) as ::core::ffi::c_long != (2i32 * 3) as ::core::ffi::c_long {
            return 1i32;
        }
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            *ptr.offset(0) as ::core::ffi::c_int
        } else {
            -(1)
        } {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1i32;
            }
            _ => return 1,
        }
        ptr = ptr.offset(2);
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            *ptr.offset(0) as ::core::ffi::c_int
        } else {
            -(1)
        } {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1i32;
            }
            _ => return 1,
        }
        ptr = ptr.offset(2);
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            *ptr.offset(0) as ::core::ffi::c_int
        } else {
            -(1)
        } {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1i32;
            }
            _ => return 1,
        }
        if upper != 0 {
            return 0i32;
        }
        *tokPtr = crate::src::lib::xmltok::XML_TOK_XML_DECL_1;
        return 1;
    }

    pub(crate) unsafe fn little2_scanPi(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        let mut target: *const ::core::ffi::c_char = ptr;
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_32 = 14358794669692889688;
            }
            22 | 24 => {
                current_block_32 = 14358794669692889688;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_32 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_32 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_32 = 14763689060501151050;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_32 {
            14358794669692889688 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_118: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_118 = 15890151712677504458;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_118 = 15890151712677504458;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_118 = 13349765058737954042;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_118 = 13349765058737954042;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_118 = 13349765058737954042;
                }
                21 | 9 | 10 => {
                    if little2_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long
                    {
                        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        } {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            15 => {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2) as ::core::ffi::c_long)
                                {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(1) as ::core::ffi::c_int == 0
                                    && *ptr.offset(0) as ::core::ffi::c_int == 0x3e
                                {
                                    *nextTokPtr = ptr.offset(2);
                                    return tok;
                                }
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                15 => {
                    if little2_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(1) as ::core::ffi::c_int == 0
                        && *ptr.offset(0) as ::core::ffi::c_int == 0x3e
                    {
                        *nextTokPtr = ptr.offset(2);
                        return tok;
                    }
                    current_block_118 = 7312756018063861309;
                }
                _ => {
                    current_block_118 = 7312756018063861309;
                }
            }
            match current_block_118 {
                7312756018063861309 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                15890151712677504458 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_scanCdataSection(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        pub static CDATA_LSQB: [::core::ffi::c_char; 6] = [
            crate::ascii_h::ASCII_C as ::core::ffi::c_char,
            crate::ascii_h::ASCII_D as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_T as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_LSQB as ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (6i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        i = 0;
        while i < 6 {
            if !(*ptr.offset(1) as ::core::ffi::c_int == 0
                && *ptr.offset(0) as ::core::ffi::c_int
                    == CDATA_LSQB[i as usize] as ::core::ffi::c_int)
            {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            i += 1;
            ptr = ptr.offset(2);
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    }

    pub(crate) unsafe fn little2_cdataSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        }
        if 2 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (2i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(2i32 - 1) as crate::__stddef_size_t_h::size_t;
                if n == 0 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            4 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                if *ptr.offset(1) as ::core::ffi::c_int == 0
                    && *ptr.offset(0) as ::core::ffi::c_int == 0x5d
                {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr.offset(1) as ::core::ffi::c_int == 0
                        && *ptr.offset(0) as ::core::ffi::c_int == 0x3e)
                    {
                        ptr = ptr.offset(-(2isize));
                    } else {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CDATA_SECT_CLOSE_1;
                    }
                }
            }
            9 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                if (if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2isize);
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(2isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn little2_scanEndTag(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_32 = 8654814784450400207;
            }
            22 | 24 => {
                current_block_32 = 8654814784450400207;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_32 {
            8654814784450400207 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_73: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_73 = 16411184819389759620;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_73 = 16411184819389759620;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_73 = 981995395831942902;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_73 = 981995395831942902;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_73 = 981995395831942902;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long
                    {
                        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        } {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(2);
                                return crate::src::lib::xmltok::XML_TOK_END_TAG_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(2);
                    }
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                23 => {
                    ptr = ptr.offset(2);
                    current_block_73 = 981995395831942902;
                }
                11 => {
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_END_TAG_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_73 {
                16411184819389759620 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_scanHexCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2);
            while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                } {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2);
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_scanCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            if *ptr.offset(1) as ::core::ffi::c_int == 0
                && *ptr.offset(0) as ::core::ffi::c_int == 0x78
            {
                return little2_scanHexCharRef(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2);
            while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                } {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2);
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_scanRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_33: u64;
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_33 = 6679362556518655255;
            }
            22 | 24 => {
                current_block_33 = 6679362556518655255;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_33 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_33 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_33 = 14763689060501151050;
            }
            19 => {
                return little2_scanCharRef(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_33 {
            6679362556518655255 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_64: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_64 = 405996089697802199;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_64 = 405996089697802199;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_64 = 10930818133215224067;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_64 = 10930818133215224067;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_64 = 10930818133215224067;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_64 {
                405996089697802199 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_scanAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_186: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_186 = 17747718632989559416;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_186 = 17747718632989559416;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_186 = 1634947208139838470;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_186 = 1634947208139838470;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_186 = 1634947208139838470;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    hadColon = 1;
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    let mut current_block_64: u64;
                    match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int)
                                << 3)
                                + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int
                                    >> 5)) as usize]
                                & (1)
                                    << (*ptr.offset(0) as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            current_block_64 = 12531724302225488581;
                        }
                        22 | 24 => {
                            current_block_64 = 12531724302225488581;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_64 = 10930818133215224067;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_64 = 10930818133215224067;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_64 = 10930818133215224067;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_64 {
                        12531724302225488581 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                    current_block_186 = 1634947208139838470;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: ::core::ffi::c_int = 0;
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        t = if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        };
                        if t == crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                    }
                    current_block_186 = 10853015579903106591;
                }
                14 => {
                    current_block_186 = 10853015579903106591;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_186 {
                10853015579903106591 => {
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0;
                    loop {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        open = if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        };
                        if open == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                            || open == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                    }
                    ptr = ptr.offset(2);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        t_0 = if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        };
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int =
                                    little2_scanRef(enc, ptr.offset(2), end, &raw mut ptr);
                                if tok <= 0 {
                                    if tok == crate::src::lib::xmltok::XML_TOK_INVALID_1 {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                    } {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2) as ::core::ffi::c_long)
                                {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                                }
                                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                                    (*(enc as *const normal_encoding)).type_0
                                        [*ptr as ::core::ffi::c_uchar as usize]
                                        as ::core::ffi::c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                                } {
                                    29 => {
                                        if namingBitmap[(((nmstrtPages
                                            [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                            as ::core::ffi::c_int)
                                            << 3)
                                            + (*ptr.offset(0) as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                >> 5))
                                            as usize]
                                            & (1)
                                                << (*ptr.offset(0) as ::core::ffi::c_uchar
                                                    as ::core::ffi::c_int
                                                    & 0x1f)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        current_block_186 = 923465642386550266;
                                        break;
                                    }
                                    22 | 24 => {
                                        current_block_186 = 923465642386550266;
                                        break;
                                    }
                                    5 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if 0 != 0 || 0 == 0 {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2);
                                        current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if 0 != 0 || 0 == 0 {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3);
                                        current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if 0 != 0 || 0 == 0 {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4);
                                        current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        current_block_186 = 15103464935601583148;
                                        break;
                                    }
                                    17 => {
                                        current_block_186 = 619033562305054167;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            match current_block_186 {
                                15103464935601583148 => {}
                                619033562305054167 => {}
                                1634947208139838470 => {}
                                _ => {
                                    ptr = ptr.offset(2);
                                    current_block_186 = 1634947208139838470;
                                }
                            }
                        }
                        17 => {
                            current_block_186 = 619033562305054167;
                        }
                        11 => {
                            current_block_186 = 15103464935601583148;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_186 {
                        1634947208139838470 => {}
                        _ => match current_block_186 {
                            619033562305054167 => {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2) as ::core::ffi::c_long)
                                {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                                }
                                if !(*ptr.offset(1) as ::core::ffi::c_int == 0
                                    && *ptr.offset(0) as ::core::ffi::c_int == 0x3e)
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                *nextTokPtr = ptr.offset(2);
                                return crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(2);
                                return crate::src::lib::xmltok::XML_TOK_START_TAG_WITH_ATTS_1;
                            }
                        },
                    }
                }
                17747718632989559416 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_scanLt(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_45: u64;
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_45 = 18046087305847344724;
            }
            22 | 24 => {
                current_block_45 = 18046087305847344724;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_45 = 8180496224585318153;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_45 = 8180496224585318153;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_45 = 8180496224585318153;
            }
            16 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                } {
                    27 => {
                        return little2_scanComment(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    20 => {
                        return little2_scanCdataSection(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            15 => {
                return little2_scanPi(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            17 => {
                return little2_scanEndTag(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_45 {
            18046087305847344724 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        hadColon = 0;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_161: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_161 = 8998928240368606981;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_161 = 8998928240368606981;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_161 = 14714495436747744489;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_161 = 14714495436747744489;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_161 = 14714495436747744489;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    hadColon = 1;
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    let mut current_block_112: u64;
                    match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int)
                                << 3)
                                + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int
                                    >> 5)) as usize]
                                & (1)
                                    << (*ptr.offset(0) as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            current_block_112 = 14391208795021697965;
                        }
                        22 | 24 => {
                            current_block_112 = 14391208795021697965;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_112 = 2616667235040759262;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_112 = 2616667235040759262;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_112 = 2616667235040759262;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_112 {
                        14391208795021697965 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                    current_block_161 = 14714495436747744489;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2);
                    loop {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            current_block_161 = 13215501469961642988;
                            break;
                        }
                        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        } {
                            29 => {
                                if namingBitmap[(((nmstrtPages
                                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int)
                                    << 3)
                                    + (*ptr.offset(0) as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        >> 5))
                                    as usize]
                                    & (1)
                                        << (*ptr.offset(0) as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            & 0x1f)
                                    == 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                current_block_161 = 2369392326157537288;
                            }
                            22 | 24 => {
                                current_block_161 = 2369392326157537288;
                            }
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if 0 != 0 || 0 == 0 {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2);
                                current_block_161 = 16314074004867283505;
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if 0 != 0 || 0 == 0 {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3);
                                current_block_161 = 16314074004867283505;
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if 0 != 0 || 0 == 0 {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4);
                                current_block_161 = 16314074004867283505;
                            }
                            11 => {
                                current_block_161 = 1918622160084604696;
                                break;
                            }
                            17 => {
                                current_block_161 = 1114269873380682160;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(2);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                        match current_block_161 {
                            2369392326157537288 => {
                                ptr = ptr.offset(2isize);
                            }
                            _ => {}
                        }
                        return little2_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match current_block_161 {
                        1918622160084604696 => {}
                        1114269873380682160 => {}
                        _ => return crate::src::lib::xmltok::XML_TOK_PARTIAL_1,
                    }
                }
                11 => {
                    current_block_161 = 1918622160084604696;
                }
                17 => {
                    current_block_161 = 1114269873380682160;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_161 {
                1114269873380682160 => {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr.offset(1) as ::core::ffi::c_int == 0
                        && *ptr.offset(0) as ::core::ffi::c_int == 0x3e)
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                }
                1918622160084604696 => {
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_START_TAG_NO_ATTS_1;
                }
                8998928240368606981 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_contentTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        }
        if 2 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (2i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(2i32 - 1) as crate::__stddef_size_t_h::size_t;
                if n == 0 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            2 => {
                return little2_scanLt(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            3 => {
                return little2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            9 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
                }
                if (if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2isize);
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            4 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_TRAILING_RSQB_1;
                }
                if *ptr.offset(1) as ::core::ffi::c_int == 0
                    && *ptr.offset(0) as ::core::ffi::c_int == 0x5d
                {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_TRAILING_RSQB_1;
                    }
                    if !(*ptr.offset(1) as ::core::ffi::c_int == 0
                        && *ptr.offset(0) as ::core::ffi::c_int == 0x3e)
                    {
                        ptr = ptr.offset(-(2isize));
                    } else {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(2isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_76: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_76 = 7158658067966855297;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_76 = 7158658067966855297;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_76 = 7158658067966855297;
                }
                4 => {
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 2) as ::core::ffi::c_long
                    {
                        if !(*ptr.offset(2).offset(1) as ::core::ffi::c_int == 0
                            && *ptr.offset(2).offset(0) as ::core::ffi::c_int == 0x5d)
                        {
                            ptr = ptr.offset(2);
                            current_block_76 = 7158658067966855297;
                        } else if end.offset_from(ptr) as ::core::ffi::c_long
                            >= (3i32 * 2) as ::core::ffi::c_long
                        {
                            if !(*ptr.offset((2i32 * 2) as isize).offset(1) as ::core::ffi::c_int
                                == 0
                                && *ptr.offset((2i32 * 2) as isize).offset(0) as ::core::ffi::c_int
                                    == 0x3e)
                            {
                                ptr = ptr.offset(2isize);
                            } else {
                                *nextTokPtr = ptr.offset((2i32 * 2) as isize);
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            current_block_76 = 7158658067966855297;
                        } else {
                            current_block_76 = 17804070343020517427;
                        }
                    } else {
                        current_block_76 = 17804070343020517427;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    current_block_76 = 17804070343020517427;
                }
                _ => {
                    ptr = ptr.offset(2);
                    current_block_76 = 7158658067966855297;
                }
            }
            match current_block_76 {
                7158658067966855297 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn little2_scanPercent(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_34: u64;
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_34 = 27123471380826226;
            }
            22 | 24 => {
                current_block_34 = 27123471380826226;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_34 = 4761528863920922185;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_34 = 4761528863920922185;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_34 = 4761528863920922185;
            }
            21 | 10 | 9 | 30 => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_PERCENT_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_34 {
            27123471380826226 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_65: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_65 = 8394962855094477842;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_65 = 8394962855094477842;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_65 = 16415152177862271243;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_65 = 16415152177862271243;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_65 = 16415152177862271243;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_PARAM_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_65 {
                8394962855094477842 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_scanPoundName(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_32 = 14940290876465470105;
            }
            22 | 24 => {
                current_block_32 = 14940290876465470105;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_32 {
            14940290876465470105 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_63: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_63 = 11497795575834122789;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_63 = 11497795575834122789;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_63 = 10380409671385728102;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_63 = 10380409671385728102;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_63 = 10380409671385728102;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_POUND_NAME_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_63 {
                11497795575834122789 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return -crate::src::lib::xmltok::XML_TOK_POUND_NAME_1;
    }

    pub(crate) unsafe fn little2_scanLit(
        mut open: ::core::ffi::c_int,
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut t: ::core::ffi::c_int = if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            };
            match t {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                12 | 13 => {
                    ptr = ptr.offset(2);
                    if !(t != open) {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return -crate::src::lib::xmltok::XML_TOK_LITERAL_1;
                        }
                        *nextTokPtr = ptr;
                        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        } {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return crate::src::lib::xmltok::XML_TOK_LITERAL_1
                            }
                            _ => return crate::src::lib::xmltok::XML_TOK_INVALID_1,
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_prologTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        }
        if 2 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (2i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(2i32 - 1) as crate::__stddef_size_t_h::size_t;
                if n == 0 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        let mut current_block_124: u64;
        match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
        } {
            12 => {
                return little2_scanLit(
                    crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            13 => {
                return little2_scanLit(
                    crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            2 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                } {
                    16 => {
                        return little2_scanDecl(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    15 => {
                        return little2_scanPi(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(2));
                        return crate::src::lib::xmltok::XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            9 => {
                if ptr.offset(2) == end {
                    *nextTokPtr = end;
                    return -crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
                }
                current_block_124 = 17513858719706519675;
            }
            21 | 10 => {
                current_block_124 = 17513858719706519675;
            }
            30 => {
                return little2_scanPercent(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            35 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_COMMA_1;
            }
            20 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_OPEN_BRACKET_1;
            }
            4 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return -crate::src::lib::xmltok::XML_TOK_CLOSE_BRACKET_1;
                }
                if *ptr.offset(1) as ::core::ffi::c_int == 0
                    && *ptr.offset(0) as ::core::ffi::c_int == 0x5d
                {
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(2).offset(1) as ::core::ffi::c_int == 0
                        && *ptr.offset(2).offset(0) as ::core::ffi::c_int == 0x3e
                    {
                        *nextTokPtr = ptr.offset((2i32 * 2) as isize);
                        return crate::src::lib::xmltok::XML_TOK_COND_SECT_CLOSE_1;
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_CLOSE_BRACKET_1;
            }
            31 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_OPEN_PAREN_1;
            }
            32 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return -crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_1;
                }
                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                } {
                    33 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK_1;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_QUESTION_1;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_PLUS_1;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_1;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            36 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_OR_1;
            }
            11 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_DECL_CLOSE_1;
            }
            19 => {
                return little2_scanPoundName(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            22 | 24 => {
                tok = crate::src::lib::xmltok::XML_TOK_NAME;
                ptr = ptr.offset(2);
                current_block_124 = 2956972668325154207;
            }
            25 | 26 | 27 | 23 => {
                tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                ptr = ptr.offset(2);
                current_block_124 = 2956972668325154207;
            }
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    != 0
                {
                    ptr = ptr.offset(2);
                    tok = crate::src::lib::xmltok::XML_TOK_NAME;
                    current_block_124 = 2956972668325154207;
                } else if namingBitmap[(((namePages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    != 0
                {
                    ptr = ptr.offset(2);
                    tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                    current_block_124 = 2956972668325154207;
                } else {
                    current_block_124 = 2543942683527618915;
                }
            }
            _ => {
                current_block_124 = 2543942683527618915;
            }
        }
        match current_block_124 {
            2956972668325154207 => {}
            17513858719706519675 => {
                loop {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        break;
                    }
                    let mut current_block_32: u64;
                    match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                    } {
                        21 | 10 => {
                            current_block_32 = 17500079516916021833;
                        }
                        9 => {
                            if ptr.offset(2) != end {
                                current_block_32 = 17500079516916021833;
                            } else {
                                current_block_32 = 17471725448347076649;
                            }
                        }
                        _ => {
                            current_block_32 = 17471725448347076649;
                        }
                    }
                    match current_block_32 {
                        17500079516916021833 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_210: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(0) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_210 = 786388639404123072;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_210 = 786388639404123072;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_210 = 14244298717249035578;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_210 = 14244298717249035578;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_210 = 14244298717249035578;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(2);
                    match tok {
                        crate::src::lib::xmltok::XML_TOK_NAME => {
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 2) as ::core::ffi::c_long)
                            {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                            }
                            tok = crate::src::lib::xmltok::XML_TOK_PREFIXED_NAME;
                            let mut current_block_187: u64;
                            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                                (*(enc as *const normal_encoding)).type_0
                                    [*ptr as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                            } {
                                29 => {
                                    if namingBitmap[(((namePages
                                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                        as ::core::ffi::c_int)
                                        << 3)
                                        + (*ptr.offset(0) as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            >> 5))
                                        as usize]
                                        & (1)
                                            << (*ptr.offset(0) as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                & 0x1f)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    current_block_187 = 16869951820887225088;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    current_block_187 = 16869951820887225088;
                                }
                                5 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if 0 != 0 || 0 == 0 {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(2);
                                    current_block_187 = 9812798724717783973;
                                }
                                6 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if 0 != 0 || 0 == 0 {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(3);
                                    current_block_187 = 9812798724717783973;
                                }
                                7 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if 0 != 0 || 0 == 0 {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(4);
                                    current_block_187 = 9812798724717783973;
                                }
                                _ => {
                                    tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                                    current_block_187 = 9812798724717783973;
                                }
                            }
                            match current_block_187 {
                                16869951820887225088 => {
                                    ptr = ptr.offset(2isize);
                                }
                                _ => {}
                            }
                        }
                        crate::src::lib::xmltok::XML_TOK_PREFIXED_NAME => {
                            tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                        }
                        _ => {}
                    }
                    current_block_210 = 14244298717249035578;
                }
                34 => {
                    if tok == crate::src::lib::xmltok::XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_NAME_PLUS_1;
                }
                33 => {
                    if tok == crate::src::lib::xmltok::XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_NAME_ASTERISK_1;
                }
                15 => {
                    if tok == crate::src::lib::xmltok::XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_NAME_QUESTION_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_210 {
                786388639404123072 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return -tok;
    }

    pub(crate) unsafe fn little2_attributeValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2) as ::core::ffi::c_long)
        {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                5 => {
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                }
                3 => {
                    if ptr == start {
                        return little2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn little2_entityValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2) as ::core::ffi::c_long)
        {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                5 => {
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                }
                3 => {
                    if ptr == start {
                        return little2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int =
                            little2_scanPercent(enc, ptr.offset(2), end, nextTokPtr);
                        return if tok == crate::src::lib::xmltok::XML_TOK_PERCENT_1 {
                            crate::src::lib::xmltok::XML_TOK_INVALID_1
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn little2_ignoreSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut level: ::core::ffi::c_int = 0;
        if 2 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (2i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(2i32 - 1) as crate::__stddef_size_t_h::size_t;
                end = ptr.offset(n as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                2 => {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(1) as ::core::ffi::c_int == 0
                        && *ptr.offset(0) as ::core::ffi::c_int == 0x21
                    {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(1) as ::core::ffi::c_int == 0
                            && *ptr.offset(0) as ::core::ffi::c_int == 0x5b
                        {
                            level += 1;
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(1) as ::core::ffi::c_int == 0
                        && *ptr.offset(0) as ::core::ffi::c_int == 0x5d
                    {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(1) as ::core::ffi::c_int == 0
                            && *ptr.offset(0) as ::core::ffi::c_int == 0x3e
                        {
                            ptr = ptr.offset(2);
                            if level == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_IGNORE_SECT_1;
                            }
                            level -= 1;
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn little2_isPublicId(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        ptr = ptr.offset(2);
        end = end.offset(-(2));
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_8: u64;
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 5143058163439228106;
                }
                21 => {
                    if *ptr.offset(1) as ::core::ffi::c_int == 0
                        && *ptr.offset(0) as ::core::ffi::c_int == 0x9
                    {
                        *badPtr = ptr;
                        return 0i32;
                    }
                    current_block_8 = 5143058163439228106;
                }
                26 | 22 => {
                    if (if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                        *ptr.offset(0) as ::core::ffi::c_int
                    } else {
                        -(1)
                    }) & !(0x7f)
                        == 0
                    {
                        current_block_8 = 5143058163439228106;
                    } else {
                        current_block_8 = 5251475129761746025;
                    }
                }
                _ => {
                    current_block_8 = 5251475129761746025;
                }
            }
            match current_block_8 {
                5251475129761746025 => {
                    match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                        *ptr.offset(0) as ::core::ffi::c_int
                    } else {
                        -(1)
                    } {
                        36 | 64 => {}
                        _ => {
                            *badPtr = ptr;
                            return 0i32;
                        }
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2);
        }
        return 1;
    }

    pub(crate) unsafe fn little2_getAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut attsMax: ::core::ffi::c_int,
        mut atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        let mut state: crate::xmltok_impl_h::C2RustUnnamed_3 = crate::xmltok_impl_c::inName_0;
        let mut nAtts: ::core::ffi::c_int = 0;
        let mut open: ::core::ffi::c_int = 0;
        ptr = ptr.offset(2);
        loop {
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                5 => {
                    if state == crate::xmltok_impl_c::other_0 {
                        if nAtts < attsMax {
                            let ref mut fresh29 = (*atts.offset(nAtts as isize)).name;
                            *fresh29 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_0;
                    }
                    ptr = ptr.offset((2i32 - 2i32) as isize);
                }
                6 => {
                    if state == crate::xmltok_impl_c::other_0 {
                        if nAtts < attsMax {
                            let ref mut fresh30 = (*atts.offset(nAtts as isize)).name;
                            *fresh30 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_0;
                    }
                    ptr = ptr.offset((3i32 - 2i32) as isize);
                }
                7 => {
                    if state == crate::xmltok_impl_c::other_0 {
                        if nAtts < attsMax {
                            let ref mut fresh31 = (*atts.offset(nAtts as isize)).name;
                            *fresh31 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_0;
                    }
                    ptr = ptr.offset((4i32 - 2i32) as isize);
                }
                29 | 22 | 24 => {
                    if state == crate::xmltok_impl_c::other_0 {
                        if nAtts < attsMax {
                            let ref mut fresh32 = (*atts.offset(nAtts as isize)).name;
                            *fresh32 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_0;
                    }
                }
                12 => {
                    if state != crate::xmltok_impl_c::inValue_0 {
                        if nAtts < attsMax {
                            let ref mut fresh33 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh33 = ptr.offset(2isize);
                        }
                        state = crate::xmltok_impl_c::inValue_0;
                        open = crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int;
                    } else if open == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other_0;
                        if nAtts < attsMax {
                            let ref mut fresh34 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh34 = ptr;
                        }
                        nAtts += 1;
                    }
                }
                13 => {
                    if state != crate::xmltok_impl_c::inValue_0 {
                        if nAtts < attsMax {
                            let ref mut fresh35 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh35 = ptr.offset(2isize);
                        }
                        state = crate::xmltok_impl_c::inValue_0;
                        open = crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int;
                    } else if open == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other_0;
                        if nAtts < attsMax {
                            let ref mut fresh36 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh36 = ptr;
                        }
                        nAtts += 1;
                    }
                }
                3 => {
                    if nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                21 => {
                    if state == crate::xmltok_impl_c::inName_0 {
                        state = crate::xmltok_impl_c::other_0;
                    } else if state == crate::xmltok_impl_c::inValue_0
                        && nAtts < attsMax
                        && (*atts.offset(nAtts as isize)).normalized as ::core::ffi::c_int != 0
                        && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                            || (if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                                *ptr.offset(0) as ::core::ffi::c_int
                            } else {
                                -(1)
                            }) != crate::ascii_h::ASCII_SPACE
                            || (if *ptr.offset(2).offset(1) as ::core::ffi::c_int == 0 {
                                *ptr.offset(2).offset(0) as ::core::ffi::c_int
                            } else {
                                -(1)
                            }) == crate::ascii_h::ASCII_SPACE
                            || (if *ptr.offset(2).offset(1) as ::core::ffi::c_int == 0 {
                                (*(enc as *const normal_encoding)).type_0
                                    [*ptr.offset(2) as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(2).offset(1),
                                    *ptr.offset(2).offset(0),
                                )
                            }) == open)
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                9 | 10 => {
                    if state == crate::xmltok_impl_c::inName_0 {
                        state = crate::xmltok_impl_c::other_0;
                    } else if state == crate::xmltok_impl_c::inValue_0 && nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                11 | 17 => {
                    if state != crate::xmltok_impl_c::inValue_0 {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2);
        }
    }

    pub(crate) unsafe fn little2_charRefNumber(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0;
        ptr = ptr.offset((2i32 * 2) as isize);
        if *ptr.offset(1) as ::core::ffi::c_int == 0 && *ptr.offset(0) as ::core::ffi::c_int == 0x78
        {
            ptr = ptr.offset(2);
            while !(*ptr.offset(1) as ::core::ffi::c_int == 0
                && *ptr.offset(0) as ::core::ffi::c_int == 0x3b)
            {
                let mut c: ::core::ffi::c_int = if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    *ptr.offset(0) as ::core::ffi::c_int
                } else {
                    -(1)
                };
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
                        result <<= 4;
                        result |= c - crate::ascii_h::ASCII_0;
                    }
                    crate::ascii_h::ASCII_A
                    | crate::ascii_h::ASCII_B_1
                    | crate::ascii_h::ASCII_C
                    | crate::ascii_h::ASCII_D
                    | crate::ascii_h::ASCII_E_1
                    | crate::ascii_h::ASCII_F_1 => {
                        result <<= 4;
                        result += 10i32 + (c - crate::ascii_h::ASCII_A);
                    }
                    crate::ascii_h::ASCII_a_1
                    | crate::ascii_h::ASCII_b
                    | crate::ascii_h::ASCII_c_1
                    | crate::ascii_h::ASCII_d
                    | crate::ascii_h::ASCII_e_1
                    | crate::ascii_h::ASCII_f => {
                        result <<= 4;
                        result += 10i32 + (c - crate::ascii_h::ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.offset(2);
            }
        } else {
            while !(*ptr.offset(1) as ::core::ffi::c_int == 0
                && *ptr.offset(0) as ::core::ffi::c_int == 0x3b)
            {
                let mut c_0: ::core::ffi::c_int = if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    *ptr.offset(0) as ::core::ffi::c_int
                } else {
                    -(1)
                };
                result *= 10;
                result += c_0 - crate::ascii_h::ASCII_0;
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.offset(2);
            }
        }
        return checkCharRefNumber(result);
    }

    pub(crate) unsafe fn little2_predefinedEntityName(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        match end.offset_from(ptr) as ::core::ffi::c_long / 2 {
            2 => {
                if *ptr.offset(2).offset(1) as ::core::ffi::c_int == 0
                    && *ptr.offset(2).offset(0) as ::core::ffi::c_int == 0x74
                {
                    match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                        *ptr.offset(0) as ::core::ffi::c_int
                    } else {
                        -(1)
                    } {
                        crate::ascii_h::ASCII_l_1 => return crate::ascii_h::ASCII_LT,
                        crate::ascii_h::ASCII_g_1 => return crate::ascii_h::ASCII_GT,
                        _ => {}
                    }
                }
            }
            3 => {
                if *ptr.offset(1) as ::core::ffi::c_int == 0
                    && *ptr.offset(0) as ::core::ffi::c_int == 0x61
                {
                    ptr = ptr.offset(2);
                    if *ptr.offset(1) as ::core::ffi::c_int == 0
                        && *ptr.offset(0) as ::core::ffi::c_int == 0x6d
                    {
                        ptr = ptr.offset(2);
                        if *ptr.offset(1) as ::core::ffi::c_int == 0
                            && *ptr.offset(0) as ::core::ffi::c_int == 0x70
                        {
                            return crate::ascii_h::ASCII_AMP;
                        }
                    }
                }
            }
            4 => {
                match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                    *ptr.offset(0) as ::core::ffi::c_int
                } else {
                    -(1)
                } {
                    crate::ascii_h::ASCII_q => {
                        ptr = ptr.offset(2);
                        if *ptr.offset(1) as ::core::ffi::c_int == 0
                            && *ptr.offset(0) as ::core::ffi::c_int == 0x75
                        {
                            ptr = ptr.offset(2);
                            if *ptr.offset(1) as ::core::ffi::c_int == 0
                                && *ptr.offset(0) as ::core::ffi::c_int == 0x6f
                            {
                                ptr = ptr.offset(2);
                                if *ptr.offset(1) as ::core::ffi::c_int == 0
                                    && *ptr.offset(0) as ::core::ffi::c_int == 0x74
                                {
                                    return crate::ascii_h::ASCII_QUOT;
                                }
                            }
                        }
                    }
                    crate::ascii_h::ASCII_a_1 => {
                        ptr = ptr.offset(2);
                        if *ptr.offset(1) as ::core::ffi::c_int == 0
                            && *ptr.offset(0) as ::core::ffi::c_int == 0x70
                        {
                            ptr = ptr.offset(2);
                            if *ptr.offset(1) as ::core::ffi::c_int == 0
                                && *ptr.offset(0) as ::core::ffi::c_int == 0x6f
                            {
                                ptr = ptr.offset(2);
                                if *ptr.offset(1) as ::core::ffi::c_int == 0
                                    && *ptr.offset(0) as ::core::ffi::c_int == 0x73
                                {
                                    return crate::ascii_h::ASCII_APOS;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return 0;
    }

    pub(crate) unsafe fn little2_nameMatchesAscii(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr1: *const ::core::ffi::c_char,
        mut end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while *ptr2 != 0 {
            if (end1.offset_from(ptr1) as ::core::ffi::c_long) < 2 {
                return 0i32;
            }
            if !(*ptr1.offset(1) as ::core::ffi::c_int == 0
                && *ptr1.offset(0) as ::core::ffi::c_int == *ptr2 as ::core::ffi::c_int)
            {
                return 0i32;
            }
            ptr1 = ptr1.offset(2);
            ptr2 = ptr2.offset(1);
        }
        return (ptr1 == end1) as ::core::ffi::c_int;
    }

    pub(crate) unsafe fn little2_nameLength(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                5 => {
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                }
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {
                    return ptr.offset_from(start) as ::core::ffi::c_int;
                }
            }
        }
    }

    pub(crate) unsafe fn little2_skipS(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        loop {
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                10 | 9 | 21 => {
                    ptr = ptr.offset(2isize);
                }
                _ => return ptr,
            }
        }
    }

    pub(crate) unsafe fn little2_updatePosition(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut pos: *mut crate::src::lib::xmltok::POSITION,
    ) {
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
            } {
                5 => {
                    ptr = ptr.offset(2);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.offset(3);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.offset(4);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                10 => {
                    (*pos).columnNumber = 0u64;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize);
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2);
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long
                        && (if *ptr.offset(1) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(2isize);
                    }
                    (*pos).columnNumber = 0u64;
                }
                _ => {
                    ptr = ptr.offset(2);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub(crate) unsafe fn big2_scanComment(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            if !(*ptr.offset(0) as ::core::ffi::c_int == 0
                && *ptr.offset(1) as ::core::ffi::c_int == 0x2d)
            {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            ptr = ptr.offset(2);
            while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
                match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                } {
                    5 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    6 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    7 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    27 => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(0) as ::core::ffi::c_int == 0
                            && *ptr.offset(1) as ::core::ffi::c_int == 0x2d
                        {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 2) as ::core::ffi::c_long)
                            {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr.offset(0) as ::core::ffi::c_int == 0
                                && *ptr.offset(1) as ::core::ffi::c_int == 0x3e)
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            return crate::src::lib::xmltok::XML_TOK_COMMENT_1;
                        }
                    }
                    _ => {
                        ptr = ptr.offset(2isize);
                    }
                }
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_scanDecl(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            27 => {
                return big2_scanComment(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            20 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_COND_SECT_OPEN_1;
            }
            22 | 24 => {
                ptr = ptr.offset(2isize);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            's_129: {
                match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                } {
                    30 => {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (2i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        match if *ptr.offset(2).offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(2).offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(2).offset(0), *ptr.offset(2).offset(1))
                        } {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(2);
                        break 's_129;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_DECL_OPEN_1;
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_checkPiTarget(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut tokPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0;
        *tokPtr = crate::src::lib::xmltok::XML_TOK_PI_1;
        if end.offset_from(ptr) as ::core::ffi::c_long != (2i32 * 3) as ::core::ffi::c_long {
            return 1i32;
        }
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            *ptr.offset(1) as ::core::ffi::c_int
        } else {
            -(1)
        } {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1i32;
            }
            _ => return 1,
        }
        ptr = ptr.offset(2);
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            *ptr.offset(1) as ::core::ffi::c_int
        } else {
            -(1)
        } {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1i32;
            }
            _ => return 1,
        }
        ptr = ptr.offset(2);
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            *ptr.offset(1) as ::core::ffi::c_int
        } else {
            -(1)
        } {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1i32;
            }
            _ => return 1,
        }
        if upper != 0 {
            return 0i32;
        }
        *tokPtr = crate::src::lib::xmltok::XML_TOK_XML_DECL_1;
        return 1;
    }

    pub(crate) unsafe fn big2_scanPi(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        let mut target: *const ::core::ffi::c_char = ptr;
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_32 = 2802485987355401260;
            }
            22 | 24 => {
                current_block_32 = 2802485987355401260;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_32 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_32 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_32 = 14763689060501151050;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_32 {
            2802485987355401260 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_118: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_118 = 11190361564366887465;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_118 = 11190361564366887465;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_118 = 13349765058737954042;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_118 = 13349765058737954042;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_118 = 13349765058737954042;
                }
                21 | 9 | 10 => {
                    if big2_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long
                    {
                        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        } {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            15 => {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2) as ::core::ffi::c_long)
                                {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(0) as ::core::ffi::c_int == 0
                                    && *ptr.offset(1) as ::core::ffi::c_int == 0x3e
                                {
                                    *nextTokPtr = ptr.offset(2);
                                    return tok;
                                }
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                15 => {
                    if big2_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(0) as ::core::ffi::c_int == 0
                        && *ptr.offset(1) as ::core::ffi::c_int == 0x3e
                    {
                        *nextTokPtr = ptr.offset(2);
                        return tok;
                    }
                    current_block_118 = 161625824724629686;
                }
                _ => {
                    current_block_118 = 161625824724629686;
                }
            }
            match current_block_118 {
                161625824724629686 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                11190361564366887465 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_scanCdataSection(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        pub static CDATA_LSQB: [::core::ffi::c_char; 6] = [
            crate::ascii_h::ASCII_C as ::core::ffi::c_char,
            crate::ascii_h::ASCII_D as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_T as ::core::ffi::c_char,
            crate::ascii_h::ASCII_A as ::core::ffi::c_char,
            crate::ascii_h::ASCII_LSQB as ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (6i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        i = 0;
        while i < 6 {
            if !(*ptr.offset(0) as ::core::ffi::c_int == 0
                && *ptr.offset(1) as ::core::ffi::c_int
                    == CDATA_LSQB[i as usize] as ::core::ffi::c_int)
            {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            i += 1;
            ptr = ptr.offset(2);
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    }

    pub(crate) unsafe fn big2_cdataSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        }
        if 2 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (2i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(2i32 - 1) as crate::__stddef_size_t_h::size_t;
                if n == 0 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            4 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                if *ptr.offset(0) as ::core::ffi::c_int == 0
                    && *ptr.offset(1) as ::core::ffi::c_int == 0x5d
                {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr.offset(0) as ::core::ffi::c_int == 0
                        && *ptr.offset(1) as ::core::ffi::c_int == 0x3e)
                    {
                        ptr = ptr.offset(-(2isize));
                    } else {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CDATA_SECT_CLOSE_1;
                    }
                }
            }
            9 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                if (if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2isize);
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(2isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn big2_scanEndTag(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_32 = 12738221189273011712;
            }
            22 | 24 => {
                current_block_32 = 12738221189273011712;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_32 {
            12738221189273011712 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_73: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_73 = 1281007054303163758;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_73 = 1281007054303163758;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_73 = 981995395831942902;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_73 = 981995395831942902;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_73 = 981995395831942902;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long
                    {
                        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        } {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(2);
                                return crate::src::lib::xmltok::XML_TOK_END_TAG_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(2);
                    }
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                23 => {
                    ptr = ptr.offset(2);
                    current_block_73 = 981995395831942902;
                }
                11 => {
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_END_TAG_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_73 {
                1281007054303163758 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_scanHexCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2);
            while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
                match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                } {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2);
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_scanCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            if *ptr.offset(0) as ::core::ffi::c_int == 0
                && *ptr.offset(1) as ::core::ffi::c_int == 0x78
            {
                return big2_scanHexCharRef(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2);
            while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
                match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                } {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2);
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_scanRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_33: u64;
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_33 = 17794167657114565097;
            }
            22 | 24 => {
                current_block_33 = 17794167657114565097;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_33 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_33 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_33 = 14763689060501151050;
            }
            19 => {
                return big2_scanCharRef(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_33 {
            17794167657114565097 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_64: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_64 = 17251590314240005670;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_64 = 17251590314240005670;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_64 = 10930818133215224067;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_64 = 10930818133215224067;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_64 = 10930818133215224067;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_64 {
                17251590314240005670 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_scanAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_186: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_186 = 6092917267242331817;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_186 = 6092917267242331817;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_186 = 1634947208139838470;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_186 = 1634947208139838470;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_186 = 1634947208139838470;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    hadColon = 1;
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    let mut current_block_64: u64;
                    match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages
                                [*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int)
                                << 3)
                                + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int
                                    >> 5)) as usize]
                                & (1)
                                    << (*ptr.offset(1) as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            current_block_64 = 6604085902723260545;
                        }
                        22 | 24 => {
                            current_block_64 = 6604085902723260545;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_64 = 10930818133215224067;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_64 = 10930818133215224067;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_64 = 10930818133215224067;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_64 {
                        6604085902723260545 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                    current_block_186 = 1634947208139838470;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: ::core::ffi::c_int = 0;
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        t = if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        };
                        if t == crate::xmltok_impl_h::BT_EQUALS as ::core::ffi::c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                    }
                    current_block_186 = 10853015579903106591;
                }
                14 => {
                    current_block_186 = 10853015579903106591;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_186 {
                10853015579903106591 => {
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0;
                    loop {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        open = if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        };
                        if open == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int
                            || open == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                    }
                    ptr = ptr.offset(2);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        t_0 = if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        };
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int =
                                    big2_scanRef(enc, ptr.offset(2), end, &raw mut ptr);
                                if tok <= 0 {
                                    if tok == crate::src::lib::xmltok::XML_TOK_INVALID_1 {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                    } {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2) as ::core::ffi::c_long)
                                {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                                }
                                match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                                    (*(enc as *const normal_encoding)).type_0
                                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                        as ::core::ffi::c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                                } {
                                    29 => {
                                        if namingBitmap[(((nmstrtPages
                                            [*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                                            as ::core::ffi::c_int)
                                            << 3)
                                            + (*ptr.offset(1) as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                >> 5))
                                            as usize]
                                            & (1)
                                                << (*ptr.offset(1) as ::core::ffi::c_uchar
                                                    as ::core::ffi::c_int
                                                    & 0x1f)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        current_block_186 = 7794494472231011433;
                                        break;
                                    }
                                    22 | 24 => {
                                        current_block_186 = 7794494472231011433;
                                        break;
                                    }
                                    5 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if 0 != 0 || 0 == 0 {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2);
                                        current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if 0 != 0 || 0 == 0 {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3);
                                        current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if 0 != 0 || 0 == 0 {
                                            *nextTokPtr = ptr;
                                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4);
                                        current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        current_block_186 = 1783713129665224809;
                                        break;
                                    }
                                    17 => {
                                        current_block_186 = 18153789983347219713;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            match current_block_186 {
                                1783713129665224809 => {}
                                18153789983347219713 => {}
                                1634947208139838470 => {}
                                _ => {
                                    ptr = ptr.offset(2);
                                    current_block_186 = 1634947208139838470;
                                }
                            }
                        }
                        17 => {
                            current_block_186 = 18153789983347219713;
                        }
                        11 => {
                            current_block_186 = 1783713129665224809;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_186 {
                        1634947208139838470 => {}
                        _ => match current_block_186 {
                            18153789983347219713 => {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2) as ::core::ffi::c_long)
                                {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                                }
                                if !(*ptr.offset(0) as ::core::ffi::c_int == 0
                                    && *ptr.offset(1) as ::core::ffi::c_int == 0x3e)
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                *nextTokPtr = ptr.offset(2);
                                return crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(2);
                                return crate::src::lib::xmltok::XML_TOK_START_TAG_WITH_ATTS_1;
                            }
                        },
                    }
                }
                6092917267242331817 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_scanLt(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_45: u64;
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_45 = 6477200489819026004;
            }
            22 | 24 => {
                current_block_45 = 6477200489819026004;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_45 = 8180496224585318153;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_45 = 8180496224585318153;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_45 = 8180496224585318153;
            }
            16 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                } {
                    27 => {
                        return big2_scanComment(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    20 => {
                        return big2_scanCdataSection(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            15 => {
                return big2_scanPi(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            17 => {
                return big2_scanEndTag(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_45 {
            6477200489819026004 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        hadColon = 0;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_161: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_161 = 18151815167355992796;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_161 = 18151815167355992796;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_161 = 14714495436747744489;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_161 = 14714495436747744489;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_161 = 14714495436747744489;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    hadColon = 1;
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    let mut current_block_112: u64;
                    match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages
                                [*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int)
                                << 3)
                                + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int
                                    >> 5)) as usize]
                                & (1)
                                    << (*ptr.offset(1) as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            current_block_112 = 16337619596932156899;
                        }
                        22 | 24 => {
                            current_block_112 = 16337619596932156899;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_112 = 2616667235040759262;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_112 = 2616667235040759262;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                            }
                            if 0 != 0 || 0 == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_112 = 2616667235040759262;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_112 {
                        16337619596932156899 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                    current_block_161 = 14714495436747744489;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2);
                    loop {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            current_block_161 = 13215501469961642988;
                            break;
                        }
                        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        } {
                            29 => {
                                if namingBitmap[(((nmstrtPages
                                    [*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int)
                                    << 3)
                                    + (*ptr.offset(1) as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        >> 5))
                                    as usize]
                                    & (1)
                                        << (*ptr.offset(1) as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            & 0x1f)
                                    == 0
                                {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                current_block_161 = 11066148936714919733;
                            }
                            22 | 24 => {
                                current_block_161 = 11066148936714919733;
                            }
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if 0 != 0 || 0 == 0 {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2);
                                current_block_161 = 16314074004867283505;
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if 0 != 0 || 0 == 0 {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3);
                                current_block_161 = 16314074004867283505;
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                }
                                if 0 != 0 || 0 == 0 {
                                    *nextTokPtr = ptr;
                                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4);
                                current_block_161 = 16314074004867283505;
                            }
                            11 => {
                                current_block_161 = 13089361350718158941;
                                break;
                            }
                            17 => {
                                current_block_161 = 11384015785330443424;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(2);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                        }
                        match current_block_161 {
                            11066148936714919733 => {
                                ptr = ptr.offset(2isize);
                            }
                            _ => {}
                        }
                        return big2_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match current_block_161 {
                        13089361350718158941 => {}
                        11384015785330443424 => {}
                        _ => return crate::src::lib::xmltok::XML_TOK_PARTIAL_1,
                    }
                }
                11 => {
                    current_block_161 = 13089361350718158941;
                }
                17 => {
                    current_block_161 = 11384015785330443424;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_161 {
                11384015785330443424 => {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr.offset(0) as ::core::ffi::c_int == 0
                        && *ptr.offset(1) as ::core::ffi::c_int == 0x3e)
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                }
                13089361350718158941 => {
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_START_TAG_NO_ATTS_1;
                }
                18151815167355992796 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_contentTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        }
        if 2 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (2i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(2i32 - 1) as crate::__stddef_size_t_h::size_t;
                if n == 0 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            2 => {
                return big2_scanLt(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            3 => {
                return big2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            9 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
                }
                if (if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2isize);
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
            }
            4 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_TRAILING_RSQB_1;
                }
                if *ptr.offset(0) as ::core::ffi::c_int == 0
                    && *ptr.offset(1) as ::core::ffi::c_int == 0x5d
                {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_TRAILING_RSQB_1;
                    }
                    if !(*ptr.offset(0) as ::core::ffi::c_int == 0
                        && *ptr.offset(1) as ::core::ffi::c_int == 0x3e)
                    {
                        ptr = ptr.offset(-(2isize));
                    } else {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                }
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(2isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_76: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_76 = 7158658067966855297;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_76 = 7158658067966855297;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 || 0 != 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_76 = 7158658067966855297;
                }
                4 => {
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 2) as ::core::ffi::c_long
                    {
                        if !(*ptr.offset(2).offset(0) as ::core::ffi::c_int == 0
                            && *ptr.offset(2).offset(1) as ::core::ffi::c_int == 0x5d)
                        {
                            ptr = ptr.offset(2);
                            current_block_76 = 7158658067966855297;
                        } else if end.offset_from(ptr) as ::core::ffi::c_long
                            >= (3i32 * 2) as ::core::ffi::c_long
                        {
                            if !(*ptr.offset((2i32 * 2) as isize).offset(0) as ::core::ffi::c_int
                                == 0
                                && *ptr.offset((2i32 * 2) as isize).offset(1) as ::core::ffi::c_int
                                    == 0x3e)
                            {
                                ptr = ptr.offset(2isize);
                            } else {
                                *nextTokPtr = ptr.offset((2i32 * 2) as isize);
                                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                            }
                            current_block_76 = 7158658067966855297;
                        } else {
                            current_block_76 = 11890188771060868767;
                        }
                    } else {
                        current_block_76 = 11890188771060868767;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    current_block_76 = 11890188771060868767;
                }
                _ => {
                    ptr = ptr.offset(2);
                    current_block_76 = 7158658067966855297;
                }
            }
            match current_block_76 {
                7158658067966855297 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn big2_scanPercent(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_34: u64;
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_34 = 9652455934050855438;
            }
            22 | 24 => {
                current_block_34 = 9652455934050855438;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_34 = 4761528863920922185;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_34 = 4761528863920922185;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_34 = 4761528863920922185;
            }
            21 | 10 | 9 | 30 => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_PERCENT_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_34 {
            9652455934050855438 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_65: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_65 = 3947837075391501242;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_65 = 3947837075391501242;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_65 = 16415152177862271243;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_65 = 16415152177862271243;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_65 = 16415152177862271243;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_PARAM_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_65 {
                3947837075391501242 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_scanPoundName(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long) {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        let mut current_block_32: u64;
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                current_block_32 = 12219479933348349998;
            }
            22 | 24 => {
                current_block_32 = 12219479933348349998;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2);
                current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3);
                current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                if 0 != 0 || 0 == 0 {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4);
                current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        match current_block_32 {
            12219479933348349998 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_63: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_63 = 1647491770914889697;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_63 = 1647491770914889697;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_63 = 10380409671385728102;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_63 = 10380409671385728102;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_63 = 10380409671385728102;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_POUND_NAME_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_63 {
                1647491770914889697 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return -crate::src::lib::xmltok::XML_TOK_POUND_NAME_1;
    }

    pub(crate) unsafe fn big2_scanLit(
        mut open: ::core::ffi::c_int,
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut t: ::core::ffi::c_int = if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            };
            match t {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                12 | 13 => {
                    ptr = ptr.offset(2);
                    if !(t != open) {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return -crate::src::lib::xmltok::XML_TOK_LITERAL_1;
                        }
                        *nextTokPtr = ptr;
                        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        } {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return crate::src::lib::xmltok::XML_TOK_LITERAL_1
                            }
                            _ => return crate::src::lib::xmltok::XML_TOK_INVALID_1,
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_prologTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        }
        if 2 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (2i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(2i32 - 1) as crate::__stddef_size_t_h::size_t;
                if n == 0 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        let mut current_block_124: u64;
        match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1) as ::core::ffi::c_uchar as usize] as ::core::ffi::c_int
        } else {
            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
        } {
            12 => {
                return big2_scanLit(
                    crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            13 => {
                return big2_scanLit(
                    crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            2 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                } {
                    16 => {
                        return big2_scanDecl(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    15 => {
                        return big2_scanPi(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(2));
                        return crate::src::lib::xmltok::XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            9 => {
                if ptr.offset(2) == end {
                    *nextTokPtr = end;
                    return -crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
                }
                current_block_124 = 16869865525854146339;
            }
            21 | 10 => {
                current_block_124 = 16869865525854146339;
            }
            30 => {
                return big2_scanPercent(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            35 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_COMMA_1;
            }
            20 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_OPEN_BRACKET_1;
            }
            4 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return -crate::src::lib::xmltok::XML_TOK_CLOSE_BRACKET_1;
                }
                if *ptr.offset(0) as ::core::ffi::c_int == 0
                    && *ptr.offset(1) as ::core::ffi::c_int == 0x5d
                {
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(2).offset(0) as ::core::ffi::c_int == 0
                        && *ptr.offset(2).offset(1) as ::core::ffi::c_int == 0x3e
                    {
                        *nextTokPtr = ptr.offset((2i32 * 2) as isize);
                        return crate::src::lib::xmltok::XML_TOK_COND_SECT_CLOSE_1;
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_CLOSE_BRACKET_1;
            }
            31 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_OPEN_PAREN_1;
            }
            32 => {
                ptr = ptr.offset(2);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2) as ::core::ffi::c_long)
                {
                    return -crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_1;
                }
                match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    (*(enc as *const normal_encoding)).type_0
                        [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                } {
                    33 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK_1;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_QUESTION_1;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_PLUS_1;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_1;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            36 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_OR_1;
            }
            11 => {
                *nextTokPtr = ptr.offset(2);
                return crate::src::lib::xmltok::XML_TOK_DECL_CLOSE_1;
            }
            19 => {
                return big2_scanPoundName(enc, ptr.offset(2isize), end, nextTokPtr);
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                    return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
            22 | 24 => {
                tok = crate::src::lib::xmltok::XML_TOK_NAME;
                ptr = ptr.offset(2);
                current_block_124 = 2956972668325154207;
            }
            25 | 26 | 27 | 23 => {
                tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                ptr = ptr.offset(2);
                current_block_124 = 2956972668325154207;
            }
            29 => {
                if namingBitmap[(((nmstrtPages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    != 0
                {
                    ptr = ptr.offset(2);
                    tok = crate::src::lib::xmltok::XML_TOK_NAME;
                    current_block_124 = 2956972668325154207;
                } else if namingBitmap[(((namePages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3)
                    + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                    as usize]
                    & (1) << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                    != 0
                {
                    ptr = ptr.offset(2);
                    tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                    current_block_124 = 2956972668325154207;
                } else {
                    current_block_124 = 6428058487030868344;
                }
            }
            _ => {
                current_block_124 = 6428058487030868344;
            }
        }
        match current_block_124 {
            2956972668325154207 => {}
            16869865525854146339 => {
                loop {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        break;
                    }
                    let mut current_block_32: u64;
                    match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                    } {
                        21 | 10 => {
                            current_block_32 = 17500079516916021833;
                        }
                        9 => {
                            if ptr.offset(2) != end {
                                current_block_32 = 17500079516916021833;
                            } else {
                                current_block_32 = 11299462660638600073;
                            }
                        }
                        _ => {
                            current_block_32 = 11299462660638600073;
                        }
                    }
                    match current_block_32 {
                        17500079516916021833 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return crate::src::lib::xmltok::XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_210: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int)
                        << 3)
                        + (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int >> 5))
                        as usize]
                        & (1)
                            << (*ptr.offset(1) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0x1f)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    current_block_210 = 9794574411605359176;
                }
                22 | 24 | 25 | 26 | 27 => {
                    current_block_210 = 9794574411605359176;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    current_block_210 = 14244298717249035578;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3);
                    current_block_210 = 14244298717249035578;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    if 0 != 0 || 0 == 0 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4);
                    current_block_210 = 14244298717249035578;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(2);
                    match tok {
                        crate::src::lib::xmltok::XML_TOK_NAME => {
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 2) as ::core::ffi::c_long)
                            {
                                return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                            }
                            tok = crate::src::lib::xmltok::XML_TOK_PREFIXED_NAME;
                            let mut current_block_187: u64;
                            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                                (*(enc as *const normal_encoding)).type_0
                                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                            } {
                                29 => {
                                    if namingBitmap[(((namePages
                                        [*ptr.offset(0) as ::core::ffi::c_uchar as usize]
                                        as ::core::ffi::c_int)
                                        << 3)
                                        + (*ptr.offset(1) as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            >> 5))
                                        as usize]
                                        & (1)
                                            << (*ptr.offset(1) as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                & 0x1f)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    current_block_187 = 17275381528970576968;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    current_block_187 = 17275381528970576968;
                                }
                                5 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if 0 != 0 || 0 == 0 {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(2);
                                    current_block_187 = 9812798724717783973;
                                }
                                6 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if 0 != 0 || 0 == 0 {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(3);
                                    current_block_187 = 9812798724717783973;
                                }
                                7 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if 0 != 0 || 0 == 0 {
                                        *nextTokPtr = ptr;
                                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(4);
                                    current_block_187 = 9812798724717783973;
                                }
                                _ => {
                                    tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                                    current_block_187 = 9812798724717783973;
                                }
                            }
                            match current_block_187 {
                                17275381528970576968 => {
                                    ptr = ptr.offset(2isize);
                                }
                                _ => {}
                            }
                        }
                        crate::src::lib::xmltok::XML_TOK_PREFIXED_NAME => {
                            tok = crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
                        }
                        _ => {}
                    }
                    current_block_210 = 14244298717249035578;
                }
                34 => {
                    if tok == crate::src::lib::xmltok::XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_NAME_PLUS_1;
                }
                33 => {
                    if tok == crate::src::lib::xmltok::XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_NAME_ASTERISK_1;
                }
                15 => {
                    if tok == crate::src::lib::xmltok::XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2);
                    return crate::src::lib::xmltok::XML_TOK_NAME_QUESTION_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
            }
            match current_block_210 {
                9794574411605359176 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return -tok;
    }

    pub(crate) unsafe fn big2_attributeValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2) as ::core::ffi::c_long)
        {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                5 => {
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                }
                3 => {
                    if ptr == start {
                        return big2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn big2_entityValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return crate::src::lib::xmltok::XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2) as ::core::ffi::c_long)
        {
            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                5 => {
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                }
                3 => {
                    if ptr == start {
                        return big2_scanRef(enc, ptr.offset(2isize), end, nextTokPtr);
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int =
                            big2_scanPercent(enc, ptr.offset(2), end, nextTokPtr);
                        return if tok == crate::src::lib::xmltok::XML_TOK_PERCENT_1 {
                            crate::src::lib::xmltok::XML_TOK_INVALID_1
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2);
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        return crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    }

    pub(crate) unsafe fn big2_ignoreSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut level: ::core::ffi::c_int = 0;
        if 2 > 1 {
            let mut n: crate::__stddef_size_t_h::size_t =
                end.offset_from(ptr) as crate::__stddef_size_t_h::size_t;
            if n & (2i32 - 1) as crate::__stddef_size_t_h::size_t != 0 {
                n &= !(2i32 - 1) as crate::__stddef_size_t_h::size_t;
                end = ptr.offset(n as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4 {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return crate::src::lib::xmltok::XML_TOK_INVALID_1;
                }
                2 => {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(0) as ::core::ffi::c_int == 0
                        && *ptr.offset(1) as ::core::ffi::c_int == 0x21
                    {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(0) as ::core::ffi::c_int == 0
                            && *ptr.offset(1) as ::core::ffi::c_int == 0x5b
                        {
                            level += 1;
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(2);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long)
                    {
                        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(0) as ::core::ffi::c_int == 0
                        && *ptr.offset(1) as ::core::ffi::c_int == 0x5d
                    {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2) as ::core::ffi::c_long)
                        {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(0) as ::core::ffi::c_int == 0
                            && *ptr.offset(1) as ::core::ffi::c_int == 0x3e
                        {
                            ptr = ptr.offset(2);
                            if level == 0 {
                                *nextTokPtr = ptr;
                                return crate::src::lib::xmltok::XML_TOK_IGNORE_SECT_1;
                            }
                            level -= 1;
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    }

    pub(crate) unsafe fn big2_isPublicId(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        ptr = ptr.offset(2);
        end = end.offset(-(2));
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            let mut current_block_8: u64;
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    current_block_8 = 5143058163439228106;
                }
                21 => {
                    if *ptr.offset(0) as ::core::ffi::c_int == 0
                        && *ptr.offset(1) as ::core::ffi::c_int == 0x9
                    {
                        *badPtr = ptr;
                        return 0i32;
                    }
                    current_block_8 = 5143058163439228106;
                }
                26 | 22 => {
                    if (if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                        *ptr.offset(1) as ::core::ffi::c_int
                    } else {
                        -(1)
                    }) & !(0x7f)
                        == 0
                    {
                        current_block_8 = 5143058163439228106;
                    } else {
                        current_block_8 = 9906551679175889830;
                    }
                }
                _ => {
                    current_block_8 = 9906551679175889830;
                }
            }
            match current_block_8 {
                9906551679175889830 => {
                    match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                        *ptr.offset(1) as ::core::ffi::c_int
                    } else {
                        -(1)
                    } {
                        36 | 64 => {}
                        _ => {
                            *badPtr = ptr;
                            return 0i32;
                        }
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2);
        }
        return 1;
    }

    pub(crate) unsafe fn big2_getAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut attsMax: ::core::ffi::c_int,
        mut atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        let mut state: crate::xmltok_impl_h::C2RustUnnamed_3 = crate::xmltok_impl_c::inName_1;
        let mut nAtts: ::core::ffi::c_int = 0;
        let mut open: ::core::ffi::c_int = 0;
        ptr = ptr.offset(2);
        loop {
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                5 => {
                    if state == crate::xmltok_impl_c::other_1 {
                        if nAtts < attsMax {
                            let ref mut fresh48 = (*atts.offset(nAtts as isize)).name;
                            *fresh48 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_1;
                    }
                    ptr = ptr.offset((2i32 - 2i32) as isize);
                }
                6 => {
                    if state == crate::xmltok_impl_c::other_1 {
                        if nAtts < attsMax {
                            let ref mut fresh49 = (*atts.offset(nAtts as isize)).name;
                            *fresh49 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_1;
                    }
                    ptr = ptr.offset((3i32 - 2i32) as isize);
                }
                7 => {
                    if state == crate::xmltok_impl_c::other_1 {
                        if nAtts < attsMax {
                            let ref mut fresh50 = (*atts.offset(nAtts as isize)).name;
                            *fresh50 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_1;
                    }
                    ptr = ptr.offset((4i32 - 2i32) as isize);
                }
                29 | 22 | 24 => {
                    if state == crate::xmltok_impl_c::other_1 {
                        if nAtts < attsMax {
                            let ref mut fresh51 = (*atts.offset(nAtts as isize)).name;
                            *fresh51 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_1;
                    }
                }
                12 => {
                    if state != crate::xmltok_impl_c::inValue_1 {
                        if nAtts < attsMax {
                            let ref mut fresh52 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh52 = ptr.offset(2isize);
                        }
                        state = crate::xmltok_impl_c::inValue_1;
                        open = crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int;
                    } else if open == crate::xmltok_impl_h::BT_QUOT as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other_1;
                        if nAtts < attsMax {
                            let ref mut fresh53 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh53 = ptr;
                        }
                        nAtts += 1;
                    }
                }
                13 => {
                    if state != crate::xmltok_impl_c::inValue_1 {
                        if nAtts < attsMax {
                            let ref mut fresh54 = (*atts.offset(nAtts as isize)).valuePtr;
                            *fresh54 = ptr.offset(2isize);
                        }
                        state = crate::xmltok_impl_c::inValue_1;
                        open = crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int;
                    } else if open == crate::xmltok_impl_h::BT_APOS as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other_1;
                        if nAtts < attsMax {
                            let ref mut fresh55 = (*atts.offset(nAtts as isize)).valueEnd;
                            *fresh55 = ptr;
                        }
                        nAtts += 1;
                    }
                }
                3 => {
                    if nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                21 => {
                    if state == crate::xmltok_impl_c::inName_1 {
                        state = crate::xmltok_impl_c::other_1;
                    } else if state == crate::xmltok_impl_c::inValue_1
                        && nAtts < attsMax
                        && (*atts.offset(nAtts as isize)).normalized as ::core::ffi::c_int != 0
                        && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                            || (if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                                *ptr.offset(1) as ::core::ffi::c_int
                            } else {
                                -(1)
                            }) != crate::ascii_h::ASCII_SPACE
                            || (if *ptr.offset(2).offset(0) as ::core::ffi::c_int == 0 {
                                *ptr.offset(2).offset(1) as ::core::ffi::c_int
                            } else {
                                -(1)
                            }) == crate::ascii_h::ASCII_SPACE
                            || (if *ptr.offset(2).offset(0) as ::core::ffi::c_int == 0 {
                                (*(enc as *const normal_encoding)).type_0
                                    [*ptr.offset(2).offset(1) as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(2).offset(0),
                                    *ptr.offset(2).offset(1),
                                )
                            }) == open)
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                9 | 10 => {
                    if state == crate::xmltok_impl_c::inName_1 {
                        state = crate::xmltok_impl_c::other_1;
                    } else if state == crate::xmltok_impl_c::inValue_1 && nAtts < attsMax {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                11 | 17 => {
                    if state != crate::xmltok_impl_c::inValue_1 {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2);
        }
    }

    pub(crate) unsafe fn big2_charRefNumber(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0;
        ptr = ptr.offset((2i32 * 2) as isize);
        if *ptr.offset(0) as ::core::ffi::c_int == 0 && *ptr.offset(1) as ::core::ffi::c_int == 0x78
        {
            ptr = ptr.offset(2);
            while !(*ptr.offset(0) as ::core::ffi::c_int == 0
                && *ptr.offset(1) as ::core::ffi::c_int == 0x3b)
            {
                let mut c: ::core::ffi::c_int = if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    *ptr.offset(1) as ::core::ffi::c_int
                } else {
                    -(1)
                };
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
                        result <<= 4;
                        result |= c - crate::ascii_h::ASCII_0;
                    }
                    crate::ascii_h::ASCII_A
                    | crate::ascii_h::ASCII_B_1
                    | crate::ascii_h::ASCII_C
                    | crate::ascii_h::ASCII_D
                    | crate::ascii_h::ASCII_E_1
                    | crate::ascii_h::ASCII_F_1 => {
                        result <<= 4;
                        result += 10i32 + (c - crate::ascii_h::ASCII_A);
                    }
                    crate::ascii_h::ASCII_a_1
                    | crate::ascii_h::ASCII_b
                    | crate::ascii_h::ASCII_c_1
                    | crate::ascii_h::ASCII_d
                    | crate::ascii_h::ASCII_e_1
                    | crate::ascii_h::ASCII_f => {
                        result <<= 4;
                        result += 10i32 + (c - crate::ascii_h::ASCII_a_1);
                    }
                    _ => {}
                }
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.offset(2);
            }
        } else {
            while !(*ptr.offset(0) as ::core::ffi::c_int == 0
                && *ptr.offset(1) as ::core::ffi::c_int == 0x3b)
            {
                let mut c_0: ::core::ffi::c_int = if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    *ptr.offset(1) as ::core::ffi::c_int
                } else {
                    -(1)
                };
                result *= 10;
                result += c_0 - crate::ascii_h::ASCII_0;
                if result >= 0x110000 {
                    return -(1i32);
                }
                ptr = ptr.offset(2);
            }
        }
        return checkCharRefNumber(result);
    }

    pub(crate) unsafe fn big2_predefinedEntityName(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        match end.offset_from(ptr) as ::core::ffi::c_long / 2 {
            2 => {
                if *ptr.offset(2).offset(0) as ::core::ffi::c_int == 0
                    && *ptr.offset(2).offset(1) as ::core::ffi::c_int == 0x74
                {
                    match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                        *ptr.offset(1) as ::core::ffi::c_int
                    } else {
                        -(1)
                    } {
                        crate::ascii_h::ASCII_l_1 => return crate::ascii_h::ASCII_LT,
                        crate::ascii_h::ASCII_g_1 => return crate::ascii_h::ASCII_GT,
                        _ => {}
                    }
                }
            }
            3 => {
                if *ptr.offset(0) as ::core::ffi::c_int == 0
                    && *ptr.offset(1) as ::core::ffi::c_int == 0x61
                {
                    ptr = ptr.offset(2);
                    if *ptr.offset(0) as ::core::ffi::c_int == 0
                        && *ptr.offset(1) as ::core::ffi::c_int == 0x6d
                    {
                        ptr = ptr.offset(2);
                        if *ptr.offset(0) as ::core::ffi::c_int == 0
                            && *ptr.offset(1) as ::core::ffi::c_int == 0x70
                        {
                            return crate::ascii_h::ASCII_AMP;
                        }
                    }
                }
            }
            4 => {
                match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                    *ptr.offset(1) as ::core::ffi::c_int
                } else {
                    -(1)
                } {
                    crate::ascii_h::ASCII_q => {
                        ptr = ptr.offset(2);
                        if *ptr.offset(0) as ::core::ffi::c_int == 0
                            && *ptr.offset(1) as ::core::ffi::c_int == 0x75
                        {
                            ptr = ptr.offset(2);
                            if *ptr.offset(0) as ::core::ffi::c_int == 0
                                && *ptr.offset(1) as ::core::ffi::c_int == 0x6f
                            {
                                ptr = ptr.offset(2);
                                if *ptr.offset(0) as ::core::ffi::c_int == 0
                                    && *ptr.offset(1) as ::core::ffi::c_int == 0x74
                                {
                                    return crate::ascii_h::ASCII_QUOT;
                                }
                            }
                        }
                    }
                    crate::ascii_h::ASCII_a_1 => {
                        ptr = ptr.offset(2);
                        if *ptr.offset(0) as ::core::ffi::c_int == 0
                            && *ptr.offset(1) as ::core::ffi::c_int == 0x70
                        {
                            ptr = ptr.offset(2);
                            if *ptr.offset(0) as ::core::ffi::c_int == 0
                                && *ptr.offset(1) as ::core::ffi::c_int == 0x6f
                            {
                                ptr = ptr.offset(2);
                                if *ptr.offset(0) as ::core::ffi::c_int == 0
                                    && *ptr.offset(1) as ::core::ffi::c_int == 0x73
                                {
                                    return crate::ascii_h::ASCII_APOS;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return 0;
    }

    pub(crate) unsafe fn big2_nameMatchesAscii(
        mut _enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr1: *const ::core::ffi::c_char,
        mut end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while *ptr2 != 0 {
            if (end1.offset_from(ptr1) as ::core::ffi::c_long) < 2 {
                return 0i32;
            }
            if !(*ptr1.offset(0) as ::core::ffi::c_int == 0
                && *ptr1.offset(1) as ::core::ffi::c_int == *ptr2 as ::core::ffi::c_int)
            {
                return 0i32;
            }
            ptr1 = ptr1.offset(2);
            ptr2 = ptr2.offset(1);
        }
        return (ptr1 == end1) as ::core::ffi::c_int;
    }

    pub(crate) unsafe fn big2_nameLength(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                5 => {
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                }
                29 | 22 | 23 | 24 | 25 | 26 | 27 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {
                    return ptr.offset_from(start) as ::core::ffi::c_int;
                }
            }
        }
    }

    pub(crate) unsafe fn big2_skipS(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        loop {
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                10 | 9 | 21 => {
                    ptr = ptr.offset(2isize);
                }
                _ => return ptr,
            }
        }
    }

    pub(crate) unsafe fn big2_updatePosition(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut pos: *mut crate::src::lib::xmltok::POSITION,
    ) {
        while end.offset_from(ptr) as ::core::ffi::c_long >= (1i32 * 2) as ::core::ffi::c_long {
            match if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
            } {
                5 => {
                    ptr = ptr.offset(2);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.offset(3);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.offset(4);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                10 => {
                    (*pos).columnNumber = 0u64;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize);
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2);
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2) as ::core::ffi::c_long
                        && (if *ptr.offset(0) as ::core::ffi::c_int == 0 {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr.offset(1) as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        }) == crate::xmltok_impl_h::BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(2isize);
                    }
                    (*pos).columnNumber = 0u64;
                }
                _ => {
                    ptr = ptr.offset(2);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
            }
        }
    }

    use crate::src::lib::xmltok::checkCharRefNumber;
    use crate::src::lib::xmltok::nametab_h::namePages;
    use crate::src::lib::xmltok::nametab_h::namingBitmap;
    use crate::src::lib::xmltok::nametab_h::nmstrtPages;
    use crate::src::lib::xmltok::normal_encoding;
    use crate::src::lib::xmltok::unicode_byte_type;
}

pub mod xmltok_ns_c {
    pub(crate) unsafe fn XmlGetUtf8InternalEncoding() -> *const crate::src::lib::xmltok::ENCODING {
        return &raw const internal_utf8_encoding.enc;
    }
    pub(crate) unsafe fn XmlGetUtf16InternalEncoding() -> *const crate::src::lib::xmltok::ENCODING {
        return &raw const internal_little2_encoding.enc;
    }

    pub static encodings: [&crate::src::lib::xmltok::ENCODING; 7] = [
        &crate::src::lib::xmltok::latin1_encoding.enc,
        &crate::src::lib::xmltok::ascii_encoding.enc,
        &crate::src::lib::xmltok::utf8_encoding.enc,
        &crate::src::lib::xmltok::big2_encoding.enc,
        &crate::src::lib::xmltok::big2_encoding.enc,
        &crate::src::lib::xmltok::little2_encoding.enc,
        &crate::src::lib::xmltok::utf8_encoding.enc,
    ];

    pub(crate) unsafe fn initScanProlog(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            encodings.as_ptr() as *const *const crate::src::lib::xmltok::ENCODING,
            enc as *const crate::src::lib::xmltok::INIT_ENCODING,
            crate::src::lib::xmltok::XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub(crate) unsafe fn initScanContent(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            encodings.as_ptr() as *const *const crate::src::lib::xmltok::ENCODING,
            enc as *const crate::src::lib::xmltok::INIT_ENCODING,
            crate::src::lib::xmltok::XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }
    pub(crate) unsafe fn XmlInitEncoding(
        mut p: *mut crate::src::lib::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::lib::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = getEncodingIndex(name);
        if i == UNKNOWN_ENC {
            return 0i32;
        }
        (*p).initEnc.isUtf16 = i as ::core::ffi::c_char;
        (*p).initEnc.scanners[crate::src::lib::xmltok::XML_PROLOG_STATE as usize] = initScanProlog
            as unsafe fn(
                *const crate::src::lib::xmltok::ENCODING,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
        (*p).initEnc.scanners[crate::src::lib::xmltok::XML_CONTENT_STATE as usize] = initScanContent
            as unsafe fn(
                *const crate::src::lib::xmltok::ENCODING,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
        (*p).initEnc.updatePosition = initUpdatePosition
            as unsafe fn(
                *const crate::src::lib::xmltok::ENCODING,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut crate::src::lib::xmltok::POSITION,
            ) -> ();
        (*p).encPtr = encPtr;
        *encPtr = &raw mut (*p).initEnc;
        return 1;
    }

    pub(crate) unsafe fn findEncoding(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> *const crate::src::lib::xmltok::ENCODING {
        let mut buf: [::core::ffi::c_char; 128] = ::core::mem::transmute::<
            [u8; 128],
            [::core::ffi::c_char; 128],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        (*enc).utf8Convert(
            enc,
            &raw mut ptr,
            end,
            &raw mut p,
            p.offset(128).offset(-(1)),
        );
        if ptr != end {
            return ::core::ptr::null::<crate::src::lib::xmltok::ENCODING>();
        }
        *p = 0;
        if streqci(
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw const KW_UTF_16 as *const ::core::ffi::c_char,
        ) != 0
            && (*enc).minBytesPerChar == 2
        {
            return enc;
        }
        i = getEncodingIndex(&raw mut buf as *mut ::core::ffi::c_char);
        if i == UNKNOWN_ENC {
            return ::core::ptr::null::<crate::src::lib::xmltok::ENCODING>();
        }
        return encodings[i as usize] as *const crate::src::lib::xmltok::ENCODING;
    }
    pub(crate) unsafe fn XmlParseXmlDecl(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const crate::src::lib::xmltok::ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return doParseXmlDecl(
            Some(
                findEncoding
                    as unsafe fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> *const crate::src::lib::xmltok::ENCODING,
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
    pub(crate) unsafe fn XmlGetUtf8InternalEncodingNS() -> *const crate::src::lib::xmltok::ENCODING
    {
        return &raw const internal_utf8_encoding_ns.enc;
    }
    pub(crate) unsafe fn XmlGetUtf16InternalEncodingNS() -> *const crate::src::lib::xmltok::ENCODING
    {
        return &raw const internal_little2_encoding_ns.enc;
    }

    pub static encodingsNS: [&crate::src::lib::xmltok::ENCODING; 7] = [
        &crate::src::lib::xmltok::latin1_encoding_ns.enc,
        &crate::src::lib::xmltok::ascii_encoding_ns.enc,
        &crate::src::lib::xmltok::utf8_encoding_ns.enc,
        &crate::src::lib::xmltok::big2_encoding_ns.enc,
        &crate::src::lib::xmltok::big2_encoding_ns.enc,
        &crate::src::lib::xmltok::little2_encoding_ns.enc,
        &crate::src::lib::xmltok::utf8_encoding_ns.enc,
    ];

    pub(crate) unsafe fn initScanPrologNS(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            encodingsNS.as_ptr() as *const *const crate::src::lib::xmltok::ENCODING,
            enc as *const crate::src::lib::xmltok::INIT_ENCODING,
            crate::src::lib::xmltok::XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub(crate) unsafe fn initScanContentNS(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            encodingsNS.as_ptr() as *const *const crate::src::lib::xmltok::ENCODING,
            enc as *const crate::src::lib::xmltok::INIT_ENCODING,
            crate::src::lib::xmltok::XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }
    pub(crate) unsafe fn XmlInitEncodingNS(
        mut p: *mut crate::src::lib::xmltok::INIT_ENCODING,
        mut encPtr: *mut *const crate::src::lib::xmltok::ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = getEncodingIndex(name);
        if i == UNKNOWN_ENC {
            return 0i32;
        }
        (*p).initEnc.isUtf16 = i as ::core::ffi::c_char;
        (*p).initEnc.scanners[crate::src::lib::xmltok::XML_PROLOG_STATE as usize] = initScanPrologNS
            as unsafe fn(
                *const crate::src::lib::xmltok::ENCODING,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *const ::core::ffi::c_char,
            ) -> ::core::ffi::c_int;
        (*p).initEnc.scanners[crate::src::lib::xmltok::XML_CONTENT_STATE as usize] =
            initScanContentNS
                as unsafe fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int;
        (*p).initEnc.updatePosition = initUpdatePosition
            as unsafe fn(
                *const crate::src::lib::xmltok::ENCODING,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut crate::src::lib::xmltok::POSITION,
            ) -> ();
        (*p).encPtr = encPtr;
        *encPtr = &raw mut (*p).initEnc;
        return 1;
    }

    pub(crate) unsafe fn findEncodingNS(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> *const crate::src::lib::xmltok::ENCODING {
        let mut buf: [::core::ffi::c_char; 128] = ::core::mem::transmute::<
            [u8; 128],
            [::core::ffi::c_char; 128],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        (*enc).utf8Convert(
            enc,
            &raw mut ptr,
            end,
            &raw mut p,
            p.offset(128).offset(-(1)),
        );
        if ptr != end {
            return ::core::ptr::null::<crate::src::lib::xmltok::ENCODING>();
        }
        *p = 0;
        if streqci(
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw const KW_UTF_16 as *const ::core::ffi::c_char,
        ) != 0
            && (*enc).minBytesPerChar == 2
        {
            return enc;
        }
        i = getEncodingIndex(&raw mut buf as *mut ::core::ffi::c_char);
        if i == UNKNOWN_ENC {
            return ::core::ptr::null::<crate::src::lib::xmltok::ENCODING>();
        }
        return encodingsNS[i as usize] as *const crate::src::lib::xmltok::ENCODING;
    }
    pub(crate) unsafe fn XmlParseXmlDeclNS(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const crate::src::lib::xmltok::ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return doParseXmlDecl(
            Some(
                findEncodingNS
                    as unsafe fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> *const crate::src::lib::xmltok::ENCODING,
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

    use crate::src::lib::xmltok::doParseXmlDecl;
    use crate::src::lib::xmltok::getEncodingIndex;
    use crate::src::lib::xmltok::initScan;
    use crate::src::lib::xmltok::initUpdatePosition;
    use crate::src::lib::xmltok::internal_little2_encoding;
    use crate::src::lib::xmltok::internal_little2_encoding_ns;
    use crate::src::lib::xmltok::internal_utf8_encoding;
    use crate::src::lib::xmltok::internal_utf8_encoding_ns;
    use crate::src::lib::xmltok::streqci;

    use crate::src::lib::xmltok::KW_UTF_16;

    use crate::src::lib::xmltok::UNKNOWN_ENC;
}

pub mod nametab_h {

    pub static namingBitmap: [::core::ffi::c_uint; 320] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
        0xffffffff, 0xffffffff, 0xffffffff, 0, 0x4000000, 0x87fffffe, 0x7fffffe, 0, 0, 0xff7fffff,
        0xff7fffff, 0xffffffff, 0x7ff3ffff, 0xfffffdfe, 0x7fffffff, 0xffffffff, 0xffffffff,
        0xffffe00f, 0xfc31ffff, 0xffffff, 0, 0xffff0000, 0xffffffff, 0xffffffff, 0xf80001ff, 0x3,
        0, 0, 0, 0, 0, 0xffffd740, 0xfffffffb, 0x547f7fff, 0xffffd, 0xffffdffe, 0xffffffff,
        0xdffeffff, 0xffffffff, 0xffff0003, 0xffffffff, 0xffff199f, 0x33fcfff, 0, 0xfffe0000,
        0x27fffff, 0xfffffffe, 0x7f, 0, 0xffff0000, 0x707ff, 0, 0x7fffffe, 0x7fe, 0xfffe0000,
        0xffffffff, 0x7cffffff, 0x2f7fff, 0x60, 0xffffffe0, 0x23ffffff, 0xff000000, 0x3,
        0xfff99fe0, 0x3c5fdff, 0xb0000000, 0x30003, 0xfff987e0, 0x36dfdff, 0x5e000000, 0x1c0000,
        0xfffbafe0, 0x23edfdff, 0, 0x1, 0xfff99fe0, 0x23cdfdff, 0xb0000000, 0x3, 0xd63dc7e0,
        0x3bfc718, 0, 0, 0xfffddfe0, 0x3effdff, 0, 0x3, 0xfffddfe0, 0x3effdff, 0x40000000, 0x3,
        0xfffddfe0, 0x3fffdff, 0, 0x3, 0, 0, 0, 0, 0xfffffffe, 0xd7fff, 0x3f, 0, 0xfef02596,
        0x200d6cae, 0x1f, 0, 0, 0, 0xfffffeff, 0x3ff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xffffffff,
        0xffff003f, 0x7fffff, 0x7daed, 0x50000000, 0x82315001, 0x2c62ab, 0x40000000, 0xf580c900,
        0x7, 0x2010800, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xfffffff, 0xffffffff,
        0xffffffff, 0x3ffffff, 0x3f3fffff, 0xffffffff, 0xaaff3f3f, 0x3fffffff, 0xffffffff,
        0x5fdfffff, 0xfcf1fdc, 0x1fdc1fff, 0, 0x4c40, 0, 0, 0x7, 0, 0, 0, 0x80, 0x3fe, 0xfffffffe,
        0xffffffff, 0x1fffff, 0xfffffffe, 0xffffffff, 0x7ffffff, 0xffffffe0, 0x1fff, 0, 0, 0, 0, 0,
        0, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0x3f, 0, 0, 0xffffffff,
        0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, 0xf, 0, 0, 0, 0x7ff6000, 0x87fffffe,
        0x7fffffe, 0, 0x800000, 0xff7fffff, 0xff7fffff, 0xffffff, 0, 0xffff0000, 0xffffffff,
        0xffffffff, 0xf80001ff, 0x30003, 0, 0xffffffff, 0xffffffff, 0x3f, 0x3, 0xffffd7c0,
        0xfffffffb, 0x547f7fff, 0xffffd, 0xffffdffe, 0xffffffff, 0xdffeffff, 0xffffffff,
        0xffff007b, 0xffffffff, 0xffff199f, 0x33fcfff, 0, 0xfffe0000, 0x27fffff, 0xfffffffe,
        0xfffe007f, 0xbbfffffb, 0xffff0016, 0x707ff, 0, 0x7fffffe, 0x7ffff, 0xffff03ff, 0xffffffff,
        0x7cffffff, 0xffef7fff, 0x3ff3dff, 0xffffffee, 0xf3ffffff, 0xff1e3fff, 0xffcf, 0xfff99fee,
        0xd3c5fdff, 0xb080399f, 0x3ffcf, 0xfff987e4, 0xd36dfdff, 0x5e003987, 0x1fffc0, 0xfffbafee,
        0xf3edfdff, 0x3bbf, 0xffc1, 0xfff99fee, 0xf3cdfdff, 0xb0c0398f, 0xffc3, 0xd63dc7ec,
        0xc3bfc718, 0x803dc7, 0xff80, 0xfffddfee, 0xc3effdff, 0x603ddf, 0xffc3, 0xfffddfec,
        0xc3effdff, 0x40603ddf, 0xffc3, 0xfffddfec, 0xc3fffdff, 0x803dcf, 0xffc3, 0, 0, 0, 0,
        0xfffffffe, 0x7ff7fff, 0x3ff7fff, 0, 0xfef02596, 0x3bff6cae, 0x3ff3f5f, 0, 0x3000000,
        0xc2a003ff, 0xfffffeff, 0xfffe03ff, 0xfebf0fdf, 0x2fe3fff, 0, 0, 0, 0, 0, 0, 0, 0,
        0x1fff0000, 0x2, 0xa0, 0x3efffe, 0xfffffffe, 0xffffffff, 0x661fffff, 0xfffffffe,
        0xffffffff, 0x77ffffff,
    ];

    pub static nmstrtPages: [::core::ffi::c_uchar; 256] = [
        0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0, 0, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x10, 0x11, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x12, 0x13, 0, 0x14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0x15, 0x16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    pub static namePages: [::core::ffi::c_uchar; 256] = [
        0x19, 0x3, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0, 0, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25,
        0x10, 0x11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x12, 0x13, 0x26, 0x14, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0x27, 0x16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x17, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1,
        0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x18, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
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

pub use crate::stdbool_h::false_0;
pub use crate::stdbool_h::true_0;
pub use nametab_h::namePages;
pub use nametab_h::namingBitmap;
pub use nametab_h::nmstrtPages;

use crate::stdlib::memcpy;
pub use crate::xmltok_impl_c::inName;
pub use crate::xmltok_impl_c::inName_0;
pub use crate::xmltok_impl_c::inName_1;
pub use crate::xmltok_impl_c::inValue;
pub use crate::xmltok_impl_c::inValue_0;
pub use crate::xmltok_impl_c::inValue_1;
pub use crate::xmltok_impl_c::other;
pub use crate::xmltok_impl_c::other_0;
pub use crate::xmltok_impl_c::other_1;
pub use crate::xmltok_impl_h::C2RustUnnamed_3;
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
use core::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ushort, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};
pub(crate) use xmltok_impl_c::big2_attributeValueTok;
pub(crate) use xmltok_impl_c::big2_cdataSectionTok;
pub(crate) use xmltok_impl_c::big2_charRefNumber;
pub(crate) use xmltok_impl_c::big2_contentTok;
pub(crate) use xmltok_impl_c::big2_entityValueTok;
pub(crate) use xmltok_impl_c::big2_getAtts;
pub(crate) use xmltok_impl_c::big2_ignoreSectionTok;
pub(crate) use xmltok_impl_c::big2_isPublicId;
pub(crate) use xmltok_impl_c::big2_nameLength;
pub(crate) use xmltok_impl_c::big2_nameMatchesAscii;
pub(crate) use xmltok_impl_c::big2_predefinedEntityName;
pub(crate) use xmltok_impl_c::big2_prologTok;
pub(crate) use xmltok_impl_c::big2_skipS;
pub(crate) use xmltok_impl_c::big2_updatePosition;
pub(crate) use xmltok_impl_c::little2_attributeValueTok;
pub(crate) use xmltok_impl_c::little2_cdataSectionTok;
pub(crate) use xmltok_impl_c::little2_charRefNumber;
pub(crate) use xmltok_impl_c::little2_contentTok;
pub(crate) use xmltok_impl_c::little2_entityValueTok;
pub(crate) use xmltok_impl_c::little2_getAtts;
pub(crate) use xmltok_impl_c::little2_ignoreSectionTok;
pub(crate) use xmltok_impl_c::little2_isPublicId;
pub(crate) use xmltok_impl_c::little2_nameLength;
pub(crate) use xmltok_impl_c::little2_nameMatchesAscii;
pub(crate) use xmltok_impl_c::little2_predefinedEntityName;
pub(crate) use xmltok_impl_c::little2_prologTok;
pub(crate) use xmltok_impl_c::little2_skipS;
pub(crate) use xmltok_impl_c::little2_updatePosition;
pub(crate) use xmltok_impl_c::normal_attributeValueTok;
pub(crate) use xmltok_impl_c::normal_cdataSectionTok;
pub(crate) use xmltok_impl_c::normal_charRefNumber;
pub(crate) use xmltok_impl_c::normal_contentTok;
pub(crate) use xmltok_impl_c::normal_entityValueTok;
pub(crate) use xmltok_impl_c::normal_getAtts;
pub(crate) use xmltok_impl_c::normal_ignoreSectionTok;
pub(crate) use xmltok_impl_c::normal_isPublicId;
pub(crate) use xmltok_impl_c::normal_nameLength;
pub(crate) use xmltok_impl_c::normal_nameMatchesAscii;
pub(crate) use xmltok_impl_c::normal_predefinedEntityName;
pub(crate) use xmltok_impl_c::normal_prologTok;
pub(crate) use xmltok_impl_c::normal_skipS;
pub(crate) use xmltok_impl_c::normal_updatePosition;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct normal_encoding {
    pub enc: ENCODING,
    pub type_0: [c_uchar; 256],
    pub isName2: Option<unsafe fn(*const ENCODING, *const c_char) -> c_int>,
    pub isName3: Option<unsafe fn(*const ENCODING, *const c_char) -> c_int>,
    pub isName4: Option<unsafe fn(*const ENCODING, *const c_char) -> c_int>,
    pub isNmstrt2: Option<unsafe fn(*const ENCODING, *const c_char) -> c_int>,
    pub isNmstrt3: Option<unsafe fn(*const ENCODING, *const c_char) -> c_int>,
    pub isNmstrt4: Option<unsafe fn(*const ENCODING, *const c_char) -> c_int>,
    pub isInvalid2: Option<unsafe fn(*const ENCODING, *const c_char) -> c_int>,
    pub isInvalid3: Option<unsafe fn(*const ENCODING, *const c_char) -> c_int>,
    pub isInvalid4: Option<unsafe fn(*const ENCODING, *const c_char) -> c_int>,
}

pub const UTF8_cval2: C2RustUnnamed_8 = 192;

pub const UTF8_cval4: C2RustUnnamed_8 = 240;

pub const UTF8_cval3: C2RustUnnamed_8 = 224;

pub const UNKNOWN_ENC: C2RustUnnamed_9 = -1;

pub const NO_ENC: C2RustUnnamed_9 = 6;

pub const UTF_16LE_ENC: C2RustUnnamed_9 = 5;

pub const UTF_16BE_ENC: C2RustUnnamed_9 = 4;

pub const UTF_8_ENC: C2RustUnnamed_9 = 2;

pub const UTF_16_ENC: C2RustUnnamed_9 = 3;

pub const ISO_8859_1_ENC: C2RustUnnamed_9 = 0;

pub const min4: C2RustUnnamed_7 = 65536;

pub const min3: C2RustUnnamed_7 = 2048;

pub const UTF8_cval1: C2RustUnnamed_8 = 0;

pub const min2: C2RustUnnamed_7 = 128;

pub type C2RustUnnamed_7 = c_uint;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct unknown_encoding {
    pub normal: normal_encoding,
    pub convert: CONVERTER,
    pub userData: *mut c_void,
    pub utf16: [c_ushort; 256],
    pub utf8: [[c_char; 4]; 256],
}

pub type C2RustUnnamed_8 = c_uint;

pub type C2RustUnnamed_9 = c_int;

pub const US_ASCII_ENC: C2RustUnnamed_9 = 1;

unsafe fn isNever(mut _enc: *const ENCODING, mut _p: *const c_char) -> c_int {
    return 0;
}

unsafe fn utf8_isName2(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (namingBitmap[(((namePages[(*(p as *const c_uchar).offset(0) as c_int >> 2 & 7) as usize]
        as c_int)
        << 3)
        + ((*(p as *const c_uchar).offset(0) as c_int & 3) << 1)
        + (*(p as *const c_uchar).offset(1) as c_int >> 5 & 1)) as usize]
        & (1) << (*(p as *const c_uchar).offset(1) as c_int & 0x1f)) as c_int;
}

unsafe fn utf8_isName3(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (namingBitmap[(((namePages[(((*(p as *const c_uchar).offset(0) as c_int & 0xf) << 4)
        + (*(p as *const c_uchar).offset(1) as c_int >> 2 & 0xf))
        as usize] as c_int)
        << 3)
        + ((*(p as *const c_uchar).offset(1) as c_int & 3) << 1)
        + (*(p as *const c_uchar).offset(2) as c_int >> 5 & 1)) as usize]
        & (1) << (*(p as *const c_uchar).offset(2) as c_int & 0x1f)) as c_int;
}

unsafe fn utf8_isNmstrt2(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (namingBitmap[(((nmstrtPages
        [(*(p as *const c_uchar).offset(0) as c_int >> 2 & 7) as usize]
        as c_int)
        << 3)
        + ((*(p as *const c_uchar).offset(0) as c_int & 3) << 1)
        + (*(p as *const c_uchar).offset(1) as c_int >> 5 & 1)) as usize]
        & (1) << (*(p as *const c_uchar).offset(1) as c_int & 0x1f)) as c_int;
}

unsafe fn utf8_isNmstrt3(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (namingBitmap[(((nmstrtPages[(((*(p as *const c_uchar).offset(0) as c_int & 0xf) << 4)
        + (*(p as *const c_uchar).offset(1) as c_int >> 2 & 0xf))
        as usize] as c_int)
        << 3)
        + ((*(p as *const c_uchar).offset(1) as c_int & 3) << 1)
        + (*(p as *const c_uchar).offset(2) as c_int >> 5 & 1)) as usize]
        & (1) << (*(p as *const c_uchar).offset(2) as c_int & 0x1f)) as c_int;
}

unsafe fn utf8_isInvalid2(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return ((*(p as *const c_uchar) as c_int) < 0xc2
        || *(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
        || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0) as c_int;
}

unsafe fn utf8_isInvalid3(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (*(p as *const c_uchar).offset(2) as c_int & 0x80 == 0
        || (if *(p as *const c_uchar) as c_int == 0xef
            && *(p as *const c_uchar).offset(1) as c_int == 0xbf
        {
            (*(p as *const c_uchar).offset(2) as c_int > 0xbd) as c_int
        } else {
            (*(p as *const c_uchar).offset(2) as c_int & 0xc0 == 0xc0) as c_int
        }) != 0
        || (if *(p as *const c_uchar) as c_int == 0xe0 {
            ((*(p as *const c_uchar).offset(1) as c_int) < 0xa0
                || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0) as c_int
        } else {
            (*(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
                || (if *(p as *const c_uchar) as c_int == 0xed {
                    (*(p as *const c_uchar).offset(1) as c_int > 0x9f) as c_int
                } else {
                    (*(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0) as c_int
                }) != 0) as c_int
        }) != 0) as c_int;
}

unsafe fn utf8_isInvalid4(mut _enc: *const ENCODING, mut p: *const c_char) -> c_int {
    return (*(p as *const c_uchar).offset(3) as c_int & 0x80 == 0
        || *(p as *const c_uchar).offset(3) as c_int & 0xc0 == 0xc0
        || *(p as *const c_uchar).offset(2) as c_int & 0x80 == 0
        || *(p as *const c_uchar).offset(2) as c_int & 0xc0 == 0xc0
        || (if *(p as *const c_uchar) as c_int == 0xf0 {
            ((*(p as *const c_uchar).offset(1) as c_int) < 0x90
                || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0) as c_int
        } else {
            (*(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
                || (if *(p as *const c_uchar) as c_int == 0xf4 {
                    (*(p as *const c_uchar).offset(1) as c_int > 0x8f) as c_int
                } else {
                    (*(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0) as c_int
                }) != 0) as c_int
        }) != 0) as c_int;
}
pub(crate) unsafe fn _INTERNAL_trim_to_complete_utf8_characters(
    mut from: *const c_char,
    mut fromLimRef: *mut *const c_char,
) {
    let mut fromLim: *const c_char = *fromLimRef;
    let mut walked: size_t = 0;
    while fromLim > from {
        let prev: c_uchar = *fromLim.offset(-1) as c_uchar;
        if prev as c_uint & 0xf8 == 0xf0 {
            if walked.wrapping_add(1usize) >= 4usize {
                fromLim = fromLim.offset((4i32 - 1) as isize);
                break;
            } else {
                walked = 0usize;
            }
        } else if prev as c_uint & 0xf0 == 0xe0 {
            if walked.wrapping_add(1usize) >= 3usize {
                fromLim = fromLim.offset((3i32 - 1) as isize);
                break;
            } else {
                walked = 0usize;
            }
        } else if prev as c_uint & 0xe0 == 0xc0 {
            if walked.wrapping_add(1usize) >= 2usize {
                fromLim = fromLim.offset((2i32 - 1) as isize);
                break;
            } else {
                walked = 0usize;
            }
        } else if prev as c_uint & 0x80 == 0 {
            break;
        }
        fromLim = fromLim.offset(-1);
        walked = walked.wrapping_add(1);
    }
    *fromLimRef = fromLim;
}

#[cfg(feature = "expat_test_shims")]
#[export_name = "_INTERNAL_trim_to_complete_utf8_characters"]
unsafe fn internal_trim_to_complete_utf8_characters_test_shim(
    from: *const c_char,
    fromLimRef: *mut *const c_char,
) {
    _INTERNAL_trim_to_complete_utf8_characters(from, fromLimRef);
}

unsafe fn utf8_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    let mut input_incomplete: bool = false_0 != 0;
    let mut output_exhausted: bool = false_0 != 0;
    let bytesAvailable: ptrdiff_t = fromLim.offset_from(*fromP);
    let bytesStorable: ptrdiff_t = toLim.offset_from(*toP);
    if bytesAvailable > bytesStorable {
        fromLim = (*fromP).offset(bytesStorable);
        output_exhausted = true_0 != 0;
    }
    let fromLimBefore: *const c_char = fromLim;
    _INTERNAL_trim_to_complete_utf8_characters(*fromP, &raw mut fromLim);
    if fromLim < fromLimBefore {
        input_incomplete = true_0 != 0;
    }
    let bytesToCopy: ptrdiff_t = fromLim.offset_from(*fromP);
    memcpy(
        *toP as *mut c_void,
        *fromP as *const c_void,
        bytesToCopy as size_t,
    );
    *fromP = (*fromP).offset(bytesToCopy);
    *toP = (*toP).offset(bytesToCopy);
    if output_exhausted {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else if input_incomplete {
        return XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

unsafe fn utf8_toUtf16(
    mut enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    let mut current_block: u64;
    let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
    let mut to: *mut c_ushort = *toP;
    let mut from: *const c_char = *fromP;
    loop {
        if !(from < fromLim && to < toLim as *mut c_ushort) {
            current_block = 18317007320854588510;
            break;
        }
        match (*(enc as *const normal_encoding)).type_0[*from as c_uchar as usize] as c_int {
            5 => {
                if (fromLim.offset_from(from) as c_long) < 2 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    current_block = 7621590230452126720;
                    break;
                } else {
                    let fresh0 = to;
                    to = to.offset(1);
                    *fresh0 = ((*from.offset(0) as c_int & 0x1f) << 6
                        | *from.offset(1) as c_int & 0x3f)
                        as c_ushort;
                    from = from.offset(2isize);
                }
            }
            6 => {
                if (fromLim.offset_from(from) as c_long) < 3 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    current_block = 7621590230452126720;
                    break;
                } else {
                    let fresh1 = to;
                    to = to.offset(1);
                    *fresh1 = ((*from.offset(0) as c_int & 0xf) << 12
                        | (*from.offset(1) as c_int & 0x3f) << 6
                        | *from.offset(2) as c_int & 0x3f)
                        as c_ushort;
                    from = from.offset(3isize);
                }
            }
            7 => {
                let mut n: ::core::ffi::c_ulong = 0;
                if (toLim.offset_from(to) as c_long) < 2 {
                    res = XML_CONVERT_OUTPUT_EXHAUSTED;
                    current_block = 7621590230452126720;
                    break;
                } else if (fromLim.offset_from(from) as c_long) < 4 {
                    res = XML_CONVERT_INPUT_INCOMPLETE;
                    current_block = 7621590230452126720;
                    break;
                } else {
                    n = ((*from.offset(0) as c_int & 0x7) << 18
                        | (*from.offset(1) as c_int & 0x3f) << 12
                        | (*from.offset(2) as c_int & 0x3f) << 6
                        | *from.offset(3) as c_int & 0x3f)
                        as ::core::ffi::c_ulong;
                    n = n.wrapping_sub(0x10000u64);
                    *to.offset(0) = (n >> 10 | 0xd800) as c_ushort;
                    *to.offset(1) = (n & 0x3ff | 0xdc00) as c_ushort;
                    to = to.offset(2);
                    from = from.offset(4isize);
                }
            }
            _ => {
                let fresh2 = from;
                from = from.offset(1);
                let fresh3 = to;
                to = to.offset(1);
                *fresh3 = *fresh2 as c_ushort;
            }
        }
    }
    match current_block {
        18317007320854588510 => {
            if from < fromLim {
                res = XML_CONVERT_OUTPUT_EXHAUSTED;
            }
        }
        _ => {}
    }
    *fromP = from;
    *toP = to;
    return res;
}

static utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            normal_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: normal_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: normal_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: normal_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: normal_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: normal_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: normal_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: normal_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: normal_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: utf8_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: utf8_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_MALFORM as c_uchar,
        BT_MALFORM as c_uchar,
    ],
    isName2: Some(utf8_isName2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isName3: Some(utf8_isName3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isName4: Some(isNever as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt2: Some(utf8_isNmstrt2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt3: Some(utf8_isNmstrt3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt4: Some(isNever as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid2: Some(utf8_isInvalid2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid3: Some(utf8_isInvalid3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid4: Some(utf8_isInvalid4 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
};

static utf8_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            normal_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: normal_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: normal_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: normal_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: normal_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: normal_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: normal_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: normal_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: normal_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: utf8_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: utf8_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_MALFORM as c_uchar,
        BT_MALFORM as c_uchar,
    ],
    isName2: Some(utf8_isName2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isName3: Some(utf8_isName3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isName4: Some(isNever as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt2: Some(utf8_isNmstrt2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt3: Some(utf8_isNmstrt3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt4: Some(isNever as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid2: Some(utf8_isInvalid2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid3: Some(utf8_isInvalid3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid4: Some(utf8_isInvalid4 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
};

static internal_utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            normal_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: normal_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: normal_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: normal_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: normal_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: normal_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: normal_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: normal_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: normal_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: utf8_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: utf8_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_MALFORM as c_uchar,
        BT_MALFORM as c_uchar,
    ],
    isName2: Some(utf8_isName2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isName3: Some(utf8_isName3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isName4: Some(isNever as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt2: Some(utf8_isNmstrt2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt3: Some(utf8_isNmstrt3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt4: Some(isNever as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid2: Some(utf8_isInvalid2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid3: Some(utf8_isInvalid3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid4: Some(utf8_isInvalid4 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
};

static internal_utf8_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            normal_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: normal_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: normal_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: normal_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: normal_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: normal_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: normal_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: normal_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: normal_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: utf8_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: utf8_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_TRAIL as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD2 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD3 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_LEAD4 as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_MALFORM as c_uchar,
        BT_MALFORM as c_uchar,
    ],
    isName2: Some(utf8_isName2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isName3: Some(utf8_isName3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isName4: Some(isNever as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt2: Some(utf8_isNmstrt2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt3: Some(utf8_isNmstrt3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isNmstrt4: Some(isNever as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid2: Some(utf8_isInvalid2 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid3: Some(utf8_isInvalid3 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
    isInvalid4: Some(utf8_isInvalid4 as unsafe fn(*const ENCODING, *const c_char) -> c_int),
};

unsafe fn latin1_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    loop {
        let mut c: c_uchar = 0;
        if *fromP == fromLim {
            return XML_CONVERT_COMPLETED;
        }
        c = **fromP as c_uchar;
        if c as c_int & 0x80 != 0 {
            if (toLim.offset_from(*toP) as c_long) < 2 {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            let fresh6 = *toP;
            *toP = (*toP).offset(1);
            *fresh6 = (c as c_int >> 6 | UTF8_cval2 as c_int) as c_char;
            let fresh7 = *toP;
            *toP = (*toP).offset(1);
            *fresh7 = (c as c_int & 0x3f | 0x80) as c_char;
            *fromP = (*fromP).offset(1);
        } else {
            if *toP == toLim as *mut c_char {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            let fresh8 = *fromP;
            *fromP = (*fromP).offset(1);
            let fresh9 = *toP;
            *toP = (*toP).offset(1);
            *fresh9 = *fresh8;
        }
    }
}

unsafe fn latin1_toUtf16(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let fresh4 = *fromP;
        *fromP = (*fromP).offset(1);
        let fresh5 = *toP;
        *toP = (*toP).offset(1);
        *fresh5 = *fresh4 as c_uchar as c_ushort;
    }
    if *toP == toLim as *mut c_ushort && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

static latin1_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            normal_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: normal_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: normal_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: normal_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: normal_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: normal_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: normal_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: normal_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: normal_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: latin1_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: latin1_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 1,
        isUtf8: 0i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
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
    enc: encoding {
        scanners: [
            normal_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            normal_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: normal_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: normal_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: normal_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: normal_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: normal_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: normal_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: normal_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: normal_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: latin1_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: latin1_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 1,
        isUtf8: 0i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
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

unsafe fn ascii_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    while *fromP < fromLim && *toP < toLim as *mut c_char {
        let fresh56 = *fromP;
        *fromP = (*fromP).offset(1);
        let fresh57 = *toP;
        *toP = (*toP).offset(1);
        *fresh57 = *fresh56;
    }
    if *toP == toLim as *mut c_char && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

static ascii_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            normal_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: normal_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: normal_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: normal_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: normal_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: normal_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: normal_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: normal_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: normal_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: ascii_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: latin1_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
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
    enc: encoding {
        scanners: [
            normal_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            normal_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            normal_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: normal_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: normal_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: normal_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: normal_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: normal_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: normal_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: normal_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: normal_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: ascii_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: latin1_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 1,
        isUtf8: 1i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
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

unsafe fn unicode_byte_type(mut hi: c_char, mut lo: c_char) -> c_int {
    match hi as c_uchar as c_int {
        216 | 217 | 218 | 219 => return BT_LEAD4 as c_int,
        220 | 221 | 222 | 223 => return BT_TRAIL as c_int,
        255 => match lo as c_uchar as c_int {
            255 | 254 => return BT_NONXML as c_int,
            _ => {}
        },
        _ => {}
    }
    return BT_NONASCII as c_int;
}

unsafe fn little2_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    let mut from: *const c_char = *fromP;
    fromLim = from.offset(((fromLim.offset_from(from) as c_long >> 1) << 1) as isize);
    while from < fromLim {
        let mut plane: c_int = 0;
        let mut lo2: c_uchar = 0;
        let mut lo: c_uchar = *from.offset(0) as c_uchar;
        let mut hi: c_uchar = *from.offset(1) as c_uchar;
        let mut current_block_34: u64;
        match hi as c_int {
            0 => {
                if (lo as c_int) < 0x80 {
                    if *toP == toLim as *mut c_char {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    let fresh19 = *toP;
                    *toP = (*toP).offset(1);
                    *fresh19 = lo as c_char;
                    current_block_34 = 14136749492126903395;
                } else {
                    current_block_34 = 9261908759940751603;
                }
            }
            1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                current_block_34 = 9261908759940751603;
            }
            216 | 217 | 218 | 219 => {
                if (toLim.offset_from(*toP) as c_long) < 4 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                if (fromLim.offset_from(from) as c_long) < 4 {
                    *fromP = from;
                    return XML_CONVERT_INPUT_INCOMPLETE;
                }
                plane = ((hi as c_int & 0x3) << 2 | lo as c_int >> 6 & 0x3) + 1;
                let fresh25 = *toP;
                *toP = (*toP).offset(1);
                *fresh25 = (plane >> 2 | UTF8_cval4 as c_int) as c_char;
                let fresh26 = *toP;
                *toP = (*toP).offset(1);
                *fresh26 = (lo as c_int >> 2 & 0xf | (plane & 0x3) << 4 | 0x80) as c_char;
                from = from.offset(2);
                lo2 = *from.offset(0) as c_uchar;
                let fresh27 = *toP;
                *toP = (*toP).offset(1);
                *fresh27 = ((lo as c_int & 0x3) << 4
                    | (*from.offset(1) as c_uchar as c_int & 0x3) << 2
                    | lo2 as c_int >> 6
                    | 0x80) as c_char;
                let fresh28 = *toP;
                *toP = (*toP).offset(1);
                *fresh28 = (lo2 as c_int & 0x3f | 0x80) as c_char;
                current_block_34 = 14136749492126903395;
            }
            _ => {
                if (toLim.offset_from(*toP) as c_long) < 3 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let fresh22 = *toP;
                *toP = (*toP).offset(1);
                *fresh22 = (hi as c_int >> 4 | UTF8_cval3 as c_int) as c_char;
                let fresh23 = *toP;
                *toP = (*toP).offset(1);
                *fresh23 = ((hi as c_int & 0xf) << 2 | lo as c_int >> 6 | 0x80) as c_char;
                let fresh24 = *toP;
                *toP = (*toP).offset(1);
                *fresh24 = (lo as c_int & 0x3f | 0x80) as c_char;
                current_block_34 = 14136749492126903395;
            }
        }
        match current_block_34 {
            9261908759940751603 => {
                if (toLim.offset_from(*toP) as c_long) < 2 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let fresh20 = *toP;
                *toP = (*toP).offset(1);
                *fresh20 = (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char;
                let fresh21 = *toP;
                *toP = (*toP).offset(1);
                *fresh21 = (lo as c_int & 0x3fi32 | 0x80i32) as c_char;
            }
            _ => {}
        }
        from = from.offset(2);
    }
    *fromP = from;
    if from < fromLim {
        return XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

unsafe fn little2_toUtf16(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
    fromLim = (*fromP).offset(((fromLim.offset_from(*fromP) as c_long >> 1) << 1) as isize);
    if fromLim.offset_from(*fromP) as c_long > (toLim.offset_from(*toP) as c_long) << 1
        && *fromLim.offset(-(2)).offset(1) as c_uchar as c_int & 0xf8 == 0xd8
    {
        fromLim = fromLim.offset(-(2));
        res = XML_CONVERT_INPUT_INCOMPLETE;
    }
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let fresh18 = *toP;
        *toP = (*toP).offset(1);
        *fresh18 = ((*(*fromP).offset(1) as c_uchar as c_int) << 8
            | *(*fromP).offset(0) as c_uchar as c_int) as c_ushort;
        *fromP = (*fromP).offset(2);
    }
    if *toP == toLim as *mut c_ushort && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return res;
    };
}

unsafe fn big2_toUtf8(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    let mut from: *const c_char = *fromP;
    fromLim = from.offset(((fromLim.offset_from(from) as c_long >> 1) << 1) as isize);
    while from < fromLim {
        let mut plane: c_int = 0;
        let mut lo2: c_uchar = 0;
        let mut lo: c_uchar = *from.offset(1) as c_uchar;
        let mut hi: c_uchar = *from.offset(0) as c_uchar;
        let mut current_block_34: u64;
        match hi as c_int {
            0 => {
                if (lo as c_int) < 0x80 {
                    if *toP == toLim as *mut c_char {
                        *fromP = from;
                        return XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    let fresh38 = *toP;
                    *toP = (*toP).offset(1);
                    *fresh38 = lo as c_char;
                    current_block_34 = 14136749492126903395;
                } else {
                    current_block_34 = 4084411463441859965;
                }
            }
            1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                current_block_34 = 4084411463441859965;
            }
            216 | 217 | 218 | 219 => {
                if (toLim.offset_from(*toP) as c_long) < 4 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                if (fromLim.offset_from(from) as c_long) < 4 {
                    *fromP = from;
                    return XML_CONVERT_INPUT_INCOMPLETE;
                }
                plane = ((hi as c_int & 0x3) << 2 | lo as c_int >> 6 & 0x3) + 1;
                let fresh44 = *toP;
                *toP = (*toP).offset(1);
                *fresh44 = (plane >> 2 | UTF8_cval4 as c_int) as c_char;
                let fresh45 = *toP;
                *toP = (*toP).offset(1);
                *fresh45 = (lo as c_int >> 2 & 0xf | (plane & 0x3) << 4 | 0x80) as c_char;
                from = from.offset(2);
                lo2 = *from.offset(1) as c_uchar;
                let fresh46 = *toP;
                *toP = (*toP).offset(1);
                *fresh46 = ((lo as c_int & 0x3) << 4
                    | (*from.offset(0) as c_uchar as c_int & 0x3) << 2
                    | lo2 as c_int >> 6
                    | 0x80) as c_char;
                let fresh47 = *toP;
                *toP = (*toP).offset(1);
                *fresh47 = (lo2 as c_int & 0x3f | 0x80) as c_char;
                current_block_34 = 14136749492126903395;
            }
            _ => {
                if (toLim.offset_from(*toP) as c_long) < 3 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let fresh41 = *toP;
                *toP = (*toP).offset(1);
                *fresh41 = (hi as c_int >> 4 | UTF8_cval3 as c_int) as c_char;
                let fresh42 = *toP;
                *toP = (*toP).offset(1);
                *fresh42 = ((hi as c_int & 0xf) << 2 | lo as c_int >> 6 | 0x80) as c_char;
                let fresh43 = *toP;
                *toP = (*toP).offset(1);
                *fresh43 = (lo as c_int & 0x3f | 0x80) as c_char;
                current_block_34 = 14136749492126903395;
            }
        }
        match current_block_34 {
            4084411463441859965 => {
                if (toLim.offset_from(*toP) as c_long) < 2 {
                    *fromP = from;
                    return XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let fresh39 = *toP;
                *toP = (*toP).offset(1);
                *fresh39 = (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char;
                let fresh40 = *toP;
                *toP = (*toP).offset(1);
                *fresh40 = (lo as c_int & 0x3fi32 | 0x80i32) as c_char;
            }
            _ => {}
        }
        from = from.offset(2);
    }
    *fromP = from;
    if from < fromLim {
        return XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}

unsafe fn big2_toUtf16(
    mut _enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
    fromLim = (*fromP).offset(((fromLim.offset_from(*fromP) as c_long >> 1) << 1) as isize);
    if fromLim.offset_from(*fromP) as c_long > (toLim.offset_from(*toP) as c_long) << 1
        && *fromLim.offset(-(2)).offset(0) as c_uchar as c_int & 0xf8 == 0xd8
    {
        fromLim = fromLim.offset(-(2));
        res = XML_CONVERT_INPUT_INCOMPLETE;
    }
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let fresh37 = *toP;
        *toP = (*toP).offset(1);
        *fresh37 = ((*(*fromP).offset(0) as c_uchar as c_int) << 8
            | *(*fromP).offset(1) as c_uchar as c_int) as c_ushort;
        *fromP = (*fromP).offset(2);
    }
    if *toP == toLim as *mut c_ushort && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return res;
    };
}

static little2_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            little2_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            little2_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: little2_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: little2_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: little2_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: little2_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: little2_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: little2_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: little2_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: little2_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: little2_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: little2_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 1i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
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
    enc: encoding {
        scanners: [
            little2_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            little2_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: little2_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: little2_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: little2_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: little2_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: little2_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: little2_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: little2_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: little2_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: little2_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: little2_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 1i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
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
    enc: encoding {
        scanners: [
            little2_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            little2_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: little2_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: little2_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: little2_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: little2_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: little2_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: little2_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: little2_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: little2_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: little2_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: little2_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 1i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
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
    enc: encoding {
        scanners: [
            little2_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            little2_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            little2_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: little2_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: little2_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: little2_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: little2_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: little2_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: little2_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: little2_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: little2_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: little2_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: little2_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 1i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
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
    enc: encoding {
        scanners: [
            big2_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            big2_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            big2_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            big2_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            big2_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            big2_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: big2_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: big2_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: big2_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: big2_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: big2_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: big2_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: big2_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: big2_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: big2_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: big2_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_COLON_0 as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
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
    enc: encoding {
        scanners: [
            big2_prologTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            big2_contentTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            big2_cdataSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            big2_ignoreSectionTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        literalScanners: [
            big2_attributeValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
            big2_entityValueTok
                as unsafe fn(
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> c_int,
        ],
        nameMatchesAscii: big2_nameMatchesAscii
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *const c_char) -> c_int,
        nameLength: big2_nameLength as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        skipS: big2_skipS as unsafe fn(*const ENCODING, *const c_char) -> *const c_char,
        getAtts: big2_getAtts
            as unsafe fn(*const ENCODING, *const c_char, c_int, *mut ATTRIBUTE) -> c_int,
        charRefNumber: big2_charRefNumber as unsafe fn(*const ENCODING, *const c_char) -> c_int,
        predefinedEntityName: big2_predefinedEntityName
            as unsafe fn(*const ENCODING, *const c_char, *const c_char) -> c_int,
        updatePosition: big2_updatePosition
            as unsafe fn(*const ENCODING, *const c_char, *const c_char, *mut POSITION) -> (),
        isPublicId: big2_isPublicId
            as unsafe fn(
                *const ENCODING,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> c_int,
        utf8Convert: big2_toUtf8
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_char,
                *const c_char,
            ) -> XML_Convert_Result,
        utf16Convert: big2_toUtf16
            as unsafe fn(
                *const ENCODING,
                *mut *const c_char,
                *const c_char,
                *mut *mut c_ushort,
                *const c_ushort,
            ) -> XML_Convert_Result,
        minBytesPerChar: 2,
        isUtf8: 0i8,
        isUtf16: 0i8,
    },
    type_0: [
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_LF as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_CR as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_NONXML as c_uchar,
        BT_S as c_uchar,
        BT_EXCL as c_uchar,
        BT_QUOT as c_uchar,
        BT_NUM as c_uchar,
        BT_OTHER as c_uchar,
        BT_PERCNT as c_uchar,
        BT_AMP as c_uchar,
        BT_APOS as c_uchar,
        BT_LPAR as c_uchar,
        BT_RPAR as c_uchar,
        BT_AST as c_uchar,
        BT_PLUS as c_uchar,
        BT_COMMA as c_uchar,
        BT_MINUS as c_uchar,
        BT_NAME as c_uchar,
        BT_SOL as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_DIGIT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_SEMI as c_uchar,
        BT_LT as c_uchar,
        BT_EQUALS as c_uchar,
        BT_GT as c_uchar,
        BT_QUEST as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_LSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_RSQB as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_HEX as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_VERBAR as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NAME as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_OTHER as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
        BT_NMSTRT as c_uchar,
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

unsafe fn streqci(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    loop {
        let fresh58 = s1;
        s1 = s1.offset(1);
        let mut c1: c_char = *fresh58;
        let fresh59 = s2;
        s2 = s2.offset(1);
        let mut c2: c_char = *fresh59;
        if ASCII_a_1 <= c1 as c_int && c1 as c_int <= ASCII_z {
            c1 = (c1 as c_int + (ASCII_A - ASCII_a_1)) as c_char;
        }
        if ASCII_a_1 <= c2 as c_int && c2 as c_int <= ASCII_z {
            c2 = (c2 as c_int + (ASCII_A - ASCII_a_1)) as c_char;
        }
        if c1 as c_int != c2 as c_int {
            return 0i32;
        }
        if c1 == 0 {
            break;
        }
    }
    return 1;
}

unsafe fn initUpdatePosition(
    mut _enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut pos: *mut POSITION,
) {
    normal_updatePosition(&raw const utf8_encoding.enc, ptr, end, pos);
}

unsafe fn toAscii(
    mut enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
) -> c_int {
    let mut buf: [c_char; 1] = [0; 1];
    let mut p: *mut c_char = &raw mut buf as *mut c_char;
    (*enc).utf8Convert(enc, &raw mut ptr, end, &raw mut p, p.offset(1));
    if p == &raw mut buf as *mut c_char {
        return -(1i32);
    } else {
        return buf[0usize] as c_int;
    };
}

unsafe fn isSpace(mut c: c_int) -> c_int {
    match c {
        32 | 13 | 10 | 9 => return 1,
        _ => {}
    }
    return 0;
}

unsafe fn parsePseudoAttribute(
    mut enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut namePtr: *mut *const c_char,
    mut nameEndPtr: *mut *const c_char,
    mut valPtr: *mut *const c_char,
    mut nextTokPtr: *mut *const c_char,
) -> c_int {
    let mut c: c_int = 0;
    let mut open: c_char = 0;
    if ptr == end {
        *namePtr = null::<c_char>();
        return 1i32;
    }
    if isSpace(toAscii(enc, ptr, end)) == 0 {
        *nextTokPtr = ptr;
        return 0i32;
    }
    loop {
        ptr = ptr.offset((*enc).minBytesPerChar as isize);
        if !(isSpace(toAscii(enc, ptr, end)) != 0) {
            break;
        }
    }
    if ptr == end {
        *namePtr = null::<c_char>();
        return 1i32;
    }
    *namePtr = ptr;
    loop {
        c = toAscii(enc, ptr, end);
        if c == -(1) {
            *nextTokPtr = ptr;
            return 0i32;
        }
        if c == ASCII_EQUALS {
            *nameEndPtr = ptr;
            break;
        } else if isSpace(c) != 0 {
            *nameEndPtr = ptr;
            loop {
                ptr = ptr.offset((*enc).minBytesPerChar as isize);
                c = toAscii(enc, ptr, end);
                if !(isSpace(c) != 0) {
                    break;
                }
            }
            if c != ASCII_EQUALS {
                *nextTokPtr = ptr;
                return 0i32;
            }
            break;
        } else {
            ptr = ptr.offset((*enc).minBytesPerChar as isize);
        }
    }
    if ptr == *namePtr {
        *nextTokPtr = ptr;
        return 0i32;
    }
    ptr = ptr.offset((*enc).minBytesPerChar as isize);
    c = toAscii(enc, ptr, end);
    while isSpace(c) != 0 {
        ptr = ptr.offset((*enc).minBytesPerChar as isize);
        c = toAscii(enc, ptr, end);
    }
    if c != ASCII_QUOT && c != ASCII_APOS {
        *nextTokPtr = ptr;
        return 0i32;
    }
    open = c as c_char;
    ptr = ptr.offset((*enc).minBytesPerChar as isize);
    *valPtr = ptr;
    loop {
        c = toAscii(enc, ptr, end);
        if c == open as c_int {
            break;
        }
        if !(ASCII_a_1 <= c && c <= ASCII_z)
            && !(ASCII_A <= c && c <= ASCII_Z)
            && !(ASCII_0 <= c && c <= ASCII_9_1)
            && c != ASCII_PERIOD
            && c != ASCII_MINUS
            && c != ASCII_UNDERSCORE
        {
            *nextTokPtr = ptr;
            return 0i32;
        }
        ptr = ptr.offset((*enc).minBytesPerChar as isize);
    }
    *nextTokPtr = ptr.offset((*enc).minBytesPerChar as isize);
    return 1;
}

static KW_version: [c_char; 8] = [
    ASCII_v as c_char,
    ASCII_e_1 as c_char,
    ASCII_r as c_char,
    ASCII_s as c_char,
    ASCII_i as c_char,
    ASCII_o as c_char,
    ASCII_n as c_char,
    '\0' as c_char,
];

static KW_encoding: [c_char; 9] = [
    ASCII_e_1 as c_char,
    ASCII_n as c_char,
    ASCII_c_1 as c_char,
    ASCII_o as c_char,
    ASCII_d as c_char,
    ASCII_i as c_char,
    ASCII_n as c_char,
    ASCII_g_1 as c_char,
    '\0' as c_char,
];

static KW_standalone: [c_char; 11] = [
    ASCII_s as c_char,
    ASCII_t as c_char,
    ASCII_a_1 as c_char,
    ASCII_n as c_char,
    ASCII_d as c_char,
    ASCII_a_1 as c_char,
    ASCII_l_1 as c_char,
    ASCII_o as c_char,
    ASCII_n as c_char,
    ASCII_e_1 as c_char,
    '\0' as c_char,
];

static KW_yes: [c_char; 4] = [
    ASCII_y as c_char,
    ASCII_e_1 as c_char,
    ASCII_s as c_char,
    '\0' as c_char,
];

static KW_no: [c_char; 3] = [ASCII_n as c_char, ASCII_o as c_char, '\0' as c_char];

unsafe fn doParseXmlDecl(
    mut encodingFinder: Option<
        unsafe fn(*const ENCODING, *const c_char, *const c_char) -> *const ENCODING,
    >,
    mut isGeneralTextEntity: c_int,
    mut enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut badPtr: *mut *const c_char,
    mut versionPtr: *mut *const c_char,
    mut versionEndPtr: *mut *const c_char,
    mut encodingName: *mut *const c_char,
    mut encoding: *mut *const ENCODING,
    mut standalone: *mut c_int,
) -> c_int {
    let mut val: *const c_char = null::<c_char>();
    let mut name: *const c_char = null::<c_char>();
    let mut nameEnd: *const c_char = null::<c_char>();
    ptr = ptr.offset((5i32 * (*enc).minBytesPerChar) as isize);
    end = end.offset(-((2i32 * (*enc).minBytesPerChar) as isize));
    if parsePseudoAttribute(
        enc,
        ptr,
        end,
        &raw mut name,
        &raw mut nameEnd,
        &raw mut val,
        &raw mut ptr,
    ) == 0
        || name.is_null()
    {
        *badPtr = ptr;
        return 0i32;
    }
    if (*enc).nameMatchesAscii(enc, name, nameEnd, &raw const KW_version as *const c_char) == 0 {
        if isGeneralTextEntity == 0 {
            *badPtr = name;
            return 0i32;
        }
    } else {
        if !versionPtr.is_null() {
            *versionPtr = val;
        }
        if !versionEndPtr.is_null() {
            *versionEndPtr = ptr;
        }
        if parsePseudoAttribute(
            enc,
            ptr,
            end,
            &raw mut name,
            &raw mut nameEnd,
            &raw mut val,
            &raw mut ptr,
        ) == 0
        {
            *badPtr = ptr;
            return 0i32;
        }
        if name.is_null() {
            if isGeneralTextEntity != 0 {
                *badPtr = ptr;
                return 0i32;
            }
            return 1i32;
        }
    }
    if (*enc).nameMatchesAscii(enc, name, nameEnd, &raw const KW_encoding as *const c_char) != 0 {
        let mut c: c_int = toAscii(enc, val, end);
        if !(ASCII_a_1 <= c && c <= ASCII_z) && !(ASCII_A <= c && c <= ASCII_Z) {
            *badPtr = val;
            return 0i32;
        }
        if !encodingName.is_null() {
            *encodingName = val;
        }
        if !encoding.is_null() {
            *encoding = encodingFinder.expect("non-null function pointer")(
                enc,
                val,
                ptr.offset(-((*enc).minBytesPerChar as isize)),
            );
        }
        if parsePseudoAttribute(
            enc,
            ptr,
            end,
            &raw mut name,
            &raw mut nameEnd,
            &raw mut val,
            &raw mut ptr,
        ) == 0
        {
            *badPtr = ptr;
            return 0i32;
        }
        if name.is_null() {
            return 1i32;
        }
    }
    if (*enc).nameMatchesAscii(
        enc,
        name,
        nameEnd,
        &raw const KW_standalone as *const c_char,
    ) == 0
        || isGeneralTextEntity != 0
    {
        *badPtr = name;
        return 0i32;
    }
    if (*enc).nameMatchesAscii(
        enc,
        val,
        ptr.offset(-((*enc).minBytesPerChar as isize)),
        &raw const KW_yes as *const c_char,
    ) != 0
    {
        if !standalone.is_null() {
            *standalone = 1i32;
        }
    } else if (*enc).nameMatchesAscii(
        enc,
        val,
        ptr.offset(-((*enc).minBytesPerChar as isize)),
        &raw const KW_no as *const c_char,
    ) != 0
    {
        if !standalone.is_null() {
            *standalone = 0i32;
        }
    } else {
        *badPtr = val;
        return 0i32;
    }
    while isSpace(toAscii(enc, ptr, end)) != 0 {
        ptr = ptr.offset((*enc).minBytesPerChar as isize);
    }
    if ptr != end {
        *badPtr = ptr;
        return 0i32;
    }
    return 1;
}

unsafe fn checkCharRefNumber(mut result: c_int) -> c_int {
    match result >> 8 {
        216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 => {
            return -(1i32);
        }
        0 => {
            if latin1_encoding.type_0[result as usize] as c_int == BT_NONXML as c_int {
                return -(1i32);
            }
        }
        255 => {
            if result == 0xfffe || result == 0xffff {
                return -(1i32);
            }
        }
        _ => {}
    }
    return result;
}
pub(crate) unsafe fn XmlUtf8Encode(mut c: c_int, mut buf: *mut c_char) -> c_int {
    if c < 0 {
        return 0i32;
    }
    if c < min2 as c_int {
        *buf.offset(0) = (c | UTF8_cval1 as c_int) as c_char;
        return 1i32;
    }
    if c < min3 as c_int {
        *buf.offset(0) = (c >> 6 | UTF8_cval2 as c_int) as c_char;
        *buf.offset(1) = (c & 0x3fi32 | 0x80) as c_char;
        return 2i32;
    }
    if c < min4 as c_int {
        *buf.offset(0) = (c >> 12 | UTF8_cval3 as c_int) as c_char;
        *buf.offset(1) = (c >> 6 & 0x3fi32 | 0x80) as c_char;
        *buf.offset(2) = (c & 0x3fi32 | 0x80) as c_char;
        return 3i32;
    }
    if c < 0x110000 {
        *buf.offset(0) = (c >> 18 | UTF8_cval4 as c_int) as c_char;
        *buf.offset(1) = (c >> 12 & 0x3fi32 | 0x80) as c_char;
        *buf.offset(2) = (c >> 6 & 0x3fi32 | 0x80) as c_char;
        *buf.offset(3) = (c & 0x3fi32 | 0x80) as c_char;
        return 4i32;
    }
    return 0;
}
pub(crate) unsafe fn XmlUtf16Encode(mut charNum: c_int, mut buf: *mut c_ushort) -> c_int {
    if charNum < 0 {
        return 0i32;
    }
    if charNum < 0x10000 {
        *buf.offset(0) = charNum as c_ushort;
        return 1i32;
    }
    if charNum < 0x110000 {
        charNum -= 0x10000;
        *buf.offset(0) = ((charNum >> 10) + 0xd800i32) as c_ushort;
        *buf.offset(1) = ((charNum & 0x3ffi32) + 0xdc00) as c_ushort;
        return 2i32;
    }
    return 0;
}
pub(crate) unsafe fn XmlSizeOfUnknownEncoding() -> c_int {
    return size_of::<unknown_encoding>() as c_int;
}

unsafe fn unknown_isName(mut enc: *const ENCODING, mut p: *const c_char) -> c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: c_int = (*uenc).convert.expect("non-null function pointer")((*uenc).userData, p);
    if c & !(0xffff) != 0 {
        return 0i32;
    }
    return (namingBitmap
        [(((namePages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
        & (1) << (c & 0xff & 0x1f)) as c_int;
}

unsafe fn unknown_isNmstrt(mut enc: *const ENCODING, mut p: *const c_char) -> c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: c_int = (*uenc).convert.expect("non-null function pointer")((*uenc).userData, p);
    if c & !(0xffff) != 0 {
        return 0i32;
    }
    return (namingBitmap
        [(((nmstrtPages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
        & (1) << (c & 0xff & 0x1f)) as c_int;
}

unsafe fn unknown_isInvalid(mut enc: *const ENCODING, mut p: *const c_char) -> c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: c_int = (*uenc).convert.expect("non-null function pointer")((*uenc).userData, p);
    return (c & !(0xffff) != 0 || checkCharRefNumber(c) < 0) as c_int;
}

unsafe fn unknown_toUtf8(
    mut enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_char,
    mut toLim: *const c_char,
) -> XML_Convert_Result {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut buf: [c_char; 4] = [0; 4];
    loop {
        let mut utf8: *const c_char = null::<c_char>();
        let mut n: c_int = 0;
        if *fromP == fromLim {
            return XML_CONVERT_COMPLETED;
        }
        utf8 = &raw const *(&raw const (*uenc).utf8 as *const [c_char; 4])
            .offset(**fromP as c_uchar as isize) as *const c_char;
        let fresh61 = utf8;
        utf8 = utf8.offset(1);
        n = *fresh61 as c_int;
        if n == 0 {
            let mut c: c_int =
                (*uenc).convert.expect("non-null function pointer")((*uenc).userData, *fromP);
            n = XmlUtf8Encode(c, &raw mut buf as *mut c_char);
            if n as c_long > toLim.offset_from(*toP) as c_long {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            utf8 = &raw mut buf as *mut c_char;
            *fromP = (*fromP).offset(
                ((*(enc as *const normal_encoding)).type_0[**fromP as c_uchar as usize] as c_int
                    - (BT_LEAD2 as c_int - 2i32)) as isize,
            );
        } else {
            if n as c_long > toLim.offset_from(*toP) as c_long {
                return XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            *fromP = (*fromP).offset(1);
        }
        memcpy(*toP as *mut c_void, utf8 as *const c_void, n as size_t);
        *toP = (*toP).offset(n as isize);
    }
}

unsafe fn unknown_toUtf16(
    mut enc: *const ENCODING,
    mut fromP: *mut *const c_char,
    mut fromLim: *const c_char,
    mut toP: *mut *mut c_ushort,
    mut toLim: *const c_ushort,
) -> XML_Convert_Result {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    while *fromP < fromLim && *toP < toLim as *mut c_ushort {
        let mut c: c_ushort = (*uenc).utf16[**fromP as c_uchar as usize];
        if c as c_int == 0 {
            c = (*uenc).convert.expect("non-null function pointer")((*uenc).userData, *fromP)
                as c_ushort;
            *fromP = (*fromP).offset(
                ((*(enc as *const normal_encoding)).type_0[**fromP as c_uchar as usize] as c_int
                    - (BT_LEAD2 as c_int - 2i32)) as isize,
            );
        } else {
            *fromP = (*fromP).offset(1);
        }
        let fresh60 = *toP;
        *toP = (*toP).offset(1);
        *fresh60 = c;
    }
    if *toP == toLim as *mut c_ushort && *fromP < fromLim {
        return XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return XML_CONVERT_COMPLETED;
    };
}
pub(crate) unsafe fn XmlInitUnknownEncoding(
    mut mem: *mut c_void,
    mut table: *const c_int,
    mut convert: CONVERTER,
    mut userData: *mut c_void,
) -> *mut ENCODING {
    let mut i: c_int = 0;
    let mut e: *mut unknown_encoding = mem as *mut unknown_encoding;
    memcpy(
        mem,
        &raw const latin1_encoding as *const c_void,
        size_of::<normal_encoding>(),
    );
    i = 0;
    while i < 128 {
        if latin1_encoding.type_0[i as usize] as c_int != BT_OTHER as c_int
            && latin1_encoding.type_0[i as usize] as c_int != BT_NONXML as c_int
            && *table.offset(i as isize) != i
        {
            return null_mut::<ENCODING>();
        }
        i += 1;
    }
    i = 0;
    while i < 256 {
        let mut c: c_int = *table.offset(i as isize);
        if c == -(1) {
            (*e).normal.type_0[i as usize] = BT_MALFORM as c_uchar;
            (*e).utf16[i as usize] = 0xffff;
            (*e).utf8[i as usize][0] = 1;
            (*e).utf8[i as usize][1usize] = 0i8;
        } else if c < 0 {
            if c < -(4) {
                return null_mut::<ENCODING>();
            }
            if convert.is_none() {
                return null_mut::<ENCODING>();
            }
            (*e).normal.type_0[i as usize] = (BT_LEAD2 as c_int - (c + 2)) as c_uchar;
            (*e).utf8[i as usize][0] = 0;
            (*e).utf16[i as usize] = 0u16;
        } else if c < 0x80 {
            if latin1_encoding.type_0[c as usize] as c_int != BT_OTHER as c_int
                && latin1_encoding.type_0[c as usize] as c_int != BT_NONXML as c_int
                && c != i
            {
                return null_mut::<ENCODING>();
            }
            (*e).normal.type_0[i as usize] = latin1_encoding.type_0[c as usize];
            (*e).utf8[i as usize][0] = 1;
            (*e).utf8[i as usize][1] = c as c_char;
            (*e).utf16[i as usize] = (if c == 0i32 { 0xffffi32 } else { c }) as c_ushort;
        } else if checkCharRefNumber(c) < 0 {
            (*e).normal.type_0[i as usize] = BT_NONXML as c_uchar;
            (*e).utf16[i as usize] = 0xffff;
            (*e).utf8[i as usize][0] = 1;
            (*e).utf8[i as usize][1usize] = 0i8;
        } else {
            if c > 0xffff {
                return null_mut::<ENCODING>();
            }
            if namingBitmap
                [(((nmstrtPages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
                & (1) << (c & 0xff & 0x1f)
                != 0
            {
                (*e).normal.type_0[i as usize] = BT_NMSTRT as c_uchar;
            } else if namingBitmap
                [(((namePages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
                & (1) << (c & 0xff & 0x1f)
                != 0
            {
                (*e).normal.type_0[i as usize] = BT_NAME as c_uchar;
            } else {
                (*e).normal.type_0[i as usize] = BT_OTHER as c_uchar;
            }
            (*e).utf8[i as usize][0] = XmlUtf8Encode(
                c,
                (&raw mut *(&raw mut (*e).utf8 as *mut [c_char; 4]).offset(i as isize)
                    as *mut c_char)
                    .offset(1),
            ) as c_char;
            (*e).utf16[i as usize] = c as c_ushort;
        }
        i += 1;
    }
    (*e).userData = userData;
    (*e).convert = convert;
    if convert.is_some() {
        (*e).normal.isName2 =
            Some(unknown_isName as unsafe fn(*const ENCODING, *const c_char) -> c_int);
        (*e).normal.isName3 =
            Some(unknown_isName as unsafe fn(*const ENCODING, *const c_char) -> c_int);
        (*e).normal.isName4 =
            Some(unknown_isName as unsafe fn(*const ENCODING, *const c_char) -> c_int);
        (*e).normal.isNmstrt2 =
            Some(unknown_isNmstrt as unsafe fn(*const ENCODING, *const c_char) -> c_int);
        (*e).normal.isNmstrt3 =
            Some(unknown_isNmstrt as unsafe fn(*const ENCODING, *const c_char) -> c_int);
        (*e).normal.isNmstrt4 =
            Some(unknown_isNmstrt as unsafe fn(*const ENCODING, *const c_char) -> c_int);
        (*e).normal.isInvalid2 =
            Some(unknown_isInvalid as unsafe fn(*const ENCODING, *const c_char) -> c_int);
        (*e).normal.isInvalid3 =
            Some(unknown_isInvalid as unsafe fn(*const ENCODING, *const c_char) -> c_int);
        (*e).normal.isInvalid4 =
            Some(unknown_isInvalid as unsafe fn(*const ENCODING, *const c_char) -> c_int);
    }
    (*e).normal.enc.utf8Convert = unknown_toUtf8
        as unsafe fn(
            *const ENCODING,
            *mut *const c_char,
            *const c_char,
            *mut *mut c_char,
            *const c_char,
        ) -> XML_Convert_Result;
    (*e).normal.enc.utf16Convert = unknown_toUtf16
        as unsafe fn(
            *const ENCODING,
            *mut *const c_char,
            *const c_char,
            *mut *mut c_ushort,
            *const c_ushort,
        ) -> XML_Convert_Result;
    return &raw mut (*e).normal.enc;
}

static KW_ISO_8859_1: [c_char; 11] = [
    ASCII_I as c_char,
    ASCII_S as c_char,
    ASCII_O as c_char,
    ASCII_MINUS as c_char,
    ASCII_8_1 as c_char,
    ASCII_8_1 as c_char,
    ASCII_5 as c_char,
    ASCII_9_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_1_1 as c_char,
    '\0' as c_char,
];

static KW_US_ASCII: [c_char; 9] = [
    ASCII_U as c_char,
    ASCII_S as c_char,
    ASCII_MINUS as c_char,
    ASCII_A as c_char,
    ASCII_S as c_char,
    ASCII_C as c_char,
    ASCII_I as c_char,
    ASCII_I as c_char,
    '\0' as c_char,
];

static KW_UTF_8: [c_char; 6] = [
    ASCII_U as c_char,
    ASCII_T as c_char,
    ASCII_F_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_8_1 as c_char,
    '\0' as c_char,
];

static KW_UTF_16: [c_char; 7] = [
    ASCII_U as c_char,
    ASCII_T as c_char,
    ASCII_F_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_1_1 as c_char,
    ASCII_6 as c_char,
    '\0' as c_char,
];

static KW_UTF_16BE: [c_char; 9] = [
    ASCII_U as c_char,
    ASCII_T as c_char,
    ASCII_F_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_1_1 as c_char,
    ASCII_6 as c_char,
    ASCII_B_1 as c_char,
    ASCII_E_1 as c_char,
    '\0' as c_char,
];

static KW_UTF_16LE: [c_char; 9] = [
    ASCII_U as c_char,
    ASCII_T as c_char,
    ASCII_F_1 as c_char,
    ASCII_MINUS as c_char,
    ASCII_1_1 as c_char,
    ASCII_6 as c_char,
    ASCII_L_1 as c_char,
    ASCII_E_1 as c_char,
    '\0' as c_char,
];

unsafe fn getEncodingIndex(mut name: *const c_char) -> c_int {
    let encodingNames: [*const c_char; 6] = [
        &raw const KW_ISO_8859_1 as *const c_char,
        &raw const KW_US_ASCII as *const c_char,
        &raw const KW_UTF_8 as *const c_char,
        &raw const KW_UTF_16 as *const c_char,
        &raw const KW_UTF_16BE as *const c_char,
        &raw const KW_UTF_16LE as *const c_char,
    ];
    let mut i: c_int = 0;
    if name.is_null() {
        return NO_ENC;
    }
    i = 0;
    while i < (size_of::<[*const c_char; 6]>()).wrapping_div(size_of::<*const c_char>()) as c_int {
        if streqci(name, encodingNames[i as usize]) != 0 {
            return i;
        }
        i += 1;
    }
    return UNKNOWN_ENC;
}

unsafe fn initScan(
    mut encodingTable: *const *const ENCODING,
    mut enc: *const INIT_ENCODING,
    mut state: c_int,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut nextTokPtr: *mut *const c_char,
) -> c_int {
    let mut encPtr: *mut *const ENCODING = null_mut::<*const ENCODING>();
    if ptr >= end {
        return XML_TOK_NONE_1;
    }
    encPtr = (*enc).encPtr;
    if ptr.offset(1) == end {
        match (*enc).initEnc.isUtf16 as c_int {
            3 | 5 | 4 => return XML_TOK_PARTIAL_1,
            _ => {}
        }
        let mut current_block_5: u64;
        match *ptr as c_uchar as c_int {
            254 | 255 | 239 => {
                if (*enc).initEnc.isUtf16 as c_int == ISO_8859_1_ENC && state == XML_CONTENT_STATE {
                    current_block_5 = 13183875560443969876;
                } else {
                    current_block_5 = 6556540211831925522;
                }
            }
            0 | 60 => {
                current_block_5 = 6556540211831925522;
            }
            _ => {
                current_block_5 = 13183875560443969876;
            }
        }
        match current_block_5 {
            13183875560443969876 => {}
            _ => return XML_TOK_PARTIAL_1,
        }
    } else {
        let mut current_block_26: u64;
        match (*ptr.offset(0) as c_uchar as c_int) << 8 | *ptr.offset(1) as c_uchar as c_int {
            65279 => {
                if !((*enc).initEnc.isUtf16 as c_int == ISO_8859_1_ENC
                    && state == XML_CONTENT_STATE)
                {
                    *nextTokPtr = ptr.offset(2);
                    *encPtr = *encodingTable.offset(UTF_16BE_ENC as isize);
                    return XML_TOK_BOM_1;
                }
            }
            15360 => {
                if !(((*enc).initEnc.isUtf16 as c_int == UTF_16BE_ENC
                    || (*enc).initEnc.isUtf16 as c_int == UTF_16_ENC)
                    && state == XML_CONTENT_STATE)
                {
                    *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                    return (**encPtr).scanners[state as usize](*encPtr, ptr, end, nextTokPtr);
                }
            }
            65534 => {
                if !((*enc).initEnc.isUtf16 as c_int == ISO_8859_1_ENC
                    && state == XML_CONTENT_STATE)
                {
                    *nextTokPtr = ptr.offset(2);
                    *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                    return XML_TOK_BOM_1;
                }
            }
            61371 => {
                if state == XML_CONTENT_STATE {
                    let mut e: c_int = (*enc).initEnc.isUtf16 as c_int;
                    if e == ISO_8859_1_ENC
                        || e == UTF_16BE_ENC
                        || e == UTF_16LE_ENC
                        || e == UTF_16_ENC
                    {
                        current_block_26 = 2604890879466389055;
                    } else {
                        current_block_26 = 11307063007268554308;
                    }
                } else {
                    current_block_26 = 11307063007268554308;
                }
                match current_block_26 {
                    2604890879466389055 => {}
                    _ => {
                        if ptr.offset(2) == end {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(2) as c_uchar as c_int == 0xbf {
                            *nextTokPtr = ptr.offset(3);
                            *encPtr = *encodingTable.offset(UTF_8_ENC as isize);
                            return XML_TOK_BOM_1;
                        }
                    }
                }
            }
            _ => {
                if *ptr.offset(0) as c_int == '\0' as i32 {
                    if !(state == XML_CONTENT_STATE
                        && (*enc).initEnc.isUtf16 as c_int == UTF_16LE_ENC)
                    {
                        *encPtr = *encodingTable.offset(UTF_16BE_ENC as isize);
                        return (**encPtr).scanners[state as usize](*encPtr, ptr, end, nextTokPtr);
                    }
                } else if *ptr.offset(1) as c_int == '\0' as i32 {
                    if !(state == XML_CONTENT_STATE) {
                        *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                        return (**encPtr).scanners[state as usize](*encPtr, ptr, end, nextTokPtr);
                    }
                }
            }
        }
    }
    *encPtr = *encodingTable.offset((*enc).initEnc.isUtf16 as c_int as isize);
    return (**encPtr).scanners[state as usize](*encPtr, ptr, end, nextTokPtr);
}
pub(crate) unsafe fn XmlInitUnknownEncodingNS(
    mut mem: *mut c_void,
    mut table: *const c_int,
    mut convert: CONVERTER,
    mut userData: *mut c_void,
) -> *mut ENCODING {
    let mut enc: *mut ENCODING = XmlInitUnknownEncoding(mem, table, convert, userData);
    if !enc.is_null() {
        (*(enc as *mut normal_encoding)).type_0[ASCII_COLON as usize] = BT_COLON_0 as c_uchar;
    }
    return enc;
}
