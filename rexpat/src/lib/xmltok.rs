// =============== BEGIN xmltok_h ================
pub const XML_TOK_TRAILING_RSQB: ::core::ffi::c_int = -5;

pub const XML_TOK_TRAILING_RSQB_1: ::core::ffi::c_int = -5i32;

pub const XML_TOK_NONE: ::core::ffi::c_int = -4;

pub const XML_TOK_NONE_1: ::core::ffi::c_int = -4i32;

pub const XML_TOK_TRAILING_CR: ::core::ffi::c_int = -3;

pub const XML_TOK_TRAILING_CR_1: ::core::ffi::c_int = -3i32;

pub const XML_TOK_PARTIAL_CHAR: ::core::ffi::c_int = -2;

pub const XML_TOK_PARTIAL_CHAR_1: ::core::ffi::c_int = -2i32;

pub const XML_TOK_PARTIAL: ::core::ffi::c_int = -1;

pub const XML_TOK_PARTIAL_1: ::core::ffi::c_int = -1i32;

pub const XML_TOK_INVALID: ::core::ffi::c_int = 0;

pub const XML_TOK_INVALID_1: ::core::ffi::c_int = 0i32;

pub const XML_TOK_START_TAG_WITH_ATTS: ::core::ffi::c_int = 1;

pub const XML_TOK_START_TAG_WITH_ATTS_1: ::core::ffi::c_int = 1i32;

pub const XML_TOK_START_TAG_NO_ATTS: ::core::ffi::c_int = 2;

pub const XML_TOK_START_TAG_NO_ATTS_1: ::core::ffi::c_int = 2i32;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: ::core::ffi::c_int = 3;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1: ::core::ffi::c_int = 3i32;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: ::core::ffi::c_int = 4;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS_1: ::core::ffi::c_int = 4i32;

pub const XML_TOK_END_TAG: ::core::ffi::c_int = 5;

pub const XML_TOK_END_TAG_1: ::core::ffi::c_int = 5i32;

pub const XML_TOK_DATA_CHARS: ::core::ffi::c_int = 6;

pub const XML_TOK_DATA_CHARS_1: ::core::ffi::c_int = 6i32;

pub const XML_TOK_DATA_NEWLINE: ::core::ffi::c_int = 7;

pub const XML_TOK_DATA_NEWLINE_1: ::core::ffi::c_int = 7i32;

pub const XML_TOK_CDATA_SECT_OPEN: ::core::ffi::c_int = 8;

pub const XML_TOK_CDATA_SECT_OPEN_1: ::core::ffi::c_int = 8i32;

pub const XML_TOK_ENTITY_REF: ::core::ffi::c_int = 9;

pub const XML_TOK_ENTITY_REF_1: ::core::ffi::c_int = 9i32;

pub const XML_TOK_CHAR_REF: ::core::ffi::c_int = 10;

pub const XML_TOK_CHAR_REF_1: ::core::ffi::c_int = 10i32;

pub const XML_TOK_PI: ::core::ffi::c_int = 11;

pub const XML_TOK_PI_1: ::core::ffi::c_int = 11i32;

pub const XML_TOK_XML_DECL: ::core::ffi::c_int = 12;

pub const XML_TOK_XML_DECL_1: ::core::ffi::c_int = 12i32;

pub const XML_TOK_COMMENT: ::core::ffi::c_int = 13;

pub const XML_TOK_COMMENT_1: ::core::ffi::c_int = 13i32;

pub const XML_TOK_BOM: ::core::ffi::c_int = 14;

pub const XML_TOK_BOM_1: ::core::ffi::c_int = 14i32;

pub const XML_TOK_PROLOG_S: ::core::ffi::c_int = 15;

pub const XML_TOK_PROLOG_S_1: ::core::ffi::c_int = 15i32;

pub const XML_TOK_DECL_OPEN: ::core::ffi::c_int = 16;

pub const XML_TOK_DECL_OPEN_1: ::core::ffi::c_int = 16i32;

pub const XML_TOK_DECL_CLOSE: ::core::ffi::c_int = 17;

pub const XML_TOK_DECL_CLOSE_1: ::core::ffi::c_int = 17i32;

pub const XML_TOK_NAME: ::core::ffi::c_int = 18;

pub const XML_TOK_NMTOKEN: ::core::ffi::c_int = 19;

pub const XML_TOK_NMTOKEN_1: ::core::ffi::c_int = 19i32;

pub const XML_TOK_POUND_NAME: ::core::ffi::c_int = 20;

pub const XML_TOK_POUND_NAME_1: ::core::ffi::c_int = 20i32;

pub const XML_TOK_OR: ::core::ffi::c_int = 21;

pub const XML_TOK_OR_1: ::core::ffi::c_int = 21i32;

pub const XML_TOK_PERCENT: ::core::ffi::c_int = 22;

pub const XML_TOK_PERCENT_1: ::core::ffi::c_int = 22i32;

pub const XML_TOK_OPEN_PAREN: ::core::ffi::c_int = 23;

pub const XML_TOK_OPEN_PAREN_1: ::core::ffi::c_int = 23i32;

pub const XML_TOK_CLOSE_PAREN: ::core::ffi::c_int = 24;

pub const XML_TOK_CLOSE_PAREN_1: ::core::ffi::c_int = 24i32;

pub const XML_TOK_OPEN_BRACKET: ::core::ffi::c_int = 25;

pub const XML_TOK_OPEN_BRACKET_1: ::core::ffi::c_int = 25i32;

pub const XML_TOK_CLOSE_BRACKET: ::core::ffi::c_int = 26;

pub const XML_TOK_CLOSE_BRACKET_1: ::core::ffi::c_int = 26i32;

pub const XML_TOK_LITERAL: ::core::ffi::c_int = 27;

pub const XML_TOK_LITERAL_1: ::core::ffi::c_int = 27i32;

pub const XML_TOK_PARAM_ENTITY_REF: ::core::ffi::c_int = 28;

pub const XML_TOK_PARAM_ENTITY_REF_1: ::core::ffi::c_int = 28i32;

pub const XML_TOK_INSTANCE_START: ::core::ffi::c_int = 29i32;

pub const XML_TOK_INSTANCE_START_1: ::core::ffi::c_int = 29;

pub const XML_TOK_NAME_QUESTION: ::core::ffi::c_int = 30;

pub const XML_TOK_NAME_QUESTION_1: ::core::ffi::c_int = 30i32;

pub const XML_TOK_NAME_ASTERISK: ::core::ffi::c_int = 31;

pub const XML_TOK_NAME_ASTERISK_1: ::core::ffi::c_int = 31i32;

pub const XML_TOK_NAME_PLUS: ::core::ffi::c_int = 32;

pub const XML_TOK_NAME_PLUS_1: ::core::ffi::c_int = 32i32;

pub const XML_TOK_COND_SECT_OPEN: ::core::ffi::c_int = 33;

pub const XML_TOK_COND_SECT_OPEN_1: ::core::ffi::c_int = 33i32;

pub const XML_TOK_COND_SECT_CLOSE: ::core::ffi::c_int = 34;

pub const XML_TOK_COND_SECT_CLOSE_1: ::core::ffi::c_int = 34i32;

pub const XML_TOK_CLOSE_PAREN_QUESTION: ::core::ffi::c_int = 35;

pub const XML_TOK_CLOSE_PAREN_QUESTION_1: ::core::ffi::c_int = 35i32;

pub const XML_TOK_CLOSE_PAREN_ASTERISK: ::core::ffi::c_int = 36;

pub const XML_TOK_CLOSE_PAREN_ASTERISK_1: ::core::ffi::c_int = 36i32;

pub const XML_TOK_CLOSE_PAREN_PLUS: ::core::ffi::c_int = 37;

pub const XML_TOK_CLOSE_PAREN_PLUS_1: ::core::ffi::c_int = 37i32;

pub const XML_TOK_COMMA: ::core::ffi::c_int = 38;

pub const XML_TOK_COMMA_1: ::core::ffi::c_int = 38i32;

pub const XML_TOK_ATTRIBUTE_VALUE_S: ::core::ffi::c_int = 39;

pub const XML_TOK_ATTRIBUTE_VALUE_S_1: ::core::ffi::c_int = 39i32;

pub const XML_TOK_CDATA_SECT_CLOSE: ::core::ffi::c_int = 40;

pub const XML_TOK_CDATA_SECT_CLOSE_1: ::core::ffi::c_int = 40i32;

pub const XML_TOK_PREFIXED_NAME: ::core::ffi::c_int = 41;

pub const XML_TOK_IGNORE_SECT: ::core::ffi::c_int = 42;

pub const XML_TOK_IGNORE_SECT_1: ::core::ffi::c_int = 42i32;

pub const XML_PROLOG_STATE: ::core::ffi::c_int = 0i32;

pub const XML_CONTENT_STATE: ::core::ffi::c_int = 1i32;

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

pub type SCANNER = Option<
    unsafe extern "C" fn(
        *const crate::src::lib::xmltok::ENCODING,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int,
>;

pub type XML_Convert_Result = ::core::ffi::c_uint;

pub const XML_CONVERT_COMPLETED: crate::src::lib::xmltok::XML_Convert_Result = 0;

pub const XML_CONVERT_INPUT_INCOMPLETE: crate::src::lib::xmltok::XML_Convert_Result = 1;

pub const XML_CONVERT_OUTPUT_EXHAUSTED: crate::src::lib::xmltok::XML_Convert_Result = 2;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct encoding {
    pub scanners: [crate::src::lib::xmltok::SCANNER; 4],
    pub literalScanners: [crate::src::lib::xmltok::SCANNER; 2],
    pub nameMatchesAscii: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub nameLength: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub skipS: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> *const ::core::ffi::c_char,
    >,
    pub getAtts: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
            *mut crate::src::lib::xmltok::ATTRIBUTE,
        ) -> ::core::ffi::c_int,
    >,
    pub charRefNumber: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub predefinedEntityName: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub updatePosition: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut crate::src::lib::xmltok::POSITION,
        ) -> (),
    >,
    pub isPublicId: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub utf8Convert: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> crate::src::lib::xmltok::XML_Convert_Result,
    >,
    pub utf16Convert: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_ushort,
            *const ::core::ffi::c_ushort,
        ) -> crate::src::lib::xmltok::XML_Convert_Result,
    >,
    pub minBytesPerChar: ::core::ffi::c_int,
    pub isUtf8: ::core::ffi::c_char,
    pub isUtf16: ::core::ffi::c_char,
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

    pub unsafe extern "C" fn normal_scanComment(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            if !(*ptr as ::core::ffi::c_int == 0x2di32) {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            ptr = ptr.offset(1isize);
            while end.offset_from(ptr) as ::core::ffi::c_long
                >= (1i32 * 1i32) as ::core::ffi::c_long
            {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    5 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64
                        {
                            return XML_TOK_PARTIAL_CHAR_1;
                        }
                        if (*(enc as *const normal_encoding))
                            .isInvalid2
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                        {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    6 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64
                        {
                            return XML_TOK_PARTIAL_CHAR_1;
                        }
                        if (*(enc as *const normal_encoding))
                            .isInvalid3
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                        {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    7 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64
                        {
                            return XML_TOK_PARTIAL_CHAR_1;
                        }
                        if (*(enc as *const normal_encoding))
                            .isInvalid4
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                        {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    27 => {
                        ptr = ptr.offset(1isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr as ::core::ffi::c_int == 0x2di32 {
                            ptr = ptr.offset(1isize);
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 1i32)
                                    as ::core::ffi::c_long)
                            {
                                return XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr as ::core::ffi::c_int == 0x3ei32) {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(1isize);
                            return XML_TOK_COMMENT_1;
                        }
                    }
                    _ => {
                        ptr = ptr.offset(1isize);
                    }
                }
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_scanDecl(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            27 => {
                return normal_scanComment(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            20 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_COND_SECT_OPEN_1;
            }
            22 | 24 => {
                ptr = ptr.offset(1isize);
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            's_129: {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    30 => {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (2i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        match (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                        {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(1isize);
                        break 's_129;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_DECL_OPEN_1;
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_checkPiTarget(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut tokPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0i32;
        *tokPtr = XML_TOK_PI_1;
        if end.offset_from(ptr) as ::core::ffi::c_long
            != (1i32 * 3i32) as ::core::ffi::c_long
        {
            return 1i32;
        }
        match *ptr as ::core::ffi::c_int {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1i32;
            }
            _ => return 1i32,
        }
        ptr = ptr.offset(1isize);
        match *ptr as ::core::ffi::c_int {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1i32;
            }
            _ => return 1i32,
        }
        ptr = ptr.offset(1isize);
        match *ptr as ::core::ffi::c_int {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1i32;
            }
            _ => return 1i32,
        }
        if upper != 0 {
            return 0i32;
        }
        *tokPtr = XML_TOK_XML_DECL_1;
        return 1i32;
    }

    pub unsafe extern "C" fn normal_scanPi(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        let mut target: *const ::core::ffi::c_char = ptr;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_32: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_32 = 11470911313929454839;
            }
            22 | 24 => {
                c2rust_current_block_32 = 11470911313929454839;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_32 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_32 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_32 = 14763689060501151050;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_32 {
            11470911313929454839 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_118: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_118 = 8485341570193076947;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_118 = 8485341570193076947;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_118 = 13349765058737954042;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_118 = 13349765058737954042;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_118 = 13349765058737954042;
                }
                21 | 9 | 10 => {
                    if normal_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(1isize);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long
                    {
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            15 => {
                                ptr = ptr.offset(1isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 1i32)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL_1;
                                }
                                if *ptr as ::core::ffi::c_int == 0x3ei32 {
                                    *nextTokPtr = ptr.offset(1isize);
                                    return tok;
                                }
                            }
                            _ => {
                                ptr = ptr.offset(1isize);
                            }
                        }
                    }
                    return XML_TOK_PARTIAL_1;
                }
                15 => {
                    if normal_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr as ::core::ffi::c_int == 0x3ei32 {
                        *nextTokPtr = ptr.offset(1isize);
                        return tok;
                    }
                    c2rust_current_block_118 = 11310415194689177606;
                }
                _ => {
                    c2rust_current_block_118 = 11310415194689177606;
                }
            }
            match c2rust_current_block_118 {
                11310415194689177606 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                8485341570193076947 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_scanCdataSection(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        pub static mut CDATA_LSQB: [::core::ffi::c_char; 6] = [
            ASCII_C as ::core::ffi::c_char,
            ASCII_D as ::core::ffi::c_char,
            ASCII_A as ::core::ffi::c_char,
            ASCII_T as ::core::ffi::c_char,
            ASCII_A as ::core::ffi::c_char,
            ASCII_LSQB as ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (6i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        i = 0i32;
        while i < 6i32 {
            if !(*ptr as ::core::ffi::c_int == CDATA_LSQB[i as usize] as ::core::ffi::c_int) {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            i += 1;
            ptr = ptr.offset(1isize);
        }
        *nextTokPtr = ptr;
        return XML_TOK_CDATA_SECT_OPEN_1;
    }

    pub unsafe extern "C" fn normal_cdataSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return XML_TOK_NONE_1;
        }
        if 1i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (1i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(1i32 - 1i32)
                    as size_t;
                if n == 0usize {
                    return XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            4 => {
                ptr = ptr.offset(1isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                if *ptr as ::core::ffi::c_int == 0x5di32 {
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr as ::core::ffi::c_int == 0x3ei32) {
                        ptr = ptr.offset(-(1isize));
                    } else {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CDATA_SECT_CLOSE_1;
                    }
                }
            }
            9 => {
                ptr = ptr.offset(1isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                if (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                    == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1isize);
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_DATA_NEWLINE_1;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(1isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid2
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid3
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid4
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(1isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn normal_scanEndTag(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_32: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_32 = 4324628675098861213;
            }
            22 | 24 => {
                c2rust_current_block_32 = 4324628675098861213;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_32 {
            4324628675098861213 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_73: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_73 = 14883924698754021420;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_73 = 14883924698754021420;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(1isize);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long
                    {
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(1isize);
                                return XML_TOK_END_TAG_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(1isize);
                    }
                    return XML_TOK_PARTIAL_1;
                }
                23 => {
                    ptr = ptr.offset(1isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                11 => {
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_END_TAG_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_73 {
                14883924698754021420 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_scanHexCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(1isize);
            while end.offset_from(ptr) as ::core::ffi::c_long
                >= (1i32 * 1i32) as ::core::ffi::c_long
            {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(1isize);
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_scanCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            if *ptr as ::core::ffi::c_int == 0x78i32 {
                return normal_scanHexCharRef(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(1isize);
            while end.offset_from(ptr) as ::core::ffi::c_long
                >= (1i32 * 1i32) as ::core::ffi::c_long
            {
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(1isize);
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_scanRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_33: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_33 = 8911980980495988282;
            }
            22 | 24 => {
                c2rust_current_block_33 = 8911980980495988282;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_33 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_33 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_33 = 14763689060501151050;
            }
            19 => {
                return normal_scanCharRef(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_33 {
            8911980980495988282 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_64: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_64 = 11948064939145634034;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_64 = 11948064939145634034;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_64 = 10930818133215224067;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_64 = 10930818133215224067;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_64 = 10930818133215224067;
                }
                18 => {
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_64 {
                11948064939145634034 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_scanAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0i32;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_186: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_186 = 3818392175876617014;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_186 = 3818392175876617014;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    let mut c2rust_current_block_64: u64;
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        29 => {
                            if true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            c2rust_current_block_64 = 7083593080606520045;
                        }
                        22 | 24 => {
                            c2rust_current_block_64 = 7083593080606520045;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
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
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
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
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
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
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                    }
                    match c2rust_current_block_64 {
                        7083593080606520045 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_186 = 1634947208139838470;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: ::core::ffi::c_int = 0;
                        ptr = ptr.offset(1isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        t = (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int;
                        if t == BT_EQUALS as ::core::ffi::c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                    }
                    c2rust_current_block_186 = 10853015579903106591;
                }
                14 => {
                    c2rust_current_block_186 = 10853015579903106591;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_186 {
                10853015579903106591 => {
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0i32;
                    loop {
                        ptr = ptr.offset(1isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        open = (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int;
                        if open == BT_QUOT as ::core::ffi::c_int
                            || open == BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                    }
                    ptr = ptr.offset(1isize);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        t_0 = (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int;
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid2
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid3
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if (*(enc as *const normal_encoding))
                                    .isInvalid4
                                    .expect("non-null function pointer")(
                                    enc, ptr
                                ) != 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int = normal_scanRef(
                                    enc,
                                    ptr.offset(1isize),
                                    end,
                                    &raw mut ptr,
                                );
                                if tok <= 0i32 {
                                    if tok == XML_TOK_INVALID_1 {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            _ => {
                                ptr = ptr.offset(1isize);
                            }
                        }
                    }
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(1isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 1i32)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL_1;
                                }
                                match (*(enc as *const normal_encoding)).type_0
                                    [*ptr as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int
                                {
                                    29 => {
                                        if true {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID_1;
                                        }
                                        c2rust_current_block_186 = 11210999262882855128;
                                        break;
                                    }
                                    22 | 24 => {
                                        c2rust_current_block_186 = 11210999262882855128;
                                        break;
                                    }
                                    5 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 2i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR_1;
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
                                            return XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 3i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR_1;
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
                                            return XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 4i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR_1;
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
                                            return XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        c2rust_current_block_186 = 2944436519209994553;
                                        break;
                                    }
                                    17 => {
                                        c2rust_current_block_186 = 398073151373002430;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            match c2rust_current_block_186 {
                                2944436519209994553 => {}
                                398073151373002430 => {}
                                1634947208139838470 => {}
                                _ => {
                                    ptr = ptr.offset(1isize);
                                    c2rust_current_block_186 = 1634947208139838470;
                                }
                            }
                        }
                        17 => {
                            c2rust_current_block_186 = 398073151373002430;
                        }
                        11 => {
                            c2rust_current_block_186 = 2944436519209994553;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                    }
                    match c2rust_current_block_186 {
                        1634947208139838470 => {}
                        _ => match c2rust_current_block_186 {
                            398073151373002430 => {
                                ptr = ptr.offset(1isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 1i32)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL_1;
                                }
                                if !(*ptr as ::core::ffi::c_int == 0x3ei32) {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                *nextTokPtr = ptr.offset(1isize);
                                return XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(1isize);
                                return XML_TOK_START_TAG_WITH_ATTS_1;
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
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_scanLt(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_45: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_45 = 2165477741955893522;
            }
            22 | 24 => {
                c2rust_current_block_45 = 2165477741955893522;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            16 => {
                ptr = ptr.offset(1isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    27 => {
                        return normal_scanComment(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    20 => {
                        return normal_scanCdataSection(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            15 => {
                return normal_scanPi(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            17 => {
                return normal_scanEndTag(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_45 {
            2165477741955893522 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        hadColon = 0i32;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_161: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_161 = 6701753098489376273;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_161 = 6701753098489376273;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    let mut c2rust_current_block_112: u64;
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        29 => {
                            if true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            c2rust_current_block_112 = 9169466483824547789;
                        }
                        22 | 24 => {
                            c2rust_current_block_112 = 9169466483824547789;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
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
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
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
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
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
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                    }
                    match c2rust_current_block_112 {
                        9169466483824547789 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_161 = 14714495436747744489;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(1isize);
                    loop {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            c2rust_current_block_161 = 13215501469961642988;
                            break;
                        }
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            29 => {
                                if true {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                c2rust_current_block_161 = 7939927167482451446;
                            }
                            22 | 24 => {
                                c2rust_current_block_161 = 7939927167482451446;
                            }
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
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
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
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
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
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
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            11 => {
                                c2rust_current_block_161 = 5640065479517572396;
                                break;
                            }
                            17 => {
                                c2rust_current_block_161 = 12549409781983877175;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(1isize);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                        match c2rust_current_block_161 {
                            7939927167482451446 => {
                                ptr = ptr.offset(1isize);
                            }
                            _ => {}
                        }
                        return normal_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match c2rust_current_block_161 {
                        5640065479517572396 => {}
                        12549409781983877175 => {}
                        _ => return XML_TOK_PARTIAL_1,
                    }
                }
                11 => {
                    c2rust_current_block_161 = 5640065479517572396;
                }
                17 => {
                    c2rust_current_block_161 = 12549409781983877175;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_161 {
                12549409781983877175 => {
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr as ::core::ffi::c_int == 0x3ei32) {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                }
                5640065479517572396 => {
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_START_TAG_NO_ATTS_1;
                }
                6701753098489376273 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_contentTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return XML_TOK_NONE_1;
        }
        if 1i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (1i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(1i32 - 1i32)
                    as size_t;
                if n == 0usize {
                    return XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            2 => {
                return normal_scanLt(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            3 => {
                return normal_scanRef(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            9 => {
                ptr = ptr.offset(1isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_CR_1;
                }
                if (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                    == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(1isize);
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_DATA_NEWLINE_1;
            }
            4 => {
                ptr = ptr.offset(1isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_RSQB_1;
                }
                if *ptr as ::core::ffi::c_int == 0x5di32 {
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_TRAILING_RSQB_1;
                    }
                    if !(*ptr as ::core::ffi::c_int == 0x3ei32) {
                        ptr = ptr.offset(-(1isize));
                    } else {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(1isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_76: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid2
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid3
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64
                        || (*(enc as *const normal_encoding))
                            .isInvalid4
                            .expect("non-null function pointer")(enc, ptr)
                            != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                4 => {
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 1i32)
                            as ::core::ffi::c_long
                    {
                        if !(*ptr.offset(1isize) as ::core::ffi::c_int
                            == 0x5di32)
                        {
                            ptr = ptr.offset(1isize);
                            c2rust_current_block_76 = 7158658067966855297;
                        } else if end.offset_from(ptr) as ::core::ffi::c_long
                            >= (3i32 * 1i32)
                                as ::core::ffi::c_long
                        {
                            if !(*ptr.offset(
                                (2i32 * 1i32) as isize,
                            ) as ::core::ffi::c_int
                                == 0x3ei32)
                            {
                                ptr = ptr.offset(1isize);
                            } else {
                                *nextTokPtr = ptr.offset(
                                    (2i32 * 1i32) as isize,
                                );
                                return XML_TOK_INVALID_1;
                            }
                            c2rust_current_block_76 = 7158658067966855297;
                        } else {
                            c2rust_current_block_76 = 1999360611754201214;
                        }
                    } else {
                        c2rust_current_block_76 = 1999360611754201214;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    c2rust_current_block_76 = 1999360611754201214;
                }
                _ => {
                    ptr = ptr.offset(1isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
            }
            match c2rust_current_block_76 {
                7158658067966855297 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn normal_scanPercent(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_34: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_34 = 12478441211659886388;
            }
            22 | 24 => {
                c2rust_current_block_34 = 12478441211659886388;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_34 = 4761528863920922185;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_34 = 4761528863920922185;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_34 = 4761528863920922185;
            }
            21 | 10 | 9 | 30 => {
                *nextTokPtr = ptr;
                return XML_TOK_PERCENT_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_34 {
            12478441211659886388 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_65: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_65 = 7770117754142564343;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_65 = 7770117754142564343;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_65 = 16415152177862271243;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_65 = 16415152177862271243;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_65 = 16415152177862271243;
                }
                18 => {
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_PARAM_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_65 {
                7770117754142564343 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_scanPoundName(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_32: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            29 => {
                if true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_32 = 1867613116081924762;
            }
            22 | 24 => {
                c2rust_current_block_32 = 1867613116081924762;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
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
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_32 {
            1867613116081924762 => {
                ptr = ptr.offset(1isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_63: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_63 = 226587729178875444;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_63 = 226587729178875444;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_63 = 10380409671385728102;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_63 = 10380409671385728102;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_63 = 10380409671385728102;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_POUND_NAME_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_63 {
                226587729178875444 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return -20i32;
    }

    pub unsafe extern "C" fn normal_scanLit(
        mut open: ::core::ffi::c_int,
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut t: ::core::ffi::c_int = (*(enc as *const normal_encoding)).type_0
                [*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int;
            match t {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                12 | 13 => {
                    ptr = ptr.offset(1isize);
                    if !(t != open) {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return -27i32;
                        }
                        *nextTokPtr = ptr;
                        match (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                        {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return XML_TOK_LITERAL_1
                            }
                            _ => return XML_TOK_INVALID_1,
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(1isize);
                }
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_prologTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE_1;
        }
        if 1i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (1i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(1i32 - 1i32)
                    as size_t;
                if n == 0usize {
                    return XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        let mut c2rust_current_block_124: u64;
        match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            12 => {
                return normal_scanLit(
                    BT_QUOT as ::core::ffi::c_int,
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            13 => {
                return normal_scanLit(
                    BT_APOS as ::core::ffi::c_int,
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            2 => {
                ptr = ptr.offset(1isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    16 => {
                        return normal_scanDecl(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    15 => {
                        return normal_scanPi(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(1isize));
                        return XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            9 => {
                if ptr.offset(1isize) == end {
                    *nextTokPtr = end;
                    return -15i32;
                }
                c2rust_current_block_124 = 6405334113228567422;
            }
            21 | 10 => {
                c2rust_current_block_124 = 6405334113228567422;
            }
            30 => {
                return normal_scanPercent(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            35 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_COMMA_1;
            }
            20 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_OPEN_BRACKET_1;
            }
            4 => {
                ptr = ptr.offset(1isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1i32) as ::core::ffi::c_long)
                {
                    return -26i32;
                }
                if *ptr as ::core::ffi::c_int == 0x5di32 {
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0x3ei32
                    {
                        *nextTokPtr = ptr
                            .offset((2i32 * 1i32) as isize);
                        return XML_TOK_COND_SECT_CLOSE_1;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET_1;
            }
            31 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_OPEN_PAREN_1;
            }
            32 => {
                ptr = ptr.offset(1isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 1i32) as ::core::ffi::c_long)
                {
                    return -24i32;
                }
                match (*(enc as *const normal_encoding)).type_0
                    [*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                {
                    33 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK_1;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION_1;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_CLOSE_PAREN_PLUS_1;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN_1;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            36 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_OR_1;
            }
            11 => {
                *nextTokPtr = ptr.offset(1isize);
                return XML_TOK_DECL_CLOSE_1;
            }
            19 => {
                return normal_scanPoundName(
                    enc,
                    ptr.offset(1isize),
                    end,
                    nextTokPtr,
                );
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NAME;
                } else if (*(enc as *const normal_encoding))
                    .isName2
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NMTOKEN_1;
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_124 = 2956972668325154207;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(3isize);
                    tok = XML_TOK_NAME;
                } else if (*(enc as *const normal_encoding))
                    .isName3
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(3isize);
                    tok = XML_TOK_NMTOKEN_1;
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_124 = 2956972668325154207;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if (*(enc as *const normal_encoding))
                    .isInvalid4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                if (*(enc as *const normal_encoding))
                    .isNmstrt4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(4isize);
                    tok = XML_TOK_NAME;
                } else if (*(enc as *const normal_encoding))
                    .isName4
                    .expect("non-null function pointer")(enc, ptr)
                    != 0
                {
                    ptr = ptr.offset(4isize);
                    tok = XML_TOK_NMTOKEN_1;
                } else {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_124 = 2956972668325154207;
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(1isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN_1;
                ptr = ptr.offset(1isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            29 | _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_124 {
            2956972668325154207 => {}
            _ => {
                loop {
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        break;
                    }
                    let mut c2rust_current_block_32: u64;
                    match (*(enc as *const normal_encoding)).type_0
                        [*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                    {
                        21 | 10 => {
                            c2rust_current_block_32 = 17500079516916021833;
                        }
                        9 => {
                            if ptr.offset(1isize) != end {
                                c2rust_current_block_32 = 17500079516916021833;
                            } else {
                                c2rust_current_block_32 = 3687018382384043009;
                            }
                        }
                        _ => {
                            c2rust_current_block_32 = 3687018382384043009;
                        }
                    }
                    match c2rust_current_block_32 {
                        17500079516916021833 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S_1;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_210: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                29 => {
                    if true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_210 = 17210391895989911948;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_210 = 17210391895989911948;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
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
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(1isize);
                    match  tok {
    XML_TOK_NAME =>  {
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 1i32)
                                    as ::core::ffi::c_long)
                            {
                                return XML_TOK_PARTIAL_1;
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            let mut c2rust_current_block_187: u64;
                            match (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                            {
                                29 => {
                                    if true {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                    c2rust_current_block_187 = 2692573546887820791;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    c2rust_current_block_187 = 2692573546887820791;
                                }
                                5 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 2i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR_1;
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
                                        return XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(2isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                6 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 3i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR_1;
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
                                        return XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(3isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                7 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 4i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR_1;
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
                                        return XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(4isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                _ => {
                                    tok = XML_TOK_NMTOKEN_1;
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                            }
                            match c2rust_current_block_187 {
                                2692573546887820791 => {
                                    ptr = ptr.offset(1isize);
                                }
                                _ => {}
                            }
                        }
    XML_TOK_PREFIXED_NAME =>  {
                            tok = XML_TOK_NMTOKEN_1;
                        }
    _ =>  {}
}
                    c2rust_current_block_210 = 14244298717249035578;
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_NAME_PLUS_1;
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_NAME_ASTERISK_1;
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(1isize);
                    return XML_TOK_NAME_QUESTION_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_210 {
                17210391895989911948 => {
                    ptr = ptr.offset(1isize);
                }
                _ => {}
            }
        }
        return -tok;
    }

    pub unsafe extern "C" fn normal_attributeValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
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
                        return normal_scanRef(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR_1;
                        }
                        if (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(1isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(1isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn normal_entityValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
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
                        return normal_scanRef(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int = normal_scanPercent(
                            enc,
                            ptr.offset(1isize),
                            end,
                            nextTokPtr,
                        );
                        return if tok == XML_TOK_PERCENT_1 {
                            XML_TOK_INVALID_1
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(1isize);
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(1isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR_1;
                        }
                        if (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(1isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(1isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn normal_ignoreSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut level: ::core::ffi::c_int = 0i32;
        if 1i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (1i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(1i32 - 1i32)
                    as size_t;
                end = ptr.offset(n as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid2
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid3
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if (*(enc as *const normal_encoding))
                        .isInvalid4
                        .expect("non-null function pointer")(enc, ptr)
                        != 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                2 => {
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr as ::core::ffi::c_int == 0x21i32 {
                        ptr = ptr.offset(1isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr as ::core::ffi::c_int == 0x5bi32 {
                            level += 1;
                            ptr = ptr.offset(1isize);
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(1isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr as ::core::ffi::c_int == 0x5di32 {
                        ptr = ptr.offset(1isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 1i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr as ::core::ffi::c_int == 0x3ei32 {
                            ptr = ptr.offset(1isize);
                            if level == 0i32 {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT_1;
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
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn normal_isPublicId(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        ptr = ptr.offset(1isize);
        end = end.offset(-(1isize));
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_8: u64;
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    c2rust_current_block_8 = 5143058163439228106;
                }
                21 => {
                    if *ptr as ::core::ffi::c_int == 0x9i32 {
                        *badPtr = ptr;
                        return 0i32;
                    }
                    c2rust_current_block_8 = 5143058163439228106;
                }
                26 | 22 => {
                    if *ptr as ::core::ffi::c_int & !(0x7fi32) == 0 {
                        c2rust_current_block_8 = 5143058163439228106;
                    } else {
                        c2rust_current_block_8 = 10293610489198370764;
                    }
                }
                _ => {
                    c2rust_current_block_8 = 10293610489198370764;
                }
            }
            match c2rust_current_block_8 {
                10293610489198370764 => match *ptr as ::core::ffi::c_int {
                    36 | 64 => {}
                    _ => {
                        *badPtr = ptr;
                        return 0i32;
                    }
                },
                _ => {}
            }
            ptr = ptr.offset(1isize);
        }
        return 1i32;
    }

    pub unsafe extern "C" fn normal_getAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut attsMax: ::core::ffi::c_int,
        mut atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        let mut state: crate::xmltok_impl_h::C2Rust_Unnamed_3 = crate::xmltok_impl_c::inName;
        let mut nAtts: ::core::ffi::c_int = 0i32;
        let mut open: ::core::ffi::c_int = 0i32;
        ptr = ptr.offset(1isize);
        loop {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    if  state
                        ==  crate::xmltok_impl_c::other
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh10 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh10 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName;
                    }
                    ptr = ptr.offset((2i32 - 1i32) as isize);
                }
                6 => {
                    if  state
                        ==  crate::xmltok_impl_c::other
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh11 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh11 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName;
                    }
                    ptr = ptr.offset((3i32 - 1i32) as isize);
                }
                7 => {
                    if  state
                        ==  crate::xmltok_impl_c::other
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh12 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh12 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName;
                    }
                    ptr = ptr.offset((4i32 - 1i32) as isize);
                }
                29 | 22 | 24 => {
                    if  state
                        ==  crate::xmltok_impl_c::other
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh13 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh13 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName;
                    }
                }
                12 => {
                    if  state
                        !=  crate::xmltok_impl_c::inValue
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh14 = (*atts.offset(nAtts as isize)).valuePtr;
                            *c2rust_fresh14 = ptr.offset(1isize);
                        }
                        state = crate::xmltok_impl_c::inValue;
                        open = BT_QUOT as ::core::ffi::c_int;
                    } else if open == BT_QUOT as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other;
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh15 = (*atts.offset(nAtts as isize)).valueEnd;
                            *c2rust_fresh15 = ptr;
                        }
                        nAtts += 1;
                    }
                }
                13 => {
                    if  state
                        !=  crate::xmltok_impl_c::inValue
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh16 = (*atts.offset(nAtts as isize)).valuePtr;
                            *c2rust_fresh16 = ptr.offset(1isize);
                        }
                        state = crate::xmltok_impl_c::inValue;
                        open = BT_APOS as ::core::ffi::c_int;
                    } else if open == BT_APOS as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other;
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh17 = (*atts.offset(nAtts as isize)).valueEnd;
                            *c2rust_fresh17 = ptr;
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
                    if  state
                        ==  crate::xmltok_impl_c::inName
                    {
                        state = crate::xmltok_impl_c::other;
                    } else if  state
                        ==  crate::xmltok_impl_c::inValue
                        && nAtts < attsMax
                        && (*atts.offset(nAtts as isize)).normalized as ::core::ffi::c_int != 0
                        && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                            || *ptr as ::core::ffi::c_int != ASCII_SPACE
                            || *ptr.offset(1isize) as ::core::ffi::c_int
                                == ASCII_SPACE
                            || (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                                == open)
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                9 | 10 => {
                    if  state
                        ==  crate::xmltok_impl_c::inName
                    {
                        state = crate::xmltok_impl_c::other;
                    } else if  state
                        ==  crate::xmltok_impl_c::inValue
                        && nAtts < attsMax
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                11 | 17 => {
                    if  state
                        !=  crate::xmltok_impl_c::inValue
                    {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(1isize);
        }
    }

    pub unsafe extern "C" fn normal_charRefNumber(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0i32;
        ptr = ptr.offset((2i32 * 1i32) as isize);
        if *ptr as ::core::ffi::c_int == 0x78i32 {
            ptr = ptr.offset(1isize);
            while !(*ptr as ::core::ffi::c_int == 0x3bi32) {
                let mut c: ::core::ffi::c_int = *ptr as ::core::ffi::c_int;
                match  c {
    ASCII_0 | crate::ascii_h::ASCII_1_1 | crate::ascii_h::ASCII_2_1 |
        crate::ascii_h::ASCII_3_1 | crate::ascii_h::ASCII_4 |
        crate::ascii_h::ASCII_5 | crate::ascii_h::ASCII_6 |
        crate::ascii_h::ASCII_7 | crate::ascii_h::ASCII_8_1 |
        crate::ascii_h::ASCII_9_1 =>  {
                        result <<= 4i32;
                        result |= c - ASCII_0;
                    }
    ASCII_A | crate::ascii_h::ASCII_B_1 | ASCII_C | ASCII_D |
        crate::ascii_h::ASCII_E_1 | crate::ascii_h::ASCII_F_1 =>  {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_A);
                    }
    ASCII_a_1 | crate::ascii_h::ASCII_b | crate::ascii_h::ASCII_c_1 |
        crate::ascii_h::ASCII_d | crate::ascii_h::ASCII_e_1 |
        crate::ascii_h::ASCII_f =>  {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_a_1);
                    }
    _ =>  {}
}
                if result >= 0x110000i32 {
                    return -1i32;
                }
                ptr = ptr.offset(1isize);
            }
        } else {
            while !(*ptr as ::core::ffi::c_int == 0x3bi32) {
                let mut c_0: ::core::ffi::c_int = *ptr as ::core::ffi::c_int;
                result *= 10i32;
                result += c_0 - ASCII_0;
                if result >= 0x110000i32 {
                    return -1i32;
                }
                ptr = ptr.offset(1isize);
            }
        }
        return checkCharRefNumber(result);
    }

    pub unsafe extern "C" fn normal_predefinedEntityName(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        match end.offset_from(ptr) as ::core::ffi::c_long / 1i64 {
            2 => {
                if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0x74i32
                {
                    match *ptr as ::core::ffi::c_int {
                        crate::ascii_h::ASCII_l_1 => return ASCII_LT,
                        crate::ascii_h::ASCII_g_1 => return ASCII_GT,
                        _ => {}
                    }
                }
            }
            3 => {
                if *ptr as ::core::ffi::c_int == 0x61i32 {
                    ptr = ptr.offset(1isize);
                    if *ptr as ::core::ffi::c_int == 0x6di32 {
                        ptr = ptr.offset(1isize);
                        if *ptr as ::core::ffi::c_int == 0x70i32 {
                            return ASCII_AMP;
                        }
                    }
                }
            }
            4 => match  *ptr as ::core::ffi::c_int {
    crate::ascii_h::ASCII_q =>  {
                    ptr = ptr.offset(1isize);
                    if *ptr as ::core::ffi::c_int == 0x75i32 {
                        ptr = ptr.offset(1isize);
                        if *ptr as ::core::ffi::c_int == 0x6fi32 {
                            ptr = ptr.offset(1isize);
                            if *ptr as ::core::ffi::c_int == 0x74i32 {
                                return ASCII_QUOT;
                            }
                        }
                    }
                }
    ASCII_a_1 =>  {
                    ptr = ptr.offset(1isize);
                    if *ptr as ::core::ffi::c_int == 0x70i32 {
                        ptr = ptr.offset(1isize);
                        if *ptr as ::core::ffi::c_int == 0x6fi32 {
                            ptr = ptr.offset(1isize);
                            if *ptr as ::core::ffi::c_int == 0x73i32 {
                                return ASCII_APOS;
                            }
                        }
                    }
                }
    _ =>  {}
},
            _ => {}
        }
        return 0i32;
    }

    pub unsafe extern "C" fn normal_nameMatchesAscii(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr1: *const ::core::ffi::c_char,
        mut end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while *ptr2 != 0 {
            if (end1.offset_from(ptr1) as ::core::ffi::c_long) < 1i64 {
                return 0i32;
            }
            if !(*ptr1 as ::core::ffi::c_int == *ptr2 as ::core::ffi::c_int) {
                return 0i32;
            }
            ptr1 = ptr1.offset(1isize);
            ptr2 = ptr2.offset(1);
        }
        return (ptr1 == end1) as ::core::ffi::c_int;
    }

    pub unsafe extern "C" fn normal_nameLength(
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
                    return  ptr.offset_from(start) as ::core::ffi::c_int;
                }
            }
        }
    }

    pub unsafe extern "C" fn normal_skipS(
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

    pub unsafe extern "C" fn normal_updatePosition(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut pos: *mut crate::src::lib::xmltok::POSITION,
    ) {
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 1i32) as ::core::ffi::c_long
        {
            match (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
            {
                5 => {
                    ptr = ptr.offset(2isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                10 => {
                    (*pos).columnNumber = 0u64;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(1isize);
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(1isize);
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 1i32)
                            as ::core::ffi::c_long
                        && (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                            == BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(1isize);
                    }
                    (*pos).columnNumber = 0u64;
                }
                _ => {
                    ptr = ptr.offset(1isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub unsafe extern "C" fn little2_scanComment(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            if !(*ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0x2di32)
            {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            ptr = ptr.offset(2isize);
            while end.offset_from(ptr) as ::core::ffi::c_long
                >= (1i32 * 2i32) as ::core::ffi::c_long
            {
                match if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    5 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64
                        {
                            return XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    6 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64
                        {
                            return XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    7 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64
                        {
                            return XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    27 => {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(0isize) as ::core::ffi::c_int
                                == 0x2di32
                        {
                            ptr = ptr.offset(2isize);
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 2i32)
                                    as ::core::ffi::c_long)
                            {
                                return XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr.offset(1isize)
                                as ::core::ffi::c_int
                                == 0i32
                                && *ptr.offset(0isize)
                                    as ::core::ffi::c_int
                                    == 0x3ei32)
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2isize);
                            return XML_TOK_COMMENT_1;
                        }
                    }
                    _ => {
                        ptr = ptr.offset(2isize);
                    }
                }
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_scanDecl(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            27 => {
                return little2_scanComment(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            20 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_COND_SECT_OPEN_1;
            }
            22 | 24 => {
                ptr = ptr.offset(2isize);
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            's_129: {
                match if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    30 => {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (2i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        match if *ptr
                            .offset(2isize)
                            .offset(1isize)
                            as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(2isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(2isize)
                                    .offset(1isize),
                                *ptr.offset(2isize)
                                    .offset(0isize),
                            )
                        } {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(2isize);
                        break 's_129;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_DECL_OPEN_1;
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_checkPiTarget(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut tokPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0i32;
        *tokPtr = XML_TOK_PI_1;
        if end.offset_from(ptr) as ::core::ffi::c_long
            != (2i32 * 3i32) as ::core::ffi::c_long
        {
            return 1i32;
        }
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            *ptr.offset(0isize) as ::core::ffi::c_int
        } else {
            -1i32
        } {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1i32;
            }
            _ => return 1i32,
        }
        ptr = ptr.offset(2isize);
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            *ptr.offset(0isize) as ::core::ffi::c_int
        } else {
            -1i32
        } {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1i32;
            }
            _ => return 1i32,
        }
        ptr = ptr.offset(2isize);
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            *ptr.offset(0isize) as ::core::ffi::c_int
        } else {
            -1i32
        } {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1i32;
            }
            _ => return 1i32,
        }
        if upper != 0 {
            return 0i32;
        }
        *tokPtr = XML_TOK_XML_DECL_1;
        return 1i32;
    }

    pub unsafe extern "C" fn little2_scanPi(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        let mut target: *const ::core::ffi::c_char = ptr;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_32: u64;
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_32 = 14358794669692889688;
            }
            22 | 24 => {
                c2rust_current_block_32 = 14358794669692889688;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_32 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_32 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_32 = 14763689060501151050;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_32 {
            14358794669692889688 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_118: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_118 = 15890151712677504458;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_118 = 15890151712677504458;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_118 = 13349765058737954042;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_118 = 13349765058737954042;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_118 = 13349765058737954042;
                }
                21 | 9 | 10 => {
                    if little2_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long
                    {
                        match if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        } {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            15 => {
                                ptr = ptr.offset(2isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2i32)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(1isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                    && *ptr.offset(0isize)
                                        as ::core::ffi::c_int
                                        == 0x3ei32
                                {
                                    *nextTokPtr = ptr.offset(2isize);
                                    return tok;
                                }
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                    return XML_TOK_PARTIAL_1;
                }
                15 => {
                    if little2_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0x3ei32
                    {
                        *nextTokPtr = ptr.offset(2isize);
                        return tok;
                    }
                    c2rust_current_block_118 = 7312756018063861309;
                }
                _ => {
                    c2rust_current_block_118 = 7312756018063861309;
                }
            }
            match c2rust_current_block_118 {
                7312756018063861309 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                15890151712677504458 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_scanCdataSection(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        pub static mut CDATA_LSQB: [::core::ffi::c_char; 6] = [
            ASCII_C as ::core::ffi::c_char,
            ASCII_D as ::core::ffi::c_char,
            ASCII_A as ::core::ffi::c_char,
            ASCII_T as ::core::ffi::c_char,
            ASCII_A as ::core::ffi::c_char,
            ASCII_LSQB as ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (6i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        i = 0i32;
        while i < 6i32 {
            if !(*ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(0isize) as ::core::ffi::c_int
                    == CDATA_LSQB[i as usize] as ::core::ffi::c_int)
            {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            i += 1;
            ptr = ptr.offset(2isize);
        }
        *nextTokPtr = ptr;
        return XML_TOK_CDATA_SECT_OPEN_1;
    }

    pub unsafe extern "C" fn little2_cdataSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return XML_TOK_NONE_1;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (2i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(2i32 - 1i32)
                    as size_t;
                if n == 0usize {
                    return XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                    && *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0x5di32
                {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0x3ei32)
                    {
                        ptr = ptr.offset(-(2isize));
                    } else {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CDATA_SECT_CLOSE_1;
                    }
                }
            }
            9 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                if (if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                }) == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2isize);
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DATA_NEWLINE_1;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(2isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn little2_scanEndTag(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_32: u64;
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_32 = 8654814784450400207;
            }
            22 | 24 => {
                c2rust_current_block_32 = 8654814784450400207;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_32 {
            8654814784450400207 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_73: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_73 = 16411184819389759620;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_73 = 16411184819389759620;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2isize);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long
                    {
                        match if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        } {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_END_TAG_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(2isize);
                    }
                    return XML_TOK_PARTIAL_1;
                }
                23 => {
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                11 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_END_TAG_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_73 {
                16411184819389759620 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_scanHexCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2isize);
            while end.offset_from(ptr) as ::core::ffi::c_long
                >= (1i32 * 2i32) as ::core::ffi::c_long
            {
                match if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2isize);
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_scanCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0x78i32
            {
                return little2_scanHexCharRef(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2isize);
            while end.offset_from(ptr) as ::core::ffi::c_long
                >= (1i32 * 2i32) as ::core::ffi::c_long
            {
                match if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2isize);
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_scanRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_33: u64;
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_33 = 6679362556518655255;
            }
            22 | 24 => {
                c2rust_current_block_33 = 6679362556518655255;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_33 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_33 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_33 = 14763689060501151050;
            }
            19 => {
                return little2_scanCharRef(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_33 {
            6679362556518655255 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_64: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_64 = 405996089697802199;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_64 = 405996089697802199;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_64 = 10930818133215224067;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_64 = 10930818133215224067;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_64 = 10930818133215224067;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_64 {
                405996089697802199 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_scanAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0i32;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_186: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_186 = 17747718632989559416;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_186 = 17747718632989559416;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    let mut c2rust_current_block_64: u64;
                    match if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1isize),
                            *ptr.offset(0isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize]
                                as ::core::ffi::c_int)
                                << 3i32)
                                + (*ptr.offset(0isize)
                                    as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    >> 5i32))
                                as usize]
                                & (1u32)
                                    << (*ptr.offset(0isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1fi32)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            c2rust_current_block_64 = 12531724302225488581;
                        }
                        22 | 24 => {
                            c2rust_current_block_64 = 12531724302225488581;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                    }
                    match c2rust_current_block_64 {
                        12531724302225488581 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_186 = 1634947208139838470;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: ::core::ffi::c_int = 0;
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        t = if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        };
                        if t == BT_EQUALS as ::core::ffi::c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                    }
                    c2rust_current_block_186 = 10853015579903106591;
                }
                14 => {
                    c2rust_current_block_186 = 10853015579903106591;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_186 {
                10853015579903106591 => {
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0i32;
                    loop {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        open = if *ptr.offset(1isize)
                            as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        };
                        if open == BT_QUOT as ::core::ffi::c_int
                            || open == BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                    }
                    ptr = ptr.offset(2isize);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        t_0 = if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        };
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int = little2_scanRef(
                                    enc,
                                    ptr.offset(2isize),
                                    end,
                                    &raw mut ptr,
                                );
                                if tok <= 0i32 {
                                    if tok == XML_TOK_INVALID_1 {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    match if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1isize),
                            *ptr.offset(0isize),
                        )
                    } {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(2isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2i32)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL_1;
                                }
                                match if *ptr.offset(1isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                {
                                    (*(enc as *const normal_encoding)).type_0
                                        [*ptr as ::core::ffi::c_uchar as usize]
                                        as ::core::ffi::c_int
                                } else {
                                    unicode_byte_type(
                                        *ptr.offset(1isize),
                                        *ptr.offset(0isize),
                                    )
                                } {
                                    29 => {
                                        if namingBitmap[(((nmstrtPages[*ptr
                                            .offset(1isize)
                                            as ::core::ffi::c_uchar
                                            as usize]
                                            as ::core::ffi::c_int)
                                            << 3i32)
                                            + (*ptr.offset(0isize)
                                                as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                >> 5i32))
                                            as usize]
                                            & (1u32)
                                                << (*ptr.offset(0isize)
                                                    as ::core::ffi::c_uchar
                                                    as ::core::ffi::c_int
                                                    & 0x1fi32)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID_1;
                                        }
                                        c2rust_current_block_186 = 923465642386550266;
                                        break;
                                    }
                                    22 | 24 => {
                                        c2rust_current_block_186 = 923465642386550266;
                                        break;
                                    }
                                    5 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 2i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if false || true {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 3i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if false || true {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 4i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if false || true {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        c2rust_current_block_186 = 15103464935601583148;
                                        break;
                                    }
                                    17 => {
                                        c2rust_current_block_186 = 619033562305054167;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            match c2rust_current_block_186 {
                                15103464935601583148 => {}
                                619033562305054167 => {}
                                1634947208139838470 => {}
                                _ => {
                                    ptr = ptr.offset(2isize);
                                    c2rust_current_block_186 = 1634947208139838470;
                                }
                            }
                        }
                        17 => {
                            c2rust_current_block_186 = 619033562305054167;
                        }
                        11 => {
                            c2rust_current_block_186 = 15103464935601583148;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                    }
                    match c2rust_current_block_186 {
                        1634947208139838470 => {}
                        _ => match c2rust_current_block_186 {
                            619033562305054167 => {
                                ptr = ptr.offset(2isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2i32)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL_1;
                                }
                                if !(*ptr.offset(1isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                    && *ptr.offset(0isize)
                                        as ::core::ffi::c_int
                                        == 0x3ei32)
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_START_TAG_WITH_ATTS_1;
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
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_scanLt(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_45: u64;
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_45 = 18046087305847344724;
            }
            22 | 24 => {
                c2rust_current_block_45 = 18046087305847344724;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            16 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    27 => {
                        return little2_scanComment(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    20 => {
                        return little2_scanCdataSection(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            15 => {
                return little2_scanPi(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            17 => {
                return little2_scanEndTag(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_45 {
            18046087305847344724 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        hadColon = 0i32;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_161: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_161 = 8998928240368606981;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_161 = 8998928240368606981;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    let mut c2rust_current_block_112: u64;
                    match if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1isize),
                            *ptr.offset(0isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize]
                                as ::core::ffi::c_int)
                                << 3i32)
                                + (*ptr.offset(0isize)
                                    as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    >> 5i32))
                                as usize]
                                & (1u32)
                                    << (*ptr.offset(0isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1fi32)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            c2rust_current_block_112 = 14391208795021697965;
                        }
                        22 | 24 => {
                            c2rust_current_block_112 = 14391208795021697965;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                    }
                    match c2rust_current_block_112 {
                        14391208795021697965 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_161 = 14714495436747744489;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2isize);
                    loop {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            c2rust_current_block_161 = 13215501469961642988;
                            break;
                        }
                        match if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        } {
                            29 => {
                                if namingBitmap[(((nmstrtPages[*ptr
                                    .offset(1isize)
                                    as ::core::ffi::c_uchar
                                    as usize]
                                    as ::core::ffi::c_int)
                                    << 3i32)
                                    + (*ptr.offset(0isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        >> 5i32))
                                    as usize]
                                    & (1u32)
                                        << (*ptr.offset(0isize)
                                            as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            & 0x1fi32)
                                    == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                c2rust_current_block_161 = 2369392326157537288;
                            }
                            22 | 24 => {
                                c2rust_current_block_161 = 2369392326157537288;
                            }
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if false || true {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if false || true {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if false || true {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            11 => {
                                c2rust_current_block_161 = 1918622160084604696;
                                break;
                            }
                            17 => {
                                c2rust_current_block_161 = 1114269873380682160;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(2isize);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                        match c2rust_current_block_161 {
                            2369392326157537288 => {
                                ptr = ptr.offset(2isize);
                            }
                            _ => {}
                        }
                        return little2_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match c2rust_current_block_161 {
                        1918622160084604696 => {}
                        1114269873380682160 => {}
                        _ => return XML_TOK_PARTIAL_1,
                    }
                }
                11 => {
                    c2rust_current_block_161 = 1918622160084604696;
                }
                17 => {
                    c2rust_current_block_161 = 1114269873380682160;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_161 {
                1114269873380682160 => {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0x3ei32)
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                }
                1918622160084604696 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_START_TAG_NO_ATTS_1;
                }
                8998928240368606981 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_contentTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return XML_TOK_NONE_1;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (2i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(2i32 - 1i32)
                    as size_t;
                if n == 0usize {
                    return XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            2 => {
                return little2_scanLt(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            3 => {
                return little2_scanRef(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            9 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_CR_1;
                }
                if (if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                }) == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2isize);
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DATA_NEWLINE_1;
            }
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_RSQB_1;
                }
                if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                    && *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0x5di32
                {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_TRAILING_RSQB_1;
                    }
                    if !(*ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0x3ei32)
                    {
                        ptr = ptr.offset(-(2isize));
                    } else {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(2isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_76: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                4 => {
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 2i32)
                            as ::core::ffi::c_long
                    {
                        if !(*ptr
                            .offset(2isize)
                            .offset(1isize)
                            as ::core::ffi::c_int
                            == 0i32
                            && *ptr
                                .offset(2isize)
                                .offset(0isize)
                                as ::core::ffi::c_int
                                == 0x5di32)
                        {
                            ptr = ptr.offset(2isize);
                            c2rust_current_block_76 = 7158658067966855297;
                        } else if end.offset_from(ptr) as ::core::ffi::c_long
                            >= (3i32 * 2i32)
                                as ::core::ffi::c_long
                        {
                            if !(*ptr
                                .offset(
                                    (2i32 * 2i32) as isize,
                                )
                                .offset(1isize)
                                as ::core::ffi::c_int
                                == 0i32
                                && *ptr
                                    .offset(
                                        (2i32 * 2i32)
                                            as isize,
                                    )
                                    .offset(0isize)
                                    as ::core::ffi::c_int
                                    == 0x3ei32)
                            {
                                ptr = ptr.offset(2isize);
                            } else {
                                *nextTokPtr = ptr.offset(
                                    (2i32 * 2i32) as isize,
                                );
                                return XML_TOK_INVALID_1;
                            }
                            c2rust_current_block_76 = 7158658067966855297;
                        } else {
                            c2rust_current_block_76 = 17804070343020517427;
                        }
                    } else {
                        c2rust_current_block_76 = 17804070343020517427;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    c2rust_current_block_76 = 17804070343020517427;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
            }
            match c2rust_current_block_76 {
                7158658067966855297 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn little2_scanPercent(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_34: u64;
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_34 = 27123471380826226;
            }
            22 | 24 => {
                c2rust_current_block_34 = 27123471380826226;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_34 = 4761528863920922185;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_34 = 4761528863920922185;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_34 = 4761528863920922185;
            }
            21 | 10 | 9 | 30 => {
                *nextTokPtr = ptr;
                return XML_TOK_PERCENT_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_34 {
            27123471380826226 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_65: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_65 = 8394962855094477842;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_65 = 8394962855094477842;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_65 = 16415152177862271243;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_65 = 16415152177862271243;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_65 = 16415152177862271243;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_PARAM_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_65 {
                8394962855094477842 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_scanPoundName(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_32: u64;
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_32 = 14940290876465470105;
            }
            22 | 24 => {
                c2rust_current_block_32 = 14940290876465470105;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_32 {
            14940290876465470105 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_63: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_63 = 11497795575834122789;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_63 = 11497795575834122789;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_63 = 10380409671385728102;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_63 = 10380409671385728102;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_63 = 10380409671385728102;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_POUND_NAME_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_63 {
                11497795575834122789 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return -20i32;
    }

    pub unsafe extern "C" fn little2_scanLit(
        mut open: ::core::ffi::c_int,
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut t: ::core::ffi::c_int = if *ptr.offset(1isize)
                as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            };
            match t {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                12 | 13 => {
                    ptr = ptr.offset(2isize);
                    if !(t != open) {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return -27i32;
                        }
                        *nextTokPtr = ptr;
                        match if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        } {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return XML_TOK_LITERAL_1
                            }
                            _ => return XML_TOK_INVALID_1,
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_prologTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE_1;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (2i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(2i32 - 1i32)
                    as size_t;
                if n == 0usize {
                    return XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        let mut c2rust_current_block_124: u64;
        match if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(1isize),
                *ptr.offset(0isize),
            )
        } {
            12 => {
                return little2_scanLit(
                    BT_QUOT as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            13 => {
                return little2_scanLit(
                    BT_APOS as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            2 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    16 => {
                        return little2_scanDecl(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    15 => {
                        return little2_scanPi(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(2isize));
                        return XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            9 => {
                if ptr.offset(2isize) == end {
                    *nextTokPtr = end;
                    return -15i32;
                }
                c2rust_current_block_124 = 17513858719706519675;
            }
            21 | 10 => {
                c2rust_current_block_124 = 17513858719706519675;
            }
            30 => {
                return little2_scanPercent(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            35 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_COMMA_1;
            }
            20 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OPEN_BRACKET_1;
            }
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return -26i32;
                }
                if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                    && *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0x5di32
                {
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr
                        .offset(2isize)
                        .offset(1isize)
                        as ::core::ffi::c_int
                        == 0i32
                        && *ptr
                            .offset(2isize)
                            .offset(0isize)
                            as ::core::ffi::c_int
                            == 0x3ei32
                    {
                        *nextTokPtr = ptr
                            .offset((2i32 * 2i32) as isize);
                        return XML_TOK_COND_SECT_CLOSE_1;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET_1;
            }
            31 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OPEN_PAREN_1;
            }
            32 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return -24i32;
                }
                match if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                        as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(1isize),
                        *ptr.offset(0isize),
                    )
                } {
                    33 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK_1;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION_1;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_PLUS_1;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN_1;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            36 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OR_1;
            }
            11 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DECL_CLOSE_1;
            }
            19 => {
                return little2_scanPoundName(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(2isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN_1;
                ptr = ptr.offset(2isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NAME;
                    c2rust_current_block_124 = 2956972668325154207;
                } else if namingBitmap[(((namePages
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NMTOKEN_1;
                    c2rust_current_block_124 = 2956972668325154207;
                } else {
                    c2rust_current_block_124 = 2543942683527618915;
                }
            }
            _ => {
                c2rust_current_block_124 = 2543942683527618915;
            }
        }
        match c2rust_current_block_124 {
            2956972668325154207 => {}
            17513858719706519675 => {
                loop {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        break;
                    }
                    let mut c2rust_current_block_32: u64;
                    match if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        (*(enc as *const normal_encoding)).type_0
                            [*ptr as ::core::ffi::c_uchar as usize]
                            as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(1isize),
                            *ptr.offset(0isize),
                        )
                    } {
                        21 | 10 => {
                            c2rust_current_block_32 = 17500079516916021833;
                        }
                        9 => {
                            if ptr.offset(2isize) != end {
                                c2rust_current_block_32 = 17500079516916021833;
                            } else {
                                c2rust_current_block_32 = 17471725448347076649;
                            }
                        }
                        _ => {
                            c2rust_current_block_32 = 17471725448347076649;
                        }
                    }
                    match c2rust_current_block_32 {
                        17500079516916021833 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S_1;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_210: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(0isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(0isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_210 = 786388639404123072;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_210 = 786388639404123072;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(2isize);
                    match  tok {
    XML_TOK_NAME =>  {
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 2i32)
                                    as ::core::ffi::c_long)
                            {
                                return XML_TOK_PARTIAL_1;
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            let mut c2rust_current_block_187: u64;
                            match if *ptr.offset(1isize)
                                as ::core::ffi::c_int
                                == 0i32
                            {
                                (*(enc as *const normal_encoding)).type_0
                                    [*ptr as ::core::ffi::c_uchar as usize]
                                    as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(1isize),
                                    *ptr.offset(0isize),
                                )
                            } {
                                29 => {
                                    if namingBitmap[(((namePages[*ptr
                                        .offset(1isize)
                                        as ::core::ffi::c_uchar
                                        as usize]
                                        as ::core::ffi::c_int)
                                        << 3i32)
                                        + (*ptr.offset(0isize)
                                            as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            >> 5i32))
                                        as usize]
                                        & (1u32)
                                            << (*ptr.offset(0isize)
                                                as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                & 0x1fi32)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                    c2rust_current_block_187 = 16869951820887225088;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    c2rust_current_block_187 = 16869951820887225088;
                                }
                                5 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 2i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if false || true {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(2isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                6 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 3i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if false || true {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(3isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                7 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 4i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if false || true {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(4isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                _ => {
                                    tok = XML_TOK_NMTOKEN_1;
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                            }
                            match c2rust_current_block_187 {
                                16869951820887225088 => {
                                    ptr = ptr.offset(2isize);
                                }
                                _ => {}
                            }
                        }
    XML_TOK_PREFIXED_NAME =>  {
                            tok = XML_TOK_NMTOKEN_1;
                        }
    _ =>  {}
}
                    c2rust_current_block_210 = 14244298717249035578;
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_PLUS_1;
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_ASTERISK_1;
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_QUESTION_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_210 {
                786388639404123072 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return -tok;
    }

    pub unsafe extern "C" fn little2_attributeValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
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
                        return little2_scanRef(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn little2_entityValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
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
                        return little2_scanRef(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int = little2_scanPercent(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                        return if tok == XML_TOK_PERCENT_1 {
                            XML_TOK_INVALID_1
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn little2_ignoreSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut level: ::core::ffi::c_int = 0i32;
        if 2i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (2i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(2i32 - 1i32)
                    as size_t;
                end = ptr.offset(n as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                2 => {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0x21i32
                    {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(0isize) as ::core::ffi::c_int
                                == 0x5bi32
                        {
                            level += 1;
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0x5di32
                    {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(0isize) as ::core::ffi::c_int
                                == 0x3ei32
                        {
                            ptr = ptr.offset(2isize);
                            if level == 0i32 {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT_1;
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
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn little2_isPublicId(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        ptr = ptr.offset(2isize);
        end = end.offset(-(2isize));
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_8: u64;
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    c2rust_current_block_8 = 5143058163439228106;
                }
                21 => {
                    if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0x9i32
                    {
                        *badPtr = ptr;
                        return 0i32;
                    }
                    c2rust_current_block_8 = 5143058163439228106;
                }
                26 | 22 => {
                    if (if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        *ptr.offset(0isize) as ::core::ffi::c_int
                    } else {
                        -1i32
                    }) & !(0x7fi32)
                        == 0
                    {
                        c2rust_current_block_8 = 5143058163439228106;
                    } else {
                        c2rust_current_block_8 = 5251475129761746025;
                    }
                }
                _ => {
                    c2rust_current_block_8 = 5251475129761746025;
                }
            }
            match c2rust_current_block_8 {
                5251475129761746025 => {
                    match if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        *ptr.offset(0isize) as ::core::ffi::c_int
                    } else {
                        -1i32
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
            ptr = ptr.offset(2isize);
        }
        return 1i32;
    }

    pub unsafe extern "C" fn little2_getAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut attsMax: ::core::ffi::c_int,
        mut atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        let mut state: crate::xmltok_impl_h::C2Rust_Unnamed_3 = crate::xmltok_impl_c::inName_0;
        let mut nAtts: ::core::ffi::c_int = 0i32;
        let mut open: ::core::ffi::c_int = 0i32;
        ptr = ptr.offset(2isize);
        loop {
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => {
                    if  state
                        ==  crate::xmltok_impl_c::other_0
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh29 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh29 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_0;
                    }
                    ptr = ptr.offset((2i32 - 2i32) as isize);
                }
                6 => {
                    if  state
                        ==  crate::xmltok_impl_c::other_0
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh30 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh30 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_0;
                    }
                    ptr = ptr.offset((3i32 - 2i32) as isize);
                }
                7 => {
                    if  state
                        ==  crate::xmltok_impl_c::other_0
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh31 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh31 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_0;
                    }
                    ptr = ptr.offset((4i32 - 2i32) as isize);
                }
                29 | 22 | 24 => {
                    if  state
                        ==  crate::xmltok_impl_c::other_0
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh32 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh32 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_0;
                    }
                }
                12 => {
                    if  state
                        !=  crate::xmltok_impl_c::inValue_0
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh33 = (*atts.offset(nAtts as isize)).valuePtr;
                            *c2rust_fresh33 = ptr.offset(2isize);
                        }
                        state = crate::xmltok_impl_c::inValue_0;
                        open = BT_QUOT as ::core::ffi::c_int;
                    } else if open == BT_QUOT as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other_0;
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh34 = (*atts.offset(nAtts as isize)).valueEnd;
                            *c2rust_fresh34 = ptr;
                        }
                        nAtts += 1;
                    }
                }
                13 => {
                    if  state
                        !=  crate::xmltok_impl_c::inValue_0
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh35 = (*atts.offset(nAtts as isize)).valuePtr;
                            *c2rust_fresh35 = ptr.offset(2isize);
                        }
                        state = crate::xmltok_impl_c::inValue_0;
                        open = BT_APOS as ::core::ffi::c_int;
                    } else if open == BT_APOS as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other_0;
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh36 = (*atts.offset(nAtts as isize)).valueEnd;
                            *c2rust_fresh36 = ptr;
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
                    if  state
                        ==  crate::xmltok_impl_c::inName_0
                    {
                        state = crate::xmltok_impl_c::other_0;
                    } else if  state
                        ==  crate::xmltok_impl_c::inValue_0
                        && nAtts < attsMax
                        && (*atts.offset(nAtts as isize)).normalized as ::core::ffi::c_int != 0
                        && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                            || (if *ptr.offset(1isize)
                                as ::core::ffi::c_int
                                == 0i32
                            {
                                *ptr.offset(0isize) as ::core::ffi::c_int
                            } else {
                                -1i32
                            }) != ASCII_SPACE
                            || (if *ptr
                                .offset(2isize)
                                .offset(1isize)
                                as ::core::ffi::c_int
                                == 0i32
                            {
                                *ptr.offset(2isize)
                                    .offset(0isize)
                                    as ::core::ffi::c_int
                            } else {
                                -1i32
                            }) == ASCII_SPACE
                            || (if *ptr
                                .offset(2isize)
                                .offset(1isize)
                                as ::core::ffi::c_int
                                == 0i32
                            {
                                (*(enc as *const normal_encoding)).type_0[*ptr
                                    .offset(2isize)
                                    as ::core::ffi::c_uchar
                                    as usize] as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(2isize)
                                        .offset(1isize),
                                    *ptr.offset(2isize)
                                        .offset(0isize),
                                )
                            }) == open)
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                9 | 10 => {
                    if  state
                        ==  crate::xmltok_impl_c::inName_0
                    {
                        state = crate::xmltok_impl_c::other_0;
                    } else if  state
                        ==  crate::xmltok_impl_c::inValue_0
                        && nAtts < attsMax
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                11 | 17 => {
                    if  state
                        !=  crate::xmltok_impl_c::inValue_0
                    {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2isize);
        }
    }

    pub unsafe extern "C" fn little2_charRefNumber(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0i32;
        ptr = ptr.offset((2i32 * 2i32) as isize);
        if *ptr.offset(1isize) as ::core::ffi::c_int
            == 0i32
            && *ptr.offset(0isize) as ::core::ffi::c_int
                == 0x78i32
        {
            ptr = ptr.offset(2isize);
            while !(*ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0x3bi32)
            {
                let mut c: ::core::ffi::c_int = if *ptr.offset(1isize)
                    as ::core::ffi::c_int
                    == 0i32
                {
                    *ptr.offset(0isize) as ::core::ffi::c_int
                } else {
                    -1i32
                };
                match  c {
    ASCII_0 | crate::ascii_h::ASCII_1_1 | crate::ascii_h::ASCII_2_1 |
        crate::ascii_h::ASCII_3_1 | crate::ascii_h::ASCII_4 |
        crate::ascii_h::ASCII_5 | crate::ascii_h::ASCII_6 |
        crate::ascii_h::ASCII_7 | crate::ascii_h::ASCII_8_1 |
        crate::ascii_h::ASCII_9_1 =>  {
                        result <<= 4i32;
                        result |= c - ASCII_0;
                    }
    ASCII_A | crate::ascii_h::ASCII_B_1 | ASCII_C | ASCII_D |
        crate::ascii_h::ASCII_E_1 | crate::ascii_h::ASCII_F_1 =>  {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_A);
                    }
    ASCII_a_1 | crate::ascii_h::ASCII_b | crate::ascii_h::ASCII_c_1 |
        crate::ascii_h::ASCII_d | crate::ascii_h::ASCII_e_1 |
        crate::ascii_h::ASCII_f =>  {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_a_1);
                    }
    _ =>  {}
}
                if result >= 0x110000i32 {
                    return -1i32;
                }
                ptr = ptr.offset(2isize);
            }
        } else {
            while !(*ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0x3bi32)
            {
                let mut c_0: ::core::ffi::c_int = if *ptr.offset(1isize)
                    as ::core::ffi::c_int
                    == 0i32
                {
                    *ptr.offset(0isize) as ::core::ffi::c_int
                } else {
                    -1i32
                };
                result *= 10i32;
                result += c_0 - ASCII_0;
                if result >= 0x110000i32 {
                    return -1i32;
                }
                ptr = ptr.offset(2isize);
            }
        }
        return checkCharRefNumber(result);
    }

    pub unsafe extern "C" fn little2_predefinedEntityName(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        match end.offset_from(ptr) as ::core::ffi::c_long / 2i64 {
            2 => {
                if *ptr
                    .offset(2isize)
                    .offset(1isize)
                    as ::core::ffi::c_int
                    == 0i32
                    && *ptr
                        .offset(2isize)
                        .offset(0isize)
                        as ::core::ffi::c_int
                        == 0x74i32
                {
                    match if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        *ptr.offset(0isize) as ::core::ffi::c_int
                    } else {
                        -1i32
                    } {
                        crate::ascii_h::ASCII_l_1 => return ASCII_LT,
                        crate::ascii_h::ASCII_g_1 => return ASCII_GT,
                        _ => {}
                    }
                }
            }
            3 => {
                if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                    && *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0x61i32
                {
                    ptr = ptr.offset(2isize);
                    if *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0x6di32
                    {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(0isize) as ::core::ffi::c_int
                                == 0x70i32
                        {
                            return ASCII_AMP;
                        }
                    }
                }
            }
            4 => {
                match  if *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0i32
                {
                    *ptr.offset(0isize) as ::core::ffi::c_int
                } else {
                    -1i32
                } {
    crate::ascii_h::ASCII_q =>  {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(0isize) as ::core::ffi::c_int
                                == 0x75i32
                        {
                            ptr = ptr.offset(2isize);
                            if *ptr.offset(1isize) as ::core::ffi::c_int
                                == 0i32
                                && *ptr.offset(0isize)
                                    as ::core::ffi::c_int
                                    == 0x6fi32
                            {
                                ptr = ptr.offset(2isize);
                                if *ptr.offset(1isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                    && *ptr.offset(0isize)
                                        as ::core::ffi::c_int
                                        == 0x74i32
                                {
                                    return ASCII_QUOT;
                                }
                            }
                        }
                    }
    ASCII_a_1 =>  {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(0isize) as ::core::ffi::c_int
                                == 0x70i32
                        {
                            ptr = ptr.offset(2isize);
                            if *ptr.offset(1isize) as ::core::ffi::c_int
                                == 0i32
                                && *ptr.offset(0isize)
                                    as ::core::ffi::c_int
                                    == 0x6fi32
                            {
                                ptr = ptr.offset(2isize);
                                if *ptr.offset(1isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                    && *ptr.offset(0isize)
                                        as ::core::ffi::c_int
                                        == 0x73i32
                                {
                                    return ASCII_APOS;
                                }
                            }
                        }
                    }
    _ =>  {}
}
            }
            _ => {}
        }
        return 0i32;
    }

    pub unsafe extern "C" fn little2_nameMatchesAscii(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr1: *const ::core::ffi::c_char,
        mut end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while *ptr2 != 0 {
            if (end1.offset_from(ptr1) as ::core::ffi::c_long) < 2i64 {
                return 0i32;
            }
            if !(*ptr1.offset(1isize) as ::core::ffi::c_int
                == 0i32
                && *ptr1.offset(0isize) as ::core::ffi::c_int
                    == *ptr2 as ::core::ffi::c_int)
            {
                return 0i32;
            }
            ptr1 = ptr1.offset(2isize);
            ptr2 = ptr2.offset(1);
        }
        return (ptr1 == end1) as ::core::ffi::c_int;
    }

    pub unsafe extern "C" fn little2_nameLength(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
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
                    return  ptr.offset_from(start) as ::core::ffi::c_int;
                }
            }
        }
    }

    pub unsafe extern "C" fn little2_skipS(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        loop {
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                10 | 9 | 21 => {
                    ptr = ptr.offset(2isize);
                }
                _ => return ptr,
            }
        }
    }

    pub unsafe extern "C" fn little2_updatePosition(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut pos: *mut crate::src::lib::xmltok::POSITION,
    ) {
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(1isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0[*ptr as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(1isize),
                    *ptr.offset(0isize),
                )
            } {
                5 => {
                    ptr = ptr.offset(2isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                10 => {
                    (*pos).columnNumber = 0u64;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize);
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize);
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long
                        && (if *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0
                                [*ptr as ::core::ffi::c_uchar as usize]
                                as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(1isize),
                                *ptr.offset(0isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(2isize);
                    }
                    (*pos).columnNumber = 0u64;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
            }
        }
    }

    pub unsafe extern "C" fn big2_scanComment(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            if !(*ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0x2di32)
            {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            ptr = ptr.offset(2isize);
            while end.offset_from(ptr) as ::core::ffi::c_long
                >= (1i32 * 2i32) as ::core::ffi::c_long
            {
                match if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    5 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64
                        {
                            return XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    6 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64
                        {
                            return XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    7 => {
                        if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64
                        {
                            return XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    0 | 1 | 8 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    27 => {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(1isize) as ::core::ffi::c_int
                                == 0x2di32
                        {
                            ptr = ptr.offset(2isize);
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 2i32)
                                    as ::core::ffi::c_long)
                            {
                                return XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr.offset(0isize)
                                as ::core::ffi::c_int
                                == 0i32
                                && *ptr.offset(1isize)
                                    as ::core::ffi::c_int
                                    == 0x3ei32)
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2isize);
                            return XML_TOK_COMMENT_1;
                        }
                    }
                    _ => {
                        ptr = ptr.offset(2isize);
                    }
                }
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_scanDecl(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            27 => {
                return big2_scanComment(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            20 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_COND_SECT_OPEN_1;
            }
            22 | 24 => {
                ptr = ptr.offset(2isize);
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            's_129: {
                match if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    30 => {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (2i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        match if *ptr
                            .offset(2isize)
                            .offset(0isize)
                            as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(2isize)
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(2isize)
                                    .offset(0isize),
                                *ptr.offset(2isize)
                                    .offset(1isize),
                            )
                        } {
                            21 | 9 | 10 | 30 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            _ => {}
                        }
                    }
                    21 | 9 | 10 => {}
                    22 | 24 => {
                        ptr = ptr.offset(2isize);
                        break 's_129;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_DECL_OPEN_1;
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_checkPiTarget(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut tokPtr: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut upper: ::core::ffi::c_int = 0i32;
        *tokPtr = XML_TOK_PI_1;
        if end.offset_from(ptr) as ::core::ffi::c_long
            != (2i32 * 3i32) as ::core::ffi::c_long
        {
            return 1i32;
        }
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            *ptr.offset(1isize) as ::core::ffi::c_int
        } else {
            -1i32
        } {
            crate::ascii_h::ASCII_x_1 => {}
            crate::ascii_h::ASCII_X_1 => {
                upper = 1i32;
            }
            _ => return 1i32,
        }
        ptr = ptr.offset(2isize);
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            *ptr.offset(1isize) as ::core::ffi::c_int
        } else {
            -1i32
        } {
            crate::ascii_h::ASCII_m_1 => {}
            crate::ascii_h::ASCII_M_1 => {
                upper = 1i32;
            }
            _ => return 1i32,
        }
        ptr = ptr.offset(2isize);
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            *ptr.offset(1isize) as ::core::ffi::c_int
        } else {
            -1i32
        } {
            crate::ascii_h::ASCII_l_1 => {}
            crate::ascii_h::ASCII_L_1 => {
                upper = 1i32;
            }
            _ => return 1i32,
        }
        if upper != 0 {
            return 0i32;
        }
        *tokPtr = XML_TOK_XML_DECL_1;
        return 1i32;
    }

    pub unsafe extern "C" fn big2_scanPi(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        let mut target: *const ::core::ffi::c_char = ptr;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_32: u64;
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_32 = 2802485987355401260;
            }
            22 | 24 => {
                c2rust_current_block_32 = 2802485987355401260;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_32 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_32 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_32 = 14763689060501151050;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_32 {
            2802485987355401260 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_118: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_118 = 11190361564366887465;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_118 = 11190361564366887465;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_118 = 13349765058737954042;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_118 = 13349765058737954042;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_118 = 13349765058737954042;
                }
                21 | 9 | 10 => {
                    if big2_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long
                    {
                        match if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        } {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            15 => {
                                ptr = ptr.offset(2isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2i32)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(0isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                    && *ptr.offset(1isize)
                                        as ::core::ffi::c_int
                                        == 0x3ei32
                                {
                                    *nextTokPtr = ptr.offset(2isize);
                                    return tok;
                                }
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                    return XML_TOK_PARTIAL_1;
                }
                15 => {
                    if big2_checkPiTarget(enc, target, ptr, &raw mut tok) == 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0x3ei32
                    {
                        *nextTokPtr = ptr.offset(2isize);
                        return tok;
                    }
                    c2rust_current_block_118 = 161625824724629686;
                }
                _ => {
                    c2rust_current_block_118 = 161625824724629686;
                }
            }
            match c2rust_current_block_118 {
                161625824724629686 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                11190361564366887465 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_scanCdataSection(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        pub static mut CDATA_LSQB: [::core::ffi::c_char; 6] = [
            ASCII_C as ::core::ffi::c_char,
            ASCII_D as ::core::ffi::c_char,
            ASCII_A as ::core::ffi::c_char,
            ASCII_T as ::core::ffi::c_char,
            ASCII_A as ::core::ffi::c_char,
            ASCII_LSQB as ::core::ffi::c_char,
        ];
        let mut i: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (6i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        i = 0i32;
        while i < 6i32 {
            if !(*ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(1isize) as ::core::ffi::c_int
                    == CDATA_LSQB[i as usize] as ::core::ffi::c_int)
            {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            i += 1;
            ptr = ptr.offset(2isize);
        }
        *nextTokPtr = ptr;
        return XML_TOK_CDATA_SECT_OPEN_1;
    }

    pub unsafe extern "C" fn big2_cdataSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return XML_TOK_NONE_1;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (2i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(2i32 - 1i32)
                    as size_t;
                if n == 0usize {
                    return XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                    && *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0x5di32
                {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0x3ei32)
                    {
                        ptr = ptr.offset(-(2isize));
                    } else {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CDATA_SECT_CLOSE_1;
                    }
                }
            }
            9 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                if (if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                }) == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2isize);
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DATA_NEWLINE_1;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(2isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 | 9 | 10 | 4 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn big2_scanEndTag(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_32: u64;
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_32 = 12738221189273011712;
            }
            22 | 24 => {
                c2rust_current_block_32 = 12738221189273011712;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_32 {
            12738221189273011712 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_73: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_73 = 1281007054303163758;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_73 = 1281007054303163758;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2isize);
                    while end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long
                    {
                        match if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        } {
                            21 | 9 | 10 => {}
                            11 => {
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_END_TAG_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(2isize);
                    }
                    return XML_TOK_PARTIAL_1;
                }
                23 => {
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_73 = 981995395831942902;
                }
                11 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_END_TAG_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_73 {
                1281007054303163758 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_scanHexCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                25 | 24 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2isize);
            while end.offset_from(ptr) as ::core::ffi::c_long
                >= (1i32 * 2i32) as ::core::ffi::c_long
            {
                match if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    25 | 24 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2isize);
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_scanCharRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0x78i32
            {
                return big2_scanHexCharRef(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                25 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            ptr = ptr.offset(2isize);
            while end.offset_from(ptr) as ::core::ffi::c_long
                >= (1i32 * 2i32) as ::core::ffi::c_long
            {
                match if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    25 => {}
                    18 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CHAR_REF_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
                ptr = ptr.offset(2isize);
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_scanRef(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_33: u64;
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_33 = 17794167657114565097;
            }
            22 | 24 => {
                c2rust_current_block_33 = 17794167657114565097;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_33 = 14763689060501151050;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_33 = 14763689060501151050;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_33 = 14763689060501151050;
            }
            19 => {
                return big2_scanCharRef(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_33 {
            17794167657114565097 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_64: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_64 = 17251590314240005670;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_64 = 17251590314240005670;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_64 = 10930818133215224067;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_64 = 10930818133215224067;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_64 = 10930818133215224067;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_64 {
                17251590314240005670 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_scanAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0i32;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_186: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_186 = 6092917267242331817;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_186 = 6092917267242331817;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_186 = 1634947208139838470;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    let mut c2rust_current_block_64: u64;
                    match if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0isize),
                            *ptr.offset(1isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages[*ptr
                                .offset(0isize)
                                as ::core::ffi::c_uchar
                                as usize]
                                as ::core::ffi::c_int)
                                << 3i32)
                                + (*ptr.offset(1isize)
                                    as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    >> 5i32))
                                as usize]
                                & (1u32)
                                    << (*ptr.offset(1isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1fi32)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            c2rust_current_block_64 = 6604085902723260545;
                        }
                        22 | 24 => {
                            c2rust_current_block_64 = 6604085902723260545;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4isize);
                            c2rust_current_block_64 = 10930818133215224067;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                    }
                    match c2rust_current_block_64 {
                        6604085902723260545 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_186 = 1634947208139838470;
                }
                21 | 9 | 10 => {
                    loop {
                        let mut t: ::core::ffi::c_int = 0;
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        t = if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        };
                        if t == BT_EQUALS as ::core::ffi::c_int {
                            break;
                        }
                        match t {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                    }
                    c2rust_current_block_186 = 10853015579903106591;
                }
                14 => {
                    c2rust_current_block_186 = 10853015579903106591;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_186 {
                10853015579903106591 => {
                    let mut open: ::core::ffi::c_int = 0;
                    hadColon = 0i32;
                    loop {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        open = if *ptr.offset(0isize)
                            as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        };
                        if open == BT_QUOT as ::core::ffi::c_int
                            || open == BT_APOS as ::core::ffi::c_int
                        {
                            break;
                        }
                        match open {
                            21 | 10 | 9 => {}
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                    }
                    ptr = ptr.offset(2isize);
                    loop {
                        let mut t_0: ::core::ffi::c_int = 0;
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        t_0 = if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        };
                        if t_0 == open {
                            break;
                        }
                        match t_0 {
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            0 | 1 | 8 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            3 => {
                                let mut tok: ::core::ffi::c_int = big2_scanRef(
                                    enc,
                                    ptr.offset(2isize),
                                    end,
                                    &raw mut ptr,
                                );
                                if tok <= 0i32 {
                                    if tok == XML_TOK_INVALID_1 {
                                        *nextTokPtr = ptr;
                                    }
                                    return tok;
                                }
                            }
                            2 => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    match if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0isize),
                            *ptr.offset(1isize),
                        )
                    } {
                        21 | 9 | 10 => {
                            loop {
                                ptr = ptr.offset(2isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2i32)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL_1;
                                }
                                match if *ptr.offset(0isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                {
                                    (*(enc as *const normal_encoding)).type_0[*ptr
                                        .offset(1isize)
                                        as ::core::ffi::c_uchar
                                        as usize]
                                        as ::core::ffi::c_int
                                } else {
                                    unicode_byte_type(
                                        *ptr.offset(0isize),
                                        *ptr.offset(1isize),
                                    )
                                } {
                                    29 => {
                                        if namingBitmap[(((nmstrtPages[*ptr
                                            .offset(0isize)
                                            as ::core::ffi::c_uchar
                                            as usize]
                                            as ::core::ffi::c_int)
                                            << 3i32)
                                            + (*ptr.offset(1isize)
                                                as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                >> 5i32))
                                            as usize]
                                            & (1u32)
                                                << (*ptr.offset(1isize)
                                                    as ::core::ffi::c_uchar
                                                    as ::core::ffi::c_int
                                                    & 0x1fi32)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID_1;
                                        }
                                        c2rust_current_block_186 = 7794494472231011433;
                                        break;
                                    }
                                    22 | 24 => {
                                        c2rust_current_block_186 = 7794494472231011433;
                                        break;
                                    }
                                    5 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 2i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if false || true {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 3i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if false || true {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as ::core::ffi::c_long)
                                            < 4i64
                                        {
                                            return XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if false || true {
                                            *nextTokPtr = ptr;
                                            return XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4isize);
                                        c2rust_current_block_186 = 1634947208139838470;
                                        break;
                                    }
                                    21 | 9 | 10 => {}
                                    11 => {
                                        c2rust_current_block_186 = 1783713129665224809;
                                        break;
                                    }
                                    17 => {
                                        c2rust_current_block_186 = 18153789983347219713;
                                        break;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            match c2rust_current_block_186 {
                                1783713129665224809 => {}
                                18153789983347219713 => {}
                                1634947208139838470 => {}
                                _ => {
                                    ptr = ptr.offset(2isize);
                                    c2rust_current_block_186 = 1634947208139838470;
                                }
                            }
                        }
                        17 => {
                            c2rust_current_block_186 = 18153789983347219713;
                        }
                        11 => {
                            c2rust_current_block_186 = 1783713129665224809;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                    }
                    match c2rust_current_block_186 {
                        1634947208139838470 => {}
                        _ => match c2rust_current_block_186 {
                            18153789983347219713 => {
                                ptr = ptr.offset(2isize);
                                if !(end.offset_from(ptr) as ::core::ffi::c_long
                                    >= (1i32 * 2i32)
                                        as ::core::ffi::c_long)
                                {
                                    return XML_TOK_PARTIAL_1;
                                }
                                if !(*ptr.offset(0isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                    && *ptr.offset(1isize)
                                        as ::core::ffi::c_int
                                        == 0x3ei32)
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                            }
                            _ => {
                                *nextTokPtr = ptr.offset(2isize);
                                return XML_TOK_START_TAG_WITH_ATTS_1;
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
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_scanLt(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut hadColon: ::core::ffi::c_int = 0;
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_45: u64;
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_45 = 6477200489819026004;
            }
            22 | 24 => {
                c2rust_current_block_45 = 6477200489819026004;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_45 = 8180496224585318153;
            }
            16 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    27 => {
                        return big2_scanComment(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    20 => {
                        return big2_scanCdataSection(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            15 => {
                return big2_scanPi(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            17 => {
                return big2_scanEndTag(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_45 {
            6477200489819026004 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        hadColon = 0i32;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_161: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_161 = 18151815167355992796;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_161 = 18151815167355992796;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_161 = 14714495436747744489;
                }
                23 => {
                    if hadColon != 0 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    hadColon = 1i32;
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    let mut c2rust_current_block_112: u64;
                    match if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0isize),
                            *ptr.offset(1isize),
                        )
                    } {
                        29 => {
                            if namingBitmap[(((nmstrtPages[*ptr
                                .offset(0isize)
                                as ::core::ffi::c_uchar
                                as usize]
                                as ::core::ffi::c_int)
                                << 3i32)
                                + (*ptr.offset(1isize)
                                    as ::core::ffi::c_uchar
                                    as ::core::ffi::c_int
                                    >> 5i32))
                                as usize]
                                & (1u32)
                                    << (*ptr.offset(1isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        & 0x1fi32)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            c2rust_current_block_112 = 16337619596932156899;
                        }
                        22 | 24 => {
                            c2rust_current_block_112 = 16337619596932156899;
                        }
                        5 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 2i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        6 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 3i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        7 => {
                            if (end.offset_from(ptr) as ::core::ffi::c_long)
                                < 4i64
                            {
                                return XML_TOK_PARTIAL_CHAR_1;
                            }
                            if false || true {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4isize);
                            c2rust_current_block_112 = 2616667235040759262;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_INVALID_1;
                        }
                    }
                    match c2rust_current_block_112 {
                        16337619596932156899 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                    c2rust_current_block_161 = 14714495436747744489;
                }
                21 | 9 | 10 => {
                    ptr = ptr.offset(2isize);
                    loop {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            c2rust_current_block_161 = 13215501469961642988;
                            break;
                        }
                        match if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        } {
                            29 => {
                                if namingBitmap[(((nmstrtPages[*ptr
                                    .offset(0isize)
                                    as ::core::ffi::c_uchar
                                    as usize]
                                    as ::core::ffi::c_int)
                                    << 3i32)
                                    + (*ptr.offset(1isize)
                                        as ::core::ffi::c_uchar
                                        as ::core::ffi::c_int
                                        >> 5i32))
                                    as usize]
                                    & (1u32)
                                        << (*ptr.offset(1isize)
                                            as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            & 0x1fi32)
                                    == 0
                                {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                c2rust_current_block_161 = 11066148936714919733;
                            }
                            22 | 24 => {
                                c2rust_current_block_161 = 11066148936714919733;
                            }
                            5 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 2i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if false || true {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            6 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 3i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if false || true {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            7 => {
                                if (end.offset_from(ptr) as ::core::ffi::c_long)
                                    < 4i64
                                {
                                    return XML_TOK_PARTIAL_CHAR_1;
                                }
                                if false || true {
                                    *nextTokPtr = ptr;
                                    return XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4isize);
                                c2rust_current_block_161 = 16314074004867283505;
                            }
                            11 => {
                                c2rust_current_block_161 = 13089361350718158941;
                                break;
                            }
                            17 => {
                                c2rust_current_block_161 = 11384015785330443424;
                                break;
                            }
                            21 | 9 | 10 => {
                                ptr = ptr.offset(2isize);
                                continue;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                return XML_TOK_INVALID_1;
                            }
                        }
                        match c2rust_current_block_161 {
                            11066148936714919733 => {
                                ptr = ptr.offset(2isize);
                            }
                            _ => {}
                        }
                        return big2_scanAtts(enc, ptr, end, nextTokPtr);
                    }
                    match c2rust_current_block_161 {
                        13089361350718158941 => {}
                        11384015785330443424 => {}
                        _ => return XML_TOK_PARTIAL_1,
                    }
                }
                11 => {
                    c2rust_current_block_161 = 13089361350718158941;
                }
                17 => {
                    c2rust_current_block_161 = 11384015785330443424;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_161 {
                11384015785330443424 => {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if !(*ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0x3ei32)
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                }
                13089361350718158941 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_START_TAG_NO_ATTS_1;
                }
                18151815167355992796 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_contentTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if ptr >= end {
            return XML_TOK_NONE_1;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (2i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(2i32 - 1i32)
                    as size_t;
                if n == 0usize {
                    return XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            2 => {
                return big2_scanLt(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            3 => {
                return big2_scanRef(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            9 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_CR_1;
                }
                if (if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                }) == BT_LF as ::core::ffi::c_int
                {
                    ptr = ptr.offset(2isize);
                }
                *nextTokPtr = ptr;
                return XML_TOK_DATA_NEWLINE_1;
            }
            10 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DATA_NEWLINE_1;
            }
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_TRAILING_RSQB_1;
                }
                if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                    && *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0x5di32
                {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_TRAILING_RSQB_1;
                    }
                    if !(*ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0x3ei32)
                    {
                        ptr = ptr.offset(-(2isize));
                    } else {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                }
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(2isize);
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(3isize);
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                ptr = ptr.offset(4isize);
            }
            0 | 1 | 8 => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            _ => {
                ptr = ptr.offset(2isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_76: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64
                        || false
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_CHARS_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
                4 => {
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 2i32)
                            as ::core::ffi::c_long
                    {
                        if !(*ptr
                            .offset(2isize)
                            .offset(0isize)
                            as ::core::ffi::c_int
                            == 0i32
                            && *ptr
                                .offset(2isize)
                                .offset(1isize)
                                as ::core::ffi::c_int
                                == 0x5di32)
                        {
                            ptr = ptr.offset(2isize);
                            c2rust_current_block_76 = 7158658067966855297;
                        } else if end.offset_from(ptr) as ::core::ffi::c_long
                            >= (3i32 * 2i32)
                                as ::core::ffi::c_long
                        {
                            if !(*ptr
                                .offset(
                                    (2i32 * 2i32) as isize,
                                )
                                .offset(0isize)
                                as ::core::ffi::c_int
                                == 0i32
                                && *ptr
                                    .offset(
                                        (2i32 * 2i32)
                                            as isize,
                                    )
                                    .offset(1isize)
                                    as ::core::ffi::c_int
                                    == 0x3ei32)
                            {
                                ptr = ptr.offset(2isize);
                            } else {
                                *nextTokPtr = ptr.offset(
                                    (2i32 * 2i32) as isize,
                                );
                                return XML_TOK_INVALID_1;
                            }
                            c2rust_current_block_76 = 7158658067966855297;
                        } else {
                            c2rust_current_block_76 = 11890188771060868767;
                        }
                    } else {
                        c2rust_current_block_76 = 11890188771060868767;
                    }
                }
                3 | 2 | 0 | 1 | 8 | 9 | 10 => {
                    c2rust_current_block_76 = 11890188771060868767;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_76 = 7158658067966855297;
                }
            }
            match c2rust_current_block_76 {
                7158658067966855297 => {}
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn big2_scanPercent(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_34: u64;
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_34 = 9652455934050855438;
            }
            22 | 24 => {
                c2rust_current_block_34 = 9652455934050855438;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_34 = 4761528863920922185;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_34 = 4761528863920922185;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_34 = 4761528863920922185;
            }
            21 | 10 | 9 | 30 => {
                *nextTokPtr = ptr;
                return XML_TOK_PERCENT_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_34 {
            9652455934050855438 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_65: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_65 = 3947837075391501242;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_65 = 3947837075391501242;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_65 = 16415152177862271243;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_65 = 16415152177862271243;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_65 = 16415152177862271243;
                }
                18 => {
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_PARAM_ENTITY_REF_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_65 {
                3947837075391501242 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_scanPoundName(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        let mut c2rust_current_block_32: u64;
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    == 0
                {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                c2rust_current_block_32 = 12219479933348349998;
            }
            22 | 24 => {
                c2rust_current_block_32 = 12219479933348349998;
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(2isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(3isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                if false || true {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                ptr = ptr.offset(4isize);
                c2rust_current_block_32 = 7056779235015430508;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        match c2rust_current_block_32 {
            12219479933348349998 => {
                ptr = ptr.offset(2isize);
            }
            _ => {}
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_63: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_63 = 1647491770914889697;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_63 = 1647491770914889697;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_63 = 10380409671385728102;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_63 = 10380409671385728102;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_63 = 10380409671385728102;
                }
                9 | 10 | 21 | 32 | 11 | 30 | 36 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_POUND_NAME_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_63 {
                1647491770914889697 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return -20i32;
    }

    pub unsafe extern "C" fn big2_scanLit(
        mut open: ::core::ffi::c_int,
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut t: ::core::ffi::c_int = if *ptr.offset(0isize)
                as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            };
            match t {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                12 | 13 => {
                    ptr = ptr.offset(2isize);
                    if !(t != open) {
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return -27i32;
                        }
                        *nextTokPtr = ptr;
                        match if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        } {
                            21 | 9 | 10 | 11 | 30 | 20 => {
                                return XML_TOK_LITERAL_1
                            }
                            _ => return XML_TOK_INVALID_1,
                        }
                    }
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_prologTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut tok: ::core::ffi::c_int = 0;
        if ptr >= end {
            return XML_TOK_NONE_1;
        }
        if 2i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (2i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(2i32 - 1i32)
                    as size_t;
                if n == 0usize {
                    return XML_TOK_PARTIAL_1;
                }
                end = ptr.offset(n as isize);
            }
        }
        let mut c2rust_current_block_124: u64;
        match if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
        {
            (*(enc as *const normal_encoding)).type_0
                [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                as ::core::ffi::c_int
        } else {
            unicode_byte_type(
                *ptr.offset(0isize),
                *ptr.offset(1isize),
            )
        } {
            12 => {
                return big2_scanLit(
                    BT_QUOT as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            13 => {
                return big2_scanLit(
                    BT_APOS as ::core::ffi::c_int,
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            2 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    16 => {
                        return big2_scanDecl(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    15 => {
                        return big2_scanPi(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    22 | 24 | 29 | 5 | 6 | 7 => {
                        *nextTokPtr = ptr.offset(-(2isize));
                        return XML_TOK_INSTANCE_START;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            9 => {
                if ptr.offset(2isize) == end {
                    *nextTokPtr = end;
                    return -15i32;
                }
                c2rust_current_block_124 = 16869865525854146339;
            }
            21 | 10 => {
                c2rust_current_block_124 = 16869865525854146339;
            }
            30 => {
                return big2_scanPercent(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            35 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_COMMA_1;
            }
            20 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OPEN_BRACKET_1;
            }
            4 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return -26i32;
                }
                if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                    && *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0x5di32
                {
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (2i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr
                        .offset(2isize)
                        .offset(0isize)
                        as ::core::ffi::c_int
                        == 0i32
                        && *ptr
                            .offset(2isize)
                            .offset(1isize)
                            as ::core::ffi::c_int
                            == 0x3ei32
                    {
                        *nextTokPtr = ptr
                            .offset((2i32 * 2i32) as isize);
                        return XML_TOK_COND_SECT_CLOSE_1;
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_CLOSE_BRACKET_1;
            }
            31 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OPEN_PAREN_1;
            }
            32 => {
                ptr = ptr.offset(2isize);
                if !(end.offset_from(ptr) as ::core::ffi::c_long
                    >= (1i32 * 2i32) as ::core::ffi::c_long)
                {
                    return -24i32;
                }
                match if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    (*(enc as *const normal_encoding)).type_0[*ptr
                        .offset(1isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int
                } else {
                    unicode_byte_type(
                        *ptr.offset(0isize),
                        *ptr.offset(1isize),
                    )
                } {
                    33 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_ASTERISK_1;
                    }
                    15 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_QUESTION_1;
                    }
                    34 => {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_CLOSE_PAREN_PLUS_1;
                    }
                    9 | 10 | 21 | 11 | 35 | 36 | 32 => {
                        *nextTokPtr = ptr;
                        return XML_TOK_CLOSE_PAREN_1;
                    }
                    _ => {}
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            36 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_OR_1;
            }
            11 => {
                *nextTokPtr = ptr.offset(2isize);
                return XML_TOK_DECL_CLOSE_1;
            }
            19 => {
                return big2_scanPoundName(
                    enc,
                    ptr.offset(2isize),
                    end,
                    nextTokPtr,
                );
            }
            5 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            6 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            7 => {
                if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                    return XML_TOK_PARTIAL_CHAR_1;
                }
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
            22 | 24 => {
                tok = XML_TOK_NAME;
                ptr = ptr.offset(2isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            25 | 26 | 27 | 23 => {
                tok = XML_TOK_NMTOKEN_1;
                ptr = ptr.offset(2isize);
                c2rust_current_block_124 = 2956972668325154207;
            }
            29 => {
                if namingBitmap[(((nmstrtPages
                    [*ptr.offset(0isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32)) as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NAME;
                    c2rust_current_block_124 = 2956972668325154207;
                } else if namingBitmap[(((namePages
                    [*ptr.offset(0isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int)
                    << 3i32)
                    + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        >> 5i32))
                    as usize]
                    & (1u32)
                        << (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            & 0x1fi32)
                    != 0
                {
                    ptr = ptr.offset(2isize);
                    tok = XML_TOK_NMTOKEN_1;
                    c2rust_current_block_124 = 2956972668325154207;
                } else {
                    c2rust_current_block_124 = 6428058487030868344;
                }
            }
            _ => {
                c2rust_current_block_124 = 6428058487030868344;
            }
        }
        match c2rust_current_block_124 {
            2956972668325154207 => {}
            16869865525854146339 => {
                loop {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        break;
                    }
                    let mut c2rust_current_block_32: u64;
                    match if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        (*(enc as *const normal_encoding)).type_0[*ptr
                            .offset(1isize)
                            as ::core::ffi::c_uchar
                            as usize] as ::core::ffi::c_int
                    } else {
                        unicode_byte_type(
                            *ptr.offset(0isize),
                            *ptr.offset(1isize),
                        )
                    } {
                        21 | 10 => {
                            c2rust_current_block_32 = 17500079516916021833;
                        }
                        9 => {
                            if ptr.offset(2isize) != end {
                                c2rust_current_block_32 = 17500079516916021833;
                            } else {
                                c2rust_current_block_32 = 11299462660638600073;
                            }
                        }
                        _ => {
                            c2rust_current_block_32 = 11299462660638600073;
                        }
                    }
                    match c2rust_current_block_32 {
                        17500079516916021833 => {}
                        _ => {
                            *nextTokPtr = ptr;
                            return XML_TOK_PROLOG_S_1;
                        }
                    }
                }
                *nextTokPtr = ptr;
                return XML_TOK_PROLOG_S_1;
            }
            _ => {
                *nextTokPtr = ptr;
                return XML_TOK_INVALID_1;
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_210: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                29 => {
                    if namingBitmap[(((namePages[*ptr.offset(0isize)
                        as ::core::ffi::c_uchar
                        as usize] as ::core::ffi::c_int)
                        << 3i32)
                        + (*ptr.offset(1isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            >> 5i32))
                        as usize]
                        & (1u32)
                            << (*ptr.offset(1isize)
                                as ::core::ffi::c_uchar
                                as ::core::ffi::c_int
                                & 0x1fi32)
                        == 0
                    {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    c2rust_current_block_210 = 9794574411605359176;
                }
                22 | 24 | 25 | 26 | 27 => {
                    c2rust_current_block_210 = 9794574411605359176;
                }
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(3isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    if false || true {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(4isize);
                    c2rust_current_block_210 = 14244298717249035578;
                }
                11 | 32 | 35 | 36 | 20 | 30 | 21 | 9 | 10 => {
                    *nextTokPtr = ptr;
                    return tok;
                }
                23 => {
                    ptr = ptr.offset(2isize);
                    match  tok {
    XML_TOK_NAME =>  {
                            if !(end.offset_from(ptr) as ::core::ffi::c_long
                                >= (1i32 * 2i32)
                                    as ::core::ffi::c_long)
                            {
                                return XML_TOK_PARTIAL_1;
                            }
                            tok = XML_TOK_PREFIXED_NAME;
                            let mut c2rust_current_block_187: u64;
                            match if *ptr.offset(0isize)
                                as ::core::ffi::c_int
                                == 0i32
                            {
                                (*(enc as *const normal_encoding)).type_0[*ptr
                                    .offset(1isize)
                                    as ::core::ffi::c_uchar
                                    as usize] as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(0isize),
                                    *ptr.offset(1isize),
                                )
                            } {
                                29 => {
                                    if namingBitmap[(((namePages[*ptr
                                        .offset(0isize)
                                        as ::core::ffi::c_uchar
                                        as usize]
                                        as ::core::ffi::c_int)
                                        << 3i32)
                                        + (*ptr.offset(1isize)
                                            as ::core::ffi::c_uchar
                                            as ::core::ffi::c_int
                                            >> 5i32))
                                        as usize]
                                        & (1u32)
                                            << (*ptr.offset(1isize)
                                                as ::core::ffi::c_uchar
                                                as ::core::ffi::c_int
                                                & 0x1fi32)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                    c2rust_current_block_187 = 17275381528970576968;
                                }
                                22 | 24 | 25 | 26 | 27 => {
                                    c2rust_current_block_187 = 17275381528970576968;
                                }
                                5 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 2i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if false || true {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(2isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                6 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 3i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if false || true {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(3isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                7 => {
                                    if (end.offset_from(ptr) as ::core::ffi::c_long)
                                        < 4i64
                                    {
                                        return XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if false || true {
                                        *nextTokPtr = ptr;
                                        return XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(4isize);
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                                _ => {
                                    tok = XML_TOK_NMTOKEN_1;
                                    c2rust_current_block_187 = 9812798724717783973;
                                }
                            }
                            match c2rust_current_block_187 {
                                17275381528970576968 => {
                                    ptr = ptr.offset(2isize);
                                }
                                _ => {}
                            }
                        }
    XML_TOK_PREFIXED_NAME =>  {
                            tok = XML_TOK_NMTOKEN_1;
                        }
    _ =>  {}
}
                    c2rust_current_block_210 = 14244298717249035578;
                }
                34 => {
                    if tok == XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_PLUS_1;
                }
                33 => {
                    if tok == XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_ASTERISK_1;
                }
                15 => {
                    if tok == XML_TOK_NMTOKEN_1 {
                        *nextTokPtr = ptr;
                        return XML_TOK_INVALID_1;
                    }
                    *nextTokPtr = ptr.offset(2isize);
                    return XML_TOK_NAME_QUESTION_1;
                }
                _ => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
            }
            match c2rust_current_block_210 {
                9794574411605359176 => {
                    ptr = ptr.offset(2isize);
                }
                _ => {}
            }
        }
        return -tok;
    }

    pub unsafe extern "C" fn big2_attributeValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
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
                        return big2_scanRef(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                2 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                21 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_ATTRIBUTE_VALUE_S_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn big2_entityValueTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if ptr >= end {
            return XML_TOK_NONE_1;
        } else if !(end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long)
        {
            return XML_TOK_PARTIAL_1;
        }
        start = ptr;
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
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
                        return big2_scanRef(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                30 => {
                    if ptr == start {
                        let mut tok: ::core::ffi::c_int = big2_scanPercent(
                            enc,
                            ptr.offset(2isize),
                            end,
                            nextTokPtr,
                        );
                        return if tok == XML_TOK_PERCENT_1 {
                            XML_TOK_INVALID_1
                        } else {
                            tok
                        };
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                10 => {
                    if ptr == start {
                        *nextTokPtr = ptr.offset(2isize);
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                9 => {
                    if ptr == start {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        return XML_TOK_DATA_NEWLINE_1;
                    }
                    *nextTokPtr = ptr;
                    return XML_TOK_DATA_CHARS_1;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                }
            }
        }
        *nextTokPtr = ptr;
        return XML_TOK_DATA_CHARS_1;
    }

    pub unsafe extern "C" fn big2_ignoreSectionTok(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut level: ::core::ffi::c_int = 0i32;
        if 2i32 > 1i32 {
            let mut n: size_t =
                
                end.offset_from(ptr) as size_t;
            if n & (2i32 - 1i32)
                as size_t
                != 0
            {
                n &= !(2i32 - 1i32)
                    as size_t;
                end = ptr.offset(n as isize);
            }
        }
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 2i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(2isize);
                }
                6 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 3i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(3isize);
                }
                7 => {
                    if (end.offset_from(ptr) as ::core::ffi::c_long) < 4i64 {
                        return XML_TOK_PARTIAL_CHAR_1;
                    }
                    ptr = ptr.offset(4isize);
                }
                0 | 1 | 8 => {
                    *nextTokPtr = ptr;
                    return XML_TOK_INVALID_1;
                }
                2 => {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0x21i32
                    {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(1isize) as ::core::ffi::c_int
                                == 0x5bi32
                        {
                            level += 1;
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                4 => {
                    ptr = ptr.offset(2isize);
                    if !(end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long)
                    {
                        return XML_TOK_PARTIAL_1;
                    }
                    if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0x5di32
                    {
                        ptr = ptr.offset(2isize);
                        if !(end.offset_from(ptr) as ::core::ffi::c_long
                            >= (1i32 * 2i32)
                                as ::core::ffi::c_long)
                        {
                            return XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(1isize) as ::core::ffi::c_int
                                == 0x3ei32
                        {
                            ptr = ptr.offset(2isize);
                            if level == 0i32 {
                                *nextTokPtr = ptr;
                                return XML_TOK_IGNORE_SECT_1;
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
        return XML_TOK_PARTIAL_1;
    }

    pub unsafe extern "C" fn big2_isPublicId(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        ptr = ptr.offset(2isize);
        end = end.offset(-(2isize));
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            let mut c2rust_current_block_8: u64;
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16 | 33
                | 30 | 19 | 23 => {
                    c2rust_current_block_8 = 5143058163439228106;
                }
                21 => {
                    if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0x9i32
                    {
                        *badPtr = ptr;
                        return 0i32;
                    }
                    c2rust_current_block_8 = 5143058163439228106;
                }
                26 | 22 => {
                    if (if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        *ptr.offset(1isize) as ::core::ffi::c_int
                    } else {
                        -1i32
                    }) & !(0x7fi32)
                        == 0
                    {
                        c2rust_current_block_8 = 5143058163439228106;
                    } else {
                        c2rust_current_block_8 = 9906551679175889830;
                    }
                }
                _ => {
                    c2rust_current_block_8 = 9906551679175889830;
                }
            }
            match c2rust_current_block_8 {
                9906551679175889830 => {
                    match if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        *ptr.offset(1isize) as ::core::ffi::c_int
                    } else {
                        -1i32
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
            ptr = ptr.offset(2isize);
        }
        return 1i32;
    }

    pub unsafe extern "C" fn big2_getAtts(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut attsMax: ::core::ffi::c_int,
        mut atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> ::core::ffi::c_int {
        let mut state: crate::xmltok_impl_h::C2Rust_Unnamed_3 = crate::xmltok_impl_c::inName_1;
        let mut nAtts: ::core::ffi::c_int = 0i32;
        let mut open: ::core::ffi::c_int = 0i32;
        ptr = ptr.offset(2isize);
        loop {
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => {
                    if  state
                        ==  crate::xmltok_impl_c::other_1
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh48 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh48 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_1;
                    }
                    ptr = ptr.offset((2i32 - 2i32) as isize);
                }
                6 => {
                    if  state
                        ==  crate::xmltok_impl_c::other_1
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh49 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh49 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_1;
                    }
                    ptr = ptr.offset((3i32 - 2i32) as isize);
                }
                7 => {
                    if  state
                        ==  crate::xmltok_impl_c::other_1
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh50 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh50 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_1;
                    }
                    ptr = ptr.offset((4i32 - 2i32) as isize);
                }
                29 | 22 | 24 => {
                    if  state
                        ==  crate::xmltok_impl_c::other_1
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh51 = (*atts.offset(nAtts as isize)).name;
                            *c2rust_fresh51 = ptr;
                            (*atts.offset(nAtts as isize)).normalized = 1i8;
                        }
                        state = crate::xmltok_impl_c::inName_1;
                    }
                }
                12 => {
                    if  state
                        !=  crate::xmltok_impl_c::inValue_1
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh52 = (*atts.offset(nAtts as isize)).valuePtr;
                            *c2rust_fresh52 = ptr.offset(2isize);
                        }
                        state = crate::xmltok_impl_c::inValue_1;
                        open = BT_QUOT as ::core::ffi::c_int;
                    } else if open == BT_QUOT as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other_1;
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh53 = (*atts.offset(nAtts as isize)).valueEnd;
                            *c2rust_fresh53 = ptr;
                        }
                        nAtts += 1;
                    }
                }
                13 => {
                    if  state
                        !=  crate::xmltok_impl_c::inValue_1
                    {
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh54 = (*atts.offset(nAtts as isize)).valuePtr;
                            *c2rust_fresh54 = ptr.offset(2isize);
                        }
                        state = crate::xmltok_impl_c::inValue_1;
                        open = BT_APOS as ::core::ffi::c_int;
                    } else if open == BT_APOS as ::core::ffi::c_int {
                        state = crate::xmltok_impl_c::other_1;
                        if nAtts < attsMax {
                            let ref mut c2rust_fresh55 = (*atts.offset(nAtts as isize)).valueEnd;
                            *c2rust_fresh55 = ptr;
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
                    if  state
                        ==  crate::xmltok_impl_c::inName_1
                    {
                        state = crate::xmltok_impl_c::other_1;
                    } else if  state
                        ==  crate::xmltok_impl_c::inValue_1
                        && nAtts < attsMax
                        && (*atts.offset(nAtts as isize)).normalized as ::core::ffi::c_int != 0
                        && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                            || (if *ptr.offset(0isize)
                                as ::core::ffi::c_int
                                == 0i32
                            {
                                *ptr.offset(1isize) as ::core::ffi::c_int
                            } else {
                                -1i32
                            }) != ASCII_SPACE
                            || (if *ptr
                                .offset(2isize)
                                .offset(0isize)
                                as ::core::ffi::c_int
                                == 0i32
                            {
                                *ptr.offset(2isize)
                                    .offset(1isize)
                                    as ::core::ffi::c_int
                            } else {
                                -1i32
                            }) == ASCII_SPACE
                            || (if *ptr
                                .offset(2isize)
                                .offset(0isize)
                                as ::core::ffi::c_int
                                == 0i32
                            {
                                (*(enc as *const normal_encoding)).type_0[*ptr
                                    .offset(2isize)
                                    .offset(1isize)
                                    as ::core::ffi::c_uchar
                                    as usize] as ::core::ffi::c_int
                            } else {
                                unicode_byte_type(
                                    *ptr.offset(2isize)
                                        .offset(0isize),
                                    *ptr.offset(2isize)
                                        .offset(1isize),
                                )
                            }) == open)
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                9 | 10 => {
                    if  state
                        ==  crate::xmltok_impl_c::inName_1
                    {
                        state = crate::xmltok_impl_c::other_1;
                    } else if  state
                        ==  crate::xmltok_impl_c::inValue_1
                        && nAtts < attsMax
                    {
                        (*atts.offset(nAtts as isize)).normalized = 0i8;
                    }
                }
                11 | 17 => {
                    if  state
                        !=  crate::xmltok_impl_c::inValue_1
                    {
                        return nAtts;
                    }
                }
                _ => {}
            }
            ptr = ptr.offset(2isize);
        }
    }

    pub unsafe extern "C" fn big2_charRefNumber(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut result: ::core::ffi::c_int = 0i32;
        ptr = ptr.offset((2i32 * 2i32) as isize);
        if *ptr.offset(0isize) as ::core::ffi::c_int
            == 0i32
            && *ptr.offset(1isize) as ::core::ffi::c_int
                == 0x78i32
        {
            ptr = ptr.offset(2isize);
            while !(*ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0x3bi32)
            {
                let mut c: ::core::ffi::c_int = if *ptr.offset(0isize)
                    as ::core::ffi::c_int
                    == 0i32
                {
                    *ptr.offset(1isize) as ::core::ffi::c_int
                } else {
                    -1i32
                };
                match  c {
    ASCII_0 | crate::ascii_h::ASCII_1_1 | crate::ascii_h::ASCII_2_1 |
        crate::ascii_h::ASCII_3_1 | crate::ascii_h::ASCII_4 |
        crate::ascii_h::ASCII_5 | crate::ascii_h::ASCII_6 |
        crate::ascii_h::ASCII_7 | crate::ascii_h::ASCII_8_1 |
        crate::ascii_h::ASCII_9_1 =>  {
                        result <<= 4i32;
                        result |= c - ASCII_0;
                    }
    ASCII_A | crate::ascii_h::ASCII_B_1 | ASCII_C | ASCII_D |
        crate::ascii_h::ASCII_E_1 | crate::ascii_h::ASCII_F_1 =>  {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_A);
                    }
    ASCII_a_1 | crate::ascii_h::ASCII_b | crate::ascii_h::ASCII_c_1 |
        crate::ascii_h::ASCII_d | crate::ascii_h::ASCII_e_1 |
        crate::ascii_h::ASCII_f =>  {
                        result <<= 4i32;
                        result += 10i32 + (c - ASCII_a_1);
                    }
    _ =>  {}
}
                if result >= 0x110000i32 {
                    return -1i32;
                }
                ptr = ptr.offset(2isize);
            }
        } else {
            while !(*ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
                && *ptr.offset(1isize) as ::core::ffi::c_int
                    == 0x3bi32)
            {
                let mut c_0: ::core::ffi::c_int = if *ptr.offset(0isize)
                    as ::core::ffi::c_int
                    == 0i32
                {
                    *ptr.offset(1isize) as ::core::ffi::c_int
                } else {
                    -1i32
                };
                result *= 10i32;
                result += c_0 - ASCII_0;
                if result >= 0x110000i32 {
                    return -1i32;
                }
                ptr = ptr.offset(2isize);
            }
        }
        return checkCharRefNumber(result);
    }

    pub unsafe extern "C" fn big2_predefinedEntityName(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        match end.offset_from(ptr) as ::core::ffi::c_long / 2i64 {
            2 => {
                if *ptr
                    .offset(2isize)
                    .offset(0isize)
                    as ::core::ffi::c_int
                    == 0i32
                    && *ptr
                        .offset(2isize)
                        .offset(1isize)
                        as ::core::ffi::c_int
                        == 0x74i32
                {
                    match if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                    {
                        *ptr.offset(1isize) as ::core::ffi::c_int
                    } else {
                        -1i32
                    } {
                        crate::ascii_h::ASCII_l_1 => return ASCII_LT,
                        crate::ascii_h::ASCII_g_1 => return ASCII_GT,
                        _ => {}
                    }
                }
            }
            3 => {
                if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                    && *ptr.offset(1isize) as ::core::ffi::c_int
                        == 0x61i32
                {
                    ptr = ptr.offset(2isize);
                    if *ptr.offset(0isize) as ::core::ffi::c_int
                        == 0i32
                        && *ptr.offset(1isize) as ::core::ffi::c_int
                            == 0x6di32
                    {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(1isize) as ::core::ffi::c_int
                                == 0x70i32
                        {
                            return ASCII_AMP;
                        }
                    }
                }
            }
            4 => {
                match  if *ptr.offset(0isize) as ::core::ffi::c_int
                    == 0i32
                {
                    *ptr.offset(1isize) as ::core::ffi::c_int
                } else {
                    -1i32
                } {
    crate::ascii_h::ASCII_q =>  {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(1isize) as ::core::ffi::c_int
                                == 0x75i32
                        {
                            ptr = ptr.offset(2isize);
                            if *ptr.offset(0isize) as ::core::ffi::c_int
                                == 0i32
                                && *ptr.offset(1isize)
                                    as ::core::ffi::c_int
                                    == 0x6fi32
                            {
                                ptr = ptr.offset(2isize);
                                if *ptr.offset(0isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                    && *ptr.offset(1isize)
                                        as ::core::ffi::c_int
                                        == 0x74i32
                                {
                                    return ASCII_QUOT;
                                }
                            }
                        }
                    }
    ASCII_a_1 =>  {
                        ptr = ptr.offset(2isize);
                        if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                            && *ptr.offset(1isize) as ::core::ffi::c_int
                                == 0x70i32
                        {
                            ptr = ptr.offset(2isize);
                            if *ptr.offset(0isize) as ::core::ffi::c_int
                                == 0i32
                                && *ptr.offset(1isize)
                                    as ::core::ffi::c_int
                                    == 0x6fi32
                            {
                                ptr = ptr.offset(2isize);
                                if *ptr.offset(0isize)
                                    as ::core::ffi::c_int
                                    == 0i32
                                    && *ptr.offset(1isize)
                                        as ::core::ffi::c_int
                                        == 0x73i32
                                {
                                    return ASCII_APOS;
                                }
                            }
                        }
                    }
    _ =>  {}
}
            }
            _ => {}
        }
        return 0i32;
    }

    pub unsafe extern "C" fn big2_nameMatchesAscii(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr1: *const ::core::ffi::c_char,
        mut end1: *const ::core::ffi::c_char,
        mut ptr2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        while *ptr2 != 0 {
            if (end1.offset_from(ptr1) as ::core::ffi::c_long) < 2i64 {
                return 0i32;
            }
            if !(*ptr1.offset(0isize) as ::core::ffi::c_int
                == 0i32
                && *ptr1.offset(1isize) as ::core::ffi::c_int
                    == *ptr2 as ::core::ffi::c_int)
            {
                return 0i32;
            }
            ptr1 = ptr1.offset(2isize);
            ptr2 = ptr2.offset(1);
        }
        return (ptr1 == end1) as ::core::ffi::c_int;
    }

    pub unsafe extern "C" fn big2_nameLength(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut start: *const ::core::ffi::c_char = ptr;
        loop {
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
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
                    return  ptr.offset_from(start) as ::core::ffi::c_int;
                }
            }
        }
    }

    pub unsafe extern "C" fn big2_skipS(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char {
        loop {
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                10 | 9 | 21 => {
                    ptr = ptr.offset(2isize);
                }
                _ => return ptr,
            }
        }
    }

    pub unsafe extern "C" fn big2_updatePosition(
        mut enc: *const crate::src::lib::xmltok::ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut pos: *mut crate::src::lib::xmltok::POSITION,
    ) {
        while end.offset_from(ptr) as ::core::ffi::c_long
            >= (1i32 * 2i32) as ::core::ffi::c_long
        {
            match if *ptr.offset(0isize) as ::core::ffi::c_int
                == 0i32
            {
                (*(enc as *const normal_encoding)).type_0
                    [*ptr.offset(1isize) as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
            } else {
                unicode_byte_type(
                    *ptr.offset(0isize),
                    *ptr.offset(1isize),
                )
            } {
                5 => {
                    ptr = ptr.offset(2isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                6 => {
                    ptr = ptr.offset(3isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                7 => {
                    ptr = ptr.offset(4isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                }
                10 => {
                    (*pos).columnNumber = 0u64;
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize);
                }
                9 => {
                    (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                    ptr = ptr.offset(2isize);
                    if end.offset_from(ptr) as ::core::ffi::c_long
                        >= (1i32 * 2i32)
                            as ::core::ffi::c_long
                        && (if *ptr.offset(0isize) as ::core::ffi::c_int
                            == 0i32
                        {
                            (*(enc as *const normal_encoding)).type_0[*ptr
                                .offset(1isize)
                                as ::core::ffi::c_uchar
                                as usize] as ::core::ffi::c_int
                        } else {
                            unicode_byte_type(
                                *ptr.offset(0isize),
                                *ptr.offset(1isize),
                            )
                        }) == BT_LF as ::core::ffi::c_int
                    {
                        ptr = ptr.offset(2isize);
                    }
                    (*pos).columnNumber = 0u64;
                }
                _ => {
                    ptr = ptr.offset(2isize);
                    (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
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

    use crate::src::lib::xmltok::checkCharRefNumber;
    use crate::src::lib::xmltok::nametab_h::namePages;
    use crate::src::lib::xmltok::nametab_h::namingBitmap;
    use crate::src::lib::xmltok::nametab_h::nmstrtPages;
    use crate::src::lib::xmltok::normal_encoding;
    use crate::src::lib::xmltok::unicode_byte_type;
    use crate::src::lib::xmltok::XML_TOK_ATTRIBUTE_VALUE_S_1;
    use crate::src::lib::xmltok::XML_TOK_CDATA_SECT_CLOSE_1;
    use crate::src::lib::xmltok::XML_TOK_CDATA_SECT_OPEN_1;
    use crate::src::lib::xmltok::XML_TOK_CHAR_REF_1;
    use crate::src::lib::xmltok::XML_TOK_CLOSE_BRACKET_1;
    use crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_1;
    use crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_ASTERISK_1;
    use crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_PLUS_1;
    use crate::src::lib::xmltok::XML_TOK_CLOSE_PAREN_QUESTION_1;
    use crate::src::lib::xmltok::XML_TOK_COMMA_1;
    use crate::src::lib::xmltok::XML_TOK_COMMENT_1;
    use crate::src::lib::xmltok::XML_TOK_COND_SECT_CLOSE_1;
    use crate::src::lib::xmltok::XML_TOK_COND_SECT_OPEN_1;
    use crate::src::lib::xmltok::XML_TOK_DATA_CHARS_1;
    use crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE_1;
    use crate::src::lib::xmltok::XML_TOK_DECL_CLOSE_1;
    use crate::src::lib::xmltok::XML_TOK_DECL_OPEN_1;
    use crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
    use crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
    use crate::src::lib::xmltok::XML_TOK_END_TAG_1;
    use crate::src::lib::xmltok::XML_TOK_ENTITY_REF_1;
    use crate::src::lib::xmltok::XML_TOK_IGNORE_SECT_1;
    use crate::src::lib::xmltok::XML_TOK_INSTANCE_START;
    use crate::src::lib::xmltok::XML_TOK_INVALID_1;
    use crate::src::lib::xmltok::XML_TOK_LITERAL_1;
    use crate::src::lib::xmltok::XML_TOK_NAME;
    use crate::src::lib::xmltok::XML_TOK_NAME_ASTERISK_1;
    use crate::src::lib::xmltok::XML_TOK_NAME_PLUS_1;
    use crate::src::lib::xmltok::XML_TOK_NAME_QUESTION_1;
    use crate::src::lib::xmltok::XML_TOK_NMTOKEN_1;
    use crate::src::lib::xmltok::XML_TOK_NONE_1;
    use crate::src::lib::xmltok::XML_TOK_OPEN_BRACKET_1;
    use crate::src::lib::xmltok::XML_TOK_OPEN_PAREN_1;
    use crate::src::lib::xmltok::XML_TOK_OR_1;
    use crate::src::lib::xmltok::XML_TOK_PARAM_ENTITY_REF_1;
    use crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    use crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
    use crate::src::lib::xmltok::XML_TOK_PERCENT_1;
    use crate::src::lib::xmltok::XML_TOK_PI_1;
    use crate::src::lib::xmltok::XML_TOK_POUND_NAME_1;
    use crate::src::lib::xmltok::XML_TOK_PREFIXED_NAME;
    use crate::src::lib::xmltok::XML_TOK_PROLOG_S_1;
    use crate::src::lib::xmltok::XML_TOK_START_TAG_NO_ATTS_1;
    use crate::src::lib::xmltok::XML_TOK_START_TAG_WITH_ATTS_1;
    use crate::src::lib::xmltok::XML_TOK_TRAILING_CR_1;
    use crate::src::lib::xmltok::XML_TOK_TRAILING_RSQB_1;
    use crate::src::lib::xmltok::XML_TOK_XML_DECL_1;
    use crate::xmltok_impl_h::BT_APOS;
    use crate::xmltok_impl_h::BT_EQUALS;
    use crate::xmltok_impl_h::BT_LF;
    use crate::xmltok_impl_h::BT_QUOT;
}

pub mod xmltok_ns_c {
    #[no_mangle]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncoding() -> *const ENCODING
    {
        return &raw const internal_utf8_encoding.enc;
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncoding(
    ) -> *const ENCODING {
        return &raw const internal_little2_encoding.enc;
    }

    pub static mut encodings: [*const ENCODING; 7] =
        [::core::ptr::null::<ENCODING>(); 7];

    pub unsafe extern "C" fn initScanProlog(
        mut enc: *const ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            &raw const encodings as *const *const ENCODING,
            enc as *const INIT_ENCODING,
            XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub unsafe extern "C" fn initScanContent(
        mut enc: *const ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            &raw const encodings as *const *const ENCODING,
            enc as *const INIT_ENCODING,
            XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlInitEncoding(
        mut p: *mut INIT_ENCODING,
        mut encPtr: *mut *const ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = getEncodingIndex(name);
        if i ==  UNKNOWN_ENC {
            return 0i32;
        }
        (*p).initEnc.isUtf16 = i as ::core::ffi::c_char;
        (*p).initEnc.scanners[XML_PROLOG_STATE as usize] =  Some(
            initScanProlog
                as unsafe extern "C" fn(
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*p).initEnc.scanners[XML_CONTENT_STATE as usize] =  Some(
            initScanContent
                as unsafe extern "C" fn(
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*p).initEnc.updatePosition =  Some(
            initUpdatePosition
                as unsafe extern "C" fn(
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut POSITION,
                ) -> (),
        );
        (*p).encPtr = encPtr;
        *encPtr = &raw mut (*p).initEnc;
        return 1i32;
    }

    pub unsafe extern "C" fn findEncoding(
        mut enc: *const ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> *const ENCODING {
        let mut buf: [::core::ffi::c_char; 128] = ::core::mem::transmute::<
            [u8; 128],
            [::core::ffi::c_char; 128],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        (*enc).utf8Convert.expect("non-null function pointer")(
            enc,
            &raw mut ptr,
            end,
            &raw mut p,
            p.offset(128isize)
                .offset(-(1isize)),
        );
        if ptr != end {
            return ::core::ptr::null::<ENCODING>();
        }
        *p = 0i8;
        if streqci(
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw const KW_UTF_16 as *const ::core::ffi::c_char,
        ) != 0
            && (*enc).minBytesPerChar == 2i32
        {
            return enc;
        }
        i = getEncodingIndex(&raw mut buf as *mut ::core::ffi::c_char);
        if i ==  UNKNOWN_ENC {
            return ::core::ptr::null::<ENCODING>();
        }
        return encodings[i as usize];
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlParseXmlDecl(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return doParseXmlDecl(
            Some(
                findEncoding
                    as unsafe extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> *const ENCODING,
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
    #[no_mangle]

    pub unsafe extern "C" fn XmlGetUtf8InternalEncodingNS(
    ) -> *const ENCODING {
        return &raw const internal_utf8_encoding_ns.enc;
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlGetUtf16InternalEncodingNS(
    ) -> *const ENCODING {
        return &raw const internal_little2_encoding_ns.enc;
    }

    pub static mut encodingsNS: [*const ENCODING; 7] =
        [::core::ptr::null::<ENCODING>(); 7];

    pub unsafe extern "C" fn initScanPrologNS(
        mut enc: *const ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            &raw const encodingsNS as *const *const ENCODING,
            enc as *const INIT_ENCODING,
            XML_PROLOG_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }

    pub unsafe extern "C" fn initScanContentNS(
        mut enc: *const ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut nextTokPtr: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        return initScan(
            &raw const encodingsNS as *const *const ENCODING,
            enc as *const INIT_ENCODING,
            XML_CONTENT_STATE,
            ptr,
            end,
            nextTokPtr,
        );
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlInitEncodingNS(
        mut p: *mut INIT_ENCODING,
        mut encPtr: *mut *const ENCODING,
        mut name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = getEncodingIndex(name);
        if i ==  UNKNOWN_ENC {
            return 0i32;
        }
        (*p).initEnc.isUtf16 = i as ::core::ffi::c_char;
        (*p).initEnc.scanners[XML_PROLOG_STATE as usize] =  Some(
            initScanPrologNS
                as unsafe extern "C" fn(
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*p).initEnc.scanners[XML_CONTENT_STATE as usize] =  Some(
            initScanContentNS
                as unsafe extern "C" fn(
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*p).initEnc.updatePosition =  Some(
            initUpdatePosition
                as unsafe extern "C" fn(
                    *const ENCODING,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut POSITION,
                ) -> (),
        );
        (*p).encPtr = encPtr;
        *encPtr = &raw mut (*p).initEnc;
        return 1i32;
    }

    pub unsafe extern "C" fn findEncodingNS(
        mut enc: *const ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
    ) -> *const ENCODING {
        let mut buf: [::core::ffi::c_char; 128] = ::core::mem::transmute::<
            [u8; 128],
            [::core::ffi::c_char; 128],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
        let mut i: ::core::ffi::c_int = 0;
        (*enc).utf8Convert.expect("non-null function pointer")(
            enc,
            &raw mut ptr,
            end,
            &raw mut p,
            p.offset(128isize)
                .offset(-(1isize)),
        );
        if ptr != end {
            return ::core::ptr::null::<ENCODING>();
        }
        *p = 0i8;
        if streqci(
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw const KW_UTF_16 as *const ::core::ffi::c_char,
        ) != 0
            && (*enc).minBytesPerChar == 2i32
        {
            return enc;
        }
        i = getEncodingIndex(&raw mut buf as *mut ::core::ffi::c_char);
        if i ==  UNKNOWN_ENC {
            return ::core::ptr::null::<ENCODING>();
        }
        return encodingsNS[i as usize];
    }
    #[no_mangle]

    pub unsafe extern "C" fn XmlParseXmlDeclNS(
        mut isGeneralTextEntity: ::core::ffi::c_int,
        mut enc: *const ENCODING,
        mut ptr: *const ::core::ffi::c_char,
        mut end: *const ::core::ffi::c_char,
        mut badPtr: *mut *const ::core::ffi::c_char,
        mut versionPtr: *mut *const ::core::ffi::c_char,
        mut versionEndPtr: *mut *const ::core::ffi::c_char,
        mut encodingName: *mut *const ::core::ffi::c_char,
        mut encoding: *mut *const ENCODING,
        mut standalone: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return doParseXmlDecl(
            Some(
                findEncodingNS
                    as unsafe extern "C" fn(
                        *const ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> *const ENCODING,
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
    use crate::src::lib::xmltok::ENCODING;
    use crate::src::lib::xmltok::INIT_ENCODING;
    use crate::src::lib::xmltok::KW_UTF_16;
    use crate::src::lib::xmltok::POSITION;
    use crate::src::lib::xmltok::SCANNER;
    use crate::src::lib::xmltok::UNKNOWN_ENC;
    use crate::src::lib::xmltok::XML_CONTENT_STATE;
    use crate::src::lib::xmltok::XML_PROLOG_STATE;
}

pub mod nametab_h {

    pub static mut namingBitmap: [::core::ffi::c_uint; 320] = [
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0u32,
        0x4000000u32,
        0x87fffffeu32,
        0x7fffffeu32,
        0u32,
        0u32,
        0xff7fffffu32,
        0xff7fffffu32,
        0xffffffffu32,
        0x7ff3ffffu32,
        0xfffffdfeu32,
        0x7fffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffe00fu32,
        0xfc31ffffu32,
        0xffffffu32,
        0u32,
        0xffff0000u32,
        0xffffffffu32,
        0xffffffffu32,
        0xf80001ffu32,
        0x3u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xffffd740u32,
        0xfffffffbu32,
        0x547f7fffu32,
        0xffffdu32,
        0xffffdffeu32,
        0xffffffffu32,
        0xdffeffffu32,
        0xffffffffu32,
        0xffff0003u32,
        0xffffffffu32,
        0xffff199fu32,
        0x33fcfffu32,
        0u32,
        0xfffe0000u32,
        0x27fffffu32,
        0xfffffffeu32,
        0x7fu32,
        0u32,
        0xffff0000u32,
        0x707ffu32,
        0u32,
        0x7fffffeu32,
        0x7feu32,
        0xfffe0000u32,
        0xffffffffu32,
        0x7cffffffu32,
        0x2f7fffu32,
        0x60u32,
        0xffffffe0u32,
        0x23ffffffu32,
        0xff000000u32,
        0x3u32,
        0xfff99fe0u32,
        0x3c5fdffu32,
        0xb0000000u32,
        0x30003u32,
        0xfff987e0u32,
        0x36dfdffu32,
        0x5e000000u32,
        0x1c0000u32,
        0xfffbafe0u32,
        0x23edfdffu32,
        0u32,
        0x1u32,
        0xfff99fe0u32,
        0x23cdfdffu32,
        0xb0000000u32,
        0x3u32,
        0xd63dc7e0u32,
        0x3bfc718u32,
        0u32,
        0u32,
        0xfffddfe0u32,
        0x3effdffu32,
        0u32,
        0x3u32,
        0xfffddfe0u32,
        0x3effdffu32,
        0x40000000u32,
        0x3u32,
        0xfffddfe0u32,
        0x3fffdffu32,
        0u32,
        0x3u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xfffffffeu32,
        0xd7fffu32,
        0x3fu32,
        0u32,
        0xfef02596u32,
        0x200d6caeu32,
        0x1fu32,
        0u32,
        0u32,
        0u32,
        0xfffffeffu32,
        0x3ffu32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xffffffffu32,
        0xffff003fu32,
        0x7fffffu32,
        0x7daedu32,
        0x50000000u32,
        0x82315001u32,
        0x2c62abu32,
        0x40000000u32,
        0xf580c900u32,
        0x7u32,
        0x2010800u32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xfffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0x3ffffffu32,
        0x3f3fffffu32,
        0xffffffffu32,
        0xaaff3f3fu32,
        0x3fffffffu32,
        0xffffffffu32,
        0x5fdfffffu32,
        0xfcf1fdcu32,
        0x1fdc1fffu32,
        0u32,
        0x4c40u32,
        0u32,
        0u32,
        0x7u32,
        0u32,
        0u32,
        0u32,
        0x80u32,
        0x3feu32,
        0xfffffffeu32,
        0xffffffffu32,
        0x1fffffu32,
        0xfffffffeu32,
        0xffffffffu32,
        0x7ffffffu32,
        0xffffffe0u32,
        0x1fffu32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0x3fu32,
        0u32,
        0u32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xffffffffu32,
        0xfu32,
        0u32,
        0u32,
        0u32,
        0x7ff6000u32,
        0x87fffffeu32,
        0x7fffffeu32,
        0u32,
        0x800000u32,
        0xff7fffffu32,
        0xff7fffffu32,
        0xffffffu32,
        0u32,
        0xffff0000u32,
        0xffffffffu32,
        0xffffffffu32,
        0xf80001ffu32,
        0x30003u32,
        0u32,
        0xffffffffu32,
        0xffffffffu32,
        0x3fu32,
        0x3u32,
        0xffffd7c0u32,
        0xfffffffbu32,
        0x547f7fffu32,
        0xffffdu32,
        0xffffdffeu32,
        0xffffffffu32,
        0xdffeffffu32,
        0xffffffffu32,
        0xffff007bu32,
        0xffffffffu32,
        0xffff199fu32,
        0x33fcfffu32,
        0u32,
        0xfffe0000u32,
        0x27fffffu32,
        0xfffffffeu32,
        0xfffe007fu32,
        0xbbfffffbu32,
        0xffff0016u32,
        0x707ffu32,
        0u32,
        0x7fffffeu32,
        0x7ffffu32,
        0xffff03ffu32,
        0xffffffffu32,
        0x7cffffffu32,
        0xffef7fffu32,
        0x3ff3dffu32,
        0xffffffeeu32,
        0xf3ffffffu32,
        0xff1e3fffu32,
        0xffcfu32,
        0xfff99feeu32,
        0xd3c5fdffu32,
        0xb080399fu32,
        0x3ffcfu32,
        0xfff987e4u32,
        0xd36dfdffu32,
        0x5e003987u32,
        0x1fffc0u32,
        0xfffbafeeu32,
        0xf3edfdffu32,
        0x3bbfu32,
        0xffc1u32,
        0xfff99feeu32,
        0xf3cdfdffu32,
        0xb0c0398fu32,
        0xffc3u32,
        0xd63dc7ecu32,
        0xc3bfc718u32,
        0x803dc7u32,
        0xff80u32,
        0xfffddfeeu32,
        0xc3effdffu32,
        0x603ddfu32,
        0xffc3u32,
        0xfffddfecu32,
        0xc3effdffu32,
        0x40603ddfu32,
        0xffc3u32,
        0xfffddfecu32,
        0xc3fffdffu32,
        0x803dcfu32,
        0xffc3u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0xfffffffeu32,
        0x7ff7fffu32,
        0x3ff7fffu32,
        0u32,
        0xfef02596u32,
        0x3bff6caeu32,
        0x3ff3f5fu32,
        0u32,
        0x3000000u32,
        0xc2a003ffu32,
        0xfffffeffu32,
        0xfffe03ffu32,
        0xfebf0fdfu32,
        0x2fe3fffu32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0u32,
        0x1fff0000u32,
        0x2u32,
        0xa0u32,
        0x3efffeu32,
        0xfffffffeu32,
        0xffffffffu32,
        0x661fffffu32,
        0xfffffffeu32,
        0xffffffffu32,
        0x77ffffffu32,
    ];

    pub static mut nmstrtPages: [::core::ffi::c_uchar; 256] = [
        0x2u8,
        0x3u8,
        0x4u8,
        0x5u8,
        0x6u8,
        0x7u8,
        0x8u8,
        0u8,
        0u8,
        0x9u8,
        0xau8,
        0xbu8,
        0xcu8,
        0xdu8,
        0xeu8,
        0xfu8,
        0x10u8,
        0x11u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x12u8,
        0x13u8,
        0u8,
        0x14u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x15u8,
        0x16u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x17u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x18u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
    ];

    pub static mut namePages: [::core::ffi::c_uchar; 256] = [
        0x19u8,
        0x3u8,
        0x1au8,
        0x1bu8,
        0x1cu8,
        0x1du8,
        0x1eu8,
        0u8,
        0u8,
        0x1fu8,
        0x20u8,
        0x21u8,
        0x22u8,
        0x23u8,
        0x24u8,
        0x25u8,
        0x10u8,
        0x11u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x12u8,
        0x13u8,
        0x26u8,
        0x14u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x27u8,
        0x16u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x17u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x1u8,
        0x18u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
        0u8,
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

pub use crate::src::lib::xmltok::nametab_h::namePages;
pub use crate::src::lib::xmltok::nametab_h::namingBitmap;
pub use crate::src::lib::xmltok::nametab_h::nmstrtPages;
pub use crate::stdbool_h::false_0;
pub use crate::stdbool_h::true_0;
use crate::stdlib::memcpy;

pub use crate::src::lib::xmltok::xmltok_impl_c::big2_attributeValueTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_cdataSectionTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_charRefNumber;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_checkPiTarget;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_contentTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_entityValueTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_getAtts;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_ignoreSectionTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_isPublicId;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_nameLength;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_nameMatchesAscii;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_predefinedEntityName;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_prologTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanAtts;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanCdataSection;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanCharRef;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanComment;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanDecl;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanEndTag;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanHexCharRef;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanLit;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanLt;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanPercent;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanPi;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanPoundName;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_scanRef;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_skipS;
pub use crate::src::lib::xmltok::xmltok_impl_c::big2_updatePosition;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_attributeValueTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_cdataSectionTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_charRefNumber;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_checkPiTarget;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_contentTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_entityValueTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_getAtts;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_ignoreSectionTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_isPublicId;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_nameLength;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_nameMatchesAscii;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_predefinedEntityName;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_prologTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanAtts;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanCdataSection;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanCharRef;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanComment;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanDecl;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanEndTag;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanHexCharRef;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanLit;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanLt;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanPercent;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanPi;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanPoundName;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_scanRef;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_skipS;
pub use crate::src::lib::xmltok::xmltok_impl_c::little2_updatePosition;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_attributeValueTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_cdataSectionTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_charRefNumber;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_checkPiTarget;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_contentTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_entityValueTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_getAtts;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_ignoreSectionTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_isPublicId;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_nameLength;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_nameMatchesAscii;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_predefinedEntityName;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_prologTok;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanAtts;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanCdataSection;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanCharRef;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanComment;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanDecl;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanEndTag;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanHexCharRef;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanLit;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanLt;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanPercent;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanPi;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanPoundName;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_scanRef;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_skipS;
pub use crate::src::lib::xmltok::xmltok_impl_c::normal_updatePosition;
pub use crate::src::lib::xmltok::xmltok_ns_c::encodings;
pub use crate::src::lib::xmltok::xmltok_ns_c::encodingsNS;
pub use crate::src::lib::xmltok::xmltok_ns_c::findEncoding;
pub use crate::src::lib::xmltok::xmltok_ns_c::findEncodingNS;
pub use crate::src::lib::xmltok::xmltok_ns_c::initScanContent;
pub use crate::src::lib::xmltok::xmltok_ns_c::initScanContentNS;
pub use crate::src::lib::xmltok::xmltok_ns_c::initScanProlog;
pub use crate::src::lib::xmltok::xmltok_ns_c::initScanPrologNS;
pub use crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf16InternalEncoding;
pub use crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf16InternalEncodingNS;
pub use crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding;
pub use crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS;
pub use crate::src::lib::xmltok::xmltok_ns_c::XmlInitEncoding;
pub use crate::src::lib::xmltok::xmltok_ns_c::XmlInitEncodingNS;
pub use crate::src::lib::xmltok::xmltok_ns_c::XmlParseXmlDecl;
pub use crate::src::lib::xmltok::xmltok_ns_c::XmlParseXmlDeclNS;
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
#[derive(Copy, Clone)]
#[repr(C)]

pub struct normal_encoding {
    pub enc: crate::src::lib::xmltok::ENCODING,
    pub type_0: [::core::ffi::c_uchar; 256],
    pub isName2: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub isName3: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub isName4: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub isNmstrt2: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub isNmstrt3: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub isNmstrt4: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub isInvalid2: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub isInvalid3: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub isInvalid4: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
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
    pub convert: crate::src::lib::xmltok::CONVERTER,
    pub userData: *mut ::core::ffi::c_void,
    pub utf16: [::core::ffi::c_ushort; 256],
    pub utf8: [[::core::ffi::c_char; 4]; 256],
}

pub type C2Rust_Unnamed_8 = ::core::ffi::c_uint;

pub type C2Rust_Unnamed_9 = ::core::ffi::c_int;

pub const US_ASCII_ENC: C2Rust_Unnamed_9 = 1;

unsafe extern "C" fn isNever(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return 0i32;
}

unsafe extern "C" fn utf8_isName2(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (namingBitmap[(((namePages[(*(p as *const ::core::ffi::c_uchar)
        .offset(0isize)
        as ::core::ffi::c_int
        >> 2i32
        & 7i32) as usize] as ::core::ffi::c_int)
        << 3i32)
        + ((*(p as *const ::core::ffi::c_uchar).offset(0isize)
            as ::core::ffi::c_int
            & 3i32)
            << 1i32)
        + (*(p as *const ::core::ffi::c_uchar).offset(1isize)
            as ::core::ffi::c_int
            >> 5i32
            & 1i32)) as usize]
        & (1u32)
            << (*(p as *const ::core::ffi::c_uchar).offset(1isize)
                as ::core::ffi::c_int
                & 0x1fi32)) as ::core::ffi::c_int;
}

unsafe extern "C" fn utf8_isName3(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (namingBitmap[(((namePages[(((*(p as *const ::core::ffi::c_uchar)
        .offset(0isize)
        as ::core::ffi::c_int
        & 0xfi32)
        << 4i32)
        + (*(p as *const ::core::ffi::c_uchar).offset(1isize)
            as ::core::ffi::c_int
            >> 2i32
            & 0xfi32)) as usize]
        as ::core::ffi::c_int)
        << 3i32)
        + ((*(p as *const ::core::ffi::c_uchar).offset(1isize)
            as ::core::ffi::c_int
            & 3i32)
            << 1i32)
        + (*(p as *const ::core::ffi::c_uchar).offset(2isize)
            as ::core::ffi::c_int
            >> 5i32
            & 1i32)) as usize]
        & (1u32)
            << (*(p as *const ::core::ffi::c_uchar).offset(2isize)
                as ::core::ffi::c_int
                & 0x1fi32)) as ::core::ffi::c_int;
}

unsafe extern "C" fn utf8_isNmstrt2(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (namingBitmap[(((nmstrtPages[(*(p as *const ::core::ffi::c_uchar)
        .offset(0isize)
        as ::core::ffi::c_int
        >> 2i32
        & 7i32) as usize] as ::core::ffi::c_int)
        << 3i32)
        + ((*(p as *const ::core::ffi::c_uchar).offset(0isize)
            as ::core::ffi::c_int
            & 3i32)
            << 1i32)
        + (*(p as *const ::core::ffi::c_uchar).offset(1isize)
            as ::core::ffi::c_int
            >> 5i32
            & 1i32)) as usize]
        & (1u32)
            << (*(p as *const ::core::ffi::c_uchar).offset(1isize)
                as ::core::ffi::c_int
                & 0x1fi32)) as ::core::ffi::c_int;
}

unsafe extern "C" fn utf8_isNmstrt3(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (namingBitmap[(((nmstrtPages[(((*(p as *const ::core::ffi::c_uchar)
        .offset(0isize)
        as ::core::ffi::c_int
        & 0xfi32)
        << 4i32)
        + (*(p as *const ::core::ffi::c_uchar).offset(1isize)
            as ::core::ffi::c_int
            >> 2i32
            & 0xfi32)) as usize]
        as ::core::ffi::c_int)
        << 3i32)
        + ((*(p as *const ::core::ffi::c_uchar).offset(1isize)
            as ::core::ffi::c_int
            & 3i32)
            << 1i32)
        + (*(p as *const ::core::ffi::c_uchar).offset(2isize)
            as ::core::ffi::c_int
            >> 5i32
            & 1i32)) as usize]
        & (1u32)
            << (*(p as *const ::core::ffi::c_uchar).offset(2isize)
                as ::core::ffi::c_int
                & 0x1fi32)) as ::core::ffi::c_int;
}

unsafe extern "C" fn utf8_isInvalid2(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return ((*(p as *const ::core::ffi::c_uchar) as ::core::ffi::c_int)
        < 0xc2i32
        || *(p as *const ::core::ffi::c_uchar).offset(1isize)
            as ::core::ffi::c_int
            & 0x80i32
            == 0i32
        || *(p as *const ::core::ffi::c_uchar).offset(1isize)
            as ::core::ffi::c_int
            & 0xc0i32
            == 0xc0i32) as ::core::ffi::c_int;
}

unsafe extern "C" fn utf8_isInvalid3(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (*(p as *const ::core::ffi::c_uchar).offset(2isize)
        as ::core::ffi::c_int
        & 0x80i32
        == 0i32
        || (if *(p as *const ::core::ffi::c_uchar) as ::core::ffi::c_int
            == 0xefi32
            && *(p as *const ::core::ffi::c_uchar).offset(1isize)
                as ::core::ffi::c_int
                == 0xbfi32
        {
            (*(p as *const ::core::ffi::c_uchar).offset(2isize)
                as ::core::ffi::c_int
                > 0xbdi32) as ::core::ffi::c_int
        } else {
            (*(p as *const ::core::ffi::c_uchar).offset(2isize)
                as ::core::ffi::c_int
                & 0xc0i32
                == 0xc0i32) as ::core::ffi::c_int
        }) != 0
        || (if *(p as *const ::core::ffi::c_uchar) as ::core::ffi::c_int
            == 0xe0i32
        {
            ((*(p as *const ::core::ffi::c_uchar).offset(1isize)
                as ::core::ffi::c_int)
                < 0xa0i32
                || *(p as *const ::core::ffi::c_uchar).offset(1isize)
                    as ::core::ffi::c_int
                    & 0xc0i32
                    == 0xc0i32) as ::core::ffi::c_int
        } else {
            (*(p as *const ::core::ffi::c_uchar).offset(1isize)
                as ::core::ffi::c_int
                & 0x80i32
                == 0i32
                || (if *(p as *const ::core::ffi::c_uchar) as ::core::ffi::c_int
                    == 0xedi32
                {
                    (*(p as *const ::core::ffi::c_uchar).offset(1isize)
                        as ::core::ffi::c_int
                        > 0x9fi32) as ::core::ffi::c_int
                } else {
                    (*(p as *const ::core::ffi::c_uchar).offset(1isize)
                        as ::core::ffi::c_int
                        & 0xc0i32
                        == 0xc0i32) as ::core::ffi::c_int
                }) != 0) as ::core::ffi::c_int
        }) != 0) as ::core::ffi::c_int;
}

unsafe extern "C" fn utf8_isInvalid4(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return (*(p as *const ::core::ffi::c_uchar).offset(3isize)
        as ::core::ffi::c_int
        & 0x80i32
        == 0i32
        || *(p as *const ::core::ffi::c_uchar).offset(3isize)
            as ::core::ffi::c_int
            & 0xc0i32
            == 0xc0i32
        || *(p as *const ::core::ffi::c_uchar).offset(2isize)
            as ::core::ffi::c_int
            & 0x80i32
            == 0i32
        || *(p as *const ::core::ffi::c_uchar).offset(2isize)
            as ::core::ffi::c_int
            & 0xc0i32
            == 0xc0i32
        || (if *(p as *const ::core::ffi::c_uchar) as ::core::ffi::c_int
            == 0xf0i32
        {
            ((*(p as *const ::core::ffi::c_uchar).offset(1isize)
                as ::core::ffi::c_int)
                < 0x90i32
                || *(p as *const ::core::ffi::c_uchar).offset(1isize)
                    as ::core::ffi::c_int
                    & 0xc0i32
                    == 0xc0i32) as ::core::ffi::c_int
        } else {
            (*(p as *const ::core::ffi::c_uchar).offset(1isize)
                as ::core::ffi::c_int
                & 0x80i32
                == 0i32
                || (if *(p as *const ::core::ffi::c_uchar) as ::core::ffi::c_int
                    == 0xf4i32
                {
                    (*(p as *const ::core::ffi::c_uchar).offset(1isize)
                        as ::core::ffi::c_int
                        > 0x8fi32) as ::core::ffi::c_int
                } else {
                    (*(p as *const ::core::ffi::c_uchar).offset(1isize)
                        as ::core::ffi::c_int
                        & 0xc0i32
                        == 0xc0i32) as ::core::ffi::c_int
                }) != 0) as ::core::ffi::c_int
        }) != 0) as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn _INTERNAL_trim_to_complete_utf8_characters(
    mut from: *const ::core::ffi::c_char,
    mut fromLimRef: *mut *const ::core::ffi::c_char,
) {
    let mut fromLim: *const ::core::ffi::c_char = *fromLimRef;
    let mut walked: size_t = 0usize;
    while fromLim > from {
        let prev: ::core::ffi::c_uchar =
            *fromLim.offset(-1isize) as ::core::ffi::c_uchar;
        if prev as ::core::ffi::c_uint & 0xf8u32 == 0xf0u32
        {
            if walked.wrapping_add(1usize)
                >= 4usize
            {
                fromLim =
                    fromLim.offset((4i32 - 1i32) as isize);
                break;
            } else {
                walked = 0usize;
            }
        } else if prev as ::core::ffi::c_uint & 0xf0u32
            == 0xe0u32
        {
            if walked.wrapping_add(1usize)
                >= 3usize
            {
                fromLim =
                    fromLim.offset((3i32 - 1i32) as isize);
                break;
            } else {
                walked = 0usize;
            }
        } else if prev as ::core::ffi::c_uint & 0xe0u32
            == 0xc0u32
        {
            if walked.wrapping_add(1usize)
                >= 2usize
            {
                fromLim =
                    fromLim.offset((2i32 - 1i32) as isize);
                break;
            } else {
                walked = 0usize;
            }
        } else if prev as ::core::ffi::c_uint & 0x80u32
            == 0u32
        {
            break;
        }
        fromLim = fromLim.offset(-1);
        walked = walked.wrapping_add(1);
    }
    *fromLimRef = fromLim;
}

unsafe extern "C" fn utf8_toUtf8(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    let mut input_incomplete: bool = false_0 != 0;
    let mut output_exhausted: bool = false_0 != 0;
    let bytesAvailable: ptrdiff_t =
        
        fromLim.offset_from(*fromP);
    let bytesStorable: ptrdiff_t =
        
        toLim.offset_from(*toP);
    if bytesAvailable > bytesStorable {
        fromLim = (*fromP).offset(bytesStorable);
        output_exhausted = true_0 != 0;
    }
    let fromLimBefore: *const ::core::ffi::c_char = fromLim;
    _INTERNAL_trim_to_complete_utf8_characters(*fromP, &raw mut fromLim);
    if fromLim < fromLimBefore {
        input_incomplete = true_0 != 0;
    }
    let bytesToCopy: ptrdiff_t =
        
        fromLim.offset_from(*fromP);
    memcpy(
        *toP as *mut ::core::ffi::c_void,
        *fromP as *const ::core::ffi::c_void,
        bytesToCopy as size_t,
    );
    *fromP = (*fromP).offset(bytesToCopy);
    *toP = (*toP).offset(bytesToCopy);
    if output_exhausted {
        return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else if input_incomplete {
        return crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
    };
}

unsafe extern "C" fn utf8_toUtf16(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    let mut c2rust_current_block: u64;
    let mut res: crate::src::lib::xmltok::XML_Convert_Result =
        crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
    let mut to: *mut ::core::ffi::c_ushort = *toP;
    let mut from: *const ::core::ffi::c_char = *fromP;
    loop {
        if !(from < fromLim && to < toLim as *mut ::core::ffi::c_ushort) {
            c2rust_current_block = 18317007320854588510;
            break;
        }
        match (*(enc as *const normal_encoding)).type_0[*from as ::core::ffi::c_uchar as usize]
            as ::core::ffi::c_int
        {
            5 => {
                if (fromLim.offset_from(from) as ::core::ffi::c_long) < 2i64 {
                    res = crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
                    c2rust_current_block = 7621590230452126720;
                    break;
                } else {
                    let c2rust_fresh0 = to;
                    to = to.offset(1);
                    *c2rust_fresh0 = ((*from.offset(0isize)
                        as ::core::ffi::c_int
                        & 0x1fi32)
                        << 6i32
                        | *from.offset(1isize) as ::core::ffi::c_int
                            & 0x3fi32)
                        as ::core::ffi::c_ushort;
                    from = from.offset(2isize);
                }
            }
            6 => {
                if (fromLim.offset_from(from) as ::core::ffi::c_long) < 3i64 {
                    res = crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
                    c2rust_current_block = 7621590230452126720;
                    break;
                } else {
                    let c2rust_fresh1 = to;
                    to = to.offset(1);
                    *c2rust_fresh1 = ((*from.offset(0isize)
                        as ::core::ffi::c_int
                        & 0xfi32)
                        << 12i32
                        | (*from.offset(1isize) as ::core::ffi::c_int
                            & 0x3fi32)
                            << 6i32
                        | *from.offset(2isize) as ::core::ffi::c_int
                            & 0x3fi32)
                        as ::core::ffi::c_ushort;
                    from = from.offset(3isize);
                }
            }
            7 => {
                let mut n: ::core::ffi::c_ulong = 0;
                if (toLim.offset_from(to) as ::core::ffi::c_long) < 2i64 {
                    res = crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                    c2rust_current_block = 7621590230452126720;
                    break;
                } else if (fromLim.offset_from(from) as ::core::ffi::c_long)
                    < 4i64
                {
                    res = crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
                    c2rust_current_block = 7621590230452126720;
                    break;
                } else {
                    n = ((*from.offset(0isize) as ::core::ffi::c_int
                        & 0x7i32)
                        << 18i32
                        | (*from.offset(1isize) as ::core::ffi::c_int
                            & 0x3fi32)
                            << 12i32
                        | (*from.offset(2isize) as ::core::ffi::c_int
                            & 0x3fi32)
                            << 6i32
                        | *from.offset(3isize) as ::core::ffi::c_int
                            & 0x3fi32)
                        as ::core::ffi::c_ulong;
                    n = n.wrapping_sub(0x10000u64);
                    *to.offset(0isize) = (n >> 10i32
                        | 0xd800u64)
                        as ::core::ffi::c_ushort;
                    *to.offset(1isize) =
                        (n & 0x3ffu64 | 0xdc00u64)
                            as ::core::ffi::c_ushort;
                    to = to.offset(2isize);
                    from = from.offset(4isize);
                }
            }
            _ => {
                let c2rust_fresh2 = from;
                from = from.offset(1);
                let c2rust_fresh3 = to;
                to = to.offset(1);
                *c2rust_fresh3 = *c2rust_fresh2 as ::core::ffi::c_ushort;
            }
        }
    }
    match c2rust_current_block {
        18317007320854588510 => {
            if from < fromLim {
                res = crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
            }
        }
        _ => {}
    }
    *fromP = from;
    *toP = to;
    return res;
}

static mut utf8_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                utf8_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                utf8_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 1i32,
            isUtf8: 1i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_COLON_0 as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_MALFORM as ::core::ffi::c_uchar,
            
            BT_MALFORM as ::core::ffi::c_uchar,
        ],
        isName2: Some(
            utf8_isName2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isName3: Some(
            utf8_isName3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isName4: Some(
            isNever
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt2: Some(
            utf8_isNmstrt2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt3: Some(
            utf8_isNmstrt3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt4: Some(
            isNever
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid2: Some(
            utf8_isInvalid2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid3: Some(
            utf8_isInvalid3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid4: Some(
            utf8_isInvalid4
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    }
};

static mut utf8_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                utf8_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                utf8_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 1i32,
            isUtf8: 1i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_MALFORM as ::core::ffi::c_uchar,
            
            BT_MALFORM as ::core::ffi::c_uchar,
        ],
        isName2: Some(
            utf8_isName2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isName3: Some(
            utf8_isName3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isName4: Some(
            isNever
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt2: Some(
            utf8_isNmstrt2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt3: Some(
            utf8_isNmstrt3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt4: Some(
            isNever
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid2: Some(
            utf8_isInvalid2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid3: Some(
            utf8_isInvalid3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid4: Some(
            utf8_isInvalid4
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    }
};

static mut internal_utf8_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                utf8_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                utf8_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 1i32,
            isUtf8: 1i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_COLON_0 as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_MALFORM as ::core::ffi::c_uchar,
            
            BT_MALFORM as ::core::ffi::c_uchar,
        ],
        isName2: Some(
            utf8_isName2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isName3: Some(
            utf8_isName3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isName4: Some(
            isNever
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt2: Some(
            utf8_isNmstrt2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt3: Some(
            utf8_isNmstrt3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt4: Some(
            isNever
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid2: Some(
            utf8_isInvalid2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid3: Some(
            utf8_isInvalid3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid4: Some(
            utf8_isInvalid4
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    }
};

static mut internal_utf8_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                utf8_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                utf8_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 1i32,
            isUtf8: 1i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_TRAIL as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD2 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD3 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_LEAD4 as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_MALFORM as ::core::ffi::c_uchar,
            
            BT_MALFORM as ::core::ffi::c_uchar,
        ],
        isName2: Some(
            utf8_isName2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isName3: Some(
            utf8_isName3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isName4: Some(
            isNever
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt2: Some(
            utf8_isNmstrt2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt3: Some(
            utf8_isNmstrt3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isNmstrt4: Some(
            isNever
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid2: Some(
            utf8_isInvalid2
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid3: Some(
            utf8_isInvalid3
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
        isInvalid4: Some(
            utf8_isInvalid4
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        ),
    }
};

unsafe extern "C" fn latin1_toUtf8(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    loop {
        let mut c: ::core::ffi::c_uchar = 0;
        if *fromP == fromLim {
            return crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
        }
        c = **fromP as ::core::ffi::c_uchar;
        if c as ::core::ffi::c_int & 0x80i32 != 0 {
            if (toLim.offset_from(*toP) as ::core::ffi::c_long) < 2i64 {
                return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            let c2rust_fresh6 = *toP;
            *toP = (*toP).offset(1);
            *c2rust_fresh6 = (c as ::core::ffi::c_int >> 6i32
                | UTF8_cval2 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            let c2rust_fresh7 = *toP;
            *toP = (*toP).offset(1);
            *c2rust_fresh7 = (c as ::core::ffi::c_int & 0x3fi32
                | 0x80i32) as ::core::ffi::c_char;
            *fromP = (*fromP).offset(1);
        } else {
            if *toP == toLim as *mut ::core::ffi::c_char {
                return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            let c2rust_fresh8 = *fromP;
            *fromP = (*fromP).offset(1);
            let c2rust_fresh9 = *toP;
            *toP = (*toP).offset(1);
            *c2rust_fresh9 = *c2rust_fresh8;
        }
    }
}

unsafe extern "C" fn latin1_toUtf16(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_ushort {
        let c2rust_fresh4 = *fromP;
        *fromP = (*fromP).offset(1);
        let c2rust_fresh5 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh5 = *c2rust_fresh4 as ::core::ffi::c_uchar as ::core::ffi::c_ushort;
    }
    if *toP == toLim as *mut ::core::ffi::c_ushort && *fromP < fromLim {
        return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
    };
}

static mut latin1_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                latin1_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                latin1_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 1i32,
            isUtf8: 0i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_COLON_0 as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
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
    }
};

static mut latin1_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                latin1_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                latin1_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 1i32,
            isUtf8: 0i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
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
    }
};

unsafe extern "C" fn ascii_toUtf8(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_char {
        let c2rust_fresh56 = *fromP;
        *fromP = (*fromP).offset(1);
        let c2rust_fresh57 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh57 = *c2rust_fresh56;
    }
    if *toP == toLim as *mut ::core::ffi::c_char && *fromP < fromLim {
        return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
    };
}

static mut ascii_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                ascii_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                latin1_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 1i32,
            isUtf8: 1i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_COLON_0 as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
    }
};

static mut ascii_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    normal_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    normal_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                normal_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                normal_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                normal_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                normal_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                normal_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                normal_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                normal_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                normal_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                ascii_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                latin1_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 1i32,
            isUtf8: 1i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
    }
};

unsafe extern "C" fn unicode_byte_type(
    mut hi: ::core::ffi::c_char,
    mut lo: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    match hi as ::core::ffi::c_uchar as ::core::ffi::c_int {
        216 | 217 | 218 | 219 => return BT_LEAD4 as ::core::ffi::c_int,
        220 | 221 | 222 | 223 => return BT_TRAIL as ::core::ffi::c_int,
        255 => match lo as ::core::ffi::c_uchar as ::core::ffi::c_int {
            255 | 254 => return BT_NONXML as ::core::ffi::c_int,
            _ => {}
        },
        _ => {}
    }
    return BT_NONASCII as ::core::ffi::c_int;
}

unsafe extern "C" fn little2_toUtf8(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    let mut from: *const ::core::ffi::c_char = *fromP;
    fromLim = from.offset(
        ((fromLim.offset_from(from) as ::core::ffi::c_long >> 1i32)
            << 1i32) as isize,
    );
    while from < fromLim {
        let mut plane: ::core::ffi::c_int = 0;
        let mut lo2: ::core::ffi::c_uchar = 0;
        let mut lo: ::core::ffi::c_uchar =
            *from.offset(0isize) as ::core::ffi::c_uchar;
        let mut hi: ::core::ffi::c_uchar =
            *from.offset(1isize) as ::core::ffi::c_uchar;
        let mut c2rust_current_block_34: u64;
        match hi as ::core::ffi::c_int {
            0 => {
                if (lo as ::core::ffi::c_int) < 0x80i32 {
                    if *toP == toLim as *mut ::core::ffi::c_char {
                        *fromP = from;
                        return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    let c2rust_fresh19 = *toP;
                    *toP = (*toP).offset(1);
                    *c2rust_fresh19 = lo as ::core::ffi::c_char;
                    c2rust_current_block_34 = 14136749492126903395;
                } else {
                    c2rust_current_block_34 = 9261908759940751603;
                }
            }
            1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                c2rust_current_block_34 = 9261908759940751603;
            }
            216 | 217 | 218 | 219 => {
                if (toLim.offset_from(*toP) as ::core::ffi::c_long) < 4i64 {
                    *fromP = from;
                    return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                if (fromLim.offset_from(from) as ::core::ffi::c_long) < 4i64 {
                    *fromP = from;
                    return crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
                }
                plane = ((hi as ::core::ffi::c_int & 0x3i32)
                    << 2i32
                    | lo as ::core::ffi::c_int >> 6i32
                        & 0x3i32)
                    + 1i32;
                let c2rust_fresh25 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh25 = (plane >> 2i32
                    | UTF8_cval4 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                let c2rust_fresh26 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh26 = (lo as ::core::ffi::c_int >> 2i32
                    & 0xfi32
                    | (plane & 0x3i32) << 4i32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                from = from.offset(2isize);
                lo2 = *from.offset(0isize) as ::core::ffi::c_uchar;
                let c2rust_fresh27 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh27 = ((lo as ::core::ffi::c_int & 0x3i32)
                    << 4i32
                    | (*from.offset(1isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        & 0x3i32)
                        << 2i32
                    | lo2 as ::core::ffi::c_int >> 6i32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                let c2rust_fresh28 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh28 = (lo2 as ::core::ffi::c_int & 0x3fi32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                c2rust_current_block_34 = 14136749492126903395;
            }
            _ => {
                if (toLim.offset_from(*toP) as ::core::ffi::c_long) < 3i64 {
                    *fromP = from;
                    return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let c2rust_fresh22 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh22 = (hi as ::core::ffi::c_int >> 4i32
                    | UTF8_cval3 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                let c2rust_fresh23 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh23 = ((hi as ::core::ffi::c_int & 0xfi32)
                    << 2i32
                    | lo as ::core::ffi::c_int >> 6i32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                let c2rust_fresh24 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh24 = (lo as ::core::ffi::c_int & 0x3fi32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                c2rust_current_block_34 = 14136749492126903395;
            }
        }
        match c2rust_current_block_34 {
            9261908759940751603 => {
                if (toLim.offset_from(*toP) as ::core::ffi::c_long) < 2i64 {
                    *fromP = from;
                    return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let c2rust_fresh20 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh20 = (lo as ::core::ffi::c_int >> 6i32
                    | (hi as ::core::ffi::c_int) << 2i32
                    | UTF8_cval2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                let c2rust_fresh21 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh21 = (lo as ::core::ffi::c_int & 0x3fi32
                    | 0x80i32)
                    as ::core::ffi::c_char;
            }
            _ => {}
        }
        from = from.offset(2isize);
    }
    *fromP = from;
    if from < fromLim {
        return crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
    };
}

unsafe extern "C" fn little2_toUtf16(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    let mut res: crate::src::lib::xmltok::XML_Convert_Result =
        crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
    fromLim = (*fromP).offset(
        ((fromLim.offset_from(*fromP) as ::core::ffi::c_long >> 1i32)
            << 1i32) as isize,
    );
    if fromLim.offset_from(*fromP) as ::core::ffi::c_long
        > (toLim.offset_from(*toP) as ::core::ffi::c_long) << 1i32
        && *fromLim
            .offset(-(2isize))
            .offset(1isize) as ::core::ffi::c_uchar
            as ::core::ffi::c_int
            & 0xf8i32
            == 0xd8i32
    {
        fromLim = fromLim.offset(-(2isize));
        res = crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    }
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_ushort {
        let c2rust_fresh18 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh18 = ((*(*fromP).offset(1isize)
            as ::core::ffi::c_uchar as ::core::ffi::c_int)
            << 8i32
            | *(*fromP).offset(0isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int) as ::core::ffi::c_ushort;
        *fromP = (*fromP).offset(2isize);
    }
    if *toP == toLim as *mut ::core::ffi::c_ushort && *fromP < fromLim {
        return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return res;
    };
}

unsafe extern "C" fn big2_toUtf8(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    let mut from: *const ::core::ffi::c_char = *fromP;
    fromLim = from.offset(
        ((fromLim.offset_from(from) as ::core::ffi::c_long >> 1i32)
            << 1i32) as isize,
    );
    while from < fromLim {
        let mut plane: ::core::ffi::c_int = 0;
        let mut lo2: ::core::ffi::c_uchar = 0;
        let mut lo: ::core::ffi::c_uchar =
            *from.offset(1isize) as ::core::ffi::c_uchar;
        let mut hi: ::core::ffi::c_uchar =
            *from.offset(0isize) as ::core::ffi::c_uchar;
        let mut c2rust_current_block_34: u64;
        match hi as ::core::ffi::c_int {
            0 => {
                if (lo as ::core::ffi::c_int) < 0x80i32 {
                    if *toP == toLim as *mut ::core::ffi::c_char {
                        *fromP = from;
                        return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                    }
                    let c2rust_fresh38 = *toP;
                    *toP = (*toP).offset(1);
                    *c2rust_fresh38 = lo as ::core::ffi::c_char;
                    c2rust_current_block_34 = 14136749492126903395;
                } else {
                    c2rust_current_block_34 = 4084411463441859965;
                }
            }
            1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                c2rust_current_block_34 = 4084411463441859965;
            }
            216 | 217 | 218 | 219 => {
                if (toLim.offset_from(*toP) as ::core::ffi::c_long) < 4i64 {
                    *fromP = from;
                    return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                if (fromLim.offset_from(from) as ::core::ffi::c_long) < 4i64 {
                    *fromP = from;
                    return crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
                }
                plane = ((hi as ::core::ffi::c_int & 0x3i32)
                    << 2i32
                    | lo as ::core::ffi::c_int >> 6i32
                        & 0x3i32)
                    + 1i32;
                let c2rust_fresh44 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh44 = (plane >> 2i32
                    | UTF8_cval4 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                let c2rust_fresh45 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh45 = (lo as ::core::ffi::c_int >> 2i32
                    & 0xfi32
                    | (plane & 0x3i32) << 4i32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                from = from.offset(2isize);
                lo2 = *from.offset(1isize) as ::core::ffi::c_uchar;
                let c2rust_fresh46 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh46 = ((lo as ::core::ffi::c_int & 0x3i32)
                    << 4i32
                    | (*from.offset(0isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int
                        & 0x3i32)
                        << 2i32
                    | lo2 as ::core::ffi::c_int >> 6i32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                let c2rust_fresh47 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh47 = (lo2 as ::core::ffi::c_int & 0x3fi32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                c2rust_current_block_34 = 14136749492126903395;
            }
            _ => {
                if (toLim.offset_from(*toP) as ::core::ffi::c_long) < 3i64 {
                    *fromP = from;
                    return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let c2rust_fresh41 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh41 = (hi as ::core::ffi::c_int >> 4i32
                    | UTF8_cval3 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                let c2rust_fresh42 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh42 = ((hi as ::core::ffi::c_int & 0xfi32)
                    << 2i32
                    | lo as ::core::ffi::c_int >> 6i32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                let c2rust_fresh43 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh43 = (lo as ::core::ffi::c_int & 0x3fi32
                    | 0x80i32)
                    as ::core::ffi::c_char;
                c2rust_current_block_34 = 14136749492126903395;
            }
        }
        match c2rust_current_block_34 {
            4084411463441859965 => {
                if (toLim.offset_from(*toP) as ::core::ffi::c_long) < 2i64 {
                    *fromP = from;
                    return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
                }
                let c2rust_fresh39 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh39 = (lo as ::core::ffi::c_int >> 6i32
                    | (hi as ::core::ffi::c_int) << 2i32
                    | UTF8_cval2 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                let c2rust_fresh40 = *toP;
                *toP = (*toP).offset(1);
                *c2rust_fresh40 = (lo as ::core::ffi::c_int & 0x3fi32
                    | 0x80i32)
                    as ::core::ffi::c_char;
            }
            _ => {}
        }
        from = from.offset(2isize);
    }
    *fromP = from;
    if from < fromLim {
        return crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    } else {
        return crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
    };
}

unsafe extern "C" fn big2_toUtf16(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    let mut res: crate::src::lib::xmltok::XML_Convert_Result =
        crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
    fromLim = (*fromP).offset(
        ((fromLim.offset_from(*fromP) as ::core::ffi::c_long >> 1i32)
            << 1i32) as isize,
    );
    if fromLim.offset_from(*fromP) as ::core::ffi::c_long
        > (toLim.offset_from(*toP) as ::core::ffi::c_long) << 1i32
        && *fromLim
            .offset(-(2isize))
            .offset(0isize) as ::core::ffi::c_uchar
            as ::core::ffi::c_int
            & 0xf8i32
            == 0xd8i32
    {
        fromLim = fromLim.offset(-(2isize));
        res = crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
    }
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_ushort {
        let c2rust_fresh37 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh37 = ((*(*fromP).offset(0isize)
            as ::core::ffi::c_uchar as ::core::ffi::c_int)
            << 8i32
            | *(*fromP).offset(1isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int) as ::core::ffi::c_ushort;
        *fromP = (*fromP).offset(2isize);
    }
    if *toP == toLim as *mut ::core::ffi::c_ushort && *fromP < fromLim {
        return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return res;
    };
}

static mut little2_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    little2_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                little2_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                little2_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                little2_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                little2_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                little2_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                little2_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                little2_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                little2_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                little2_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                little2_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 2i32,
            isUtf8: 0i8,
            isUtf16: 1i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_COLON_0 as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
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
    }
};

static mut little2_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    little2_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                little2_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                little2_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                little2_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                little2_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                little2_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                little2_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                little2_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                little2_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                little2_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                little2_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 2i32,
            isUtf8: 0i8,
            isUtf16: 1i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
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
    }
};

static mut internal_little2_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    little2_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                little2_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                little2_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                little2_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                little2_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                little2_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                little2_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                little2_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                little2_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                little2_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                little2_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 2i32,
            isUtf8: 0i8,
            isUtf16: 1i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_COLON_0 as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
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
    }
};

static mut internal_little2_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    little2_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    little2_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                little2_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                little2_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                little2_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                little2_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                little2_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                little2_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                little2_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                little2_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                little2_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                little2_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 2i32,
            isUtf8: 0i8,
            isUtf16: 1i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
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
    }
};

static mut big2_encoding_ns: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    big2_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                big2_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                big2_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                big2_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                big2_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                big2_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                big2_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                big2_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                big2_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                big2_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                big2_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 2i32,
            isUtf8: 0i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_COLON_0 as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
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
    }
};

static mut big2_encoding: normal_encoding = unsafe {
    normal_encoding {
        enc: crate::src::lib::xmltok::encoding {
            scanners: [
                Some(
                    big2_prologTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_contentTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_cdataSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_ignoreSectionTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
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
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
                Some(
                    big2_entityValueTok
                        as unsafe extern "C" fn(
                            *const crate::src::lib::xmltok::ENCODING,
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                ),
            ],
            nameMatchesAscii: Some(
                big2_nameMatchesAscii
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            nameLength: Some(
                big2_nameLength
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            skipS: Some(
                big2_skipS
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> *const ::core::ffi::c_char,
            ),
            getAtts: Some(
                big2_getAtts
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        ::core::ffi::c_int,
                        *mut crate::src::lib::xmltok::ATTRIBUTE,
                    ) -> ::core::ffi::c_int,
            ),
            charRefNumber: Some(
                big2_charRefNumber
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            predefinedEntityName: Some(
                big2_predefinedEntityName
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            updatePosition: Some(
                big2_updatePosition
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut crate::src::lib::xmltok::POSITION,
                    ) -> (),
            ),
            isPublicId: Some(
                big2_isPublicId
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            utf8Convert: Some(
                big2_toUtf8
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            utf16Convert: Some(
                big2_toUtf16
                    as unsafe extern "C" fn(
                        *const crate::src::lib::xmltok::ENCODING,
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut *mut ::core::ffi::c_ushort,
                        *const ::core::ffi::c_ushort,
                    )
                        -> crate::src::lib::xmltok::XML_Convert_Result,
            ),
            minBytesPerChar: 2i32,
            isUtf8: 0i8,
            isUtf16: 0i8,
        },
        type_0: [
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_LF as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_CR as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_NONXML as ::core::ffi::c_uchar,
            
            BT_S as ::core::ffi::c_uchar,
            
            BT_EXCL as ::core::ffi::c_uchar,
            
            BT_QUOT as ::core::ffi::c_uchar,
            
            BT_NUM as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_PERCNT as ::core::ffi::c_uchar,
            
            BT_AMP as ::core::ffi::c_uchar,
            
            BT_APOS as ::core::ffi::c_uchar,
            
            BT_LPAR as ::core::ffi::c_uchar,
            
            BT_RPAR as ::core::ffi::c_uchar,
            
            BT_AST as ::core::ffi::c_uchar,
            
            BT_PLUS as ::core::ffi::c_uchar,
            
            BT_COMMA as ::core::ffi::c_uchar,
            
            BT_MINUS as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_SOL as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_DIGIT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_SEMI as ::core::ffi::c_uchar,
            
            BT_LT as ::core::ffi::c_uchar,
            
            BT_EQUALS as ::core::ffi::c_uchar,
            
            BT_GT as ::core::ffi::c_uchar,
            
            BT_QUEST as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_LSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_RSQB as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_HEX as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_VERBAR as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NAME as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_OTHER as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
            
            BT_NMSTRT as ::core::ffi::c_uchar,
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
    }
};

unsafe extern "C" fn streqci(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    loop {
        let c2rust_fresh58 = s1;
        s1 = s1.offset(1);
        let mut c1: ::core::ffi::c_char = *c2rust_fresh58;
        let c2rust_fresh59 = s2;
        s2 = s2.offset(1);
        let mut c2: ::core::ffi::c_char = *c2rust_fresh59;
        if ASCII_a_1 <= c1 as ::core::ffi::c_int
            && c1 as ::core::ffi::c_int <= ASCII_z
        {
            c1 = (c1 as ::core::ffi::c_int + (ASCII_A - ASCII_a_1))
                as ::core::ffi::c_char;
        }
        if ASCII_a_1 <= c2 as ::core::ffi::c_int
            && c2 as ::core::ffi::c_int <= ASCII_z
        {
            c2 = (c2 as ::core::ffi::c_int + (ASCII_A - ASCII_a_1))
                as ::core::ffi::c_char;
        }
        if c1 as ::core::ffi::c_int != c2 as ::core::ffi::c_int {
            return 0i32;
        }
        if c1 == 0 {
            break;
        }
    }
    return 1i32;
}

unsafe extern "C" fn initUpdatePosition(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut pos: *mut crate::src::lib::xmltok::POSITION,
) {
    normal_updatePosition(&raw const utf8_encoding.enc, ptr, end, pos);
}

unsafe extern "C" fn toAscii(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 1] = [0; 1];
    let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    (*enc).utf8Convert.expect("non-null function pointer")(
        enc,
        &raw mut ptr,
        end,
        &raw mut p,
        p.offset(1isize),
    );
    if p == &raw mut buf as *mut ::core::ffi::c_char {
        return -1i32;
    } else {
        return buf[0usize] as ::core::ffi::c_int;
    };
}

unsafe extern "C" fn isSpace(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match c {
        32 | 13 | 10 | 9 => return 1i32,
        _ => {}
    }
    return 0i32;
}

unsafe extern "C" fn parsePseudoAttribute(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut namePtr: *mut *const ::core::ffi::c_char,
    mut nameEndPtr: *mut *const ::core::ffi::c_char,
    mut valPtr: *mut *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut open: ::core::ffi::c_char = 0;
    if ptr == end {
        *namePtr = ::core::ptr::null::<::core::ffi::c_char>();
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
        *namePtr = ::core::ptr::null::<::core::ffi::c_char>();
        return 1i32;
    }
    *namePtr = ptr;
    loop {
        c = toAscii(enc, ptr, end);
        if c == -1i32 {
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
    open = c as ::core::ffi::c_char;
    ptr = ptr.offset((*enc).minBytesPerChar as isize);
    *valPtr = ptr;
    loop {
        c = toAscii(enc, ptr, end);
        if c == open as ::core::ffi::c_int {
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
    return 1i32;
}

static mut KW_version: [::core::ffi::c_char; 8] = [
    ASCII_v as ::core::ffi::c_char,
    ASCII_e_1 as ::core::ffi::c_char,
    ASCII_r as ::core::ffi::c_char,
    ASCII_s as ::core::ffi::c_char,
    ASCII_i as ::core::ffi::c_char,
    ASCII_o as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

static mut KW_encoding: [::core::ffi::c_char; 9] = [
    ASCII_e_1 as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    ASCII_c_1 as ::core::ffi::c_char,
    ASCII_o as ::core::ffi::c_char,
    ASCII_d as ::core::ffi::c_char,
    ASCII_i as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    ASCII_g_1 as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

static mut KW_standalone: [::core::ffi::c_char; 11] = [
    ASCII_s as ::core::ffi::c_char,
    ASCII_t as ::core::ffi::c_char,
    ASCII_a_1 as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    ASCII_d as ::core::ffi::c_char,
    ASCII_a_1 as ::core::ffi::c_char,
    ASCII_l_1 as ::core::ffi::c_char,
    ASCII_o as ::core::ffi::c_char,
    ASCII_n as ::core::ffi::c_char,
    ASCII_e_1 as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

static mut KW_yes: [::core::ffi::c_char; 4] = [
    ASCII_y as ::core::ffi::c_char,
    ASCII_e_1 as ::core::ffi::c_char,
    ASCII_s as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

static mut KW_no: [::core::ffi::c_char; 3] = [
    ASCII_n as ::core::ffi::c_char,
    ASCII_o as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

unsafe extern "C" fn doParseXmlDecl(
    mut encodingFinder: Option<
        unsafe extern "C" fn(
            *const crate::src::lib::xmltok::ENCODING,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> *const crate::src::lib::xmltok::ENCODING,
    >,
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
    let mut val: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nameEnd: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
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
    if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        name,
        nameEnd,
        &raw const KW_version as *const ::core::ffi::c_char,
    ) == 0
    {
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
    if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        name,
        nameEnd,
        &raw const KW_encoding as *const ::core::ffi::c_char,
    ) != 0
    {
        let mut c: ::core::ffi::c_int = toAscii(enc, val, end);
        if !(ASCII_a_1 <= c && c <= ASCII_z)
            && !(ASCII_A <= c && c <= ASCII_Z)
        {
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
    if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        name,
        nameEnd,
        &raw const KW_standalone as *const ::core::ffi::c_char,
    ) == 0
        || isGeneralTextEntity != 0
    {
        *badPtr = name;
        return 0i32;
    }
    if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        val,
        ptr.offset(-((*enc).minBytesPerChar as isize)),
        &raw const KW_yes as *const ::core::ffi::c_char,
    ) != 0
    {
        if !standalone.is_null() {
            *standalone = 1i32;
        }
    } else if (*enc).nameMatchesAscii.expect("non-null function pointer")(
        enc,
        val,
        ptr.offset(-((*enc).minBytesPerChar as isize)),
        &raw const KW_no as *const ::core::ffi::c_char,
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
    return 1i32;
}

unsafe extern "C" fn checkCharRefNumber(mut result: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match result >> 8i32 {
        216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 => return -1i32,
        0 => {
            if latin1_encoding.type_0[result as usize] as ::core::ffi::c_int
                == BT_NONXML as ::core::ffi::c_int
            {
                return -1i32;
            }
        }
        255 => {
            if result == 0xfffei32 || result == 0xffffi32 {
                return -1i32;
            }
        }
        _ => {}
    }
    return result;
}
#[no_mangle]

pub unsafe extern "C" fn XmlUtf8Encode(
    mut c: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if c < 0i32 {
        return 0i32;
    }
    if c < min2 as ::core::ffi::c_int {
        *buf.offset(0isize) =
            (c | UTF8_cval1 as ::core::ffi::c_int) as ::core::ffi::c_char;
        return 1i32;
    }
    if c < min3 as ::core::ffi::c_int {
        *buf.offset(0isize) = (c >> 6i32
            | UTF8_cval2 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        *buf.offset(1isize) =
            (c & 0x3fi32 | 0x80i32) as ::core::ffi::c_char;
        return 2i32;
    }
    if c < min4 as ::core::ffi::c_int {
        *buf.offset(0isize) = (c >> 12i32
            | UTF8_cval3 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        *buf.offset(1isize) = (c >> 6i32
            & 0x3fi32
            | 0x80i32)
            as ::core::ffi::c_char;
        *buf.offset(2isize) =
            (c & 0x3fi32 | 0x80i32) as ::core::ffi::c_char;
        return 3i32;
    }
    if c < 0x110000i32 {
        *buf.offset(0isize) = (c >> 18i32
            | UTF8_cval4 as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        *buf.offset(1isize) =
            (c >> 12i32 & 0x3fi32
                | 0x80i32) as ::core::ffi::c_char;
        *buf.offset(2isize) = (c >> 6i32
            & 0x3fi32
            | 0x80i32)
            as ::core::ffi::c_char;
        *buf.offset(3isize) =
            (c & 0x3fi32 | 0x80i32) as ::core::ffi::c_char;
        return 4i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn XmlUtf16Encode(
    mut charNum: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_ushort,
) -> ::core::ffi::c_int {
    if charNum < 0i32 {
        return 0i32;
    }
    if charNum < 0x10000i32 {
        *buf.offset(0isize) = charNum as ::core::ffi::c_ushort;
        return 1i32;
    }
    if charNum < 0x110000i32 {
        charNum -= 0x10000i32;
        *buf.offset(0isize) = ((charNum >> 10i32)
            + 0xd800i32)
            as ::core::ffi::c_ushort;
        *buf.offset(1isize) = ((charNum & 0x3ffi32)
            + 0xdc00i32)
            as ::core::ffi::c_ushort;
        return 2i32;
    }
    return 0i32;
}
#[no_mangle]

pub unsafe extern "C" fn XmlSizeOfUnknownEncoding() -> ::core::ffi::c_int {
    return ::core::mem::size_of::<unknown_encoding>() as ::core::ffi::c_int;
}

unsafe extern "C" fn unknown_isName(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: ::core::ffi::c_int =
        (*uenc).convert.expect("non-null function pointer")((*uenc).userData, p);
    if c & !(0xffffi32) != 0 {
        return 0i32;
    }
    return (namingBitmap[(((namePages[(c >> 8i32) as usize]
        as ::core::ffi::c_int)
        << 3i32)
        + ((c & 0xffi32) >> 5i32))
        as usize]
        & (1u32)
            << (c & 0xffi32 & 0x1fi32))
        as ::core::ffi::c_int;
}

unsafe extern "C" fn unknown_isNmstrt(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: ::core::ffi::c_int =
        (*uenc).convert.expect("non-null function pointer")((*uenc).userData, p);
    if c & !(0xffffi32) != 0 {
        return 0i32;
    }
    return (namingBitmap[(((nmstrtPages[(c >> 8i32) as usize]
        as ::core::ffi::c_int)
        << 3i32)
        + ((c & 0xffi32) >> 5i32))
        as usize]
        & (1u32)
            << (c & 0xffi32 & 0x1fi32))
        as ::core::ffi::c_int;
}

unsafe extern "C" fn unknown_isInvalid(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut p: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut c: ::core::ffi::c_int =
        (*uenc).convert.expect("non-null function pointer")((*uenc).userData, p);
    return (c & !(0xffffi32) != 0
        || checkCharRefNumber(c) < 0i32) as ::core::ffi::c_int;
}

unsafe extern "C" fn unknown_toUtf8(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_char,
    mut toLim: *const ::core::ffi::c_char,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    let mut buf: [::core::ffi::c_char; 4] = [0; 4];
    loop {
        let mut utf8: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut n: ::core::ffi::c_int = 0;
        if *fromP == fromLim {
            return crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
        }
        utf8 = &raw const *(&raw const (*uenc).utf8 as *const [::core::ffi::c_char; 4])
            .offset(**fromP as ::core::ffi::c_uchar as isize)
            as *const ::core::ffi::c_char;
        let c2rust_fresh61 = utf8;
        utf8 = utf8.offset(1);
        n = *c2rust_fresh61 as ::core::ffi::c_int;
        if n == 0i32 {
            let mut c: ::core::ffi::c_int =
                (*uenc).convert.expect("non-null function pointer")((*uenc).userData, *fromP);
            n = XmlUtf8Encode(c, &raw mut buf as *mut ::core::ffi::c_char);
            if n as ::core::ffi::c_long > toLim.offset_from(*toP) as ::core::ffi::c_long {
                return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            utf8 = &raw mut buf as *mut ::core::ffi::c_char;
            *fromP = (*fromP).offset(
                ((*(enc as *const normal_encoding)).type_0[**fromP as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                    - (BT_LEAD2 as ::core::ffi::c_int
                        - 2i32)) as isize,
            );
        } else {
            if n as ::core::ffi::c_long > toLim.offset_from(*toP) as ::core::ffi::c_long {
                return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
            }
            *fromP = (*fromP).offset(1);
        }
        memcpy(
            *toP as *mut ::core::ffi::c_void,
            utf8 as *const ::core::ffi::c_void,
            n as size_t,
        );
        *toP = (*toP).offset(n as isize);
    }
}

unsafe extern "C" fn unknown_toUtf16(
    mut enc: *const crate::src::lib::xmltok::ENCODING,
    mut fromP: *mut *const ::core::ffi::c_char,
    mut fromLim: *const ::core::ffi::c_char,
    mut toP: *mut *mut ::core::ffi::c_ushort,
    mut toLim: *const ::core::ffi::c_ushort,
) -> crate::src::lib::xmltok::XML_Convert_Result {
    let mut uenc: *const unknown_encoding = enc as *const unknown_encoding;
    while *fromP < fromLim && *toP < toLim as *mut ::core::ffi::c_ushort {
        let mut c: ::core::ffi::c_ushort = (*uenc).utf16[**fromP as ::core::ffi::c_uchar as usize];
        if c as ::core::ffi::c_int == 0i32 {
            c = (*uenc).convert.expect("non-null function pointer")((*uenc).userData, *fromP)
                as ::core::ffi::c_ushort;
            *fromP = (*fromP).offset(
                ((*(enc as *const normal_encoding)).type_0[**fromP as ::core::ffi::c_uchar as usize]
                    as ::core::ffi::c_int
                    - (BT_LEAD2 as ::core::ffi::c_int
                        - 2i32)) as isize,
            );
        } else {
            *fromP = (*fromP).offset(1);
        }
        let c2rust_fresh60 = *toP;
        *toP = (*toP).offset(1);
        *c2rust_fresh60 = c;
    }
    if *toP == toLim as *mut ::core::ffi::c_ushort && *fromP < fromLim {
        return crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
    } else {
        return crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
    };
}
#[no_mangle]

pub unsafe extern "C" fn XmlInitUnknownEncoding(
    mut mem: *mut ::core::ffi::c_void,
    mut table: *const ::core::ffi::c_int,
    mut convert: crate::src::lib::xmltok::CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> *mut crate::src::lib::xmltok::ENCODING {
    let mut i: ::core::ffi::c_int = 0;
    let mut e: *mut unknown_encoding = mem as *mut unknown_encoding;
    memcpy(
        mem,
        &raw const latin1_encoding as *const ::core::ffi::c_void,
        
        ::core::mem::size_of::<normal_encoding>(),
    );
    i = 0i32;
    while i < 128i32 {
        if latin1_encoding.type_0[i as usize] as ::core::ffi::c_int
            != BT_OTHER as ::core::ffi::c_int
            && latin1_encoding.type_0[i as usize] as ::core::ffi::c_int
                != BT_NONXML as ::core::ffi::c_int
            && *table.offset(i as isize) != i
        {
            return ::core::ptr::null_mut::<crate::src::lib::xmltok::ENCODING>();
        }
        i += 1;
    }
    i = 0i32;
    while i < 256i32 {
        let mut c: ::core::ffi::c_int = *table.offset(i as isize);
        if c == -1i32 {
            (*e).normal.type_0[i as usize] =
                
                BT_MALFORM as ::core::ffi::c_uchar;
            (*e).utf16[i as usize] = 0xffffu16;
            (*e).utf8[i as usize][0usize] = 1i8;
            (*e).utf8[i as usize][1usize] = 0i8;
        } else if c < 0i32 {
            if c < -4i32 {
                return ::core::ptr::null_mut::<crate::src::lib::xmltok::ENCODING>();
            }
            if convert.is_none() {
                return ::core::ptr::null_mut::<crate::src::lib::xmltok::ENCODING>();
            }
            (*e).normal.type_0[i as usize] = (BT_LEAD2 as ::core::ffi::c_int
                - (c + 2i32))
                as ::core::ffi::c_uchar;
            (*e).utf8[i as usize][0usize] = 0i8;
            (*e).utf16[i as usize] = 0u16;
        } else if c < 0x80i32 {
            if latin1_encoding.type_0[c as usize] as ::core::ffi::c_int
                != BT_OTHER as ::core::ffi::c_int
                && latin1_encoding.type_0[c as usize] as ::core::ffi::c_int
                    != BT_NONXML as ::core::ffi::c_int
                && c != i
            {
                return ::core::ptr::null_mut::<crate::src::lib::xmltok::ENCODING>();
            }
            (*e).normal.type_0[i as usize] = latin1_encoding.type_0[c as usize];
            (*e).utf8[i as usize][0usize] = 1i8;
            (*e).utf8[i as usize][1usize] = c as ::core::ffi::c_char;
            (*e).utf16[i as usize] = (if c == 0i32 {
                0xffffi32
            } else {
                c
            }) as ::core::ffi::c_ushort;
        } else if checkCharRefNumber(c) < 0i32 {
            (*e).normal.type_0[i as usize] =
                
                BT_NONXML as ::core::ffi::c_uchar;
            (*e).utf16[i as usize] = 0xffffu16;
            (*e).utf8[i as usize][0usize] = 1i8;
            (*e).utf8[i as usize][1usize] = 0i8;
        } else {
            if c > 0xffffi32 {
                return ::core::ptr::null_mut::<crate::src::lib::xmltok::ENCODING>();
            }
            if namingBitmap[(((nmstrtPages[(c >> 8i32) as usize]
                as ::core::ffi::c_int)
                << 3i32)
                + ((c & 0xffi32) >> 5i32))
                as usize]
                & (1u32)
                    << (c & 0xffi32 & 0x1fi32)
                != 0
            {
                (*e).normal.type_0[i as usize] =
                    
                    BT_NMSTRT as ::core::ffi::c_uchar;
            } else if namingBitmap[(((namePages[(c >> 8i32) as usize]
                as ::core::ffi::c_int)
                << 3i32)
                + ((c & 0xffi32) >> 5i32))
                as usize]
                & (1u32)
                    << (c & 0xffi32 & 0x1fi32)
                != 0
            {
                (*e).normal.type_0[i as usize] =
                    
                    BT_NAME as ::core::ffi::c_uchar;
            } else {
                (*e).normal.type_0[i as usize] =
                    
                    BT_OTHER as ::core::ffi::c_uchar;
            }
            (*e).utf8[i as usize][0usize] = XmlUtf8Encode(
                c,
                (&raw mut *(&raw mut (*e).utf8 as *mut [::core::ffi::c_char; 4]).offset(i as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(1isize),
            )
                as ::core::ffi::c_char;
            (*e).utf16[i as usize] = c as ::core::ffi::c_ushort;
        }
        i += 1;
    }
    (*e).userData = userData;
    (*e).convert = convert;
    if convert.is_some() {
        (*e).normal.isName2 =  Some(
            unknown_isName
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*e).normal.isName3 =  Some(
            unknown_isName
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*e).normal.isName4 =  Some(
            unknown_isName
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*e).normal.isNmstrt2 =  Some(
            unknown_isNmstrt
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*e).normal.isNmstrt3 =  Some(
            unknown_isNmstrt
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*e).normal.isNmstrt4 =  Some(
            unknown_isNmstrt
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*e).normal.isInvalid2 =  Some(
            unknown_isInvalid
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*e).normal.isInvalid3 =  Some(
            unknown_isInvalid
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
        (*e).normal.isInvalid4 =  Some(
            unknown_isInvalid
                as unsafe extern "C" fn(
                    *const crate::src::lib::xmltok::ENCODING,
                    *const ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        );
    }
    (*e).normal.enc.utf8Convert =  Some(
        unknown_toUtf8
            as unsafe extern "C" fn(
                *const crate::src::lib::xmltok::ENCODING,
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
                *const ::core::ffi::c_char,
            ) -> crate::src::lib::xmltok::XML_Convert_Result,
    );
    (*e).normal.enc.utf16Convert =  Some(
        unknown_toUtf16
            as unsafe extern "C" fn(
                *const crate::src::lib::xmltok::ENCODING,
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_ushort,
                *const ::core::ffi::c_ushort,
            ) -> crate::src::lib::xmltok::XML_Convert_Result,
    );
    return &raw mut (*e).normal.enc;
}

static mut KW_ISO_8859_1: [::core::ffi::c_char; 11] = [
    ASCII_I as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_O as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_8_1 as ::core::ffi::c_char,
    ASCII_8_1 as ::core::ffi::c_char,
    ASCII_5 as ::core::ffi::c_char,
    ASCII_9_1 as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_1_1 as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

static mut KW_US_ASCII: [::core::ffi::c_char; 9] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_A as ::core::ffi::c_char,
    ASCII_S as ::core::ffi::c_char,
    ASCII_C as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    ASCII_I as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

static mut KW_UTF_8: [::core::ffi::c_char; 6] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_F_1 as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_8_1 as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

static mut KW_UTF_16: [::core::ffi::c_char; 7] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_F_1 as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_1_1 as ::core::ffi::c_char,
    ASCII_6 as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

static mut KW_UTF_16BE: [::core::ffi::c_char; 9] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_F_1 as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_1_1 as ::core::ffi::c_char,
    ASCII_6 as ::core::ffi::c_char,
    ASCII_B_1 as ::core::ffi::c_char,
    ASCII_E_1 as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

static mut KW_UTF_16LE: [::core::ffi::c_char; 9] = [
    ASCII_U as ::core::ffi::c_char,
    ASCII_T as ::core::ffi::c_char,
    ASCII_F_1 as ::core::ffi::c_char,
    ASCII_MINUS as ::core::ffi::c_char,
    ASCII_1_1 as ::core::ffi::c_char,
    ASCII_6 as ::core::ffi::c_char,
    ASCII_L_1 as ::core::ffi::c_char,
    ASCII_E_1 as ::core::ffi::c_char,
    
    '\0' as ::core::ffi::c_char,
];

unsafe extern "C" fn getEncodingIndex(mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    static mut encodingNames: [*const ::core::ffi::c_char; 6] = unsafe {
        [
            &raw const KW_ISO_8859_1 as *const ::core::ffi::c_char,
            &raw const KW_US_ASCII as *const ::core::ffi::c_char,
            &raw const KW_UTF_8 as *const ::core::ffi::c_char,
            &raw const KW_UTF_16 as *const ::core::ffi::c_char,
            &raw const KW_UTF_16BE as *const ::core::ffi::c_char,
            &raw const KW_UTF_16LE as *const ::core::ffi::c_char,
        ]
    };
    let mut i: ::core::ffi::c_int = 0;
    if name.is_null() {
        return  NO_ENC;
    }
    i = 0i32;
    while i
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 6]>())
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
            as ::core::ffi::c_int
    {
        if streqci(name, encodingNames[i as usize]) != 0 {
            return i;
        }
        i += 1;
    }
    return  UNKNOWN_ENC;
}

unsafe extern "C" fn initScan(
    mut encodingTable: *const *const crate::src::lib::xmltok::ENCODING,
    mut enc: *const crate::src::lib::xmltok::INIT_ENCODING,
    mut state: ::core::ffi::c_int,
    mut ptr: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut nextTokPtr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut encPtr: *mut *const crate::src::lib::xmltok::ENCODING =
        ::core::ptr::null_mut::<*const crate::src::lib::xmltok::ENCODING>();
    if ptr >= end {
        return crate::src::lib::xmltok::XML_TOK_NONE_1;
    }
    encPtr = (*enc).encPtr;
    if ptr.offset(1isize) == end {
        match (*enc).initEnc.isUtf16 as ::core::ffi::c_int {
            3 | 5 | 4 => return crate::src::lib::xmltok::XML_TOK_PARTIAL_1,
            _ => {}
        }
        let mut c2rust_current_block_5: u64;
        match *ptr as ::core::ffi::c_uchar as ::core::ffi::c_int {
            254 | 255 | 239 => {
                if (*enc).initEnc.isUtf16 as ::core::ffi::c_int
                    ==  ISO_8859_1_ENC
                    && state == crate::src::lib::xmltok::XML_CONTENT_STATE
                {
                    c2rust_current_block_5 = 13183875560443969876;
                } else {
                    c2rust_current_block_5 = 6556540211831925522;
                }
            }
            0 | 60 => {
                c2rust_current_block_5 = 6556540211831925522;
            }
            _ => {
                c2rust_current_block_5 = 13183875560443969876;
            }
        }
        match c2rust_current_block_5 {
            13183875560443969876 => {}
            _ => return crate::src::lib::xmltok::XML_TOK_PARTIAL_1,
        }
    } else {
        let mut c2rust_current_block_26: u64;
        match (*ptr.offset(0isize) as ::core::ffi::c_uchar
            as ::core::ffi::c_int)
            << 8i32
            | *ptr.offset(1isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int
        {
            65279 => {
                if !((*enc).initEnc.isUtf16 as ::core::ffi::c_int
                    ==  ISO_8859_1_ENC
                    && state == crate::src::lib::xmltok::XML_CONTENT_STATE)
                {
                    *nextTokPtr = ptr.offset(2isize);
                    *encPtr = *encodingTable.offset(UTF_16BE_ENC as isize);
                    return crate::src::lib::xmltok::XML_TOK_BOM_1;
                }
            }
            15360 => {
                if !(((*enc).initEnc.isUtf16 as ::core::ffi::c_int
                    ==  UTF_16BE_ENC
                    || (*enc).initEnc.isUtf16 as ::core::ffi::c_int
                        ==  UTF_16_ENC)
                    && state == crate::src::lib::xmltok::XML_CONTENT_STATE)
                {
                    *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                    return (**encPtr).scanners[state as usize].expect("non-null function pointer")(
                        *encPtr, ptr, end, nextTokPtr,
                    );
                }
            }
            65534 => {
                if !((*enc).initEnc.isUtf16 as ::core::ffi::c_int
                    ==  ISO_8859_1_ENC
                    && state == crate::src::lib::xmltok::XML_CONTENT_STATE)
                {
                    *nextTokPtr = ptr.offset(2isize);
                    *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                    return crate::src::lib::xmltok::XML_TOK_BOM_1;
                }
            }
            61371 => {
                if state == crate::src::lib::xmltok::XML_CONTENT_STATE {
                    let mut e: ::core::ffi::c_int = (*enc).initEnc.isUtf16 as ::core::ffi::c_int;
                    if e ==  ISO_8859_1_ENC
                        || e ==  UTF_16BE_ENC
                        || e ==  UTF_16LE_ENC
                        || e ==  UTF_16_ENC
                    {
                        c2rust_current_block_26 = 2604890879466389055;
                    } else {
                        c2rust_current_block_26 = 11307063007268554308;
                    }
                } else {
                    c2rust_current_block_26 = 11307063007268554308;
                }
                match c2rust_current_block_26 {
                    2604890879466389055 => {}
                    _ => {
                        if ptr.offset(2isize) == end {
                            return crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(2isize) as ::core::ffi::c_uchar
                            as ::core::ffi::c_int
                            == 0xbfi32
                        {
                            *nextTokPtr = ptr.offset(3isize);
                            *encPtr =
                                *encodingTable.offset(UTF_8_ENC as isize);
                            return crate::src::lib::xmltok::XML_TOK_BOM_1;
                        }
                    }
                }
            }
            _ => {
                if *ptr.offset(0isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    if !(state == crate::src::lib::xmltok::XML_CONTENT_STATE
                        && (*enc).initEnc.isUtf16 as ::core::ffi::c_int
                            ==  UTF_16LE_ENC)
                    {
                        *encPtr =
                            *encodingTable.offset(UTF_16BE_ENC as isize);
                        return (**encPtr).scanners[state as usize]
                            .expect("non-null function pointer")(
                            *encPtr, ptr, end, nextTokPtr
                        );
                    }
                } else if *ptr.offset(1isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    if !(state == crate::src::lib::xmltok::XML_CONTENT_STATE) {
                        *encPtr =
                            *encodingTable.offset(UTF_16LE_ENC as isize);
                        return (**encPtr).scanners[state as usize]
                            .expect("non-null function pointer")(
                            *encPtr, ptr, end, nextTokPtr
                        );
                    }
                }
            }
        }
    }
    *encPtr = *encodingTable.offset((*enc).initEnc.isUtf16 as ::core::ffi::c_int as isize);
    return (**encPtr).scanners[state as usize].expect("non-null function pointer")(
        *encPtr, ptr, end, nextTokPtr,
    );
}
#[no_mangle]

pub unsafe extern "C" fn XmlInitUnknownEncodingNS(
    mut mem: *mut ::core::ffi::c_void,
    mut table: *const ::core::ffi::c_int,
    mut convert: crate::src::lib::xmltok::CONVERTER,
    mut userData: *mut ::core::ffi::c_void,
) -> *mut crate::src::lib::xmltok::ENCODING {
    let mut enc: *mut crate::src::lib::xmltok::ENCODING =
        XmlInitUnknownEncoding(mem, table, convert, userData);
    if !enc.is_null() {
        (*(enc as *mut normal_encoding)).type_0[ASCII_COLON as usize] =
            
            BT_COLON_0 as ::core::ffi::c_uchar;
    }
    return enc;
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    encodings = [
        &raw const latin1_encoding.enc,
        &raw const ascii_encoding.enc,
        &raw const utf8_encoding.enc,
        &raw const big2_encoding.enc,
        &raw const big2_encoding.enc,
        &raw const little2_encoding.enc,
        &raw const utf8_encoding.enc,
    ];
    encodingsNS = [
        &raw const latin1_encoding_ns.enc,
        &raw const ascii_encoding_ns.enc,
        &raw const utf8_encoding_ns.enc,
        &raw const big2_encoding_ns.enc,
        &raw const big2_encoding_ns.enc,
        &raw const little2_encoding_ns.enc,
        &raw const utf8_encoding_ns.enc,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
