// =============== BEGIN xmltok_h ================
pub const XML_TOK_TRAILING_RSQB: c_int = -5;

pub const XML_TOK_TRAILING_RSQB_1: c_int = -(5);

pub const XML_TOK_NONE: c_int = -4;

pub const XML_TOK_NONE_1: c_int = -(4);

pub const XML_TOK_TRAILING_CR: c_int = -3;

pub const XML_TOK_TRAILING_CR_1: c_int = -(3);

pub const XML_TOK_PARTIAL_CHAR: c_int = -2;

pub const XML_TOK_PARTIAL_CHAR_1: c_int = -(2);

pub const XML_TOK_PARTIAL: c_int = -1;

pub const XML_TOK_PARTIAL_1: c_int = -(1);

pub const XML_TOK_INVALID: c_int = 0;

pub const XML_TOK_INVALID_1: c_int = 0;

pub const XML_TOK_START_TAG_WITH_ATTS: c_int = 1;

pub const XML_TOK_START_TAG_WITH_ATTS_1: c_int = 1;

pub const XML_TOK_START_TAG_NO_ATTS: c_int = 2;

pub const XML_TOK_START_TAG_NO_ATTS_1: c_int = 2;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS: c_int = 3;

pub const XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1: c_int = 3;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS: c_int = 4;

pub const XML_TOK_EMPTY_ELEMENT_NO_ATTS_1: c_int = 4;

pub const XML_TOK_END_TAG: c_int = 5;

pub const XML_TOK_END_TAG_1: c_int = 5;

pub const XML_TOK_DATA_CHARS: c_int = 6;

pub const XML_TOK_DATA_CHARS_1: c_int = 6;

pub const XML_TOK_DATA_NEWLINE: c_int = 7;

pub const XML_TOK_DATA_NEWLINE_1: c_int = 7;

pub const XML_TOK_CDATA_SECT_OPEN: c_int = 8;

pub const XML_TOK_CDATA_SECT_OPEN_1: c_int = 8;

pub const XML_TOK_ENTITY_REF: c_int = 9;

pub const XML_TOK_ENTITY_REF_1: c_int = 9;

pub const XML_TOK_CHAR_REF: c_int = 10;

pub const XML_TOK_CHAR_REF_1: c_int = 10;

pub const XML_TOK_PI: c_int = 11;

pub const XML_TOK_PI_1: c_int = 11;

pub const XML_TOK_XML_DECL: c_int = 12;

pub const XML_TOK_XML_DECL_1: c_int = 12;

pub const XML_TOK_COMMENT: c_int = 13;

pub const XML_TOK_COMMENT_1: c_int = 13;

pub const XML_TOK_BOM: c_int = 14;

pub const XML_TOK_BOM_1: c_int = 14;

pub const XML_TOK_PROLOG_S: c_int = 15;

pub const XML_TOK_PROLOG_S_1: c_int = 15;

pub const XML_TOK_DECL_OPEN: c_int = 16;

pub const XML_TOK_DECL_OPEN_1: c_int = 16;

pub const XML_TOK_DECL_CLOSE: c_int = 17;

pub const XML_TOK_DECL_CLOSE_1: c_int = 17;

pub const XML_TOK_NAME: c_int = 18;

pub const XML_TOK_NMTOKEN: c_int = 19;

pub const XML_TOK_NMTOKEN_1: c_int = 19;

pub const XML_TOK_POUND_NAME: c_int = 20;

pub const XML_TOK_POUND_NAME_1: c_int = 20;

pub const XML_TOK_OR: c_int = 21;

pub const XML_TOK_OR_1: c_int = 21;

pub const XML_TOK_PERCENT: c_int = 22;

pub const XML_TOK_PERCENT_1: c_int = 22;

pub const XML_TOK_OPEN_PAREN: c_int = 23;

pub const XML_TOK_OPEN_PAREN_1: c_int = 23;

pub const XML_TOK_CLOSE_PAREN: c_int = 24;

pub const XML_TOK_CLOSE_PAREN_1: c_int = 24;

pub const XML_TOK_OPEN_BRACKET: c_int = 25;

pub const XML_TOK_OPEN_BRACKET_1: c_int = 25;

pub const XML_TOK_CLOSE_BRACKET: c_int = 26;

pub const XML_TOK_CLOSE_BRACKET_1: c_int = 26;

pub const XML_TOK_LITERAL: c_int = 27;

pub const XML_TOK_LITERAL_1: c_int = 27;

pub const XML_TOK_PARAM_ENTITY_REF: c_int = 28;

pub const XML_TOK_PARAM_ENTITY_REF_1: c_int = 28;

pub const XML_TOK_INSTANCE_START: c_int = 29;

pub const XML_TOK_INSTANCE_START_1: c_int = 29;

pub const XML_TOK_NAME_QUESTION: c_int = 30;

pub const XML_TOK_NAME_QUESTION_1: c_int = 30;

pub const XML_TOK_NAME_ASTERISK: c_int = 31;

pub const XML_TOK_NAME_ASTERISK_1: c_int = 31;

pub const XML_TOK_NAME_PLUS: c_int = 32;

pub const XML_TOK_NAME_PLUS_1: c_int = 32;

pub const XML_TOK_COND_SECT_OPEN: c_int = 33;

pub const XML_TOK_COND_SECT_OPEN_1: c_int = 33;

pub const XML_TOK_COND_SECT_CLOSE: c_int = 34;

pub const XML_TOK_COND_SECT_CLOSE_1: c_int = 34;

pub const XML_TOK_CLOSE_PAREN_QUESTION: c_int = 35;

pub const XML_TOK_CLOSE_PAREN_QUESTION_1: c_int = 35;

pub const XML_TOK_CLOSE_PAREN_ASTERISK: c_int = 36;

pub const XML_TOK_CLOSE_PAREN_ASTERISK_1: c_int = 36;

pub const XML_TOK_CLOSE_PAREN_PLUS: c_int = 37;

pub const XML_TOK_CLOSE_PAREN_PLUS_1: c_int = 37;

pub const XML_TOK_COMMA: c_int = 38;

pub const XML_TOK_COMMA_1: c_int = 38;

pub const XML_TOK_ATTRIBUTE_VALUE_S: c_int = 39;

pub const XML_TOK_ATTRIBUTE_VALUE_S_1: c_int = 39;

pub const XML_TOK_CDATA_SECT_CLOSE: c_int = 40;

pub const XML_TOK_CDATA_SECT_CLOSE_1: c_int = 40;

pub const XML_TOK_PREFIXED_NAME: c_int = 41;

pub const XML_TOK_IGNORE_SECT: c_int = 42;

pub const XML_TOK_IGNORE_SECT_1: c_int = 42;

pub const XML_PROLOG_STATE: c_int = 0;

pub const XML_CONTENT_STATE: c_int = 1;

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
    pub name: *const c_char,
    pub valuePtr: *const c_char,
    pub valueEnd: *const c_char,
    pub normalized: c_char,
}

pub type ENCODING = crate::src::lib::xmltok::encoding;

pub type ScannerResult = (c_int, *const c_char);
pub type SCANNER = fn(&ENCODING, &[c_char]) -> ScannerResult;

pub type XML_Convert_Result = c_uint;

pub const XML_CONVERT_COMPLETED: XML_Convert_Result = 0;

pub const XML_CONVERT_INPUT_INCOMPLETE: XML_Convert_Result = 1;

pub const XML_CONVERT_OUTPUT_EXHAUSTED: XML_Convert_Result = 2;

pub(crate) type XmlInitEncodingResult = (c_int, *const ENCODING);
pub(crate) type ParsePseudoAttributeResult = (
    c_int,
    *const c_char,
    *const c_char,
    *const c_char,
    *const c_char,
);
pub(crate) type ParseXmlDeclResult = (
    c_int,
    *const c_char,
    *const c_char,
    *const c_char,
    *const c_char,
    *const ENCODING,
    c_int,
);
pub(crate) type IsPublicIdResult = (c_int, *const c_char);
pub(crate) type Utf8ConvertResult = (XML_Convert_Result, *const c_char, *mut c_char);
pub(crate) type Utf16ConvertResult = (XML_Convert_Result, *const c_char, *mut c_ushort);
pub(crate) type CheckPiTargetResult = (c_int, c_int);

trait EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int;
    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char;
    fn getAtts(
        &self,
        enc: &ENCODING,
        ptr: *const c_char,
        n: c_int,
        atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> c_int;
    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int;
    fn updatePosition(
        &self,
        enc: &ENCODING,
        input: &[c_char],
        pos: *mut crate::src::lib::xmltok::POSITION,
    );
    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult;
    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult;
    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult;
}

#[derive(Copy, Clone)]
#[repr(C)]

pub struct encoding {
    pub scanners: [crate::src::lib::xmltok::SCANNER; 4],
    pub literalScanners: [crate::src::lib::xmltok::SCANNER; 2],
    functions: &'static (dyn EncodingFunctions + Sync),
    pub minBytesPerChar: c_int,
    pub isUtf8: c_char,
    pub isUtf16: c_char,
}

impl encoding {
    #[inline]
    fn functions(&self) -> &(dyn EncodingFunctions + Sync) {
        self.functions
    }

    pub(crate) fn nameMatchesAscii(
        &self,
        enc: &ENCODING,
        input: &[c_char],
        kw: *const c_char,
    ) -> c_int {
        self.functions().nameMatchesAscii(enc, input, kw)
    }

    pub(crate) fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.functions().nameLength(enc, ptr)
    }

    pub(crate) fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        self.functions().skipS(enc, ptr)
    }

    pub(crate) fn getAtts(
        &self,
        enc: &ENCODING,
        ptr: *const c_char,
        n: c_int,
        atts: *mut crate::src::lib::xmltok::ATTRIBUTE,
    ) -> c_int {
        self.functions().getAtts(enc, ptr, n, atts)
    }

    pub(crate) fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.functions().charRefNumber(enc, ptr)
    }

    pub(crate) fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        self.functions().predefinedEntityName(enc, input)
    }

    pub(crate) fn updatePosition(
        &self,
        enc: &ENCODING,
        input: &[c_char],
        pos: *mut crate::src::lib::xmltok::POSITION,
    ) {
        self.functions().updatePosition(enc, input, pos)
    }

    pub(crate) fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        self.functions().isPublicId(enc, input)
    }

    pub(crate) fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        self.functions()
            .utf8Convert(enc, from_p, from_lim, to_p, to_lim)
    }

    pub(crate) fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        self.functions()
            .utf16Convert(enc, from_p, from_lim, to_p, to_lim)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct INIT_ENCODING {
    pub initEnc: ENCODING,
    pub encPtr: *mut *const ENCODING,
}

pub type CONVERTER = Option<extern "C" fn(*mut c_void, *const c_char) -> c_int>;

#[inline]
fn as_normal_encoding(enc: &ENCODING) -> &normal_encoding {
    unsafe { &*(enc as *const ENCODING as *const normal_encoding) }
}

#[inline]
fn as_unknown_encoding(enc: &ENCODING) -> &unknown_encoding {
    unsafe { &*(enc as *const ENCODING as *const unknown_encoding) }
}

#[inline]
fn as_init_encoding(enc: &ENCODING) -> &INIT_ENCODING {
    unsafe { &*(enc as *const ENCODING as *const INIT_ENCODING) }
}

#[inline]
fn c_char_slice_from_ptr_end<'a>(ptr: *const c_char, end: *const c_char) -> &'a [c_char] {
    unsafe { core::slice::from_raw_parts(ptr, end.offset_from(ptr) as usize) }
}

pub mod xmltok_impl_c {
    use super::*;
    use crate::src::lib::xmltok::XML_TOK_COMMENT_1;
    use crate::src::lib::xmltok::XML_TOK_COND_SECT_OPEN_1;
    use crate::src::lib::xmltok::XML_TOK_DECL_OPEN_1;
    use crate::src::lib::xmltok::XML_TOK_INVALID_1;
    use crate::src::lib::xmltok::XML_TOK_PARTIAL_1;
    use crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR_1;
    use core::ffi::c_char;
    use core::ffi::c_int;
    use core::ffi::c_long;
    use core::ffi::c_uchar;

    pub(crate) fn normal_scanComment(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_1: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    if !(*ptr as c_int == 0x2d) {
                        *nextTokPtr = ptr;
                        break 'iife_ret_1 XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(1);
                    while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                        match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                            BT_LEAD2 => {
                                if (end.offset_from(ptr) as c_long) < 2 {
                                    break 'iife_ret_1 XML_TOK_PARTIAL_CHAR_1;
                                }
                                if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0 {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_1 XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            BT_LEAD3 => {
                                if (end.offset_from(ptr) as c_long) < 3 {
                                    break 'iife_ret_1 XML_TOK_PARTIAL_CHAR_1;
                                }
                                if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0 {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_1 XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            BT_LEAD4 => {
                                if (end.offset_from(ptr) as c_long) < 4 {
                                    break 'iife_ret_1 XML_TOK_PARTIAL_CHAR_1;
                                }
                                if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0 {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_1 XML_TOK_INVALID_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            BT_NONXML | BT_MALFORM | BT_TRAIL => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_1 XML_TOK_INVALID_1;
                            }
                            BT_MINUS => {
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    break 'iife_ret_1 XML_TOK_PARTIAL_1;
                                }
                                if *ptr as c_int == 0x2d {
                                    ptr = ptr.offset(1);
                                    if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                        break 'iife_ret_1 XML_TOK_PARTIAL_1;
                                    }
                                    if !(*ptr as c_int == 0x3e) {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_1 XML_TOK_INVALID_1;
                                    }
                                    *nextTokPtr = ptr.offset(1);
                                    break 'iife_ret_1 XML_TOK_COMMENT_1;
                                }
                            }
                            _ => {
                                ptr = ptr.offset(1isize);
                            }
                        }
                    }
                }
                break 'iife_ret_1 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanDecl(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_2: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                    break 'iife_ret_2 XML_TOK_PARTIAL_1;
                }
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_MINUS => {
                        break 'iife_ret_2 ({
                            let (tok_value, next_tok_value) = normal_scanComment(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_LSQB => {
                        *nextTokPtr = ptr.offset(1);
                        break 'iife_ret_2 XML_TOK_COND_SECT_OPEN_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        ptr = ptr.offset(1isize);
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_2 XML_TOK_INVALID_1;
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    's_129: {
                        match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                            BT_PERCNT => {
                                if !(end.offset_from(ptr) as c_long >= (2i32 * 1) as c_long) {
                                    break 'iife_ret_2 XML_TOK_PARTIAL_1;
                                }
                                match as_normal_encoding(enc).type_0
                                    [*ptr.offset(1) as c_uchar as usize]
                                    as c_uint
                                {
                                    BT_S | BT_CR | BT_LF | BT_PERCNT => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_2 XML_TOK_INVALID_1;
                                    }
                                    _ => {}
                                }
                            }
                            BT_S | BT_CR | BT_LF => {}
                            BT_NMSTRT | BT_HEX => {
                                ptr = ptr.offset(1);
                                break 's_129;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_2 XML_TOK_INVALID_1;
                            }
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_2 XML_TOK_DECL_OPEN_1;
                    }
                }
                break 'iife_ret_2 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_checkPiTarget(_enc: &ENCODING, input: &[c_char]) -> CheckPiTargetResult {
        unsafe {
            let mut tok: c_int = 0;
            let tokPtr = &mut tok;
            let result: c_int = 'iife_ret_3: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut upper: c_int = 0;
                *tokPtr = XML_TOK_PI_1;
                if end.offset_from(ptr) as c_long != (1i32 * 3) as c_long {
                    break 'iife_ret_3 1i32;
                }
                match *ptr as c_int {
                    ASCII_x_1 => {}
                    ASCII_X_1 => {
                        upper = 1i32;
                    }
                    _ => break 'iife_ret_3 1,
                }
                ptr = ptr.offset(1);
                match *ptr as c_int {
                    ASCII_m_1 => {}
                    ASCII_M_1 => {
                        upper = 1i32;
                    }
                    _ => break 'iife_ret_3 1,
                }
                ptr = ptr.offset(1);
                match *ptr as c_int {
                    ASCII_l_1 => {}
                    ASCII_L_1 => {
                        upper = 1i32;
                    }
                    _ => break 'iife_ret_3 1,
                }
                if upper != 0 {
                    break 'iife_ret_3 0i32;
                }
                *tokPtr = XML_TOK_XML_DECL_1;
                break 'iife_ret_3 1;
            };
            return (result, tok);
        }
    }

    pub(crate) fn normal_scanPi(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_4: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut tok: c_int = 0;
                let mut target: *const c_char = ptr;
                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                    break 'iife_ret_4 XML_TOK_PARTIAL_1;
                }
                let mut current_block_32: u64;
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_4 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_32 = 11470911313929454839;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt2(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2);
                        current_block_32 = 14763689060501151050;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt3(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3);
                        current_block_32 = 14763689060501151050;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt4(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4);
                        current_block_32 = 14763689060501151050;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_4 XML_TOK_INVALID_1;
                    }
                }
                match current_block_32 {
                    11470911313929454839 => {
                        ptr = ptr.offset(1isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut current_block_118: u64;
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_NONASCII => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_118 = 8485341570193076947;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                || as_normal_encoding(enc).isName2(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_4 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_118 = 13349765058737954042;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                || as_normal_encoding(enc).isName3(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_4 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_118 = 13349765058737954042;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                || as_normal_encoding(enc).isName4(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_4 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_118 = 13349765058737954042;
                        }
                        BT_S | BT_CR | BT_LF => {
                            if {
                                let (ok_value, tok_value) = normal_checkPiTarget(
                                    enc,
                                    c_char_slice_from_ptr_end(target, ptr),
                                );
                                tok = tok_value;
                                ok_value
                            } == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_4 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(1);
                            while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                    as c_uint
                                {
                                    BT_LEAD2 => {
                                        if (end.offset_from(ptr) as c_long) < 2 {
                                            break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0 {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_4 XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2isize);
                                    }
                                    BT_LEAD3 => {
                                        if (end.offset_from(ptr) as c_long) < 3 {
                                            break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0 {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_4 XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3isize);
                                    }
                                    BT_LEAD4 => {
                                        if (end.offset_from(ptr) as c_long) < 4 {
                                            break 'iife_ret_4 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0 {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_4 XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4isize);
                                    }
                                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_4 XML_TOK_INVALID_1;
                                    }
                                    BT_QUEST => {
                                        ptr = ptr.offset(1);
                                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long)
                                        {
                                            break 'iife_ret_4 XML_TOK_PARTIAL_1;
                                        }
                                        if *ptr as c_int == 0x3e {
                                            *nextTokPtr = ptr.offset(1);
                                            break 'iife_ret_4 tok;
                                        }
                                    }
                                    _ => {
                                        ptr = ptr.offset(1isize);
                                    }
                                }
                            }
                            break 'iife_ret_4 XML_TOK_PARTIAL_1;
                        }
                        BT_QUEST => {
                            if {
                                let (ok_value, tok_value) = normal_checkPiTarget(
                                    enc,
                                    c_char_slice_from_ptr_end(target, ptr),
                                );
                                tok = tok_value;
                                ok_value
                            } == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_4 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break 'iife_ret_4 XML_TOK_PARTIAL_1;
                            }
                            if *ptr as c_int == 0x3e {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_4 tok;
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
                            break 'iife_ret_4 XML_TOK_INVALID_1;
                        }
                        8485341570193076947 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_4 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanCdataSection(_enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_5: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                pub static CDATA_LSQB: [c_char; 6] = [
                    ASCII_C as c_char,
                    ASCII_D as c_char,
                    ASCII_A as c_char,
                    ASCII_T as c_char,
                    ASCII_A as c_char,
                    ASCII_LSQB as c_char,
                ];
                let mut i: c_int = 0;
                if !(end.offset_from(ptr) as c_long >= (6i32 * 1) as c_long) {
                    break 'iife_ret_5 XML_TOK_PARTIAL_1;
                }
                i = 0;
                while i < 6 {
                    if !(*ptr as c_int == CDATA_LSQB[i as usize] as c_int) {
                        *nextTokPtr = ptr;
                        break 'iife_ret_5 XML_TOK_INVALID_1;
                    }
                    i += 1;
                    ptr = ptr.offset(1);
                }
                *nextTokPtr = ptr;
                break 'iife_ret_5 XML_TOK_CDATA_SECT_OPEN_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_cdataSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_6: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if ptr >= end {
                    break 'iife_ret_6 XML_TOK_NONE_1;
                }
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_RSQB => {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                            break 'iife_ret_6 XML_TOK_PARTIAL_1;
                        }
                        if *ptr as c_int == 0x5d {
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break 'iife_ret_6 XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr as c_int == 0x3e) {
                                ptr = ptr.offset(-(1isize));
                            } else {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_6 XML_TOK_CDATA_SECT_CLOSE_1;
                            }
                        }
                    }
                    BT_CR => {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                            break 'iife_ret_6 XML_TOK_PARTIAL_1;
                        }
                        if as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int
                            == BT_LF as c_int
                        {
                            ptr = ptr.offset(1isize);
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_6 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_LF => {
                        *nextTokPtr = ptr.offset(1);
                        break 'iife_ret_6 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_6 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0 {
                            *nextTokPtr = ptr;
                            break 'iife_ret_6 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_6 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0 {
                            *nextTokPtr = ptr;
                            break 'iife_ret_6 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_6 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0 {
                            *nextTokPtr = ptr;
                            break 'iife_ret_6 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_6 XML_TOK_INVALID_1;
                    }
                    _ => {
                        ptr = ptr.offset(1isize);
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2
                                || as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3
                                || as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4
                                || as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(4isize);
                        }
                        BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF | BT_RSQB => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
                        }
                        _ => {
                            ptr = ptr.offset(1isize);
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_6 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanEndTag(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_7: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                    break 'iife_ret_7 XML_TOK_PARTIAL_1;
                }
                let mut current_block_32: u64;
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_7 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_32 = 4324628675098861213;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt2(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_7 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2);
                        current_block_32 = 7056779235015430508;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt3(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_7 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3);
                        current_block_32 = 7056779235015430508;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt4(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_7 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4);
                        current_block_32 = 7056779235015430508;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_7 XML_TOK_INVALID_1;
                    }
                }
                match current_block_32 {
                    4324628675098861213 => {
                        ptr = ptr.offset(1isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut current_block_73: u64;
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_NONASCII => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_7 XML_TOK_INVALID_1;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_73 = 14883924698754021420;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                || as_normal_encoding(enc).isName2(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_7 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_73 = 981995395831942902;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                || as_normal_encoding(enc).isName3(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_7 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_73 = 981995395831942902;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_7 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                || as_normal_encoding(enc).isName4(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_7 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_73 = 981995395831942902;
                        }
                        BT_S | BT_CR | BT_LF => {
                            ptr = ptr.offset(1);
                            while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                    as c_uint
                                {
                                    BT_S | BT_CR | BT_LF => {}
                                    BT_GT => {
                                        *nextTokPtr = ptr.offset(1);
                                        break 'iife_ret_7 XML_TOK_END_TAG_1;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_7 XML_TOK_INVALID_1;
                                    }
                                }
                                ptr = ptr.offset(1);
                            }
                            break 'iife_ret_7 XML_TOK_PARTIAL_1;
                        }
                        BT_COLON_0 => {
                            ptr = ptr.offset(1);
                            current_block_73 = 981995395831942902;
                        }
                        BT_GT => {
                            *nextTokPtr = ptr.offset(1);
                            break 'iife_ret_7 XML_TOK_END_TAG_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_7 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_73 {
                        14883924698754021420 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_7 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanHexCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_8: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_DIGIT | BT_HEX => {}
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_8 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.offset(1);
                    while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                        match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                            BT_DIGIT | BT_HEX => {}
                            BT_SEMI => {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_8 XML_TOK_CHAR_REF_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_8 XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(1);
                    }
                }
                break 'iife_ret_8 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_9: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    if *ptr as c_int == 0x78 {
                        break 'iife_ret_9 ({
                            let (tok_value, next_tok_value) = normal_scanHexCharRef(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_DIGIT => {}
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_9 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.offset(1);
                    while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                        match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                            BT_DIGIT => {}
                            BT_SEMI => {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_9 XML_TOK_CHAR_REF_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_9 XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(1);
                    }
                }
                break 'iife_ret_9 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_10: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                    break 'iife_ret_10 XML_TOK_PARTIAL_1;
                }
                let mut current_block_33: u64;
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_10 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_33 = 8911980980495988282;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt2(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_10 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2);
                        current_block_33 = 14763689060501151050;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt3(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_10 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3);
                        current_block_33 = 14763689060501151050;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt4(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_10 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4);
                        current_block_33 = 14763689060501151050;
                    }
                    BT_NUM => {
                        break 'iife_ret_10 ({
                            let (tok_value, next_tok_value) = normal_scanCharRef(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_10 XML_TOK_INVALID_1;
                    }
                }
                match current_block_33 {
                    8911980980495988282 => {
                        ptr = ptr.offset(1isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut current_block_64: u64;
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_NONASCII => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_10 XML_TOK_INVALID_1;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_64 = 11948064939145634034;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                || as_normal_encoding(enc).isName2(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_10 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_64 = 10930818133215224067;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                || as_normal_encoding(enc).isName3(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_10 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_64 = 10930818133215224067;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_10 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                || as_normal_encoding(enc).isName4(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_10 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_64 = 10930818133215224067;
                        }
                        BT_SEMI => {
                            *nextTokPtr = ptr.offset(1);
                            break 'iife_ret_10 XML_TOK_ENTITY_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_10 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_64 {
                        11948064939145634034 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_10 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanAtts(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_11: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut hadColon: c_int = 0;
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut current_block_186: u64;
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_NONASCII => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_11 XML_TOK_INVALID_1;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_186 = 3818392175876617014;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                || as_normal_encoding(enc).isName2(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_11 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_186 = 1634947208139838470;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                || as_normal_encoding(enc).isName3(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_11 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_186 = 1634947208139838470;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                || as_normal_encoding(enc).isName4(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_11 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_186 = 1634947208139838470;
                        }
                        BT_COLON_0 => {
                            if hadColon != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_11 XML_TOK_INVALID_1;
                            }
                            hadColon = 1;
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break 'iife_ret_11 XML_TOK_PARTIAL_1;
                            }
                            let mut current_block_64: u64;
                            match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                            {
                                BT_NONASCII => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                }
                                BT_NMSTRT | BT_HEX => {
                                    current_block_64 = 7083593080606520045;
                                }
                                BT_LEAD2 => {
                                    if (end.offset_from(ptr) as c_long) < 2 {
                                        break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                        || as_normal_encoding(enc).isNmstrt2(enc, ptr) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(2);
                                    current_block_64 = 10930818133215224067;
                                }
                                BT_LEAD3 => {
                                    if (end.offset_from(ptr) as c_long) < 3 {
                                        break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                        || as_normal_encoding(enc).isNmstrt3(enc, ptr) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(3);
                                    current_block_64 = 10930818133215224067;
                                }
                                BT_LEAD4 => {
                                    if (end.offset_from(ptr) as c_long) < 4 {
                                        break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                        || as_normal_encoding(enc).isNmstrt4(enc, ptr) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(4);
                                    current_block_64 = 10930818133215224067;
                                }
                                _ => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
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
                        BT_S | BT_CR | BT_LF => {
                            loop {
                                let mut t: c_int = 0;
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    break 'iife_ret_11 XML_TOK_PARTIAL_1;
                                }
                                t = as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                    as c_int;
                                if t == BT_EQUALS as c_int {
                                    break;
                                }
                                match t {
                                    21 | 10 | 9 => {}
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            current_block_186 = 10853015579903106591;
                        }
                        BT_EQUALS => {
                            current_block_186 = 10853015579903106591;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_11 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_186 {
                        10853015579903106591 => {
                            let mut open: c_int = 0;
                            hadColon = 0;
                            loop {
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    break 'iife_ret_11 XML_TOK_PARTIAL_1;
                                }
                                open = as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                    as c_int;
                                if open == BT_QUOT as c_int || open == BT_APOS as c_int {
                                    break;
                                }
                                match open {
                                    21 | 10 | 9 => {}
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            ptr = ptr.offset(1);
                            loop {
                                let mut t_0: c_int = 0;
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    break 'iife_ret_11 XML_TOK_PARTIAL_1;
                                }
                                t_0 = as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                    as c_int;
                                if t_0 == open {
                                    break;
                                }
                                match t_0 {
                                    5 => {
                                        if (end.offset_from(ptr) as c_long) < 2 {
                                            break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0 {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_11 XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2isize);
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as c_long) < 3 {
                                            break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0 {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_11 XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3isize);
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as c_long) < 4 {
                                            break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0 {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_11 XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4isize);
                                    }
                                    0 | 1 | 8 => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                    3 => {
                                        let mut tok: c_int = {
                                            let (tok_value, next_tok_value) = normal_scanRef(
                                                enc,
                                                c_char_slice_from_ptr_end(ptr.offset(1), end),
                                            );
                                            ptr = next_tok_value;
                                            tok_value
                                        };
                                        if tok <= 0 {
                                            if tok == XML_TOK_INVALID_1 {
                                                *nextTokPtr = ptr;
                                            }
                                            break 'iife_ret_11 tok;
                                        }
                                    }
                                    2 => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_11 XML_TOK_INVALID_1;
                                    }
                                    _ => {
                                        ptr = ptr.offset(1isize);
                                    }
                                }
                            }
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break 'iife_ret_11 XML_TOK_PARTIAL_1;
                            }
                            match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                            {
                                BT_S | BT_CR | BT_LF => {
                                    loop {
                                        ptr = ptr.offset(1);
                                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long)
                                        {
                                            break 'iife_ret_11 XML_TOK_PARTIAL_1;
                                        }
                                        match as_normal_encoding(enc).type_0
                                            [*ptr as c_uchar as usize]
                                            as c_uint
                                        {
                                            BT_NONASCII => {
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_11 XML_TOK_INVALID_1;
                                            }
                                            BT_NMSTRT | BT_HEX => {
                                                current_block_186 = 11210999262882855128;
                                                break;
                                            }
                                            BT_LEAD2 => {
                                                if (end.offset_from(ptr) as c_long) < 2 {
                                                    break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                                    || as_normal_encoding(enc).isNmstrt2(enc, ptr)
                                                        == 0
                                                {
                                                    *nextTokPtr = ptr;
                                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                                }
                                                ptr = ptr.offset(2);
                                                current_block_186 = 1634947208139838470;
                                                break;
                                            }
                                            BT_LEAD3 => {
                                                if (end.offset_from(ptr) as c_long) < 3 {
                                                    break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                                    || as_normal_encoding(enc).isNmstrt3(enc, ptr)
                                                        == 0
                                                {
                                                    *nextTokPtr = ptr;
                                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                                }
                                                ptr = ptr.offset(3);
                                                current_block_186 = 1634947208139838470;
                                                break;
                                            }
                                            BT_LEAD4 => {
                                                if (end.offset_from(ptr) as c_long) < 4 {
                                                    break 'iife_ret_11 XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                                    || as_normal_encoding(enc).isNmstrt4(enc, ptr)
                                                        == 0
                                                {
                                                    *nextTokPtr = ptr;
                                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                                }
                                                ptr = ptr.offset(4);
                                                current_block_186 = 1634947208139838470;
                                                break;
                                            }
                                            BT_S | BT_CR | BT_LF => {}
                                            BT_GT => {
                                                current_block_186 = 2944436519209994553;
                                                break;
                                            }
                                            BT_SOL => {
                                                current_block_186 = 398073151373002430;
                                                break;
                                            }
                                            _ => {
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_11 XML_TOK_INVALID_1;
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
                                BT_SOL => {
                                    current_block_186 = 398073151373002430;
                                }
                                BT_GT => {
                                    current_block_186 = 2944436519209994553;
                                }
                                _ => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_11 XML_TOK_INVALID_1;
                                }
                            }
                            match current_block_186 {
                                1634947208139838470 => {}
                                _ => match current_block_186 {
                                    398073151373002430 => {
                                        ptr = ptr.offset(1);
                                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long)
                                        {
                                            break 'iife_ret_11 XML_TOK_PARTIAL_1;
                                        }
                                        if !(*ptr as c_int == 0x3e) {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_11 XML_TOK_INVALID_1;
                                        }
                                        *nextTokPtr = ptr.offset(1);
                                        break 'iife_ret_11 XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr.offset(1);
                                        break 'iife_ret_11 XML_TOK_START_TAG_WITH_ATTS_1;
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
                break 'iife_ret_11 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanLt(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_12: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut hadColon: c_int = 0;
                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                    break 'iife_ret_12 XML_TOK_PARTIAL_1;
                }
                let mut current_block_45: u64;
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_12 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_45 = 2165477741955893522;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt2(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2);
                        current_block_45 = 8180496224585318153;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt3(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3);
                        current_block_45 = 8180496224585318153;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt4(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4);
                        current_block_45 = 8180496224585318153;
                    }
                    BT_EXCL => {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                            break 'iife_ret_12 XML_TOK_PARTIAL_1;
                        }
                        match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                            BT_MINUS => {
                                break 'iife_ret_12 ({
                                    let (tok_value, next_tok_value) = normal_scanComment(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            BT_LSQB => {
                                break 'iife_ret_12 ({
                                    let (tok_value, next_tok_value) = normal_scanCdataSection(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            _ => {}
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_12 XML_TOK_INVALID_1;
                    }
                    BT_QUEST => {
                        break 'iife_ret_12 ({
                            let (tok_value, next_tok_value) = normal_scanPi(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_SOL => {
                        break 'iife_ret_12 ({
                            let (tok_value, next_tok_value) = normal_scanEndTag(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_12 XML_TOK_INVALID_1;
                    }
                }
                match current_block_45 {
                    2165477741955893522 => {
                        ptr = ptr.offset(1isize);
                    }
                    _ => {}
                }
                hadColon = 0;
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut current_block_161: u64;
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_NONASCII => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_161 = 6701753098489376273;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                || as_normal_encoding(enc).isName2(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_12 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_161 = 14714495436747744489;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                || as_normal_encoding(enc).isName3(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_12 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_161 = 14714495436747744489;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                || as_normal_encoding(enc).isName4(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_12 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_161 = 14714495436747744489;
                        }
                        BT_COLON_0 => {
                            if hadColon != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_12 XML_TOK_INVALID_1;
                            }
                            hadColon = 1;
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break 'iife_ret_12 XML_TOK_PARTIAL_1;
                            }
                            let mut current_block_112: u64;
                            match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                            {
                                BT_NONASCII => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_12 XML_TOK_INVALID_1;
                                }
                                BT_NMSTRT | BT_HEX => {
                                    current_block_112 = 9169466483824547789;
                                }
                                BT_LEAD2 => {
                                    if (end.offset_from(ptr) as c_long) < 2 {
                                        break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                        || as_normal_encoding(enc).isNmstrt2(enc, ptr) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_12 XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(2);
                                    current_block_112 = 2616667235040759262;
                                }
                                BT_LEAD3 => {
                                    if (end.offset_from(ptr) as c_long) < 3 {
                                        break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                        || as_normal_encoding(enc).isNmstrt3(enc, ptr) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_12 XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(3);
                                    current_block_112 = 2616667235040759262;
                                }
                                BT_LEAD4 => {
                                    if (end.offset_from(ptr) as c_long) < 4 {
                                        break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                        || as_normal_encoding(enc).isNmstrt4(enc, ptr) == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_12 XML_TOK_INVALID_1;
                                    }
                                    ptr = ptr.offset(4);
                                    current_block_112 = 2616667235040759262;
                                }
                                _ => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_12 XML_TOK_INVALID_1;
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
                        BT_S | BT_CR | BT_LF => {
                            ptr = ptr.offset(1);
                            loop {
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    current_block_161 = 13215501469961642988;
                                    break;
                                }
                                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                    as c_uint
                                {
                                    BT_NONASCII => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_12 XML_TOK_INVALID_1;
                                    }
                                    BT_NMSTRT | BT_HEX => {
                                        current_block_161 = 7939927167482451446;
                                    }
                                    BT_LEAD2 => {
                                        if (end.offset_from(ptr) as c_long) < 2 {
                                            break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                            || as_normal_encoding(enc).isNmstrt2(enc, ptr) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_12 XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(2);
                                        current_block_161 = 16314074004867283505;
                                    }
                                    BT_LEAD3 => {
                                        if (end.offset_from(ptr) as c_long) < 3 {
                                            break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                            || as_normal_encoding(enc).isNmstrt3(enc, ptr) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_12 XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(3);
                                        current_block_161 = 16314074004867283505;
                                    }
                                    BT_LEAD4 => {
                                        if (end.offset_from(ptr) as c_long) < 4 {
                                            break 'iife_ret_12 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                            || as_normal_encoding(enc).isNmstrt4(enc, ptr) == 0
                                        {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_12 XML_TOK_INVALID_1;
                                        }
                                        ptr = ptr.offset(4);
                                        current_block_161 = 16314074004867283505;
                                    }
                                    BT_GT => {
                                        current_block_161 = 5640065479517572396;
                                        break;
                                    }
                                    BT_SOL => {
                                        current_block_161 = 12549409781983877175;
                                        break;
                                    }
                                    BT_S | BT_CR | BT_LF => {
                                        ptr = ptr.offset(1);
                                        continue;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_12 XML_TOK_INVALID_1;
                                    }
                                }
                                match current_block_161 {
                                    7939927167482451446 => {
                                        ptr = ptr.offset(1isize);
                                    }
                                    _ => {}
                                }
                                break 'iife_ret_12 ({
                                    let (tok_value, next_tok_value) =
                                        normal_scanAtts(enc, c_char_slice_from_ptr_end(ptr, end));
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            match current_block_161 {
                                5640065479517572396 => {}
                                12549409781983877175 => {}
                                _ => break 'iife_ret_12 XML_TOK_PARTIAL_1,
                            }
                        }
                        BT_GT => {
                            current_block_161 = 5640065479517572396;
                        }
                        BT_SOL => {
                            current_block_161 = 12549409781983877175;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_12 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_161 {
                        12549409781983877175 => {
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break 'iife_ret_12 XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr as c_int == 0x3e) {
                                *nextTokPtr = ptr;
                                break 'iife_ret_12 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(1);
                            break 'iife_ret_12 XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                        }
                        5640065479517572396 => {
                            *nextTokPtr = ptr.offset(1);
                            break 'iife_ret_12 XML_TOK_START_TAG_NO_ATTS_1;
                        }
                        6701753098489376273 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_12 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_contentTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_13: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if ptr >= end {
                    break 'iife_ret_13 XML_TOK_NONE_1;
                }
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_LT => {
                        break 'iife_ret_13 ({
                            let (tok_value, next_tok_value) = normal_scanLt(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_AMP => {
                        break 'iife_ret_13 ({
                            let (tok_value, next_tok_value) = normal_scanRef(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_CR => {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                            break 'iife_ret_13 XML_TOK_TRAILING_CR_1;
                        }
                        if as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int
                            == BT_LF as c_int
                        {
                            ptr = ptr.offset(1isize);
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_13 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_LF => {
                        *nextTokPtr = ptr.offset(1);
                        break 'iife_ret_13 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_RSQB => {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                            break 'iife_ret_13 XML_TOK_TRAILING_RSQB_1;
                        }
                        if *ptr as c_int == 0x5d {
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break 'iife_ret_13 XML_TOK_TRAILING_RSQB_1;
                            }
                            if !(*ptr as c_int == 0x3e) {
                                ptr = ptr.offset(-(1isize));
                            } else {
                                *nextTokPtr = ptr;
                                break 'iife_ret_13 XML_TOK_INVALID_1;
                            }
                        }
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_13 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0 {
                            *nextTokPtr = ptr;
                            break 'iife_ret_13 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_13 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0 {
                            *nextTokPtr = ptr;
                            break 'iife_ret_13 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_13 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0 {
                            *nextTokPtr = ptr;
                            break 'iife_ret_13 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_13 XML_TOK_INVALID_1;
                    }
                    _ => {
                        ptr = ptr.offset(1isize);
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut current_block_76: u64;
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2
                                || as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_76 = 7158658067966855297;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3
                                || as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_76 = 7158658067966855297;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4
                                || as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_76 = 7158658067966855297;
                        }
                        BT_RSQB => {
                            if end.offset_from(ptr) as c_long >= (2i32 * 1) as c_long {
                                if !(*ptr.offset(1) as c_int == 0x5d) {
                                    ptr = ptr.offset(1);
                                    current_block_76 = 7158658067966855297;
                                } else if end.offset_from(ptr) as c_long >= (3i32 * 1) as c_long {
                                    if !(*ptr.offset((2i32 * 1) as isize) as c_int == 0x3e) {
                                        ptr = ptr.offset(1isize);
                                    } else {
                                        *nextTokPtr = ptr.offset((2i32 * 1) as isize);
                                        break 'iife_ret_13 XML_TOK_INVALID_1;
                                    }
                                    current_block_76 = 7158658067966855297;
                                } else {
                                    current_block_76 = 1999360611754201214;
                                }
                            } else {
                                current_block_76 = 1999360611754201214;
                            }
                        }
                        BT_AMP | BT_LT | BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF => {
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
                            break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_13 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanPercent(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_14: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                    break 'iife_ret_14 XML_TOK_PARTIAL_1;
                }
                let mut current_block_34: u64;
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_14 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_34 = 12478441211659886388;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt2(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_14 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2);
                        current_block_34 = 4761528863920922185;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt3(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_14 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3);
                        current_block_34 = 4761528863920922185;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt4(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_14 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4);
                        current_block_34 = 4761528863920922185;
                    }
                    BT_S | BT_LF | BT_CR | BT_PERCNT => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_14 XML_TOK_PERCENT_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_14 XML_TOK_INVALID_1;
                    }
                }
                match current_block_34 {
                    12478441211659886388 => {
                        ptr = ptr.offset(1isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut current_block_65: u64;
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_NONASCII => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_14 XML_TOK_INVALID_1;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_65 = 7770117754142564343;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                || as_normal_encoding(enc).isName2(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_14 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_65 = 16415152177862271243;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                || as_normal_encoding(enc).isName3(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_14 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_65 = 16415152177862271243;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_14 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                || as_normal_encoding(enc).isName4(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_14 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_65 = 16415152177862271243;
                        }
                        BT_SEMI => {
                            *nextTokPtr = ptr.offset(1);
                            break 'iife_ret_14 XML_TOK_PARAM_ENTITY_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_14 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_65 {
                        7770117754142564343 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_14 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanPoundName(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_15: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                    break 'iife_ret_15 XML_TOK_PARTIAL_1;
                }
                let mut current_block_32: u64;
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_NONASCII => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_15 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_32 = 1867613116081924762;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt2(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_15 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(2);
                        current_block_32 = 7056779235015430508;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt3(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_15 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(3);
                        current_block_32 = 7056779235015430508;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                            || as_normal_encoding(enc).isNmstrt4(enc, ptr) == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_15 XML_TOK_INVALID_1;
                        }
                        ptr = ptr.offset(4);
                        current_block_32 = 7056779235015430508;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_15 XML_TOK_INVALID_1;
                    }
                }
                match current_block_32 {
                    1867613116081924762 => {
                        ptr = ptr.offset(1isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut current_block_63: u64;
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_NONASCII => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_15 XML_TOK_INVALID_1;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_63 = 226587729178875444;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                || as_normal_encoding(enc).isName2(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_15 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_63 = 10380409671385728102;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                || as_normal_encoding(enc).isName3(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_15 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_63 = 10380409671385728102;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_15 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                || as_normal_encoding(enc).isName4(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_15 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_63 = 10380409671385728102;
                        }
                        BT_CR | BT_LF | BT_S | BT_RPAR | BT_GT | BT_PERCNT | BT_VERBAR => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_15 XML_TOK_POUND_NAME_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_15 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_63 {
                        226587729178875444 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_15 -XML_TOK_POUND_NAME_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_scanLit(
        mut open: c_int,
        enc: &ENCODING,
        input: &[c_char],
    ) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_16: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut t: c_int =
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int;
                    match t {
                        5 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_16 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_16 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2isize);
                        }
                        6 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_16 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_16 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3isize);
                        }
                        7 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_16 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_16 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4isize);
                        }
                        0 | 1 | 8 => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_16 XML_TOK_INVALID_1;
                        }
                        12 | 13 => {
                            ptr = ptr.offset(1);
                            if !(t != open) {
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    break 'iife_ret_16 -XML_TOK_LITERAL_1;
                                }
                                *nextTokPtr = ptr;
                                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                    as c_uint
                                {
                                    BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => {
                                        break 'iife_ret_16 XML_TOK_LITERAL_1
                                    }
                                    _ => break 'iife_ret_16 XML_TOK_INVALID_1,
                                }
                            }
                        }
                        _ => {
                            ptr = ptr.offset(1isize);
                        }
                    }
                }
                break 'iife_ret_16 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_prologTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_17: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut tok: c_int = 0;
                if ptr >= end {
                    break 'iife_ret_17 XML_TOK_NONE_1;
                }
                let mut current_block_124: u64;
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_QUOT => {
                        break 'iife_ret_17 ({
                            let (tok_value, next_tok_value) = normal_scanLit(
                                BT_QUOT as c_int,
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_APOS => {
                        break 'iife_ret_17 ({
                            let (tok_value, next_tok_value) = normal_scanLit(
                                BT_APOS as c_int,
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_LT => {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                            break 'iife_ret_17 XML_TOK_PARTIAL_1;
                        }
                        match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                            BT_EXCL => {
                                break 'iife_ret_17 ({
                                    let (tok_value, next_tok_value) = normal_scanDecl(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            BT_QUEST => {
                                break 'iife_ret_17 ({
                                    let (tok_value, next_tok_value) = normal_scanPi(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                                *nextTokPtr = ptr.offset(-(1));
                                break 'iife_ret_17 XML_TOK_INSTANCE_START;
                            }
                            _ => {}
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                    BT_CR => {
                        if ptr.offset(1) == end {
                            *nextTokPtr = end;
                            break 'iife_ret_17 -XML_TOK_PROLOG_S_1;
                        }
                        current_block_124 = 6405334113228567422;
                    }
                    BT_S | BT_LF => {
                        current_block_124 = 6405334113228567422;
                    }
                    BT_PERCNT => {
                        break 'iife_ret_17 ({
                            let (tok_value, next_tok_value) = normal_scanPercent(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_COMMA => {
                        *nextTokPtr = ptr.offset(1);
                        break 'iife_ret_17 XML_TOK_COMMA_1;
                    }
                    BT_LSQB => {
                        *nextTokPtr = ptr.offset(1);
                        break 'iife_ret_17 XML_TOK_OPEN_BRACKET_1;
                    }
                    BT_RSQB => {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                            break 'iife_ret_17 -XML_TOK_CLOSE_BRACKET_1;
                        }
                        if *ptr as c_int == 0x5d {
                            if !(end.offset_from(ptr) as c_long >= (2i32 * 1) as c_long) {
                                break 'iife_ret_17 XML_TOK_PARTIAL_1;
                            }
                            if *ptr.offset(1) as c_int == 0x3e {
                                *nextTokPtr = ptr.offset((2i32 * 1) as isize);
                                break 'iife_ret_17 XML_TOK_COND_SECT_CLOSE_1;
                            }
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_17 XML_TOK_CLOSE_BRACKET_1;
                    }
                    BT_LPAR => {
                        *nextTokPtr = ptr.offset(1);
                        break 'iife_ret_17 XML_TOK_OPEN_PAREN_1;
                    }
                    BT_RPAR => {
                        ptr = ptr.offset(1);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                            break 'iife_ret_17 -XML_TOK_CLOSE_PAREN_1;
                        }
                        match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                            BT_AST => {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_17 XML_TOK_CLOSE_PAREN_ASTERISK_1;
                            }
                            BT_QUEST => {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_17 XML_TOK_CLOSE_PAREN_QUESTION_1;
                            }
                            BT_PLUS => {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_17 XML_TOK_CLOSE_PAREN_PLUS_1;
                            }
                            BT_CR | BT_LF | BT_S | BT_GT | BT_COMMA | BT_VERBAR | BT_RPAR => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_17 XML_TOK_CLOSE_PAREN_1;
                            }
                            _ => {}
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                    BT_VERBAR => {
                        *nextTokPtr = ptr.offset(1);
                        break 'iife_ret_17 XML_TOK_OR_1;
                    }
                    BT_GT => {
                        *nextTokPtr = ptr.offset(1);
                        break 'iife_ret_17 XML_TOK_DECL_CLOSE_1;
                    }
                    BT_NUM => {
                        break 'iife_ret_17 ({
                            let (tok_value, next_tok_value) = normal_scanPoundName(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0 {
                            *nextTokPtr = ptr;
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        if as_normal_encoding(enc).isNmstrt2(enc, ptr) != 0 {
                            ptr = ptr.offset(2);
                            tok = XML_TOK_NAME;
                        } else if as_normal_encoding(enc).isName2(enc, ptr) != 0 {
                            ptr = ptr.offset(2);
                            tok = XML_TOK_NMTOKEN_1;
                        } else {
                            *nextTokPtr = ptr;
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        current_block_124 = 2956972668325154207;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0 {
                            *nextTokPtr = ptr;
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        if as_normal_encoding(enc).isNmstrt3(enc, ptr) != 0 {
                            ptr = ptr.offset(3);
                            tok = XML_TOK_NAME;
                        } else if as_normal_encoding(enc).isName3(enc, ptr) != 0 {
                            ptr = ptr.offset(3);
                            tok = XML_TOK_NMTOKEN_1;
                        } else {
                            *nextTokPtr = ptr;
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        current_block_124 = 2956972668325154207;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                        }
                        if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0 {
                            *nextTokPtr = ptr;
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        if as_normal_encoding(enc).isNmstrt4(enc, ptr) != 0 {
                            ptr = ptr.offset(4);
                            tok = XML_TOK_NAME;
                        } else if as_normal_encoding(enc).isName4(enc, ptr) != 0 {
                            ptr = ptr.offset(4);
                            tok = XML_TOK_NMTOKEN_1;
                        } else {
                            *nextTokPtr = ptr;
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        current_block_124 = 2956972668325154207;
                    }
                    BT_NMSTRT | BT_HEX => {
                        tok = XML_TOK_NAME;
                        ptr = ptr.offset(1);
                        current_block_124 = 2956972668325154207;
                    }
                    BT_DIGIT | BT_NAME | BT_MINUS | BT_COLON_0 => {
                        tok = XML_TOK_NMTOKEN_1;
                        ptr = ptr.offset(1);
                        current_block_124 = 2956972668325154207;
                    }
                    29 | _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_17 XML_TOK_INVALID_1;
                    }
                }
                match current_block_124 {
                    2956972668325154207 => {}
                    _ => {
                        loop {
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break;
                            }
                            let mut current_block_32: u64;
                            match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                            {
                                BT_S | BT_LF => {
                                    current_block_32 = 17500079516916021833;
                                }
                                BT_CR => {
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
                                    break 'iife_ret_17 XML_TOK_PROLOG_S_1;
                                }
                            }
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_17 XML_TOK_PROLOG_S_1;
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    let mut current_block_210: u64;
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_NONASCII => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_210 = 17210391895989911948;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                || as_normal_encoding(enc).isName2(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_17 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_210 = 14244298717249035578;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                || as_normal_encoding(enc).isName3(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_17 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_210 = 14244298717249035578;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                || as_normal_encoding(enc).isName4(enc, ptr) == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_17 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_210 = 14244298717249035578;
                        }
                        BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S
                        | BT_CR | BT_LF => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_17 tok;
                        }
                        BT_COLON_0 => {
                            ptr = ptr.offset(1);
                            match tok {
                                XML_TOK_NAME => {
                                    if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                        break 'iife_ret_17 XML_TOK_PARTIAL_1;
                                    }
                                    tok = XML_TOK_PREFIXED_NAME;
                                    let mut current_block_187: u64;
                                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_uint
                                    {
                                        BT_NONASCII => {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_17 XML_TOK_INVALID_1;
                                        }
                                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                                            current_block_187 = 2692573546887820791;
                                        }
                                        BT_LEAD2 => {
                                            if (end.offset_from(ptr) as c_long) < 2 {
                                                break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0
                                                || as_normal_encoding(enc).isName2(enc, ptr) == 0
                                            {
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_17 XML_TOK_INVALID_1;
                                            }
                                            ptr = ptr.offset(2);
                                            current_block_187 = 9812798724717783973;
                                        }
                                        BT_LEAD3 => {
                                            if (end.offset_from(ptr) as c_long) < 3 {
                                                break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0
                                                || as_normal_encoding(enc).isName3(enc, ptr) == 0
                                            {
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_17 XML_TOK_INVALID_1;
                                            }
                                            ptr = ptr.offset(3);
                                            current_block_187 = 9812798724717783973;
                                        }
                                        BT_LEAD4 => {
                                            if (end.offset_from(ptr) as c_long) < 4 {
                                                break 'iife_ret_17 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0
                                                || as_normal_encoding(enc).isName4(enc, ptr) == 0
                                            {
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_17 XML_TOK_INVALID_1;
                                            }
                                            ptr = ptr.offset(4);
                                            current_block_187 = 9812798724717783973;
                                        }
                                        _ => {
                                            tok = XML_TOK_NMTOKEN_1;
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
                                XML_TOK_PREFIXED_NAME => {
                                    tok = XML_TOK_NMTOKEN_1;
                                }
                                _ => {}
                            }
                            current_block_210 = 14244298717249035578;
                        }
                        BT_PLUS => {
                            if tok == XML_TOK_NMTOKEN_1 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_17 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(1);
                            break 'iife_ret_17 XML_TOK_NAME_PLUS_1;
                        }
                        BT_AST => {
                            if tok == XML_TOK_NMTOKEN_1 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_17 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(1);
                            break 'iife_ret_17 XML_TOK_NAME_ASTERISK_1;
                        }
                        BT_QUEST => {
                            if tok == XML_TOK_NMTOKEN_1 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_17 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(1);
                            break 'iife_ret_17 XML_TOK_NAME_QUESTION_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_17 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_210 {
                        17210391895989911948 => {
                            ptr = ptr.offset(1isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_17 -tok;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_attributeValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_18: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut start: *const c_char = null::<c_char>();
                if ptr >= end {
                    break 'iife_ret_18 XML_TOK_NONE_1;
                } else if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                    break 'iife_ret_18 XML_TOK_PARTIAL_1;
                }
                start = ptr;
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_LEAD2 => {
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            ptr = ptr.offset(4isize);
                        }
                        BT_AMP => {
                            if ptr == start {
                                break 'iife_ret_18 ({
                                    let (tok_value, next_tok_value) = normal_scanRef(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
                        }
                        BT_LT => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_18 XML_TOK_INVALID_1;
                        }
                        BT_LF => {
                            if ptr == start {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_18 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
                        }
                        BT_CR => {
                            if ptr == start {
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    break 'iife_ret_18 XML_TOK_TRAILING_CR_1;
                                }
                                if as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int
                                    == BT_LF as c_int
                                {
                                    ptr = ptr.offset(1isize);
                                }
                                *nextTokPtr = ptr;
                                break 'iife_ret_18 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
                        }
                        BT_S => {
                            if ptr == start {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_18 XML_TOK_ATTRIBUTE_VALUE_S_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
                        }
                        _ => {
                            ptr = ptr.offset(1isize);
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_18 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_entityValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_19: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut start: *const c_char = null::<c_char>();
                if ptr >= end {
                    break 'iife_ret_19 XML_TOK_NONE_1;
                } else if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                    break 'iife_ret_19 XML_TOK_PARTIAL_1;
                }
                start = ptr;
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_LEAD2 => {
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            ptr = ptr.offset(4isize);
                        }
                        BT_AMP => {
                            if ptr == start {
                                break 'iife_ret_19 ({
                                    let (tok_value, next_tok_value) = normal_scanRef(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(1isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
                        }
                        BT_PERCNT => {
                            if ptr == start {
                                let mut tok: c_int = {
                                    let (tok_value, next_tok_value) = normal_scanPercent(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(1), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                };
                                break 'iife_ret_19 if tok == XML_TOK_PERCENT_1 {
                                    XML_TOK_INVALID_1
                                } else {
                                    tok
                                };
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
                        }
                        BT_LF => {
                            if ptr == start {
                                *nextTokPtr = ptr.offset(1);
                                break 'iife_ret_19 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
                        }
                        BT_CR => {
                            if ptr == start {
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    break 'iife_ret_19 XML_TOK_TRAILING_CR_1;
                                }
                                if as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int
                                    == BT_LF as c_int
                                {
                                    ptr = ptr.offset(1isize);
                                }
                                *nextTokPtr = ptr;
                                break 'iife_ret_19 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
                        }
                        _ => {
                            ptr = ptr.offset(1isize);
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_19 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_ignoreSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_20: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut level: c_int = 0;
                while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                    match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_20 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid2(enc, ptr) != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_20 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_20 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid3(enc, ptr) != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_20 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_20 XML_TOK_PARTIAL_CHAR_1;
                            }
                            if as_normal_encoding(enc).isInvalid4(enc, ptr) != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_20 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(4isize);
                        }
                        BT_NONXML | BT_MALFORM | BT_TRAIL => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_20 XML_TOK_INVALID_1;
                        }
                        BT_LT => {
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break 'iife_ret_20 XML_TOK_PARTIAL_1;
                            }
                            if *ptr as c_int == 0x21 {
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    break 'iife_ret_20 XML_TOK_PARTIAL_1;
                                }
                                if *ptr as c_int == 0x5b {
                                    level += 1;
                                    ptr = ptr.offset(1isize);
                                }
                            }
                        }
                        BT_RSQB => {
                            ptr = ptr.offset(1);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                break 'iife_ret_20 XML_TOK_PARTIAL_1;
                            }
                            if *ptr as c_int == 0x5d {
                                ptr = ptr.offset(1);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long) {
                                    break 'iife_ret_20 XML_TOK_PARTIAL_1;
                                }
                                if *ptr as c_int == 0x3e {
                                    ptr = ptr.offset(1);
                                    if level == 0 {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_20 XML_TOK_IGNORE_SECT_1;
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
                break 'iife_ret_20 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn normal_isPublicId(enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        unsafe {
            let mut badPtrVal: *const c_char = null::<c_char>();
            let badPtr = &mut badPtrVal;
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            ptr = ptr.offset(1);
            end = end.offset(-(1));
            while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                let mut current_block_8: u64;
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16
                    | 33 | 30 | 19 | 23 => {
                        current_block_8 = 5143058163439228106;
                    }
                    BT_S => {
                        if *ptr as c_int == 0x9 {
                            *badPtr = ptr;
                            return (0i32, badPtrVal);
                        }
                        current_block_8 = 5143058163439228106;
                    }
                    BT_NAME | BT_NMSTRT => {
                        if *ptr as c_int & !(0x7f) == 0 {
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
                    10293610489198370764 => match *ptr as c_int {
                        36 | 64 => {}
                        _ => {
                            *badPtr = ptr;
                            return (0i32, badPtrVal);
                        }
                    },
                    _ => {}
                }
                ptr = ptr.offset(1);
            }
            return (1, badPtrVal);
        }
    }

    pub(crate) fn normal_getAtts(
        enc: &ENCODING,
        mut ptr: *const c_char,
        mut attsMax: c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> c_int {
        unsafe {
            let mut state: C2RustUnnamed_3 = inName;
            let mut nAtts: c_int = 0;
            let mut open: c_int = 0;
            ptr = ptr.offset(1);
            loop {
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_LEAD2 => {
                        if state == other {
                            if nAtts < attsMax {
                                let ref mut fresh10 = (*atts.offset(nAtts as isize)).name;
                                *fresh10 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName;
                        }
                        ptr = ptr.offset((2i32 - 1i32) as isize);
                    }
                    BT_LEAD3 => {
                        if state == other {
                            if nAtts < attsMax {
                                let ref mut fresh11 = (*atts.offset(nAtts as isize)).name;
                                *fresh11 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName;
                        }
                        ptr = ptr.offset((3i32 - 1i32) as isize);
                    }
                    BT_LEAD4 => {
                        if state == other {
                            if nAtts < attsMax {
                                let ref mut fresh12 = (*atts.offset(nAtts as isize)).name;
                                *fresh12 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName;
                        }
                        ptr = ptr.offset((4i32 - 1i32) as isize);
                    }
                    BT_NONASCII | BT_NMSTRT | BT_HEX => {
                        if state == other {
                            if nAtts < attsMax {
                                let ref mut fresh13 = (*atts.offset(nAtts as isize)).name;
                                *fresh13 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName;
                        }
                    }
                    BT_QUOT => {
                        if state != inValue {
                            if nAtts < attsMax {
                                let ref mut fresh14 = (*atts.offset(nAtts as isize)).valuePtr;
                                *fresh14 = ptr.offset(1isize);
                            }
                            state = inValue;
                            open = BT_QUOT as c_int;
                        } else if open == BT_QUOT as c_int {
                            state = other;
                            if nAtts < attsMax {
                                let ref mut fresh15 = (*atts.offset(nAtts as isize)).valueEnd;
                                *fresh15 = ptr;
                            }
                            nAtts += 1;
                        }
                    }
                    BT_APOS => {
                        if state != inValue {
                            if nAtts < attsMax {
                                let ref mut fresh16 = (*atts.offset(nAtts as isize)).valuePtr;
                                *fresh16 = ptr.offset(1isize);
                            }
                            state = inValue;
                            open = BT_APOS as c_int;
                        } else if open == BT_APOS as c_int {
                            state = other;
                            if nAtts < attsMax {
                                let ref mut fresh17 = (*atts.offset(nAtts as isize)).valueEnd;
                                *fresh17 = ptr;
                            }
                            nAtts += 1;
                        }
                    }
                    BT_AMP => {
                        if nAtts < attsMax {
                            (*atts.offset(nAtts as isize)).normalized = 0i8;
                        }
                    }
                    BT_S => {
                        if state == inName {
                            state = other;
                        } else if state == inValue
                            && nAtts < attsMax
                            && (*atts.offset(nAtts as isize)).normalized as c_int != 0
                            && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                                || *ptr as c_int != ASCII_SPACE
                                || *ptr.offset(1) as c_int == ASCII_SPACE
                                || as_normal_encoding(enc).type_0
                                    [*ptr.offset(1) as c_uchar as usize]
                                    as c_int
                                    == open)
                        {
                            (*atts.offset(nAtts as isize)).normalized = 0i8;
                        }
                    }
                    BT_CR | BT_LF => {
                        if state == inName {
                            state = other;
                        } else if state == inValue && nAtts < attsMax {
                            (*atts.offset(nAtts as isize)).normalized = 0i8;
                        }
                    }
                    BT_GT | BT_SOL => {
                        if state != inValue {
                            return nAtts;
                        }
                    }
                    _ => {}
                }
                ptr = ptr.offset(1);
            }
        }
    }

    pub(crate) fn normal_charRefNumber(_enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        unsafe {
            let mut result: c_int = 0;
            ptr = ptr.offset((2i32 * 1) as isize);
            if *ptr as c_int == 0x78 {
                ptr = ptr.offset(1);
                while !(*ptr as c_int == 0x3b) {
                    let mut c: c_int = *ptr as c_int;
                    match c {
                        ASCII_0 | ASCII_1_1 | ASCII_2_1 | ASCII_3_1 | ASCII_4 | ASCII_5
                        | ASCII_6 | ASCII_7 | ASCII_8_1 | ASCII_9_1 => {
                            result <<= 4;
                            result |= c - ASCII_0;
                        }
                        ASCII_A | ASCII_B_1 | ASCII_C | ASCII_D | ASCII_E_1 | ASCII_F_1 => {
                            result <<= 4;
                            result += 10i32 + (c - ASCII_A);
                        }
                        ASCII_a_1 | ASCII_b | ASCII_c_1 | ASCII_d | ASCII_e_1 | ASCII_f => {
                            result <<= 4;
                            result += 10i32 + (c - ASCII_a_1);
                        }
                        _ => {}
                    }
                    if result >= 0x110000 {
                        return -(1i32);
                    }
                    ptr = ptr.offset(1);
                }
            } else {
                while !(*ptr as c_int == 0x3b) {
                    let mut c_0: c_int = *ptr as c_int;
                    result *= 10;
                    result += c_0 - ASCII_0;
                    if result >= 0x110000 {
                        return -(1i32);
                    }
                    ptr = ptr.offset(1);
                }
            }
            return checkCharRefNumber(result);
        }
    }

    pub(crate) fn normal_predefinedEntityName(_enc: &ENCODING, input: &[c_char]) -> c_int {
        unsafe {
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            match end.offset_from(ptr) as c_long / 1 {
                2 => {
                    if *ptr.offset(1) as c_int == 0x74 {
                        match *ptr as c_int {
                            ASCII_l_1 => return ASCII_LT,
                            ASCII_g_1 => return ASCII_GT,
                            _ => {}
                        }
                    }
                }
                3 => {
                    if *ptr as c_int == 0x61 {
                        ptr = ptr.offset(1);
                        if *ptr as c_int == 0x6d {
                            ptr = ptr.offset(1);
                            if *ptr as c_int == 0x70 {
                                return ASCII_AMP;
                            }
                        }
                    }
                }
                4 => match *ptr as c_int {
                    ASCII_q => {
                        ptr = ptr.offset(1);
                        if *ptr as c_int == 0x75 {
                            ptr = ptr.offset(1);
                            if *ptr as c_int == 0x6f {
                                ptr = ptr.offset(1);
                                if *ptr as c_int == 0x74 {
                                    return ASCII_QUOT;
                                }
                            }
                        }
                    }
                    ASCII_a_1 => {
                        ptr = ptr.offset(1);
                        if *ptr as c_int == 0x70 {
                            ptr = ptr.offset(1);
                            if *ptr as c_int == 0x6f {
                                ptr = ptr.offset(1);
                                if *ptr as c_int == 0x73 {
                                    return ASCII_APOS;
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
    }

    pub(crate) fn normal_nameMatchesAscii(
        _enc: &ENCODING,
        input: &[c_char],
        mut ptr2: *const c_char,
    ) -> c_int {
        unsafe {
            let mut ptr1 = input.as_ptr();
            let mut end1 = ptr1.add(input.len());
            while *ptr2 != 0 {
                if (end1.offset_from(ptr1) as c_long) < 1 {
                    return 0i32;
                }
                if !(*ptr1 as c_int == *ptr2 as c_int) {
                    return 0i32;
                }
                ptr1 = ptr1.offset(1);
                ptr2 = ptr2.offset(1);
            }
            return (ptr1 == end1) as c_int;
        }
    }

    pub(crate) fn normal_nameLength(enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        unsafe {
            let mut start: *const c_char = ptr;
            loop {
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_LEAD2 => {
                        ptr = ptr.offset(2isize);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.offset(3isize);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.offset(4isize);
                    }
                    BT_NONASCII | BT_NMSTRT | BT_COLON_0 | BT_HEX | BT_DIGIT | BT_NAME
                    | BT_MINUS => {
                        ptr = ptr.offset(1isize);
                    }
                    _ => {
                        return ptr.offset_from(start) as c_int;
                    }
                }
            }
        }
    }

    pub(crate) fn normal_skipS(enc: &ENCODING, mut ptr: *const c_char) -> *const c_char {
        unsafe {
            loop {
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_LF | BT_CR | BT_S => {
                        ptr = ptr.offset(1isize);
                    }
                    _ => return ptr,
                }
            }
        }
    }

    pub(crate) fn normal_updatePosition(enc: &ENCODING, input: &[c_char], mut pos: *mut POSITION) {
        unsafe {
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            while end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long {
                match as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint {
                    BT_LEAD2 => {
                        ptr = ptr.offset(2);
                        (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.offset(3);
                        (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.offset(4);
                        (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                    }
                    BT_LF => {
                        (*pos).columnNumber = 0u64;
                        (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                        ptr = ptr.offset(1isize);
                    }
                    BT_CR => {
                        (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                        ptr = ptr.offset(1);
                        if end.offset_from(ptr) as c_long >= (1i32 * 1) as c_long
                            && as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int
                                == BT_LF as c_int
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
    }

    pub(crate) fn little2_scanComment(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_21: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    if !(*ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x2d) {
                        *nextTokPtr = ptr;
                        break 'iife_ret_21 XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                        match if *ptr.offset(1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                        } {
                            BT_LEAD2 => {
                                if (end.offset_from(ptr) as c_long) < 2 {
                                    break 'iife_ret_21 XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            BT_LEAD3 => {
                                if (end.offset_from(ptr) as c_long) < 3 {
                                    break 'iife_ret_21 XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            BT_LEAD4 => {
                                if (end.offset_from(ptr) as c_long) < 4 {
                                    break 'iife_ret_21 XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            BT_NONXML | BT_MALFORM | BT_TRAIL => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_21 XML_TOK_INVALID_1;
                            }
                            BT_MINUS => {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_21 XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x2d {
                                    ptr = ptr.offset(2);
                                    if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                        break 'iife_ret_21 XML_TOK_PARTIAL_1;
                                    }
                                    if !(*ptr.offset(1) as c_int == 0
                                        && *ptr.offset(0) as c_int == 0x3e)
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_21 XML_TOK_INVALID_1;
                                    }
                                    *nextTokPtr = ptr.offset(2);
                                    break 'iife_ret_21 XML_TOK_COMMENT_1;
                                }
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                }
                break 'iife_ret_21 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanDecl(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_22: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_22 XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_MINUS => {
                        break 'iife_ret_22 ({
                            let (tok_value, next_tok_value) = little2_scanComment(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_LSQB => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_22 XML_TOK_COND_SECT_OPEN_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_22 XML_TOK_INVALID_1;
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    's_129: {
                        match if *ptr.offset(1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                        } {
                            BT_PERCNT => {
                                if !(end.offset_from(ptr) as c_long >= (2i32 * 2) as c_long) {
                                    break 'iife_ret_22 XML_TOK_PARTIAL_1;
                                }
                                match if *ptr.offset(2).offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(2) as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(
                                        *ptr.offset(2).offset(1),
                                        *ptr.offset(2).offset(0),
                                    ) as c_uint
                                } {
                                    BT_S | BT_CR | BT_LF | BT_PERCNT => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_22 XML_TOK_INVALID_1;
                                    }
                                    _ => {}
                                }
                            }
                            BT_S | BT_CR | BT_LF => {}
                            BT_NMSTRT | BT_HEX => {
                                ptr = ptr.offset(2);
                                break 's_129;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_22 XML_TOK_INVALID_1;
                            }
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_22 XML_TOK_DECL_OPEN_1;
                    }
                }
                break 'iife_ret_22 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_checkPiTarget(_enc: &ENCODING, input: &[c_char]) -> CheckPiTargetResult {
        unsafe {
            let mut tok: c_int = 0;
            let tokPtr = &mut tok;
            let result: c_int = 'iife_ret_23: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut upper: c_int = 0;
                *tokPtr = XML_TOK_PI_1;
                if end.offset_from(ptr) as c_long != (2i32 * 3) as c_long {
                    break 'iife_ret_23 1i32;
                }
                match if *ptr.offset(1) as c_int == 0 {
                    *ptr.offset(0) as c_int
                } else {
                    -(1)
                } {
                    ASCII_x_1 => {}
                    ASCII_X_1 => {
                        upper = 1i32;
                    }
                    _ => break 'iife_ret_23 1,
                }
                ptr = ptr.offset(2);
                match if *ptr.offset(1) as c_int == 0 {
                    *ptr.offset(0) as c_int
                } else {
                    -(1)
                } {
                    ASCII_m_1 => {}
                    ASCII_M_1 => {
                        upper = 1i32;
                    }
                    _ => break 'iife_ret_23 1,
                }
                ptr = ptr.offset(2);
                match if *ptr.offset(1) as c_int == 0 {
                    *ptr.offset(0) as c_int
                } else {
                    -(1)
                } {
                    ASCII_l_1 => {}
                    ASCII_L_1 => {
                        upper = 1i32;
                    }
                    _ => break 'iife_ret_23 1,
                }
                if upper != 0 {
                    break 'iife_ret_23 0i32;
                }
                *tokPtr = XML_TOK_XML_DECL_1;
                break 'iife_ret_23 1;
            };
            return (result, tok);
        }
    }

    pub(crate) fn little2_scanPi(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_24: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut tok: c_int = 0;
                let mut target: *const c_char = ptr;
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_24 XML_TOK_PARTIAL_1;
                }
                let mut current_block_32: u64;
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_24 XML_TOK_INVALID_1;
                        }
                        current_block_32 = 14358794669692889688;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_32 = 14358794669692889688;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_24 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_24 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_24 XML_TOK_INVALID_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_24 XML_TOK_INVALID_1;
                    }
                }
                match current_block_32 {
                    14358794669692889688 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_118: u64;
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(1) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_24 XML_TOK_INVALID_1;
                            }
                            current_block_118 = 15890151712677504458;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_118 = 15890151712677504458;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_24 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_24 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_24 XML_TOK_INVALID_1;
                        }
                        BT_S | BT_CR | BT_LF => {
                            if {
                                let (ok_value, tok_value) = little2_checkPiTarget(
                                    enc,
                                    c_char_slice_from_ptr_end(target, ptr),
                                );
                                tok = tok_value;
                                ok_value
                            } == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_24 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                                match if *ptr.offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                                } {
                                    BT_LEAD2 => {
                                        if (end.offset_from(ptr) as c_long) < 2 {
                                            break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(2isize);
                                    }
                                    BT_LEAD3 => {
                                        if (end.offset_from(ptr) as c_long) < 3 {
                                            break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(3isize);
                                    }
                                    BT_LEAD4 => {
                                        if (end.offset_from(ptr) as c_long) < 4 {
                                            break 'iife_ret_24 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(4isize);
                                    }
                                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_24 XML_TOK_INVALID_1;
                                    }
                                    BT_QUEST => {
                                        ptr = ptr.offset(2);
                                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long)
                                        {
                                            break 'iife_ret_24 XML_TOK_PARTIAL_1;
                                        }
                                        if *ptr.offset(1) as c_int == 0
                                            && *ptr.offset(0) as c_int == 0x3e
                                        {
                                            *nextTokPtr = ptr.offset(2);
                                            break 'iife_ret_24 tok;
                                        }
                                    }
                                    _ => {
                                        ptr = ptr.offset(2isize);
                                    }
                                }
                            }
                            break 'iife_ret_24 XML_TOK_PARTIAL_1;
                        }
                        BT_QUEST => {
                            if {
                                let (ok_value, tok_value) = little2_checkPiTarget(
                                    enc,
                                    c_char_slice_from_ptr_end(target, ptr),
                                );
                                tok = tok_value;
                                ok_value
                            } == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_24 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_24 XML_TOK_PARTIAL_1;
                            }
                            if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x3e {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_24 tok;
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
                            break 'iife_ret_24 XML_TOK_INVALID_1;
                        }
                        15890151712677504458 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_24 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanCdataSection(_enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_25: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                pub static CDATA_LSQB: [c_char; 6] = [
                    ASCII_C as c_char,
                    ASCII_D as c_char,
                    ASCII_A as c_char,
                    ASCII_T as c_char,
                    ASCII_A as c_char,
                    ASCII_LSQB as c_char,
                ];
                let mut i: c_int = 0;
                if !(end.offset_from(ptr) as c_long >= (6i32 * 2) as c_long) {
                    break 'iife_ret_25 XML_TOK_PARTIAL_1;
                }
                i = 0;
                while i < 6 {
                    if !(*ptr.offset(1) as c_int == 0
                        && *ptr.offset(0) as c_int == CDATA_LSQB[i as usize] as c_int)
                    {
                        *nextTokPtr = ptr;
                        break 'iife_ret_25 XML_TOK_INVALID_1;
                    }
                    i += 1;
                    ptr = ptr.offset(2);
                }
                *nextTokPtr = ptr;
                break 'iife_ret_25 XML_TOK_CDATA_SECT_OPEN_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_cdataSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_26: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if ptr >= end {
                    break 'iife_ret_26 XML_TOK_NONE_1;
                }
                {
                    let mut n: size_t = end.offset_from(ptr) as size_t;
                    if n & (2i32 - 1) as size_t != 0 {
                        n &= !(2i32 - 1) as size_t;
                        if n == 0 {
                            break 'iife_ret_26 XML_TOK_PARTIAL_1;
                        }
                        end = ptr.offset(n as isize);
                    }
                }
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_RSQB => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_26 XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x5d {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_26 XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x3e) {
                                ptr = ptr.offset(-(2isize));
                            } else {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_26 XML_TOK_CDATA_SECT_CLOSE_1;
                            }
                        }
                    }
                    BT_CR => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_26 XML_TOK_PARTIAL_1;
                        }
                        if (if *ptr.offset(1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        }) == BT_LF as c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_26 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_LF => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_26 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_26 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_26 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_26 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_26 XML_TOK_INVALID_1;
                    }
                    _ => {
                        ptr = ptr.offset(2isize);
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(4isize);
                        }
                        BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF | BT_RSQB => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
                        }
                        _ => {
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_26 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanEndTag(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_27: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_27 XML_TOK_PARTIAL_1;
                }
                let mut current_block_32: u64;
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_27 XML_TOK_INVALID_1;
                        }
                        current_block_32 = 8654814784450400207;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_32 = 8654814784450400207;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_27 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_27 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_27 XML_TOK_INVALID_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_27 XML_TOK_INVALID_1;
                    }
                }
                match current_block_32 {
                    8654814784450400207 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_73: u64;
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(1) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_27 XML_TOK_INVALID_1;
                            }
                            current_block_73 = 16411184819389759620;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_73 = 16411184819389759620;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_27 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_27 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_27 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_27 XML_TOK_INVALID_1;
                        }
                        BT_S | BT_CR | BT_LF => {
                            ptr = ptr.offset(2);
                            while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                                match if *ptr.offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                                } {
                                    BT_S | BT_CR | BT_LF => {}
                                    BT_GT => {
                                        *nextTokPtr = ptr.offset(2);
                                        break 'iife_ret_27 XML_TOK_END_TAG_1;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_27 XML_TOK_INVALID_1;
                                    }
                                }
                                ptr = ptr.offset(2);
                            }
                            break 'iife_ret_27 XML_TOK_PARTIAL_1;
                        }
                        BT_COLON_0 => {
                            ptr = ptr.offset(2);
                            current_block_73 = 981995395831942902;
                        }
                        BT_GT => {
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_27 XML_TOK_END_TAG_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_27 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_73 {
                        16411184819389759620 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_27 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanHexCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_28: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_DIGIT | BT_HEX => {}
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_28 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                        match if *ptr.offset(1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                        } {
                            BT_DIGIT | BT_HEX => {}
                            BT_SEMI => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_28 XML_TOK_CHAR_REF_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_28 XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(2);
                    }
                }
                break 'iife_ret_28 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_29: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x78 {
                        break 'iife_ret_29 ({
                            let (tok_value, next_tok_value) = little2_scanHexCharRef(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_DIGIT => {}
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_29 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                        match if *ptr.offset(1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                        } {
                            BT_DIGIT => {}
                            BT_SEMI => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_29 XML_TOK_CHAR_REF_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_29 XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(2);
                    }
                }
                break 'iife_ret_29 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_30: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_30 XML_TOK_PARTIAL_1;
                }
                let mut current_block_33: u64;
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_30 XML_TOK_INVALID_1;
                        }
                        current_block_33 = 6679362556518655255;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_33 = 6679362556518655255;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_30 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_30 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_30 XML_TOK_INVALID_1;
                    }
                    BT_NUM => {
                        break 'iife_ret_30 ({
                            let (tok_value, next_tok_value) = little2_scanCharRef(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_30 XML_TOK_INVALID_1;
                    }
                }
                match current_block_33 {
                    6679362556518655255 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_64: u64;
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(1) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_30 XML_TOK_INVALID_1;
                            }
                            current_block_64 = 405996089697802199;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_64 = 405996089697802199;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_30 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_30 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_30 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_30 XML_TOK_INVALID_1;
                        }
                        BT_SEMI => {
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_30 XML_TOK_ENTITY_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_30 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_64 {
                        405996089697802199 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_30 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanAtts(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_31: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut hadColon: c_int = 0;
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_186: u64;
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(1) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_31 XML_TOK_INVALID_1;
                            }
                            current_block_186 = 17747718632989559416;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_186 = 17747718632989559416;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_31 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_31 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_31 XML_TOK_INVALID_1;
                        }
                        BT_COLON_0 => {
                            if hadColon != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_31 XML_TOK_INVALID_1;
                            }
                            hadColon = 1;
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_31 XML_TOK_PARTIAL_1;
                            }
                            let mut current_block_64: u64;
                            match if *ptr.offset(1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                            } else {
                                unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                            } {
                                BT_NONASCII => {
                                    if namingBitmap[(((nmstrtPages
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_int)
                                        << 3)
                                        + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                        as usize]
                                        & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_31 XML_TOK_INVALID_1;
                                    }
                                    current_block_64 = 12531724302225488581;
                                }
                                BT_NMSTRT | BT_HEX => {
                                    current_block_64 = 12531724302225488581;
                                }
                                BT_LEAD2 => {
                                    if (end.offset_from(ptr) as c_long) < 2 {
                                        break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                }
                                BT_LEAD3 => {
                                    if (end.offset_from(ptr) as c_long) < 3 {
                                        break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                }
                                BT_LEAD4 => {
                                    if (end.offset_from(ptr) as c_long) < 4 {
                                        break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                }
                                _ => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
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
                        BT_S | BT_CR | BT_LF => {
                            loop {
                                let mut t: c_int = 0;
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_31 XML_TOK_PARTIAL_1;
                                }
                                t = if *ptr.offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                                };
                                if t == BT_EQUALS as c_int {
                                    break;
                                }
                                match t {
                                    21 | 10 | 9 => {}
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_31 XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            current_block_186 = 10853015579903106591;
                        }
                        BT_EQUALS => {
                            current_block_186 = 10853015579903106591;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_31 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_186 {
                        10853015579903106591 => {
                            let mut open: c_int = 0;
                            hadColon = 0;
                            loop {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_31 XML_TOK_PARTIAL_1;
                                }
                                open = if *ptr.offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                                };
                                if open == BT_QUOT as c_int || open == BT_APOS as c_int {
                                    break;
                                }
                                match open {
                                    21 | 10 | 9 => {}
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_31 XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            ptr = ptr.offset(2);
                            loop {
                                let mut t_0: c_int = 0;
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_31 XML_TOK_PARTIAL_1;
                                }
                                t_0 = if *ptr.offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                                };
                                if t_0 == open {
                                    break;
                                }
                                match t_0 {
                                    5 => {
                                        if (end.offset_from(ptr) as c_long) < 2 {
                                            break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(2isize);
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as c_long) < 3 {
                                            break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(3isize);
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as c_long) < 4 {
                                            break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(4isize);
                                    }
                                    0 | 1 | 8 => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_31 XML_TOK_INVALID_1;
                                    }
                                    3 => {
                                        let mut tok: c_int = {
                                            let (tok_value, next_tok_value) = little2_scanRef(
                                                enc,
                                                c_char_slice_from_ptr_end(ptr.offset(2), end),
                                            );
                                            ptr = next_tok_value;
                                            tok_value
                                        };
                                        if tok <= 0 {
                                            if tok == XML_TOK_INVALID_1 {
                                                *nextTokPtr = ptr;
                                            }
                                            break 'iife_ret_31 tok;
                                        }
                                    }
                                    2 => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_31 XML_TOK_INVALID_1;
                                    }
                                    _ => {
                                        ptr = ptr.offset(2isize);
                                    }
                                }
                            }
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_31 XML_TOK_PARTIAL_1;
                            }
                            match if *ptr.offset(1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                            } else {
                                unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                            } {
                                BT_S | BT_CR | BT_LF => {
                                    loop {
                                        ptr = ptr.offset(2);
                                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long)
                                        {
                                            break 'iife_ret_31 XML_TOK_PARTIAL_1;
                                        }
                                        match if *ptr.offset(1) as c_int == 0 {
                                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                                as c_uint
                                        } else {
                                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                                                as c_uint
                                        } {
                                            BT_NONASCII => {
                                                if namingBitmap[(((nmstrtPages
                                                    [*ptr.offset(1) as c_uchar as usize]
                                                    as c_int)
                                                    << 3)
                                                    + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                                    as usize]
                                                    & (1)
                                                        << (*ptr.offset(0) as c_uchar as c_int
                                                            & 0x1f)
                                                    == 0
                                                {
                                                    *nextTokPtr = ptr;
                                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                                }
                                                current_block_186 = 923465642386550266;
                                                break;
                                            }
                                            BT_NMSTRT | BT_HEX => {
                                                current_block_186 = 923465642386550266;
                                                break;
                                            }
                                            BT_LEAD2 => {
                                                if (end.offset_from(ptr) as c_long) < 2 {
                                                    break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_31 XML_TOK_INVALID_1;
                                            }
                                            BT_LEAD3 => {
                                                if (end.offset_from(ptr) as c_long) < 3 {
                                                    break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_31 XML_TOK_INVALID_1;
                                            }
                                            BT_LEAD4 => {
                                                if (end.offset_from(ptr) as c_long) < 4 {
                                                    break 'iife_ret_31 XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_31 XML_TOK_INVALID_1;
                                            }
                                            BT_S | BT_CR | BT_LF => {}
                                            BT_GT => {
                                                current_block_186 = 15103464935601583148;
                                                break;
                                            }
                                            BT_SOL => {
                                                current_block_186 = 619033562305054167;
                                                break;
                                            }
                                            _ => {
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_31 XML_TOK_INVALID_1;
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
                                BT_SOL => {
                                    current_block_186 = 619033562305054167;
                                }
                                BT_GT => {
                                    current_block_186 = 15103464935601583148;
                                }
                                _ => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_31 XML_TOK_INVALID_1;
                                }
                            }
                            match current_block_186 {
                                1634947208139838470 => {}
                                _ => match current_block_186 {
                                    619033562305054167 => {
                                        ptr = ptr.offset(2);
                                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long)
                                        {
                                            break 'iife_ret_31 XML_TOK_PARTIAL_1;
                                        }
                                        if !(*ptr.offset(1) as c_int == 0
                                            && *ptr.offset(0) as c_int == 0x3e)
                                        {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_31 XML_TOK_INVALID_1;
                                        }
                                        *nextTokPtr = ptr.offset(2);
                                        break 'iife_ret_31 XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr.offset(2);
                                        break 'iife_ret_31 XML_TOK_START_TAG_WITH_ATTS_1;
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
                break 'iife_ret_31 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanLt(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_32: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut hadColon: c_int = 0;
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_32 XML_TOK_PARTIAL_1;
                }
                let mut current_block_45: u64;
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_32 XML_TOK_INVALID_1;
                        }
                        current_block_45 = 18046087305847344724;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_45 = 18046087305847344724;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                    BT_EXCL => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_32 XML_TOK_PARTIAL_1;
                        }
                        match if *ptr.offset(1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                        } {
                            BT_MINUS => {
                                break 'iife_ret_32 ({
                                    let (tok_value, next_tok_value) = little2_scanComment(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            BT_LSQB => {
                                break 'iife_ret_32 ({
                                    let (tok_value, next_tok_value) = little2_scanCdataSection(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            _ => {}
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                    BT_QUEST => {
                        break 'iife_ret_32 ({
                            let (tok_value, next_tok_value) = little2_scanPi(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_SOL => {
                        break 'iife_ret_32 ({
                            let (tok_value, next_tok_value) = little2_scanEndTag(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_32 XML_TOK_INVALID_1;
                    }
                }
                match current_block_45 {
                    18046087305847344724 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                hadColon = 0;
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_161: u64;
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(1) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_32 XML_TOK_INVALID_1;
                            }
                            current_block_161 = 8998928240368606981;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_161 = 8998928240368606981;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_32 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_32 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_32 XML_TOK_INVALID_1;
                        }
                        BT_COLON_0 => {
                            if hadColon != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_32 XML_TOK_INVALID_1;
                            }
                            hadColon = 1;
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_32 XML_TOK_PARTIAL_1;
                            }
                            let mut current_block_112: u64;
                            match if *ptr.offset(1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                            } else {
                                unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                            } {
                                BT_NONASCII => {
                                    if namingBitmap[(((nmstrtPages
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_int)
                                        << 3)
                                        + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                        as usize]
                                        & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_32 XML_TOK_INVALID_1;
                                    }
                                    current_block_112 = 14391208795021697965;
                                }
                                BT_NMSTRT | BT_HEX => {
                                    current_block_112 = 14391208795021697965;
                                }
                                BT_LEAD2 => {
                                    if (end.offset_from(ptr) as c_long) < 2 {
                                        break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_32 XML_TOK_INVALID_1;
                                }
                                BT_LEAD3 => {
                                    if (end.offset_from(ptr) as c_long) < 3 {
                                        break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_32 XML_TOK_INVALID_1;
                                }
                                BT_LEAD4 => {
                                    if (end.offset_from(ptr) as c_long) < 4 {
                                        break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_32 XML_TOK_INVALID_1;
                                }
                                _ => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_32 XML_TOK_INVALID_1;
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
                        BT_S | BT_CR | BT_LF => {
                            ptr = ptr.offset(2);
                            loop {
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    current_block_161 = 13215501469961642988;
                                    break;
                                }
                                match if *ptr.offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                                } {
                                    BT_NONASCII => {
                                        if namingBitmap[(((nmstrtPages
                                            [*ptr.offset(1) as c_uchar as usize]
                                            as c_int)
                                            << 3)
                                            + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                            as usize]
                                            & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_32 XML_TOK_INVALID_1;
                                        }
                                        current_block_161 = 2369392326157537288;
                                    }
                                    BT_NMSTRT | BT_HEX => {
                                        current_block_161 = 2369392326157537288;
                                    }
                                    BT_LEAD2 => {
                                        if (end.offset_from(ptr) as c_long) < 2 {
                                            break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_32 XML_TOK_INVALID_1;
                                    }
                                    BT_LEAD3 => {
                                        if (end.offset_from(ptr) as c_long) < 3 {
                                            break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_32 XML_TOK_INVALID_1;
                                    }
                                    BT_LEAD4 => {
                                        if (end.offset_from(ptr) as c_long) < 4 {
                                            break 'iife_ret_32 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_32 XML_TOK_INVALID_1;
                                    }
                                    BT_GT => {
                                        current_block_161 = 1918622160084604696;
                                        break;
                                    }
                                    BT_SOL => {
                                        current_block_161 = 1114269873380682160;
                                        break;
                                    }
                                    BT_S | BT_CR | BT_LF => {
                                        ptr = ptr.offset(2);
                                        continue;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_32 XML_TOK_INVALID_1;
                                    }
                                }
                                match current_block_161 {
                                    2369392326157537288 => {
                                        ptr = ptr.offset(2isize);
                                    }
                                    _ => {}
                                }
                                break 'iife_ret_32 ({
                                    let (tok_value, next_tok_value) =
                                        little2_scanAtts(enc, c_char_slice_from_ptr_end(ptr, end));
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            match current_block_161 {
                                1918622160084604696 => {}
                                1114269873380682160 => {}
                                _ => break 'iife_ret_32 XML_TOK_PARTIAL_1,
                            }
                        }
                        BT_GT => {
                            current_block_161 = 1918622160084604696;
                        }
                        BT_SOL => {
                            current_block_161 = 1114269873380682160;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_32 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_161 {
                        1114269873380682160 => {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_32 XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x3e) {
                                *nextTokPtr = ptr;
                                break 'iife_ret_32 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_32 XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                        }
                        1918622160084604696 => {
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_32 XML_TOK_START_TAG_NO_ATTS_1;
                        }
                        8998928240368606981 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_32 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_contentTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_33: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if ptr >= end {
                    break 'iife_ret_33 XML_TOK_NONE_1;
                }
                {
                    let mut n: size_t = end.offset_from(ptr) as size_t;
                    if n & (2i32 - 1) as size_t != 0 {
                        n &= !(2i32 - 1) as size_t;
                        if n == 0 {
                            break 'iife_ret_33 XML_TOK_PARTIAL_1;
                        }
                        end = ptr.offset(n as isize);
                    }
                }
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_LT => {
                        break 'iife_ret_33 ({
                            let (tok_value, next_tok_value) = little2_scanLt(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_AMP => {
                        break 'iife_ret_33 ({
                            let (tok_value, next_tok_value) = little2_scanRef(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_CR => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_33 XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                        }) == BT_LF as c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_33 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_LF => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_33 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_RSQB => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_33 XML_TOK_TRAILING_RSQB_1;
                        }
                        if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x5d {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_33 XML_TOK_TRAILING_RSQB_1;
                            }
                            if !(*ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x3e) {
                                ptr = ptr.offset(-(2isize));
                            } else {
                                *nextTokPtr = ptr;
                                break 'iife_ret_33 XML_TOK_INVALID_1;
                            }
                        }
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_33 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_33 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_33 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_33 XML_TOK_INVALID_1;
                    }
                    _ => {
                        ptr = ptr.offset(2isize);
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_76: u64;
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_76 = 7158658067966855297;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_76 = 7158658067966855297;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_76 = 7158658067966855297;
                        }
                        BT_RSQB => {
                            if end.offset_from(ptr) as c_long >= (2i32 * 2) as c_long {
                                if !(*ptr.offset(2).offset(1) as c_int == 0
                                    && *ptr.offset(2).offset(0) as c_int == 0x5d)
                                {
                                    ptr = ptr.offset(2);
                                    current_block_76 = 7158658067966855297;
                                } else if end.offset_from(ptr) as c_long >= (3i32 * 2) as c_long {
                                    if !(*ptr.offset((2i32 * 2) as isize).offset(1) as c_int == 0
                                        && *ptr.offset((2i32 * 2) as isize).offset(0) as c_int
                                            == 0x3e)
                                    {
                                        ptr = ptr.offset(2isize);
                                    } else {
                                        *nextTokPtr = ptr.offset((2i32 * 2) as isize);
                                        break 'iife_ret_33 XML_TOK_INVALID_1;
                                    }
                                    current_block_76 = 7158658067966855297;
                                } else {
                                    current_block_76 = 17804070343020517427;
                                }
                            } else {
                                current_block_76 = 17804070343020517427;
                            }
                        }
                        BT_AMP | BT_LT | BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF => {
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
                            break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_33 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanPercent(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_34: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_34 XML_TOK_PARTIAL_1;
                }
                let mut current_block_34: u64;
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_34 XML_TOK_INVALID_1;
                        }
                        current_block_34 = 27123471380826226;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_34 = 27123471380826226;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_34 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_34 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_34 XML_TOK_INVALID_1;
                    }
                    BT_S | BT_LF | BT_CR | BT_PERCNT => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_34 XML_TOK_PERCENT_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_34 XML_TOK_INVALID_1;
                    }
                }
                match current_block_34 {
                    27123471380826226 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_65: u64;
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(1) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_34 XML_TOK_INVALID_1;
                            }
                            current_block_65 = 8394962855094477842;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_65 = 8394962855094477842;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_34 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_34 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_34 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_34 XML_TOK_INVALID_1;
                        }
                        BT_SEMI => {
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_34 XML_TOK_PARAM_ENTITY_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_34 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_65 {
                        8394962855094477842 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_34 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanPoundName(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_35: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_35 XML_TOK_PARTIAL_1;
                }
                let mut current_block_32: u64;
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_35 XML_TOK_INVALID_1;
                        }
                        current_block_32 = 14940290876465470105;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_32 = 14940290876465470105;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_35 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_35 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_35 XML_TOK_INVALID_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_35 XML_TOK_INVALID_1;
                    }
                }
                match current_block_32 {
                    14940290876465470105 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_63: u64;
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(1) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_35 XML_TOK_INVALID_1;
                            }
                            current_block_63 = 11497795575834122789;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_63 = 11497795575834122789;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_35 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_35 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_35 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_35 XML_TOK_INVALID_1;
                        }
                        BT_CR | BT_LF | BT_S | BT_RPAR | BT_GT | BT_PERCNT | BT_VERBAR => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_35 XML_TOK_POUND_NAME_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_35 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_63 {
                        11497795575834122789 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_35 -XML_TOK_POUND_NAME_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_scanLit(
        mut open: c_int,
        enc: &ENCODING,
        input: &[c_char],
    ) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_36: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut t: c_int = if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                    };
                    match t {
                        5 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_36 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(2isize);
                        }
                        6 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_36 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(3isize);
                        }
                        7 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_36 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(4isize);
                        }
                        0 | 1 | 8 => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_36 XML_TOK_INVALID_1;
                        }
                        12 | 13 => {
                            ptr = ptr.offset(2);
                            if !(t != open) {
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_36 -XML_TOK_LITERAL_1;
                                }
                                *nextTokPtr = ptr;
                                match if *ptr.offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                                } {
                                    BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => {
                                        break 'iife_ret_36 XML_TOK_LITERAL_1
                                    }
                                    _ => break 'iife_ret_36 XML_TOK_INVALID_1,
                                }
                            }
                        }
                        _ => {
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                break 'iife_ret_36 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_prologTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_37: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut tok: c_int = 0;
                if ptr >= end {
                    break 'iife_ret_37 XML_TOK_NONE_1;
                }
                {
                    let mut n: size_t = end.offset_from(ptr) as size_t;
                    if n & (2i32 - 1) as size_t != 0 {
                        n &= !(2i32 - 1) as size_t;
                        if n == 0 {
                            break 'iife_ret_37 XML_TOK_PARTIAL_1;
                        }
                        end = ptr.offset(n as isize);
                    }
                }
                let mut current_block_124: u64;
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_QUOT => {
                        break 'iife_ret_37 ({
                            let (tok_value, next_tok_value) = little2_scanLit(
                                BT_QUOT as c_int,
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_APOS => {
                        break 'iife_ret_37 ({
                            let (tok_value, next_tok_value) = little2_scanLit(
                                BT_APOS as c_int,
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_LT => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_37 XML_TOK_PARTIAL_1;
                        }
                        match if *ptr.offset(1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                        } {
                            BT_EXCL => {
                                break 'iife_ret_37 ({
                                    let (tok_value, next_tok_value) = little2_scanDecl(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            BT_QUEST => {
                                break 'iife_ret_37 ({
                                    let (tok_value, next_tok_value) = little2_scanPi(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                                *nextTokPtr = ptr.offset(-(2));
                                break 'iife_ret_37 XML_TOK_INSTANCE_START;
                            }
                            _ => {}
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                    BT_CR => {
                        if ptr.offset(2) == end {
                            *nextTokPtr = end;
                            break 'iife_ret_37 -XML_TOK_PROLOG_S_1;
                        }
                        current_block_124 = 17513858719706519675;
                    }
                    BT_S | BT_LF => {
                        current_block_124 = 17513858719706519675;
                    }
                    BT_PERCNT => {
                        break 'iife_ret_37 ({
                            let (tok_value, next_tok_value) = little2_scanPercent(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_COMMA => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_37 XML_TOK_COMMA_1;
                    }
                    BT_LSQB => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_37 XML_TOK_OPEN_BRACKET_1;
                    }
                    BT_RSQB => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_37 -XML_TOK_CLOSE_BRACKET_1;
                        }
                        if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x5d {
                            if !(end.offset_from(ptr) as c_long >= (2i32 * 2) as c_long) {
                                break 'iife_ret_37 XML_TOK_PARTIAL_1;
                            }
                            if *ptr.offset(2).offset(1) as c_int == 0
                                && *ptr.offset(2).offset(0) as c_int == 0x3e
                            {
                                *nextTokPtr = ptr.offset((2i32 * 2) as isize);
                                break 'iife_ret_37 XML_TOK_COND_SECT_CLOSE_1;
                            }
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_37 XML_TOK_CLOSE_BRACKET_1;
                    }
                    BT_LPAR => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_37 XML_TOK_OPEN_PAREN_1;
                    }
                    BT_RPAR => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_37 -XML_TOK_CLOSE_PAREN_1;
                        }
                        match if *ptr.offset(1) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                        } {
                            BT_AST => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_37 XML_TOK_CLOSE_PAREN_ASTERISK_1;
                            }
                            BT_QUEST => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_37 XML_TOK_CLOSE_PAREN_QUESTION_1;
                            }
                            BT_PLUS => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_37 XML_TOK_CLOSE_PAREN_PLUS_1;
                            }
                            BT_CR | BT_LF | BT_S | BT_GT | BT_COMMA | BT_VERBAR | BT_RPAR => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_37 XML_TOK_CLOSE_PAREN_1;
                            }
                            _ => {}
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                    BT_VERBAR => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_37 XML_TOK_OR_1;
                    }
                    BT_GT => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_37 XML_TOK_DECL_CLOSE_1;
                    }
                    BT_NUM => {
                        break 'iife_ret_37 ({
                            let (tok_value, next_tok_value) = little2_scanPoundName(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        tok = XML_TOK_NAME;
                        ptr = ptr.offset(2);
                        current_block_124 = 2956972668325154207;
                    }
                    BT_DIGIT | BT_NAME | BT_MINUS | BT_COLON_0 => {
                        tok = XML_TOK_NMTOKEN_1;
                        ptr = ptr.offset(2);
                        current_block_124 = 2956972668325154207;
                    }
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                            != 0
                        {
                            ptr = ptr.offset(2);
                            tok = XML_TOK_NAME;
                            current_block_124 = 2956972668325154207;
                        } else if namingBitmap[(((namePages[*ptr.offset(1) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(0) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                            != 0
                        {
                            ptr = ptr.offset(2);
                            tok = XML_TOK_NMTOKEN_1;
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
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break;
                            }
                            let mut current_block_32: u64;
                            match if *ptr.offset(1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                            } else {
                                unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                            } {
                                BT_S | BT_LF => {
                                    current_block_32 = 17500079516916021833;
                                }
                                BT_CR => {
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
                                    break 'iife_ret_37 XML_TOK_PROLOG_S_1;
                                }
                            }
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_37 XML_TOK_PROLOG_S_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_37 XML_TOK_INVALID_1;
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_210: u64;
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(1) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_37 XML_TOK_INVALID_1;
                            }
                            current_block_210 = 786388639404123072;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_210 = 786388639404123072;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_37 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_37 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_37 XML_TOK_INVALID_1;
                        }
                        BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S
                        | BT_CR | BT_LF => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_37 tok;
                        }
                        BT_COLON_0 => {
                            ptr = ptr.offset(2);
                            match tok {
                                XML_TOK_NAME => {
                                    if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                        break 'iife_ret_37 XML_TOK_PARTIAL_1;
                                    }
                                    tok = XML_TOK_PREFIXED_NAME;
                                    let mut current_block_187: u64;
                                    match if *ptr.offset(1) as c_int == 0 {
                                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                            as c_uint
                                    } else {
                                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                                    } {
                                        BT_NONASCII => {
                                            if namingBitmap[(((namePages
                                                [*ptr.offset(1) as c_uchar as usize]
                                                as c_int)
                                                << 3)
                                                + (*ptr.offset(0) as c_uchar as c_int >> 5))
                                                as usize]
                                                & (1) << (*ptr.offset(0) as c_uchar as c_int & 0x1f)
                                                == 0
                                            {
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_37 XML_TOK_INVALID_1;
                                            }
                                            current_block_187 = 16869951820887225088;
                                        }
                                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                                            current_block_187 = 16869951820887225088;
                                        }
                                        BT_LEAD2 => {
                                            if (end.offset_from(ptr) as c_long) < 2 {
                                                break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_37 XML_TOK_INVALID_1;
                                        }
                                        BT_LEAD3 => {
                                            if (end.offset_from(ptr) as c_long) < 3 {
                                                break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_37 XML_TOK_INVALID_1;
                                        }
                                        BT_LEAD4 => {
                                            if (end.offset_from(ptr) as c_long) < 4 {
                                                break 'iife_ret_37 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_37 XML_TOK_INVALID_1;
                                        }
                                        _ => {
                                            tok = XML_TOK_NMTOKEN_1;
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
                                XML_TOK_PREFIXED_NAME => {
                                    tok = XML_TOK_NMTOKEN_1;
                                }
                                _ => {}
                            }
                            current_block_210 = 14244298717249035578;
                        }
                        BT_PLUS => {
                            if tok == XML_TOK_NMTOKEN_1 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_37 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_37 XML_TOK_NAME_PLUS_1;
                        }
                        BT_AST => {
                            if tok == XML_TOK_NMTOKEN_1 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_37 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_37 XML_TOK_NAME_ASTERISK_1;
                        }
                        BT_QUEST => {
                            if tok == XML_TOK_NMTOKEN_1 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_37 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_37 XML_TOK_NAME_QUESTION_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_37 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_210 {
                        786388639404123072 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_37 -tok;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_attributeValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_38: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut start: *const c_char = null::<c_char>();
                if ptr >= end {
                    break 'iife_ret_38 XML_TOK_NONE_1;
                } else if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_38 XML_TOK_PARTIAL_1;
                }
                start = ptr;
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_LEAD2 => {
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            ptr = ptr.offset(4isize);
                        }
                        BT_AMP => {
                            if ptr == start {
                                break 'iife_ret_38 ({
                                    let (tok_value, next_tok_value) = little2_scanRef(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
                        }
                        BT_LT => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_38 XML_TOK_INVALID_1;
                        }
                        BT_LF => {
                            if ptr == start {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_38 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
                        }
                        BT_CR => {
                            if ptr == start {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_38 XML_TOK_TRAILING_CR_1;
                                }
                                if (if *ptr.offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                                }) == BT_LF as c_int
                                {
                                    ptr = ptr.offset(2isize);
                                }
                                *nextTokPtr = ptr;
                                break 'iife_ret_38 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
                        }
                        BT_S => {
                            if ptr == start {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_38 XML_TOK_ATTRIBUTE_VALUE_S_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
                        }
                        _ => {
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_38 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_entityValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_39: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut start: *const c_char = null::<c_char>();
                if ptr >= end {
                    break 'iife_ret_39 XML_TOK_NONE_1;
                } else if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_39 XML_TOK_PARTIAL_1;
                }
                start = ptr;
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_LEAD2 => {
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            ptr = ptr.offset(4isize);
                        }
                        BT_AMP => {
                            if ptr == start {
                                break 'iife_ret_39 ({
                                    let (tok_value, next_tok_value) = little2_scanRef(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
                        }
                        BT_PERCNT => {
                            if ptr == start {
                                let mut tok: c_int = {
                                    let (tok_value, next_tok_value) = little2_scanPercent(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                };
                                break 'iife_ret_39 if tok == XML_TOK_PERCENT_1 {
                                    XML_TOK_INVALID_1
                                } else {
                                    tok
                                };
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
                        }
                        BT_LF => {
                            if ptr == start {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_39 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
                        }
                        BT_CR => {
                            if ptr == start {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_39 XML_TOK_TRAILING_CR_1;
                                }
                                if (if *ptr.offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                                }) == BT_LF as c_int
                                {
                                    ptr = ptr.offset(2isize);
                                }
                                *nextTokPtr = ptr;
                                break 'iife_ret_39 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
                        }
                        _ => {
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_39 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_ignoreSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_40: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut level: c_int = 0;
                {
                    let mut n: size_t = end.offset_from(ptr) as size_t;
                    if n & (2i32 - 1) as size_t != 0 {
                        n &= !(2i32 - 1) as size_t;
                        end = ptr.offset(n as isize);
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(1) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                    } {
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_40 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_40 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_40 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(4isize);
                        }
                        BT_NONXML | BT_MALFORM | BT_TRAIL => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_40 XML_TOK_INVALID_1;
                        }
                        BT_LT => {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_40 XML_TOK_PARTIAL_1;
                            }
                            if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x21 {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_40 XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x5b {
                                    level += 1;
                                    ptr = ptr.offset(2isize);
                                }
                            }
                        }
                        BT_RSQB => {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_40 XML_TOK_PARTIAL_1;
                            }
                            if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x5d {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_40 XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x3e {
                                    ptr = ptr.offset(2);
                                    if level == 0 {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_40 XML_TOK_IGNORE_SECT_1;
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
                break 'iife_ret_40 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn little2_isPublicId(enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        unsafe {
            let mut badPtrVal: *const c_char = null::<c_char>();
            let badPtr = &mut badPtrVal;
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            ptr = ptr.offset(2);
            end = end.offset(-(2));
            while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                let mut current_block_8: u64;
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16
                    | 33 | 30 | 19 | 23 => {
                        current_block_8 = 5143058163439228106;
                    }
                    BT_S => {
                        if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x9 {
                            *badPtr = ptr;
                            return (0i32, badPtrVal);
                        }
                        current_block_8 = 5143058163439228106;
                    }
                    BT_NAME | BT_NMSTRT => {
                        if (if *ptr.offset(1) as c_int == 0 {
                            *ptr.offset(0) as c_int
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
                        match if *ptr.offset(1) as c_int == 0 {
                            *ptr.offset(0) as c_int
                        } else {
                            -(1)
                        } {
                            36 | 64 => {}
                            _ => {
                                *badPtr = ptr;
                                return (0i32, badPtrVal);
                            }
                        }
                    }
                    _ => {}
                }
                ptr = ptr.offset(2);
            }
            return (1, badPtrVal);
        }
    }

    pub(crate) fn little2_getAtts(
        enc: &ENCODING,
        mut ptr: *const c_char,
        mut attsMax: c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> c_int {
        unsafe {
            let mut state: C2RustUnnamed_3 = inName_0;
            let mut nAtts: c_int = 0;
            let mut open: c_int = 0;
            ptr = ptr.offset(2);
            loop {
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_LEAD2 => {
                        if state == other_0 {
                            if nAtts < attsMax {
                                let ref mut fresh29 = (*atts.offset(nAtts as isize)).name;
                                *fresh29 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName_0;
                        }
                        ptr = ptr.offset((2i32 - 2i32) as isize);
                    }
                    BT_LEAD3 => {
                        if state == other_0 {
                            if nAtts < attsMax {
                                let ref mut fresh30 = (*atts.offset(nAtts as isize)).name;
                                *fresh30 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName_0;
                        }
                        ptr = ptr.offset((3i32 - 2i32) as isize);
                    }
                    BT_LEAD4 => {
                        if state == other_0 {
                            if nAtts < attsMax {
                                let ref mut fresh31 = (*atts.offset(nAtts as isize)).name;
                                *fresh31 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName_0;
                        }
                        ptr = ptr.offset((4i32 - 2i32) as isize);
                    }
                    BT_NONASCII | BT_NMSTRT | BT_HEX => {
                        if state == other_0 {
                            if nAtts < attsMax {
                                let ref mut fresh32 = (*atts.offset(nAtts as isize)).name;
                                *fresh32 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName_0;
                        }
                    }
                    BT_QUOT => {
                        if state != inValue_0 {
                            if nAtts < attsMax {
                                let ref mut fresh33 = (*atts.offset(nAtts as isize)).valuePtr;
                                *fresh33 = ptr.offset(2isize);
                            }
                            state = inValue_0;
                            open = BT_QUOT as c_int;
                        } else if open == BT_QUOT as c_int {
                            state = other_0;
                            if nAtts < attsMax {
                                let ref mut fresh34 = (*atts.offset(nAtts as isize)).valueEnd;
                                *fresh34 = ptr;
                            }
                            nAtts += 1;
                        }
                    }
                    BT_APOS => {
                        if state != inValue_0 {
                            if nAtts < attsMax {
                                let ref mut fresh35 = (*atts.offset(nAtts as isize)).valuePtr;
                                *fresh35 = ptr.offset(2isize);
                            }
                            state = inValue_0;
                            open = BT_APOS as c_int;
                        } else if open == BT_APOS as c_int {
                            state = other_0;
                            if nAtts < attsMax {
                                let ref mut fresh36 = (*atts.offset(nAtts as isize)).valueEnd;
                                *fresh36 = ptr;
                            }
                            nAtts += 1;
                        }
                    }
                    BT_AMP => {
                        if nAtts < attsMax {
                            (*atts.offset(nAtts as isize)).normalized = 0i8;
                        }
                    }
                    BT_S => {
                        if state == inName_0 {
                            state = other_0;
                        } else if state == inValue_0
                            && nAtts < attsMax
                            && (*atts.offset(nAtts as isize)).normalized as c_int != 0
                            && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                                || (if *ptr.offset(1) as c_int == 0 {
                                    *ptr.offset(0) as c_int
                                } else {
                                    -(1)
                                }) != ASCII_SPACE
                                || (if *ptr.offset(2).offset(1) as c_int == 0 {
                                    *ptr.offset(2).offset(0) as c_int
                                } else {
                                    -(1)
                                }) == ASCII_SPACE
                                || (if *ptr.offset(2).offset(1) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(2) as c_uchar as usize]
                                        as c_int
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
                    BT_CR | BT_LF => {
                        if state == inName_0 {
                            state = other_0;
                        } else if state == inValue_0 && nAtts < attsMax {
                            (*atts.offset(nAtts as isize)).normalized = 0i8;
                        }
                    }
                    BT_GT | BT_SOL => {
                        if state != inValue_0 {
                            return nAtts;
                        }
                    }
                    _ => {}
                }
                ptr = ptr.offset(2);
            }
        }
    }

    pub(crate) fn little2_charRefNumber(_enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        unsafe {
            let mut result: c_int = 0;
            ptr = ptr.offset((2i32 * 2) as isize);
            if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x78 {
                ptr = ptr.offset(2);
                while !(*ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x3b) {
                    let mut c: c_int = if *ptr.offset(1) as c_int == 0 {
                        *ptr.offset(0) as c_int
                    } else {
                        -(1)
                    };
                    match c {
                        ASCII_0 | ASCII_1_1 | ASCII_2_1 | ASCII_3_1 | ASCII_4 | ASCII_5
                        | ASCII_6 | ASCII_7 | ASCII_8_1 | ASCII_9_1 => {
                            result <<= 4;
                            result |= c - ASCII_0;
                        }
                        ASCII_A | ASCII_B_1 | ASCII_C | ASCII_D | ASCII_E_1 | ASCII_F_1 => {
                            result <<= 4;
                            result += 10i32 + (c - ASCII_A);
                        }
                        ASCII_a_1 | ASCII_b | ASCII_c_1 | ASCII_d | ASCII_e_1 | ASCII_f => {
                            result <<= 4;
                            result += 10i32 + (c - ASCII_a_1);
                        }
                        _ => {}
                    }
                    if result >= 0x110000 {
                        return -(1i32);
                    }
                    ptr = ptr.offset(2);
                }
            } else {
                while !(*ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x3b) {
                    let mut c_0: c_int = if *ptr.offset(1) as c_int == 0 {
                        *ptr.offset(0) as c_int
                    } else {
                        -(1)
                    };
                    result *= 10;
                    result += c_0 - ASCII_0;
                    if result >= 0x110000 {
                        return -(1i32);
                    }
                    ptr = ptr.offset(2);
                }
            }
            return checkCharRefNumber(result);
        }
    }

    pub(crate) fn little2_predefinedEntityName(_enc: &ENCODING, input: &[c_char]) -> c_int {
        unsafe {
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            match end.offset_from(ptr) as c_long / 2 {
                2 => {
                    if *ptr.offset(2).offset(1) as c_int == 0
                        && *ptr.offset(2).offset(0) as c_int == 0x74
                    {
                        match if *ptr.offset(1) as c_int == 0 {
                            *ptr.offset(0) as c_int
                        } else {
                            -(1)
                        } {
                            ASCII_l_1 => return ASCII_LT,
                            ASCII_g_1 => return ASCII_GT,
                            _ => {}
                        }
                    }
                }
                3 => {
                    if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x61 {
                        ptr = ptr.offset(2);
                        if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x6d {
                            ptr = ptr.offset(2);
                            if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x70 {
                                return ASCII_AMP;
                            }
                        }
                    }
                }
                4 => {
                    match if *ptr.offset(1) as c_int == 0 {
                        *ptr.offset(0) as c_int
                    } else {
                        -(1)
                    } {
                        ASCII_q => {
                            ptr = ptr.offset(2);
                            if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x75 {
                                ptr = ptr.offset(2);
                                if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x6f {
                                    ptr = ptr.offset(2);
                                    if *ptr.offset(1) as c_int == 0
                                        && *ptr.offset(0) as c_int == 0x74
                                    {
                                        return ASCII_QUOT;
                                    }
                                }
                            }
                        }
                        ASCII_a_1 => {
                            ptr = ptr.offset(2);
                            if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x70 {
                                ptr = ptr.offset(2);
                                if *ptr.offset(1) as c_int == 0 && *ptr.offset(0) as c_int == 0x6f {
                                    ptr = ptr.offset(2);
                                    if *ptr.offset(1) as c_int == 0
                                        && *ptr.offset(0) as c_int == 0x73
                                    {
                                        return ASCII_APOS;
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
    }

    pub(crate) fn little2_nameMatchesAscii(
        _enc: &ENCODING,
        input: &[c_char],
        mut ptr2: *const c_char,
    ) -> c_int {
        unsafe {
            let mut ptr1 = input.as_ptr();
            let mut end1 = ptr1.add(input.len());
            while *ptr2 != 0 {
                if (end1.offset_from(ptr1) as c_long) < 2 {
                    return 0i32;
                }
                if !(*ptr1.offset(1) as c_int == 0 && *ptr1.offset(0) as c_int == *ptr2 as c_int) {
                    return 0i32;
                }
                ptr1 = ptr1.offset(2);
                ptr2 = ptr2.offset(1);
            }
            return (ptr1 == end1) as c_int;
        }
    }

    pub(crate) fn little2_nameLength(enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        unsafe {
            let mut start: *const c_char = ptr;
            loop {
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_LEAD2 => {
                        ptr = ptr.offset(2isize);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.offset(3isize);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.offset(4isize);
                    }
                    BT_NONASCII | BT_NMSTRT | BT_COLON_0 | BT_HEX | BT_DIGIT | BT_NAME
                    | BT_MINUS => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {
                        return ptr.offset_from(start) as c_int;
                    }
                }
            }
        }
    }

    pub(crate) fn little2_skipS(enc: &ENCODING, mut ptr: *const c_char) -> *const c_char {
        unsafe {
            loop {
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_LF | BT_CR | BT_S => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => return ptr,
                }
            }
        }
    }

    pub(crate) fn little2_updatePosition(enc: &ENCODING, input: &[c_char], mut pos: *mut POSITION) {
        unsafe {
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                match if *ptr.offset(1) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(1), *ptr.offset(0)) as c_uint
                } {
                    BT_LEAD2 => {
                        ptr = ptr.offset(2);
                        (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.offset(3);
                        (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.offset(4);
                        (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                    }
                    BT_LF => {
                        (*pos).columnNumber = 0u64;
                        (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                        ptr = ptr.offset(2isize);
                    }
                    BT_CR => {
                        (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                        ptr = ptr.offset(2);
                        if end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long
                            && (if *ptr.offset(1) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr as c_uchar as usize] as c_int
                            } else {
                                unicode_byte_type(*ptr.offset(1), *ptr.offset(0))
                            }) == BT_LF as c_int
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
    }

    pub(crate) fn big2_scanComment(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_41: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    if !(*ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x2d) {
                        *nextTokPtr = ptr;
                        break 'iife_ret_41 XML_TOK_INVALID_1;
                    }
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                        match if *ptr.offset(0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                        } {
                            BT_LEAD2 => {
                                if (end.offset_from(ptr) as c_long) < 2 {
                                    break 'iife_ret_41 XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(2isize);
                            }
                            BT_LEAD3 => {
                                if (end.offset_from(ptr) as c_long) < 3 {
                                    break 'iife_ret_41 XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(3isize);
                            }
                            BT_LEAD4 => {
                                if (end.offset_from(ptr) as c_long) < 4 {
                                    break 'iife_ret_41 XML_TOK_PARTIAL_CHAR_1;
                                }
                                ptr = ptr.offset(4isize);
                            }
                            BT_NONXML | BT_MALFORM | BT_TRAIL => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_41 XML_TOK_INVALID_1;
                            }
                            BT_MINUS => {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_41 XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x2d {
                                    ptr = ptr.offset(2);
                                    if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                        break 'iife_ret_41 XML_TOK_PARTIAL_1;
                                    }
                                    if !(*ptr.offset(0) as c_int == 0
                                        && *ptr.offset(1) as c_int == 0x3e)
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_41 XML_TOK_INVALID_1;
                                    }
                                    *nextTokPtr = ptr.offset(2);
                                    break 'iife_ret_41 XML_TOK_COMMENT_1;
                                }
                            }
                            _ => {
                                ptr = ptr.offset(2isize);
                            }
                        }
                    }
                }
                break 'iife_ret_41 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanDecl(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_42: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_42 XML_TOK_PARTIAL_1;
                }
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_MINUS => {
                        break 'iife_ret_42 ({
                            let (tok_value, next_tok_value) = big2_scanComment(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_LSQB => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_42 XML_TOK_COND_SECT_OPEN_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_42 XML_TOK_INVALID_1;
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    's_129: {
                        match if *ptr.offset(0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                        } {
                            BT_PERCNT => {
                                if !(end.offset_from(ptr) as c_long >= (2i32 * 2) as c_long) {
                                    break 'iife_ret_42 XML_TOK_PARTIAL_1;
                                }
                                match if *ptr.offset(2).offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(2).offset(1) as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(
                                        *ptr.offset(2).offset(0),
                                        *ptr.offset(2).offset(1),
                                    ) as c_uint
                                } {
                                    BT_S | BT_CR | BT_LF | BT_PERCNT => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_42 XML_TOK_INVALID_1;
                                    }
                                    _ => {}
                                }
                            }
                            BT_S | BT_CR | BT_LF => {}
                            BT_NMSTRT | BT_HEX => {
                                ptr = ptr.offset(2);
                                break 's_129;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_42 XML_TOK_INVALID_1;
                            }
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_42 XML_TOK_DECL_OPEN_1;
                    }
                }
                break 'iife_ret_42 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_checkPiTarget(_enc: &ENCODING, input: &[c_char]) -> CheckPiTargetResult {
        unsafe {
            let mut tok: c_int = 0;
            let tokPtr = &mut tok;
            let result: c_int = 'iife_ret_43: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut upper: c_int = 0;
                *tokPtr = XML_TOK_PI_1;
                if end.offset_from(ptr) as c_long != (2i32 * 3) as c_long {
                    break 'iife_ret_43 1i32;
                }
                match if *ptr.offset(0) as c_int == 0 {
                    *ptr.offset(1) as c_int
                } else {
                    -(1)
                } {
                    ASCII_x_1 => {}
                    ASCII_X_1 => {
                        upper = 1i32;
                    }
                    _ => break 'iife_ret_43 1,
                }
                ptr = ptr.offset(2);
                match if *ptr.offset(0) as c_int == 0 {
                    *ptr.offset(1) as c_int
                } else {
                    -(1)
                } {
                    ASCII_m_1 => {}
                    ASCII_M_1 => {
                        upper = 1i32;
                    }
                    _ => break 'iife_ret_43 1,
                }
                ptr = ptr.offset(2);
                match if *ptr.offset(0) as c_int == 0 {
                    *ptr.offset(1) as c_int
                } else {
                    -(1)
                } {
                    ASCII_l_1 => {}
                    ASCII_L_1 => {
                        upper = 1i32;
                    }
                    _ => break 'iife_ret_43 1,
                }
                if upper != 0 {
                    break 'iife_ret_43 0i32;
                }
                *tokPtr = XML_TOK_XML_DECL_1;
                break 'iife_ret_43 1;
            };
            return (result, tok);
        }
    }

    pub(crate) fn big2_scanPi(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_44: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut tok: c_int = 0;
                let mut target: *const c_char = ptr;
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_44 XML_TOK_PARTIAL_1;
                }
                let mut current_block_32: u64;
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_44 XML_TOK_INVALID_1;
                        }
                        current_block_32 = 2802485987355401260;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_32 = 2802485987355401260;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_44 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_44 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_44 XML_TOK_INVALID_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_44 XML_TOK_INVALID_1;
                    }
                }
                match current_block_32 {
                    2802485987355401260 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_118: u64;
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(0) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_44 XML_TOK_INVALID_1;
                            }
                            current_block_118 = 11190361564366887465;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_118 = 11190361564366887465;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_44 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_44 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_44 XML_TOK_INVALID_1;
                        }
                        BT_S | BT_CR | BT_LF => {
                            if {
                                let (ok_value, tok_value) =
                                    big2_checkPiTarget(enc, c_char_slice_from_ptr_end(target, ptr));
                                tok = tok_value;
                                ok_value
                            } == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_44 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                                match if *ptr.offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                                } {
                                    BT_LEAD2 => {
                                        if (end.offset_from(ptr) as c_long) < 2 {
                                            break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(2isize);
                                    }
                                    BT_LEAD3 => {
                                        if (end.offset_from(ptr) as c_long) < 3 {
                                            break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(3isize);
                                    }
                                    BT_LEAD4 => {
                                        if (end.offset_from(ptr) as c_long) < 4 {
                                            break 'iife_ret_44 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(4isize);
                                    }
                                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_44 XML_TOK_INVALID_1;
                                    }
                                    BT_QUEST => {
                                        ptr = ptr.offset(2);
                                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long)
                                        {
                                            break 'iife_ret_44 XML_TOK_PARTIAL_1;
                                        }
                                        if *ptr.offset(0) as c_int == 0
                                            && *ptr.offset(1) as c_int == 0x3e
                                        {
                                            *nextTokPtr = ptr.offset(2);
                                            break 'iife_ret_44 tok;
                                        }
                                    }
                                    _ => {
                                        ptr = ptr.offset(2isize);
                                    }
                                }
                            }
                            break 'iife_ret_44 XML_TOK_PARTIAL_1;
                        }
                        BT_QUEST => {
                            if {
                                let (ok_value, tok_value) =
                                    big2_checkPiTarget(enc, c_char_slice_from_ptr_end(target, ptr));
                                tok = tok_value;
                                ok_value
                            } == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_44 XML_TOK_INVALID_1;
                            }
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_44 XML_TOK_PARTIAL_1;
                            }
                            if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x3e {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_44 tok;
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
                            break 'iife_ret_44 XML_TOK_INVALID_1;
                        }
                        11190361564366887465 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_44 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanCdataSection(_enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_45: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                pub static CDATA_LSQB: [c_char; 6] = [
                    ASCII_C as c_char,
                    ASCII_D as c_char,
                    ASCII_A as c_char,
                    ASCII_T as c_char,
                    ASCII_A as c_char,
                    ASCII_LSQB as c_char,
                ];
                let mut i: c_int = 0;
                if !(end.offset_from(ptr) as c_long >= (6i32 * 2) as c_long) {
                    break 'iife_ret_45 XML_TOK_PARTIAL_1;
                }
                i = 0;
                while i < 6 {
                    if !(*ptr.offset(0) as c_int == 0
                        && *ptr.offset(1) as c_int == CDATA_LSQB[i as usize] as c_int)
                    {
                        *nextTokPtr = ptr;
                        break 'iife_ret_45 XML_TOK_INVALID_1;
                    }
                    i += 1;
                    ptr = ptr.offset(2);
                }
                *nextTokPtr = ptr;
                break 'iife_ret_45 XML_TOK_CDATA_SECT_OPEN_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_cdataSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_46: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if ptr >= end {
                    break 'iife_ret_46 XML_TOK_NONE_1;
                }
                {
                    let mut n: size_t = end.offset_from(ptr) as size_t;
                    if n & (2i32 - 1) as size_t != 0 {
                        n &= !(2i32 - 1) as size_t;
                        if n == 0 {
                            break 'iife_ret_46 XML_TOK_PARTIAL_1;
                        }
                        end = ptr.offset(n as isize);
                    }
                }
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_RSQB => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_46 XML_TOK_PARTIAL_1;
                        }
                        if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x5d {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_46 XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x3e) {
                                ptr = ptr.offset(-(2isize));
                            } else {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_46 XML_TOK_CDATA_SECT_CLOSE_1;
                            }
                        }
                    }
                    BT_CR => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_46 XML_TOK_PARTIAL_1;
                        }
                        if (if *ptr.offset(0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        }) == BT_LF as c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_46 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_LF => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_46 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_46 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_46 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_46 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_46 XML_TOK_INVALID_1;
                    }
                    _ => {
                        ptr = ptr.offset(2isize);
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(4isize);
                        }
                        BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF | BT_RSQB => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
                        }
                        _ => {
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_46 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanEndTag(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_47: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_47 XML_TOK_PARTIAL_1;
                }
                let mut current_block_32: u64;
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_47 XML_TOK_INVALID_1;
                        }
                        current_block_32 = 12738221189273011712;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_32 = 12738221189273011712;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_47 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_47 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_47 XML_TOK_INVALID_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_47 XML_TOK_INVALID_1;
                    }
                }
                match current_block_32 {
                    12738221189273011712 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_73: u64;
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(0) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_47 XML_TOK_INVALID_1;
                            }
                            current_block_73 = 1281007054303163758;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_73 = 1281007054303163758;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_47 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_47 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_47 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_47 XML_TOK_INVALID_1;
                        }
                        BT_S | BT_CR | BT_LF => {
                            ptr = ptr.offset(2);
                            while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                                match if *ptr.offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                                } {
                                    BT_S | BT_CR | BT_LF => {}
                                    BT_GT => {
                                        *nextTokPtr = ptr.offset(2);
                                        break 'iife_ret_47 XML_TOK_END_TAG_1;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_47 XML_TOK_INVALID_1;
                                    }
                                }
                                ptr = ptr.offset(2);
                            }
                            break 'iife_ret_47 XML_TOK_PARTIAL_1;
                        }
                        BT_COLON_0 => {
                            ptr = ptr.offset(2);
                            current_block_73 = 981995395831942902;
                        }
                        BT_GT => {
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_47 XML_TOK_END_TAG_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_47 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_73 {
                        1281007054303163758 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_47 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanHexCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_48: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_DIGIT | BT_HEX => {}
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_48 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                        match if *ptr.offset(0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                        } {
                            BT_DIGIT | BT_HEX => {}
                            BT_SEMI => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_48 XML_TOK_CHAR_REF_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_48 XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(2);
                    }
                }
                break 'iife_ret_48 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanCharRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_49: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x78 {
                        break 'iife_ret_49 ({
                            let (tok_value, next_tok_value) = big2_scanHexCharRef(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_DIGIT => {}
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_49 XML_TOK_INVALID_1;
                        }
                    }
                    ptr = ptr.offset(2);
                    while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                        match if *ptr.offset(0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                        } {
                            BT_DIGIT => {}
                            BT_SEMI => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_49 XML_TOK_CHAR_REF_1;
                            }
                            _ => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_49 XML_TOK_INVALID_1;
                            }
                        }
                        ptr = ptr.offset(2);
                    }
                }
                break 'iife_ret_49 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanRef(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_50: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_50 XML_TOK_PARTIAL_1;
                }
                let mut current_block_33: u64;
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_50 XML_TOK_INVALID_1;
                        }
                        current_block_33 = 17794167657114565097;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_33 = 17794167657114565097;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_50 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_50 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_50 XML_TOK_INVALID_1;
                    }
                    BT_NUM => {
                        break 'iife_ret_50 ({
                            let (tok_value, next_tok_value) = big2_scanCharRef(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_50 XML_TOK_INVALID_1;
                    }
                }
                match current_block_33 {
                    17794167657114565097 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_64: u64;
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(0) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_50 XML_TOK_INVALID_1;
                            }
                            current_block_64 = 17251590314240005670;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_64 = 17251590314240005670;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_50 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_50 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_50 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_50 XML_TOK_INVALID_1;
                        }
                        BT_SEMI => {
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_50 XML_TOK_ENTITY_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_50 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_64 {
                        17251590314240005670 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_50 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanAtts(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_51: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut hadColon: c_int = 0;
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_186: u64;
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(0) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_51 XML_TOK_INVALID_1;
                            }
                            current_block_186 = 6092917267242331817;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_186 = 6092917267242331817;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_51 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_51 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_51 XML_TOK_INVALID_1;
                        }
                        BT_COLON_0 => {
                            if hadColon != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_51 XML_TOK_INVALID_1;
                            }
                            hadColon = 1;
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_51 XML_TOK_PARTIAL_1;
                            }
                            let mut current_block_64: u64;
                            match if *ptr.offset(0) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                            } {
                                BT_NONASCII => {
                                    if namingBitmap[(((nmstrtPages
                                        [*ptr.offset(0) as c_uchar as usize]
                                        as c_int)
                                        << 3)
                                        + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                        as usize]
                                        & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_51 XML_TOK_INVALID_1;
                                    }
                                    current_block_64 = 6604085902723260545;
                                }
                                BT_NMSTRT | BT_HEX => {
                                    current_block_64 = 6604085902723260545;
                                }
                                BT_LEAD2 => {
                                    if (end.offset_from(ptr) as c_long) < 2 {
                                        break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                }
                                BT_LEAD3 => {
                                    if (end.offset_from(ptr) as c_long) < 3 {
                                        break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                }
                                BT_LEAD4 => {
                                    if (end.offset_from(ptr) as c_long) < 4 {
                                        break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                }
                                _ => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
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
                        BT_S | BT_CR | BT_LF => {
                            loop {
                                let mut t: c_int = 0;
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_51 XML_TOK_PARTIAL_1;
                                }
                                t = if *ptr.offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                                };
                                if t == BT_EQUALS as c_int {
                                    break;
                                }
                                match t {
                                    21 | 10 | 9 => {}
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_51 XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            current_block_186 = 10853015579903106591;
                        }
                        BT_EQUALS => {
                            current_block_186 = 10853015579903106591;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_51 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_186 {
                        10853015579903106591 => {
                            let mut open: c_int = 0;
                            hadColon = 0;
                            loop {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_51 XML_TOK_PARTIAL_1;
                                }
                                open = if *ptr.offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                                };
                                if open == BT_QUOT as c_int || open == BT_APOS as c_int {
                                    break;
                                }
                                match open {
                                    21 | 10 | 9 => {}
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_51 XML_TOK_INVALID_1;
                                    }
                                }
                            }
                            ptr = ptr.offset(2);
                            loop {
                                let mut t_0: c_int = 0;
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_51 XML_TOK_PARTIAL_1;
                                }
                                t_0 = if *ptr.offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                                };
                                if t_0 == open {
                                    break;
                                }
                                match t_0 {
                                    5 => {
                                        if (end.offset_from(ptr) as c_long) < 2 {
                                            break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(2isize);
                                    }
                                    6 => {
                                        if (end.offset_from(ptr) as c_long) < 3 {
                                            break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(3isize);
                                    }
                                    7 => {
                                        if (end.offset_from(ptr) as c_long) < 4 {
                                            break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        ptr = ptr.offset(4isize);
                                    }
                                    0 | 1 | 8 => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_51 XML_TOK_INVALID_1;
                                    }
                                    3 => {
                                        let mut tok: c_int = {
                                            let (tok_value, next_tok_value) = big2_scanRef(
                                                enc,
                                                c_char_slice_from_ptr_end(ptr.offset(2), end),
                                            );
                                            ptr = next_tok_value;
                                            tok_value
                                        };
                                        if tok <= 0 {
                                            if tok == XML_TOK_INVALID_1 {
                                                *nextTokPtr = ptr;
                                            }
                                            break 'iife_ret_51 tok;
                                        }
                                    }
                                    2 => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_51 XML_TOK_INVALID_1;
                                    }
                                    _ => {
                                        ptr = ptr.offset(2isize);
                                    }
                                }
                            }
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_51 XML_TOK_PARTIAL_1;
                            }
                            match if *ptr.offset(0) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                            } {
                                BT_S | BT_CR | BT_LF => {
                                    loop {
                                        ptr = ptr.offset(2);
                                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long)
                                        {
                                            break 'iife_ret_51 XML_TOK_PARTIAL_1;
                                        }
                                        match if *ptr.offset(0) as c_int == 0 {
                                            as_normal_encoding(enc).type_0
                                                [*ptr.offset(1) as c_uchar as usize]
                                                as c_uint
                                        } else {
                                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                                                as c_uint
                                        } {
                                            BT_NONASCII => {
                                                if namingBitmap[(((nmstrtPages
                                                    [*ptr.offset(0) as c_uchar as usize]
                                                    as c_int)
                                                    << 3)
                                                    + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                                    as usize]
                                                    & (1)
                                                        << (*ptr.offset(1) as c_uchar as c_int
                                                            & 0x1f)
                                                    == 0
                                                {
                                                    *nextTokPtr = ptr;
                                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                                }
                                                current_block_186 = 7794494472231011433;
                                                break;
                                            }
                                            BT_NMSTRT | BT_HEX => {
                                                current_block_186 = 7794494472231011433;
                                                break;
                                            }
                                            BT_LEAD2 => {
                                                if (end.offset_from(ptr) as c_long) < 2 {
                                                    break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_51 XML_TOK_INVALID_1;
                                            }
                                            BT_LEAD3 => {
                                                if (end.offset_from(ptr) as c_long) < 3 {
                                                    break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_51 XML_TOK_INVALID_1;
                                            }
                                            BT_LEAD4 => {
                                                if (end.offset_from(ptr) as c_long) < 4 {
                                                    break 'iife_ret_51 XML_TOK_PARTIAL_CHAR_1;
                                                }
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_51 XML_TOK_INVALID_1;
                                            }
                                            BT_S | BT_CR | BT_LF => {}
                                            BT_GT => {
                                                current_block_186 = 1783713129665224809;
                                                break;
                                            }
                                            BT_SOL => {
                                                current_block_186 = 18153789983347219713;
                                                break;
                                            }
                                            _ => {
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_51 XML_TOK_INVALID_1;
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
                                BT_SOL => {
                                    current_block_186 = 18153789983347219713;
                                }
                                BT_GT => {
                                    current_block_186 = 1783713129665224809;
                                }
                                _ => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_51 XML_TOK_INVALID_1;
                                }
                            }
                            match current_block_186 {
                                1634947208139838470 => {}
                                _ => match current_block_186 {
                                    18153789983347219713 => {
                                        ptr = ptr.offset(2);
                                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long)
                                        {
                                            break 'iife_ret_51 XML_TOK_PARTIAL_1;
                                        }
                                        if !(*ptr.offset(0) as c_int == 0
                                            && *ptr.offset(1) as c_int == 0x3e)
                                        {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_51 XML_TOK_INVALID_1;
                                        }
                                        *nextTokPtr = ptr.offset(2);
                                        break 'iife_ret_51 XML_TOK_EMPTY_ELEMENT_WITH_ATTS_1;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr.offset(2);
                                        break 'iife_ret_51 XML_TOK_START_TAG_WITH_ATTS_1;
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
                break 'iife_ret_51 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanLt(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_52: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut hadColon: c_int = 0;
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_52 XML_TOK_PARTIAL_1;
                }
                let mut current_block_45: u64;
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_52 XML_TOK_INVALID_1;
                        }
                        current_block_45 = 6477200489819026004;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_45 = 6477200489819026004;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                    BT_EXCL => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_52 XML_TOK_PARTIAL_1;
                        }
                        match if *ptr.offset(0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                        } {
                            BT_MINUS => {
                                break 'iife_ret_52 ({
                                    let (tok_value, next_tok_value) = big2_scanComment(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            BT_LSQB => {
                                break 'iife_ret_52 ({
                                    let (tok_value, next_tok_value) = big2_scanCdataSection(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            _ => {}
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                    BT_QUEST => {
                        break 'iife_ret_52 ({
                            let (tok_value, next_tok_value) = big2_scanPi(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_SOL => {
                        break 'iife_ret_52 ({
                            let (tok_value, next_tok_value) = big2_scanEndTag(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_52 XML_TOK_INVALID_1;
                    }
                }
                match current_block_45 {
                    6477200489819026004 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                hadColon = 0;
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_161: u64;
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(0) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_52 XML_TOK_INVALID_1;
                            }
                            current_block_161 = 18151815167355992796;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_161 = 18151815167355992796;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_52 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_52 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_52 XML_TOK_INVALID_1;
                        }
                        BT_COLON_0 => {
                            if hadColon != 0 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_52 XML_TOK_INVALID_1;
                            }
                            hadColon = 1;
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_52 XML_TOK_PARTIAL_1;
                            }
                            let mut current_block_112: u64;
                            match if *ptr.offset(0) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                            } {
                                BT_NONASCII => {
                                    if namingBitmap[(((nmstrtPages
                                        [*ptr.offset(0) as c_uchar as usize]
                                        as c_int)
                                        << 3)
                                        + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                        as usize]
                                        & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                        == 0
                                    {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_52 XML_TOK_INVALID_1;
                                    }
                                    current_block_112 = 16337619596932156899;
                                }
                                BT_NMSTRT | BT_HEX => {
                                    current_block_112 = 16337619596932156899;
                                }
                                BT_LEAD2 => {
                                    if (end.offset_from(ptr) as c_long) < 2 {
                                        break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_52 XML_TOK_INVALID_1;
                                }
                                BT_LEAD3 => {
                                    if (end.offset_from(ptr) as c_long) < 3 {
                                        break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_52 XML_TOK_INVALID_1;
                                }
                                BT_LEAD4 => {
                                    if (end.offset_from(ptr) as c_long) < 4 {
                                        break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                    }
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_52 XML_TOK_INVALID_1;
                                }
                                _ => {
                                    *nextTokPtr = ptr;
                                    break 'iife_ret_52 XML_TOK_INVALID_1;
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
                        BT_S | BT_CR | BT_LF => {
                            ptr = ptr.offset(2);
                            loop {
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    current_block_161 = 13215501469961642988;
                                    break;
                                }
                                match if *ptr.offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                                } {
                                    BT_NONASCII => {
                                        if namingBitmap[(((nmstrtPages
                                            [*ptr.offset(0) as c_uchar as usize]
                                            as c_int)
                                            << 3)
                                            + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                            as usize]
                                            & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                            == 0
                                        {
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_52 XML_TOK_INVALID_1;
                                        }
                                        current_block_161 = 11066148936714919733;
                                    }
                                    BT_NMSTRT | BT_HEX => {
                                        current_block_161 = 11066148936714919733;
                                    }
                                    BT_LEAD2 => {
                                        if (end.offset_from(ptr) as c_long) < 2 {
                                            break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_52 XML_TOK_INVALID_1;
                                    }
                                    BT_LEAD3 => {
                                        if (end.offset_from(ptr) as c_long) < 3 {
                                            break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_52 XML_TOK_INVALID_1;
                                    }
                                    BT_LEAD4 => {
                                        if (end.offset_from(ptr) as c_long) < 4 {
                                            break 'iife_ret_52 XML_TOK_PARTIAL_CHAR_1;
                                        }
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_52 XML_TOK_INVALID_1;
                                    }
                                    BT_GT => {
                                        current_block_161 = 13089361350718158941;
                                        break;
                                    }
                                    BT_SOL => {
                                        current_block_161 = 11384015785330443424;
                                        break;
                                    }
                                    BT_S | BT_CR | BT_LF => {
                                        ptr = ptr.offset(2);
                                        continue;
                                    }
                                    _ => {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_52 XML_TOK_INVALID_1;
                                    }
                                }
                                match current_block_161 {
                                    11066148936714919733 => {
                                        ptr = ptr.offset(2isize);
                                    }
                                    _ => {}
                                }
                                break 'iife_ret_52 ({
                                    let (tok_value, next_tok_value) =
                                        big2_scanAtts(enc, c_char_slice_from_ptr_end(ptr, end));
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            match current_block_161 {
                                13089361350718158941 => {}
                                11384015785330443424 => {}
                                _ => break 'iife_ret_52 XML_TOK_PARTIAL_1,
                            }
                        }
                        BT_GT => {
                            current_block_161 = 13089361350718158941;
                        }
                        BT_SOL => {
                            current_block_161 = 11384015785330443424;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_52 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_161 {
                        11384015785330443424 => {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_52 XML_TOK_PARTIAL_1;
                            }
                            if !(*ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x3e) {
                                *nextTokPtr = ptr;
                                break 'iife_ret_52 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_52 XML_TOK_EMPTY_ELEMENT_NO_ATTS_1;
                        }
                        13089361350718158941 => {
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_52 XML_TOK_START_TAG_NO_ATTS_1;
                        }
                        18151815167355992796 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_52 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_contentTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_53: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if ptr >= end {
                    break 'iife_ret_53 XML_TOK_NONE_1;
                }
                {
                    let mut n: size_t = end.offset_from(ptr) as size_t;
                    if n & (2i32 - 1) as size_t != 0 {
                        n &= !(2i32 - 1) as size_t;
                        if n == 0 {
                            break 'iife_ret_53 XML_TOK_PARTIAL_1;
                        }
                        end = ptr.offset(n as isize);
                    }
                }
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_LT => {
                        break 'iife_ret_53 ({
                            let (tok_value, next_tok_value) = big2_scanLt(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_AMP => {
                        break 'iife_ret_53 ({
                            let (tok_value, next_tok_value) = big2_scanRef(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_CR => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_53 XML_TOK_TRAILING_CR_1;
                        }
                        if (if *ptr.offset(0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                as c_int
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                        }) == BT_LF as c_int
                        {
                            ptr = ptr.offset(2isize);
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_53 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_LF => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_53 XML_TOK_DATA_NEWLINE_1;
                    }
                    BT_RSQB => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_53 XML_TOK_TRAILING_RSQB_1;
                        }
                        if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x5d {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_53 XML_TOK_TRAILING_RSQB_1;
                            }
                            if !(*ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x3e) {
                                ptr = ptr.offset(-(2isize));
                            } else {
                                *nextTokPtr = ptr;
                                break 'iife_ret_53 XML_TOK_INVALID_1;
                            }
                        }
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_53 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(2isize);
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_53 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(3isize);
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_53 XML_TOK_PARTIAL_CHAR_1;
                        }
                        ptr = ptr.offset(4isize);
                    }
                    BT_NONXML | BT_MALFORM | BT_TRAIL => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_53 XML_TOK_INVALID_1;
                    }
                    _ => {
                        ptr = ptr.offset(2isize);
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_76: u64;
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(2);
                            current_block_76 = 7158658067966855297;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(3);
                            current_block_76 = 7158658067966855297;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
                            }
                            ptr = ptr.offset(4);
                            current_block_76 = 7158658067966855297;
                        }
                        BT_RSQB => {
                            if end.offset_from(ptr) as c_long >= (2i32 * 2) as c_long {
                                if !(*ptr.offset(2).offset(0) as c_int == 0
                                    && *ptr.offset(2).offset(1) as c_int == 0x5d)
                                {
                                    ptr = ptr.offset(2);
                                    current_block_76 = 7158658067966855297;
                                } else if end.offset_from(ptr) as c_long >= (3i32 * 2) as c_long {
                                    if !(*ptr.offset((2i32 * 2) as isize).offset(0) as c_int == 0
                                        && *ptr.offset((2i32 * 2) as isize).offset(1) as c_int
                                            == 0x3e)
                                    {
                                        ptr = ptr.offset(2isize);
                                    } else {
                                        *nextTokPtr = ptr.offset((2i32 * 2) as isize);
                                        break 'iife_ret_53 XML_TOK_INVALID_1;
                                    }
                                    current_block_76 = 7158658067966855297;
                                } else {
                                    current_block_76 = 11890188771060868767;
                                }
                            } else {
                                current_block_76 = 11890188771060868767;
                            }
                        }
                        BT_AMP | BT_LT | BT_NONXML | BT_MALFORM | BT_TRAIL | BT_CR | BT_LF => {
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
                            break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_53 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanPercent(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_54: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_54 XML_TOK_PARTIAL_1;
                }
                let mut current_block_34: u64;
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_54 XML_TOK_INVALID_1;
                        }
                        current_block_34 = 9652455934050855438;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_34 = 9652455934050855438;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_54 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_54 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_54 XML_TOK_INVALID_1;
                    }
                    BT_S | BT_LF | BT_CR | BT_PERCNT => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_54 XML_TOK_PERCENT_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_54 XML_TOK_INVALID_1;
                    }
                }
                match current_block_34 {
                    9652455934050855438 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_65: u64;
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(0) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_54 XML_TOK_INVALID_1;
                            }
                            current_block_65 = 3947837075391501242;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_65 = 3947837075391501242;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_54 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_54 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_54 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_54 XML_TOK_INVALID_1;
                        }
                        BT_SEMI => {
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_54 XML_TOK_PARAM_ENTITY_REF_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_54 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_65 {
                        3947837075391501242 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_54 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanPoundName(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_55: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_55 XML_TOK_PARTIAL_1;
                }
                let mut current_block_32: u64;
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                            == 0
                        {
                            *nextTokPtr = ptr;
                            break 'iife_ret_55 XML_TOK_INVALID_1;
                        }
                        current_block_32 = 12219479933348349998;
                    }
                    BT_NMSTRT | BT_HEX => {
                        current_block_32 = 12219479933348349998;
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_55 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_55 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_55 XML_TOK_INVALID_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_55 XML_TOK_INVALID_1;
                    }
                }
                match current_block_32 {
                    12219479933348349998 => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {}
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_63: u64;
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(0) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_55 XML_TOK_INVALID_1;
                            }
                            current_block_63 = 1647491770914889697;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_63 = 1647491770914889697;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_55 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_55 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_55 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_55 XML_TOK_INVALID_1;
                        }
                        BT_CR | BT_LF | BT_S | BT_RPAR | BT_GT | BT_PERCNT | BT_VERBAR => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_55 XML_TOK_POUND_NAME_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_55 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_63 {
                        1647491770914889697 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_55 -XML_TOK_POUND_NAME_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_scanLit(mut open: c_int, enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_56: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut t: c_int = if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_int
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                    };
                    match t {
                        5 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_56 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(2isize);
                        }
                        6 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_56 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(3isize);
                        }
                        7 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_56 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(4isize);
                        }
                        0 | 1 | 8 => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_56 XML_TOK_INVALID_1;
                        }
                        12 | 13 => {
                            ptr = ptr.offset(2);
                            if !(t != open) {
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_56 -XML_TOK_LITERAL_1;
                                }
                                *nextTokPtr = ptr;
                                match if *ptr.offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_uint
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                                } {
                                    BT_S | BT_CR | BT_LF | BT_GT | BT_PERCNT | BT_LSQB => {
                                        break 'iife_ret_56 XML_TOK_LITERAL_1
                                    }
                                    _ => break 'iife_ret_56 XML_TOK_INVALID_1,
                                }
                            }
                        }
                        _ => {
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                break 'iife_ret_56 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_prologTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_57: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut tok: c_int = 0;
                if ptr >= end {
                    break 'iife_ret_57 XML_TOK_NONE_1;
                }
                {
                    let mut n: size_t = end.offset_from(ptr) as size_t;
                    if n & (2i32 - 1) as size_t != 0 {
                        n &= !(2i32 - 1) as size_t;
                        if n == 0 {
                            break 'iife_ret_57 XML_TOK_PARTIAL_1;
                        }
                        end = ptr.offset(n as isize);
                    }
                }
                let mut current_block_124: u64;
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_QUOT => {
                        break 'iife_ret_57 ({
                            let (tok_value, next_tok_value) = big2_scanLit(
                                BT_QUOT as c_int,
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_APOS => {
                        break 'iife_ret_57 ({
                            let (tok_value, next_tok_value) = big2_scanLit(
                                BT_APOS as c_int,
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_LT => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_57 XML_TOK_PARTIAL_1;
                        }
                        match if *ptr.offset(0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                        } {
                            BT_EXCL => {
                                break 'iife_ret_57 ({
                                    let (tok_value, next_tok_value) = big2_scanDecl(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            BT_QUEST => {
                                break 'iife_ret_57 ({
                                    let (tok_value, next_tok_value) = big2_scanPi(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            BT_NMSTRT | BT_HEX | BT_NONASCII | BT_LEAD2 | BT_LEAD3 | BT_LEAD4 => {
                                *nextTokPtr = ptr.offset(-(2));
                                break 'iife_ret_57 XML_TOK_INSTANCE_START;
                            }
                            _ => {}
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                    BT_CR => {
                        if ptr.offset(2) == end {
                            *nextTokPtr = end;
                            break 'iife_ret_57 -XML_TOK_PROLOG_S_1;
                        }
                        current_block_124 = 16869865525854146339;
                    }
                    BT_S | BT_LF => {
                        current_block_124 = 16869865525854146339;
                    }
                    BT_PERCNT => {
                        break 'iife_ret_57 ({
                            let (tok_value, next_tok_value) = big2_scanPercent(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_COMMA => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_57 XML_TOK_COMMA_1;
                    }
                    BT_LSQB => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_57 XML_TOK_OPEN_BRACKET_1;
                    }
                    BT_RSQB => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_57 -XML_TOK_CLOSE_BRACKET_1;
                        }
                        if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x5d {
                            if !(end.offset_from(ptr) as c_long >= (2i32 * 2) as c_long) {
                                break 'iife_ret_57 XML_TOK_PARTIAL_1;
                            }
                            if *ptr.offset(2).offset(0) as c_int == 0
                                && *ptr.offset(2).offset(1) as c_int == 0x3e
                            {
                                *nextTokPtr = ptr.offset((2i32 * 2) as isize);
                                break 'iife_ret_57 XML_TOK_COND_SECT_CLOSE_1;
                            }
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_57 XML_TOK_CLOSE_BRACKET_1;
                    }
                    BT_LPAR => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_57 XML_TOK_OPEN_PAREN_1;
                    }
                    BT_RPAR => {
                        ptr = ptr.offset(2);
                        if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                            break 'iife_ret_57 -XML_TOK_CLOSE_PAREN_1;
                        }
                        match if *ptr.offset(0) as c_int == 0 {
                            as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                as c_uint
                        } else {
                            unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                        } {
                            BT_AST => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_57 XML_TOK_CLOSE_PAREN_ASTERISK_1;
                            }
                            BT_QUEST => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_57 XML_TOK_CLOSE_PAREN_QUESTION_1;
                            }
                            BT_PLUS => {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_57 XML_TOK_CLOSE_PAREN_PLUS_1;
                            }
                            BT_CR | BT_LF | BT_S | BT_GT | BT_COMMA | BT_VERBAR | BT_RPAR => {
                                *nextTokPtr = ptr;
                                break 'iife_ret_57 XML_TOK_CLOSE_PAREN_1;
                            }
                            _ => {}
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                    BT_VERBAR => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_57 XML_TOK_OR_1;
                    }
                    BT_GT => {
                        *nextTokPtr = ptr.offset(2);
                        break 'iife_ret_57 XML_TOK_DECL_CLOSE_1;
                    }
                    BT_NUM => {
                        break 'iife_ret_57 ({
                            let (tok_value, next_tok_value) = big2_scanPoundName(
                                enc,
                                c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                            );
                            *nextTokPtr = next_tok_value;
                            tok_value
                        });
                    }
                    BT_LEAD2 => {
                        if (end.offset_from(ptr) as c_long) < 2 {
                            break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                    BT_LEAD3 => {
                        if (end.offset_from(ptr) as c_long) < 3 {
                            break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                    BT_LEAD4 => {
                        if (end.offset_from(ptr) as c_long) < 4 {
                            break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                    BT_NMSTRT | BT_HEX => {
                        tok = XML_TOK_NAME;
                        ptr = ptr.offset(2);
                        current_block_124 = 2956972668325154207;
                    }
                    BT_DIGIT | BT_NAME | BT_MINUS | BT_COLON_0 => {
                        tok = XML_TOK_NMTOKEN_1;
                        ptr = ptr.offset(2);
                        current_block_124 = 2956972668325154207;
                    }
                    BT_NONASCII => {
                        if namingBitmap[(((nmstrtPages[*ptr.offset(0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                            != 0
                        {
                            ptr = ptr.offset(2);
                            tok = XML_TOK_NAME;
                            current_block_124 = 2956972668325154207;
                        } else if namingBitmap[(((namePages[*ptr.offset(0) as c_uchar as usize]
                            as c_int)
                            << 3)
                            + (*ptr.offset(1) as c_uchar as c_int >> 5))
                            as usize]
                            & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                            != 0
                        {
                            ptr = ptr.offset(2);
                            tok = XML_TOK_NMTOKEN_1;
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
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break;
                            }
                            let mut current_block_32: u64;
                            match if *ptr.offset(0) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                    as c_uint
                            } else {
                                unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                            } {
                                BT_S | BT_LF => {
                                    current_block_32 = 17500079516916021833;
                                }
                                BT_CR => {
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
                                    break 'iife_ret_57 XML_TOK_PROLOG_S_1;
                                }
                            }
                        }
                        *nextTokPtr = ptr;
                        break 'iife_ret_57 XML_TOK_PROLOG_S_1;
                    }
                    _ => {
                        *nextTokPtr = ptr;
                        break 'iife_ret_57 XML_TOK_INVALID_1;
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    let mut current_block_210: u64;
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_NONASCII => {
                            if namingBitmap[(((namePages[*ptr.offset(0) as c_uchar as usize]
                                as c_int)
                                << 3)
                                + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                as usize]
                                & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                == 0
                            {
                                *nextTokPtr = ptr;
                                break 'iife_ret_57 XML_TOK_INVALID_1;
                            }
                            current_block_210 = 9794574411605359176;
                        }
                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                            current_block_210 = 9794574411605359176;
                        }
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_57 XML_TOK_INVALID_1;
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_57 XML_TOK_INVALID_1;
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_57 XML_TOK_INVALID_1;
                        }
                        BT_GT | BT_RPAR | BT_COMMA | BT_VERBAR | BT_LSQB | BT_PERCNT | BT_S
                        | BT_CR | BT_LF => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_57 tok;
                        }
                        BT_COLON_0 => {
                            ptr = ptr.offset(2);
                            match tok {
                                XML_TOK_NAME => {
                                    if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                        break 'iife_ret_57 XML_TOK_PARTIAL_1;
                                    }
                                    tok = XML_TOK_PREFIXED_NAME;
                                    let mut current_block_187: u64;
                                    match if *ptr.offset(0) as c_int == 0 {
                                        as_normal_encoding(enc).type_0
                                            [*ptr.offset(1) as c_uchar as usize]
                                            as c_uint
                                    } else {
                                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                                    } {
                                        BT_NONASCII => {
                                            if namingBitmap[(((namePages
                                                [*ptr.offset(0) as c_uchar as usize]
                                                as c_int)
                                                << 3)
                                                + (*ptr.offset(1) as c_uchar as c_int >> 5))
                                                as usize]
                                                & (1) << (*ptr.offset(1) as c_uchar as c_int & 0x1f)
                                                == 0
                                            {
                                                *nextTokPtr = ptr;
                                                break 'iife_ret_57 XML_TOK_INVALID_1;
                                            }
                                            current_block_187 = 17275381528970576968;
                                        }
                                        BT_NMSTRT | BT_HEX | BT_DIGIT | BT_NAME | BT_MINUS => {
                                            current_block_187 = 17275381528970576968;
                                        }
                                        BT_LEAD2 => {
                                            if (end.offset_from(ptr) as c_long) < 2 {
                                                break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_57 XML_TOK_INVALID_1;
                                        }
                                        BT_LEAD3 => {
                                            if (end.offset_from(ptr) as c_long) < 3 {
                                                break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_57 XML_TOK_INVALID_1;
                                        }
                                        BT_LEAD4 => {
                                            if (end.offset_from(ptr) as c_long) < 4 {
                                                break 'iife_ret_57 XML_TOK_PARTIAL_CHAR_1;
                                            }
                                            *nextTokPtr = ptr;
                                            break 'iife_ret_57 XML_TOK_INVALID_1;
                                        }
                                        _ => {
                                            tok = XML_TOK_NMTOKEN_1;
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
                                XML_TOK_PREFIXED_NAME => {
                                    tok = XML_TOK_NMTOKEN_1;
                                }
                                _ => {}
                            }
                            current_block_210 = 14244298717249035578;
                        }
                        BT_PLUS => {
                            if tok == XML_TOK_NMTOKEN_1 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_57 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_57 XML_TOK_NAME_PLUS_1;
                        }
                        BT_AST => {
                            if tok == XML_TOK_NMTOKEN_1 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_57 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_57 XML_TOK_NAME_ASTERISK_1;
                        }
                        BT_QUEST => {
                            if tok == XML_TOK_NMTOKEN_1 {
                                *nextTokPtr = ptr;
                                break 'iife_ret_57 XML_TOK_INVALID_1;
                            }
                            *nextTokPtr = ptr.offset(2);
                            break 'iife_ret_57 XML_TOK_NAME_QUESTION_1;
                        }
                        _ => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_57 XML_TOK_INVALID_1;
                        }
                    }
                    match current_block_210 {
                        9794574411605359176 => {
                            ptr = ptr.offset(2isize);
                        }
                        _ => {}
                    }
                }
                break 'iife_ret_57 -tok;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_attributeValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_58: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut start: *const c_char = null::<c_char>();
                if ptr >= end {
                    break 'iife_ret_58 XML_TOK_NONE_1;
                } else if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_58 XML_TOK_PARTIAL_1;
                }
                start = ptr;
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_LEAD2 => {
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            ptr = ptr.offset(4isize);
                        }
                        BT_AMP => {
                            if ptr == start {
                                break 'iife_ret_58 ({
                                    let (tok_value, next_tok_value) = big2_scanRef(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
                        }
                        BT_LT => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_58 XML_TOK_INVALID_1;
                        }
                        BT_LF => {
                            if ptr == start {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_58 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
                        }
                        BT_CR => {
                            if ptr == start {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_58 XML_TOK_TRAILING_CR_1;
                                }
                                if (if *ptr.offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                                }) == BT_LF as c_int
                                {
                                    ptr = ptr.offset(2isize);
                                }
                                *nextTokPtr = ptr;
                                break 'iife_ret_58 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
                        }
                        BT_S => {
                            if ptr == start {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_58 XML_TOK_ATTRIBUTE_VALUE_S_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
                        }
                        _ => {
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_58 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_entityValueTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_59: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut start: *const c_char = null::<c_char>();
                if ptr >= end {
                    break 'iife_ret_59 XML_TOK_NONE_1;
                } else if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                    break 'iife_ret_59 XML_TOK_PARTIAL_1;
                }
                start = ptr;
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_LEAD2 => {
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            ptr = ptr.offset(4isize);
                        }
                        BT_AMP => {
                            if ptr == start {
                                break 'iife_ret_59 ({
                                    let (tok_value, next_tok_value) = big2_scanRef(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2isize), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
                        }
                        BT_PERCNT => {
                            if ptr == start {
                                let mut tok: c_int = {
                                    let (tok_value, next_tok_value) = big2_scanPercent(
                                        enc,
                                        c_char_slice_from_ptr_end(ptr.offset(2), end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                };
                                break 'iife_ret_59 if tok == XML_TOK_PERCENT_1 {
                                    XML_TOK_INVALID_1
                                } else {
                                    tok
                                };
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
                        }
                        BT_LF => {
                            if ptr == start {
                                *nextTokPtr = ptr.offset(2);
                                break 'iife_ret_59 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
                        }
                        BT_CR => {
                            if ptr == start {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_59 XML_TOK_TRAILING_CR_1;
                                }
                                if (if *ptr.offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(1) as c_uchar as usize]
                                        as c_int
                                } else {
                                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                                }) == BT_LF as c_int
                                {
                                    ptr = ptr.offset(2isize);
                                }
                                *nextTokPtr = ptr;
                                break 'iife_ret_59 XML_TOK_DATA_NEWLINE_1;
                            }
                            *nextTokPtr = ptr;
                            break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
                        }
                        _ => {
                            ptr = ptr.offset(2isize);
                        }
                    }
                }
                *nextTokPtr = ptr;
                break 'iife_ret_59 XML_TOK_DATA_CHARS_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_ignoreSectionTok(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        unsafe {
            let mut next_tok: *const c_char = input.as_ptr();
            let nextTokPtr = &mut next_tok;
            let tok: c_int = 'iife_ret_60: {
                let mut ptr = input.as_ptr();
                let mut end = ptr.add(input.len());
                let mut level: c_int = 0;
                {
                    let mut n: size_t = end.offset_from(ptr) as size_t;
                    if n & (2i32 - 1) as size_t != 0 {
                        n &= !(2i32 - 1) as size_t;
                        end = ptr.offset(n as isize);
                    }
                }
                while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                    match if *ptr.offset(0) as c_int == 0 {
                        as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                    } else {
                        unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                    } {
                        BT_LEAD2 => {
                            if (end.offset_from(ptr) as c_long) < 2 {
                                break 'iife_ret_60 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(2isize);
                        }
                        BT_LEAD3 => {
                            if (end.offset_from(ptr) as c_long) < 3 {
                                break 'iife_ret_60 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(3isize);
                        }
                        BT_LEAD4 => {
                            if (end.offset_from(ptr) as c_long) < 4 {
                                break 'iife_ret_60 XML_TOK_PARTIAL_CHAR_1;
                            }
                            ptr = ptr.offset(4isize);
                        }
                        BT_NONXML | BT_MALFORM | BT_TRAIL => {
                            *nextTokPtr = ptr;
                            break 'iife_ret_60 XML_TOK_INVALID_1;
                        }
                        BT_LT => {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_60 XML_TOK_PARTIAL_1;
                            }
                            if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x21 {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_60 XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x5b {
                                    level += 1;
                                    ptr = ptr.offset(2isize);
                                }
                            }
                        }
                        BT_RSQB => {
                            ptr = ptr.offset(2);
                            if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                break 'iife_ret_60 XML_TOK_PARTIAL_1;
                            }
                            if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x5d {
                                ptr = ptr.offset(2);
                                if !(end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long) {
                                    break 'iife_ret_60 XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x3e {
                                    ptr = ptr.offset(2);
                                    if level == 0 {
                                        *nextTokPtr = ptr;
                                        break 'iife_ret_60 XML_TOK_IGNORE_SECT_1;
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
                break 'iife_ret_60 XML_TOK_PARTIAL_1;
            };
            return (tok, next_tok);
        }
    }

    pub(crate) fn big2_isPublicId(enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        unsafe {
            let mut badPtrVal: *const c_char = null::<c_char>();
            let badPtr = &mut badPtrVal;
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            ptr = ptr.offset(2);
            end = end.offset(-(2));
            while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                let mut current_block_8: u64;
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    25 | 24 | 27 | 13 | 31 | 32 | 34 | 35 | 17 | 14 | 15 | 9 | 10 | 18 | 16
                    | 33 | 30 | 19 | 23 => {
                        current_block_8 = 5143058163439228106;
                    }
                    BT_S => {
                        if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x9 {
                            *badPtr = ptr;
                            return (0i32, badPtrVal);
                        }
                        current_block_8 = 5143058163439228106;
                    }
                    BT_NAME | BT_NMSTRT => {
                        if (if *ptr.offset(0) as c_int == 0 {
                            *ptr.offset(1) as c_int
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
                        match if *ptr.offset(0) as c_int == 0 {
                            *ptr.offset(1) as c_int
                        } else {
                            -(1)
                        } {
                            36 | 64 => {}
                            _ => {
                                *badPtr = ptr;
                                return (0i32, badPtrVal);
                            }
                        }
                    }
                    _ => {}
                }
                ptr = ptr.offset(2);
            }
            return (1, badPtrVal);
        }
    }

    pub(crate) fn big2_getAtts(
        enc: &ENCODING,
        mut ptr: *const c_char,
        mut attsMax: c_int,
        mut atts: *mut ATTRIBUTE,
    ) -> c_int {
        unsafe {
            let mut state: C2RustUnnamed_3 = inName_1;
            let mut nAtts: c_int = 0;
            let mut open: c_int = 0;
            ptr = ptr.offset(2);
            loop {
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_LEAD2 => {
                        if state == other_1 {
                            if nAtts < attsMax {
                                let ref mut fresh48 = (*atts.offset(nAtts as isize)).name;
                                *fresh48 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName_1;
                        }
                        ptr = ptr.offset((2i32 - 2i32) as isize);
                    }
                    BT_LEAD3 => {
                        if state == other_1 {
                            if nAtts < attsMax {
                                let ref mut fresh49 = (*atts.offset(nAtts as isize)).name;
                                *fresh49 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName_1;
                        }
                        ptr = ptr.offset((3i32 - 2i32) as isize);
                    }
                    BT_LEAD4 => {
                        if state == other_1 {
                            if nAtts < attsMax {
                                let ref mut fresh50 = (*atts.offset(nAtts as isize)).name;
                                *fresh50 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName_1;
                        }
                        ptr = ptr.offset((4i32 - 2i32) as isize);
                    }
                    BT_NONASCII | BT_NMSTRT | BT_HEX => {
                        if state == other_1 {
                            if nAtts < attsMax {
                                let ref mut fresh51 = (*atts.offset(nAtts as isize)).name;
                                *fresh51 = ptr;
                                (*atts.offset(nAtts as isize)).normalized = 1i8;
                            }
                            state = inName_1;
                        }
                    }
                    BT_QUOT => {
                        if state != inValue_1 {
                            if nAtts < attsMax {
                                let ref mut fresh52 = (*atts.offset(nAtts as isize)).valuePtr;
                                *fresh52 = ptr.offset(2isize);
                            }
                            state = inValue_1;
                            open = BT_QUOT as c_int;
                        } else if open == BT_QUOT as c_int {
                            state = other_1;
                            if nAtts < attsMax {
                                let ref mut fresh53 = (*atts.offset(nAtts as isize)).valueEnd;
                                *fresh53 = ptr;
                            }
                            nAtts += 1;
                        }
                    }
                    BT_APOS => {
                        if state != inValue_1 {
                            if nAtts < attsMax {
                                let ref mut fresh54 = (*atts.offset(nAtts as isize)).valuePtr;
                                *fresh54 = ptr.offset(2isize);
                            }
                            state = inValue_1;
                            open = BT_APOS as c_int;
                        } else if open == BT_APOS as c_int {
                            state = other_1;
                            if nAtts < attsMax {
                                let ref mut fresh55 = (*atts.offset(nAtts as isize)).valueEnd;
                                *fresh55 = ptr;
                            }
                            nAtts += 1;
                        }
                    }
                    BT_AMP => {
                        if nAtts < attsMax {
                            (*atts.offset(nAtts as isize)).normalized = 0i8;
                        }
                    }
                    BT_S => {
                        if state == inName_1 {
                            state = other_1;
                        } else if state == inValue_1
                            && nAtts < attsMax
                            && (*atts.offset(nAtts as isize)).normalized as c_int != 0
                            && (ptr == (*atts.offset(nAtts as isize)).valuePtr
                                || (if *ptr.offset(0) as c_int == 0 {
                                    *ptr.offset(1) as c_int
                                } else {
                                    -(1)
                                }) != ASCII_SPACE
                                || (if *ptr.offset(2).offset(0) as c_int == 0 {
                                    *ptr.offset(2).offset(1) as c_int
                                } else {
                                    -(1)
                                }) == ASCII_SPACE
                                || (if *ptr.offset(2).offset(0) as c_int == 0 {
                                    as_normal_encoding(enc).type_0
                                        [*ptr.offset(2).offset(1) as c_uchar as usize]
                                        as c_int
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
                    BT_CR | BT_LF => {
                        if state == inName_1 {
                            state = other_1;
                        } else if state == inValue_1 && nAtts < attsMax {
                            (*atts.offset(nAtts as isize)).normalized = 0i8;
                        }
                    }
                    BT_GT | BT_SOL => {
                        if state != inValue_1 {
                            return nAtts;
                        }
                    }
                    _ => {}
                }
                ptr = ptr.offset(2);
            }
        }
    }

    pub(crate) fn big2_charRefNumber(_enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        unsafe {
            let mut result: c_int = 0;
            ptr = ptr.offset((2i32 * 2) as isize);
            if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x78 {
                ptr = ptr.offset(2);
                while !(*ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x3b) {
                    let mut c: c_int = if *ptr.offset(0) as c_int == 0 {
                        *ptr.offset(1) as c_int
                    } else {
                        -(1)
                    };
                    match c {
                        ASCII_0 | ASCII_1_1 | ASCII_2_1 | ASCII_3_1 | ASCII_4 | ASCII_5
                        | ASCII_6 | ASCII_7 | ASCII_8_1 | ASCII_9_1 => {
                            result <<= 4;
                            result |= c - ASCII_0;
                        }
                        ASCII_A | ASCII_B_1 | ASCII_C | ASCII_D | ASCII_E_1 | ASCII_F_1 => {
                            result <<= 4;
                            result += 10i32 + (c - ASCII_A);
                        }
                        ASCII_a_1 | ASCII_b | ASCII_c_1 | ASCII_d | ASCII_e_1 | ASCII_f => {
                            result <<= 4;
                            result += 10i32 + (c - ASCII_a_1);
                        }
                        _ => {}
                    }
                    if result >= 0x110000 {
                        return -(1i32);
                    }
                    ptr = ptr.offset(2);
                }
            } else {
                while !(*ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x3b) {
                    let mut c_0: c_int = if *ptr.offset(0) as c_int == 0 {
                        *ptr.offset(1) as c_int
                    } else {
                        -(1)
                    };
                    result *= 10;
                    result += c_0 - ASCII_0;
                    if result >= 0x110000 {
                        return -(1i32);
                    }
                    ptr = ptr.offset(2);
                }
            }
            return checkCharRefNumber(result);
        }
    }

    pub(crate) fn big2_predefinedEntityName(_enc: &ENCODING, input: &[c_char]) -> c_int {
        unsafe {
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            match end.offset_from(ptr) as c_long / 2 {
                2 => {
                    if *ptr.offset(2).offset(0) as c_int == 0
                        && *ptr.offset(2).offset(1) as c_int == 0x74
                    {
                        match if *ptr.offset(0) as c_int == 0 {
                            *ptr.offset(1) as c_int
                        } else {
                            -(1)
                        } {
                            ASCII_l_1 => return ASCII_LT,
                            ASCII_g_1 => return ASCII_GT,
                            _ => {}
                        }
                    }
                }
                3 => {
                    if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x61 {
                        ptr = ptr.offset(2);
                        if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x6d {
                            ptr = ptr.offset(2);
                            if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x70 {
                                return ASCII_AMP;
                            }
                        }
                    }
                }
                4 => {
                    match if *ptr.offset(0) as c_int == 0 {
                        *ptr.offset(1) as c_int
                    } else {
                        -(1)
                    } {
                        ASCII_q => {
                            ptr = ptr.offset(2);
                            if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x75 {
                                ptr = ptr.offset(2);
                                if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x6f {
                                    ptr = ptr.offset(2);
                                    if *ptr.offset(0) as c_int == 0
                                        && *ptr.offset(1) as c_int == 0x74
                                    {
                                        return ASCII_QUOT;
                                    }
                                }
                            }
                        }
                        ASCII_a_1 => {
                            ptr = ptr.offset(2);
                            if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x70 {
                                ptr = ptr.offset(2);
                                if *ptr.offset(0) as c_int == 0 && *ptr.offset(1) as c_int == 0x6f {
                                    ptr = ptr.offset(2);
                                    if *ptr.offset(0) as c_int == 0
                                        && *ptr.offset(1) as c_int == 0x73
                                    {
                                        return ASCII_APOS;
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
    }

    pub(crate) fn big2_nameMatchesAscii(
        _enc: &ENCODING,
        input: &[c_char],
        mut ptr2: *const c_char,
    ) -> c_int {
        unsafe {
            let mut ptr1 = input.as_ptr();
            let mut end1 = ptr1.add(input.len());
            while *ptr2 != 0 {
                if (end1.offset_from(ptr1) as c_long) < 2 {
                    return 0i32;
                }
                if !(*ptr1.offset(0) as c_int == 0 && *ptr1.offset(1) as c_int == *ptr2 as c_int) {
                    return 0i32;
                }
                ptr1 = ptr1.offset(2);
                ptr2 = ptr2.offset(1);
            }
            return (ptr1 == end1) as c_int;
        }
    }

    pub(crate) fn big2_nameLength(enc: &ENCODING, mut ptr: *const c_char) -> c_int {
        unsafe {
            let mut start: *const c_char = ptr;
            loop {
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_LEAD2 => {
                        ptr = ptr.offset(2isize);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.offset(3isize);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.offset(4isize);
                    }
                    BT_NONASCII | BT_NMSTRT | BT_COLON_0 | BT_HEX | BT_DIGIT | BT_NAME
                    | BT_MINUS => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => {
                        return ptr.offset_from(start) as c_int;
                    }
                }
            }
        }
    }

    pub(crate) fn big2_skipS(enc: &ENCODING, mut ptr: *const c_char) -> *const c_char {
        unsafe {
            loop {
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_LF | BT_CR | BT_S => {
                        ptr = ptr.offset(2isize);
                    }
                    _ => return ptr,
                }
            }
        }
    }

    pub(crate) fn big2_updatePosition(enc: &ENCODING, input: &[c_char], mut pos: *mut POSITION) {
        unsafe {
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            while end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long {
                match if *ptr.offset(0) as c_int == 0 {
                    as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize] as c_uint
                } else {
                    unicode_byte_type(*ptr.offset(0), *ptr.offset(1)) as c_uint
                } {
                    BT_LEAD2 => {
                        ptr = ptr.offset(2);
                        (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                    }
                    BT_LEAD3 => {
                        ptr = ptr.offset(3);
                        (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                    }
                    BT_LEAD4 => {
                        ptr = ptr.offset(4);
                        (*pos).columnNumber = (*pos).columnNumber.wrapping_add(1);
                    }
                    BT_LF => {
                        (*pos).columnNumber = 0u64;
                        (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                        ptr = ptr.offset(2isize);
                    }
                    BT_CR => {
                        (*pos).lineNumber = (*pos).lineNumber.wrapping_add(1);
                        ptr = ptr.offset(2);
                        if end.offset_from(ptr) as c_long >= (1i32 * 2) as c_long
                            && (if *ptr.offset(0) as c_int == 0 {
                                as_normal_encoding(enc).type_0[*ptr.offset(1) as c_uchar as usize]
                                    as c_int
                            } else {
                                unicode_byte_type(*ptr.offset(0), *ptr.offset(1))
                            }) == BT_LF as c_int
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
    }

    use crate::src::lib::xmltok::checkCharRefNumber;
    use crate::src::lib::xmltok::nametab_h::namePages;
    use crate::src::lib::xmltok::nametab_h::namingBitmap;
    use crate::src::lib::xmltok::nametab_h::nmstrtPages;
    use crate::src::lib::xmltok::unicode_byte_type;
}

pub mod xmltok_ns_c {
    use super::*;
    pub(crate) fn XmlGetUtf8InternalEncoding() -> *const ENCODING {
        return &raw const internal_utf8_encoding.enc;
    }
    pub(crate) fn XmlGetUtf16InternalEncoding() -> *const ENCODING {
        return &raw const internal_little2_encoding.enc;
    }

    pub static encodings: [&ENCODING; 7] = [
        &crate::src::lib::xmltok::latin1_encoding.enc,
        &crate::src::lib::xmltok::ascii_encoding.enc,
        &crate::src::lib::xmltok::utf8_encoding.enc,
        &crate::src::lib::xmltok::big2_encoding.enc,
        &crate::src::lib::xmltok::big2_encoding.enc,
        &crate::src::lib::xmltok::little2_encoding.enc,
        &crate::src::lib::xmltok::utf8_encoding.enc,
    ];

    pub(crate) fn initScanProlog(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        return initScan(
            encodings.as_ptr() as *const *const ENCODING,
            as_init_encoding(enc),
            XML_PROLOG_STATE,
            input,
        );
    }

    pub(crate) fn initScanContent(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        return initScan(
            encodings.as_ptr() as *const *const ENCODING,
            as_init_encoding(enc),
            XML_CONTENT_STATE,
            input,
        );
    }
    pub(crate) fn XmlInitEncoding(
        mut p: *mut INIT_ENCODING,
        mut encPtr: *mut *const ENCODING,
        mut name: *const c_char,
    ) -> XmlInitEncodingResult {
        unsafe {
            let mut i: c_int = getEncodingIndex(name);
            if i == UNKNOWN_ENC {
                return (0, null::<ENCODING>());
            }
            (*p).initEnc = utf8_encoding.enc;
            (*p).initEnc.functions = &INIT_ENCODING_FUNCTIONS;
            (*p).initEnc.isUtf16 = i as c_char;
            (*p).initEnc.scanners[XML_PROLOG_STATE as usize] = initScanProlog as SCANNER;
            (*p).initEnc.scanners[XML_CONTENT_STATE as usize] = initScanContent as SCANNER;
            (*p).encPtr = encPtr;
            return (1, &raw mut (*p).initEnc);
        }
    }

    pub(crate) fn findEncoding(enc: &ENCODING, input: &[c_char]) -> *const ENCODING {
        unsafe {
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            let mut buf: [c_char; 128] = core::mem::transmute::<
            [u8; 128],
            [c_char; 128],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
            let buf_start = buf.as_mut_ptr();
            let mut p: *mut c_char = buf_start;
            let mut i: c_int = 0;
            (_, ptr, p) = (*enc).utf8Convert(enc, ptr, end, p, p.offset(128).offset(-(1)));
            if ptr != end {
                return null::<ENCODING>();
            }
            *p = 0;
            if streqci(buf.as_ptr(), &raw const KW_UTF_16 as *const c_char) != 0
                && (*enc).minBytesPerChar == 2
            {
                return enc;
            }
            i = getEncodingIndex(buf.as_ptr());
            if i == UNKNOWN_ENC {
                return null::<ENCODING>();
            }
            return encodings[i as usize] as *const ENCODING;
        }
    }
    pub(crate) fn XmlParseXmlDecl(
        mut isGeneralTextEntity: c_int,
        enc: &ENCODING,
        input: &[c_char],
    ) -> ParseXmlDeclResult {
        return doParseXmlDecl(
            Some(findEncoding as fn(&ENCODING, &[c_char]) -> *const ENCODING),
            isGeneralTextEntity,
            enc,
            input,
        );
    }
    pub(crate) fn XmlGetUtf8InternalEncodingNS() -> *const ENCODING {
        return &raw const internal_utf8_encoding_ns.enc;
    }
    pub(crate) fn XmlGetUtf16InternalEncodingNS() -> *const ENCODING {
        return &raw const internal_little2_encoding_ns.enc;
    }

    pub static encodingsNS: [&ENCODING; 7] = [
        &crate::src::lib::xmltok::latin1_encoding_ns.enc,
        &crate::src::lib::xmltok::ascii_encoding_ns.enc,
        &crate::src::lib::xmltok::utf8_encoding_ns.enc,
        &crate::src::lib::xmltok::big2_encoding_ns.enc,
        &crate::src::lib::xmltok::big2_encoding_ns.enc,
        &crate::src::lib::xmltok::little2_encoding_ns.enc,
        &crate::src::lib::xmltok::utf8_encoding_ns.enc,
    ];

    pub(crate) fn initScanPrologNS(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        return initScan(
            encodingsNS.as_ptr() as *const *const ENCODING,
            as_init_encoding(enc),
            XML_PROLOG_STATE,
            input,
        );
    }

    pub(crate) fn initScanContentNS(enc: &ENCODING, input: &[c_char]) -> ScannerResult {
        return initScan(
            encodingsNS.as_ptr() as *const *const ENCODING,
            as_init_encoding(enc),
            XML_CONTENT_STATE,
            input,
        );
    }
    pub(crate) fn XmlInitEncodingNS(
        mut p: *mut INIT_ENCODING,
        mut encPtr: *mut *const ENCODING,
        mut name: *const c_char,
    ) -> XmlInitEncodingResult {
        unsafe {
            let mut i: c_int = getEncodingIndex(name);
            if i == UNKNOWN_ENC {
                return (0, null::<ENCODING>());
            }
            (*p).initEnc = utf8_encoding_ns.enc;
            (*p).initEnc.functions = &INIT_ENCODING_FUNCTIONS;
            (*p).initEnc.isUtf16 = i as c_char;
            (*p).initEnc.scanners[XML_PROLOG_STATE as usize] = initScanPrologNS as SCANNER;
            (*p).initEnc.scanners[XML_CONTENT_STATE as usize] = initScanContentNS as SCANNER;
            (*p).encPtr = encPtr;
            return (1, &raw mut (*p).initEnc);
        }
    }

    pub(crate) fn findEncodingNS(enc: &ENCODING, input: &[c_char]) -> *const ENCODING {
        unsafe {
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            let mut buf: [c_char; 128] = core::mem::transmute::<
            [u8; 128],
            [c_char; 128],
        >(
            *b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
            let buf_start = buf.as_mut_ptr();
            let mut p: *mut c_char = buf_start;
            let mut i: c_int = 0;
            (_, ptr, p) = (*enc).utf8Convert(enc, ptr, end, p, p.offset(128).offset(-(1)));
            if ptr != end {
                return null::<ENCODING>();
            }
            *p = 0;
            if streqci(buf.as_ptr(), &raw const KW_UTF_16 as *const c_char) != 0
                && (*enc).minBytesPerChar == 2
            {
                return enc;
            }
            i = getEncodingIndex(buf.as_ptr());
            if i == UNKNOWN_ENC {
                return null::<ENCODING>();
            }
            return encodingsNS[i as usize] as *const ENCODING;
        }
    }
    pub(crate) fn XmlParseXmlDeclNS(
        mut isGeneralTextEntity: c_int,
        enc: &ENCODING,
        input: &[c_char],
    ) -> ParseXmlDeclResult {
        return doParseXmlDecl(
            Some(findEncodingNS as fn(&ENCODING, &[c_char]) -> *const ENCODING),
            isGeneralTextEntity,
            enc,
            input,
        );
    }

    use crate::src::lib::xmltok::doParseXmlDecl;
    use crate::src::lib::xmltok::getEncodingIndex;
    use crate::src::lib::xmltok::initScan;
    use crate::src::lib::xmltok::internal_little2_encoding;
    use crate::src::lib::xmltok::internal_little2_encoding_ns;
    use crate::src::lib::xmltok::internal_utf8_encoding;
    use crate::src::lib::xmltok::internal_utf8_encoding_ns;
    use crate::src::lib::xmltok::streqci;

    use crate::src::lib::xmltok::KW_UTF_16;

    use crate::src::lib::xmltok::UNKNOWN_ENC;
}

pub mod nametab_h {
    use super::*;

    pub static namingBitmap: [core::ffi::c_uint; 320] = [
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

    pub static nmstrtPages: [c_uchar; 256] = [
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

    pub static namePages: [c_uchar; 256] = [
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
use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_long;
use core::ffi::c_uchar;
use core::ffi::c_uint;
use core::ffi::c_ushort;
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::null;
use core::ptr::null_mut;
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
trait NormalEncodingCheckFunctions {
    fn isName2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isName3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isName4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isNmstrt2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isNmstrt3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isNmstrt4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isInvalid2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isInvalid3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
    fn isInvalid4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct normal_encoding {
    pub enc: ENCODING,
    pub type_0: [c_uchar; 256],
    check_functions: &'static (dyn NormalEncodingCheckFunctions + Sync),
}

impl normal_encoding {
    #[inline]
    fn checkFunctions(&self) -> &(dyn NormalEncodingCheckFunctions + Sync) {
        self.check_functions
    }

    #[inline]
    fn isName2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isName2(enc, ptr)
    }

    #[inline]
    fn isName3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isName3(enc, ptr)
    }

    #[inline]
    fn isName4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isName4(enc, ptr)
    }

    #[inline]
    fn isNmstrt2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isNmstrt2(enc, ptr)
    }

    #[inline]
    fn isNmstrt3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isNmstrt3(enc, ptr)
    }

    #[inline]
    fn isNmstrt4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isNmstrt4(enc, ptr)
    }

    #[inline]
    fn isInvalid2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isInvalid2(enc, ptr)
    }

    #[inline]
    fn isInvalid3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isInvalid3(enc, ptr)
    }

    #[inline]
    fn isInvalid4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        self.checkFunctions().isInvalid4(enc, ptr)
    }
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

fn isNever(_enc: &ENCODING, mut _p: *const c_char) -> c_int {
    return 0;
}

fn utf8_isName2(_enc: &ENCODING, mut p: *const c_char) -> c_int {
    unsafe {
        return (namingBitmap[(((namePages
            [(*(p as *const c_uchar).offset(0) as c_int >> 2 & 7) as usize]
            as c_int)
            << 3)
            + ((*(p as *const c_uchar).offset(0) as c_int & 3) << 1)
            + (*(p as *const c_uchar).offset(1) as c_int >> 5 & 1))
            as usize]
            & (1) << (*(p as *const c_uchar).offset(1) as c_int & 0x1f)) as c_int;
    }
}

fn utf8_isName3(_enc: &ENCODING, mut p: *const c_char) -> c_int {
    unsafe {
        return (namingBitmap[(((namePages[(((*(p as *const c_uchar).offset(0) as c_int & 0xf) << 4)
            + (*(p as *const c_uchar).offset(1) as c_int >> 2 & 0xf))
            as usize] as c_int)
            << 3)
            + ((*(p as *const c_uchar).offset(1) as c_int & 3) << 1)
            + (*(p as *const c_uchar).offset(2) as c_int >> 5 & 1))
            as usize]
            & (1) << (*(p as *const c_uchar).offset(2) as c_int & 0x1f)) as c_int;
    }
}

fn utf8_isNmstrt2(_enc: &ENCODING, mut p: *const c_char) -> c_int {
    unsafe {
        return (namingBitmap[(((nmstrtPages
            [(*(p as *const c_uchar).offset(0) as c_int >> 2 & 7) as usize]
            as c_int)
            << 3)
            + ((*(p as *const c_uchar).offset(0) as c_int & 3) << 1)
            + (*(p as *const c_uchar).offset(1) as c_int >> 5 & 1))
            as usize]
            & (1) << (*(p as *const c_uchar).offset(1) as c_int & 0x1f)) as c_int;
    }
}

fn utf8_isNmstrt3(_enc: &ENCODING, mut p: *const c_char) -> c_int {
    unsafe {
        return (namingBitmap[(((nmstrtPages[(((*(p as *const c_uchar).offset(0) as c_int & 0xf)
            << 4)
            + (*(p as *const c_uchar).offset(1) as c_int >> 2 & 0xf))
            as usize] as c_int)
            << 3)
            + ((*(p as *const c_uchar).offset(1) as c_int & 3) << 1)
            + (*(p as *const c_uchar).offset(2) as c_int >> 5 & 1))
            as usize]
            & (1) << (*(p as *const c_uchar).offset(2) as c_int & 0x1f)) as c_int;
    }
}

fn utf8_isInvalid2(_enc: &ENCODING, mut p: *const c_char) -> c_int {
    unsafe {
        return ((*(p as *const c_uchar) as c_int) < 0xc2
            || *(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
            || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0) as c_int;
    }
}

fn utf8_isInvalid3(_enc: &ENCODING, mut p: *const c_char) -> c_int {
    unsafe {
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
                    || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0)
                    as c_int
            } else {
                (*(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
                    || (if *(p as *const c_uchar) as c_int == 0xed {
                        (*(p as *const c_uchar).offset(1) as c_int > 0x9f) as c_int
                    } else {
                        (*(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0) as c_int
                    }) != 0) as c_int
            }) != 0) as c_int;
    }
}

fn utf8_isInvalid4(_enc: &ENCODING, mut p: *const c_char) -> c_int {
    unsafe {
        return (*(p as *const c_uchar).offset(3) as c_int & 0x80 == 0
            || *(p as *const c_uchar).offset(3) as c_int & 0xc0 == 0xc0
            || *(p as *const c_uchar).offset(2) as c_int & 0x80 == 0
            || *(p as *const c_uchar).offset(2) as c_int & 0xc0 == 0xc0
            || (if *(p as *const c_uchar) as c_int == 0xf0 {
                ((*(p as *const c_uchar).offset(1) as c_int) < 0x90
                    || *(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0)
                    as c_int
            } else {
                (*(p as *const c_uchar).offset(1) as c_int & 0x80 == 0
                    || (if *(p as *const c_uchar) as c_int == 0xf4 {
                        (*(p as *const c_uchar).offset(1) as c_int > 0x8f) as c_int
                    } else {
                        (*(p as *const c_uchar).offset(1) as c_int & 0xc0 == 0xc0) as c_int
                    }) != 0) as c_int
            }) != 0) as c_int;
    }
}

#[inline(never)]
#[cold]
fn missingNormalEncodingFunction() -> ! {
    panic!("non-null function pointer");
}

struct Utf8NormalEncodingCheckFunctions;

impl NormalEncodingCheckFunctions for Utf8NormalEncodingCheckFunctions {
    fn isName2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isName2(enc, ptr)
    }

    fn isName3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isName3(enc, ptr)
    }

    fn isName4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        isNever(enc, ptr)
    }

    fn isNmstrt2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isNmstrt2(enc, ptr)
    }

    fn isNmstrt3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isNmstrt3(enc, ptr)
    }

    fn isNmstrt4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        isNever(enc, ptr)
    }

    fn isInvalid2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isInvalid2(enc, ptr)
    }

    fn isInvalid3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isInvalid3(enc, ptr)
    }

    fn isInvalid4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        utf8_isInvalid4(enc, ptr)
    }
}

struct UnknownNormalEncodingCheckFunctions;

impl NormalEncodingCheckFunctions for UnknownNormalEncodingCheckFunctions {
    fn isName2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isName(enc, ptr)
    }

    fn isName3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isName(enc, ptr)
    }

    fn isName4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isName(enc, ptr)
    }

    fn isNmstrt2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isNmstrt(enc, ptr)
    }

    fn isNmstrt3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isNmstrt(enc, ptr)
    }

    fn isNmstrt4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isNmstrt(enc, ptr)
    }

    fn isInvalid2(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isInvalid(enc, ptr)
    }

    fn isInvalid3(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isInvalid(enc, ptr)
    }

    fn isInvalid4(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        unknown_isInvalid(enc, ptr)
    }
}

struct MissingNormalEncodingCheckFunctions;

impl NormalEncodingCheckFunctions for MissingNormalEncodingCheckFunctions {
    fn isName2(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isName3(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isName4(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isNmstrt2(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isNmstrt3(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isNmstrt4(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isInvalid2(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isInvalid3(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }

    fn isInvalid4(&self, _enc: &ENCODING, _ptr: *const c_char) -> c_int {
        missingNormalEncodingFunction()
    }
}

pub(crate) fn _INTERNAL_trim_to_complete_utf8_characters(
    mut from: *const c_char,
    mut fromLim: *const c_char,
) -> *const c_char {
    unsafe {
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
        return fromLim;
    }
}

#[cfg(feature = "expat_test_shims")]
#[export_name = "_INTERNAL_trim_to_complete_utf8_characters"]
fn internal_trim_to_complete_utf8_characters_test_shim(
    from: *const c_char,
    fromLimRef: *mut *const c_char,
) {
    unsafe {
        *fromLimRef = _INTERNAL_trim_to_complete_utf8_characters(from, *fromLimRef);
    }
}

fn utf8_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_char = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
        let mut input_incomplete: bool = false_0 != 0;
        let mut output_exhausted: bool = false_0 != 0;
        let bytesAvailable: ptrdiff_t = fromLim.offset_from(*fromP);
        let bytesStorable: ptrdiff_t = toLim.offset_from(*toP);
        if bytesAvailable > bytesStorable {
            fromLim = (*fromP).offset(bytesStorable);
            output_exhausted = true_0 != 0;
        }
        let fromLimBefore: *const c_char = fromLim;
        fromLim = _INTERNAL_trim_to_complete_utf8_characters(*fromP, fromLim);
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
        let res = if output_exhausted {
            XML_CONVERT_OUTPUT_EXHAUSTED
        } else if input_incomplete {
            XML_CONVERT_INPUT_INCOMPLETE
        } else {
            XML_CONVERT_COMPLETED
        };
        return (res, from_cursor, to_cursor);
    }
}

fn utf8_toUtf16(
    enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_ushort = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
        let mut current_block: u64;
        let mut res: XML_Convert_Result = XML_CONVERT_COMPLETED;
        let mut to: *mut c_ushort = *toP;
        let mut from: *const c_char = *fromP;
        loop {
            if !(from < fromLim && to < toLim as *mut c_ushort) {
                current_block = 18317007320854588510;
                break;
            }
            match as_normal_encoding(enc).type_0[*from as c_uchar as usize] as c_uint {
                BT_LEAD2 => {
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
                BT_LEAD3 => {
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
                BT_LEAD4 => {
                    let mut n: core::ffi::c_ulong = 0;
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
                            as core::ffi::c_ulong;
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
        return (res, from_cursor, to_cursor);
    }
}

struct Utf8EncodingFunctions;

impl EncodingFunctions for Utf8EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        normal_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        utf8_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        utf8_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct Latin1EncodingFunctions;

impl EncodingFunctions for Latin1EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        normal_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        latin1_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        latin1_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct AsciiEncodingFunctions;

impl EncodingFunctions for AsciiEncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        normal_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        ascii_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        latin1_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct Little2EncodingFunctions;

impl EncodingFunctions for Little2EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        little2_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        little2_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        little2_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        little2_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        little2_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        little2_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        little2_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        little2_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        little2_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        little2_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct Big2EncodingFunctions;

impl EncodingFunctions for Big2EncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        big2_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        big2_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        big2_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        big2_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        big2_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        big2_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        big2_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        big2_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        big2_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        big2_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct InitEncodingFunctions;

impl EncodingFunctions for InitEncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        initUpdatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        utf8_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        utf8_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

struct UnknownEncodingFunctions;

impl EncodingFunctions for UnknownEncodingFunctions {
    fn nameMatchesAscii(&self, enc: &ENCODING, input: &[c_char], kw: *const c_char) -> c_int {
        normal_nameMatchesAscii(enc, input, kw)
    }

    fn nameLength(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_nameLength(enc, ptr)
    }

    fn skipS(&self, enc: &ENCODING, ptr: *const c_char) -> *const c_char {
        normal_skipS(enc, ptr)
    }

    fn getAtts(&self, enc: &ENCODING, ptr: *const c_char, n: c_int, atts: *mut ATTRIBUTE) -> c_int {
        normal_getAtts(enc, ptr, n, atts)
    }

    fn charRefNumber(&self, enc: &ENCODING, ptr: *const c_char) -> c_int {
        normal_charRefNumber(enc, ptr)
    }

    fn predefinedEntityName(&self, enc: &ENCODING, input: &[c_char]) -> c_int {
        normal_predefinedEntityName(enc, input)
    }

    fn updatePosition(&self, enc: &ENCODING, input: &[c_char], pos: *mut POSITION) {
        normal_updatePosition(enc, input, pos);
    }

    fn isPublicId(&self, enc: &ENCODING, input: &[c_char]) -> IsPublicIdResult {
        normal_isPublicId(enc, input)
    }

    fn utf8Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_char,
        to_lim: *const c_char,
    ) -> Utf8ConvertResult {
        unknown_toUtf8(enc, from_p, from_lim, to_p, to_lim)
    }

    fn utf16Convert(
        &self,
        enc: &ENCODING,
        from_p: *const c_char,
        from_lim: *const c_char,
        to_p: *mut c_ushort,
        to_lim: *const c_ushort,
    ) -> Utf16ConvertResult {
        unknown_toUtf16(enc, from_p, from_lim, to_p, to_lim)
    }
}

static UTF8_ENCODING_FUNCTIONS: Utf8EncodingFunctions = Utf8EncodingFunctions;
static LATIN1_ENCODING_FUNCTIONS: Latin1EncodingFunctions = Latin1EncodingFunctions;
static ASCII_ENCODING_FUNCTIONS: AsciiEncodingFunctions = AsciiEncodingFunctions;
static LITTLE2_ENCODING_FUNCTIONS: Little2EncodingFunctions = Little2EncodingFunctions;
static BIG2_ENCODING_FUNCTIONS: Big2EncodingFunctions = Big2EncodingFunctions;
static INIT_ENCODING_FUNCTIONS: InitEncodingFunctions = InitEncodingFunctions;
static UNKNOWN_ENCODING_FUNCTIONS: UnknownEncodingFunctions = UnknownEncodingFunctions;
static UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS: Utf8NormalEncodingCheckFunctions =
    Utf8NormalEncodingCheckFunctions;
static UNKNOWN_NORMAL_ENCODING_CHECK_FUNCTIONS: UnknownNormalEncodingCheckFunctions =
    UnknownNormalEncodingCheckFunctions;
static MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS: MissingNormalEncodingCheckFunctions =
    MissingNormalEncodingCheckFunctions;

static utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &UTF8_ENCODING_FUNCTIONS,
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
    check_functions: &UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static utf8_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &UTF8_ENCODING_FUNCTIONS,
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
    check_functions: &UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static internal_utf8_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &UTF8_ENCODING_FUNCTIONS,
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
    check_functions: &UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static internal_utf8_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &UTF8_ENCODING_FUNCTIONS,
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
    check_functions: &UTF8_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

fn latin1_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_char = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
        loop {
            let mut c: c_uchar = 0;
            if *fromP == fromLim {
                return (XML_CONVERT_COMPLETED, from_cursor, to_cursor);
            }
            c = **fromP as c_uchar;
            if c as c_int & 0x80 != 0 {
                if (toLim.offset_from(*toP) as c_long) < 2 {
                    return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
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
                    return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                }
                let fresh8 = *fromP;
                *fromP = (*fromP).offset(1);
                let fresh9 = *toP;
                *toP = (*toP).offset(1);
                *fresh9 = *fresh8;
            }
        }
    }
}

fn latin1_toUtf16(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_ushort = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
        while *fromP < fromLim && *toP < toLim as *mut c_ushort {
            let fresh4 = *fromP;
            *fromP = (*fromP).offset(1);
            let fresh5 = *toP;
            *toP = (*toP).offset(1);
            *fresh5 = *fresh4 as c_uchar as c_ushort;
        }
        if *toP == toLim as *mut c_ushort && *fromP < fromLim {
            return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
        } else {
            return (XML_CONVERT_COMPLETED, from_cursor, to_cursor);
        };
    }
}

static latin1_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &LATIN1_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static latin1_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &LATIN1_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

fn ascii_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_char = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
        while *fromP < fromLim && *toP < toLim as *mut c_char {
            let fresh56 = *fromP;
            *fromP = (*fromP).offset(1);
            let fresh57 = *toP;
            *toP = (*toP).offset(1);
            *fresh57 = *fresh56;
        }
        if *toP == toLim as *mut c_char && *fromP < fromLim {
            return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
        } else {
            return (XML_CONVERT_COMPLETED, from_cursor, to_cursor);
        };
    }
}

static ascii_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &ASCII_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static ascii_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            normal_prologTok as SCANNER,
            normal_contentTok as SCANNER,
            normal_cdataSectionTok as SCANNER,
            normal_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            normal_attributeValueTok as SCANNER,
            normal_entityValueTok as SCANNER,
        ],
        functions: &ASCII_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

fn unicode_byte_type(mut hi: c_char, mut lo: c_char) -> c_int {
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

fn little2_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_char = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
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
                            return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
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
                        return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                    }
                    if (fromLim.offset_from(from) as c_long) < 4 {
                        *fromP = from;
                        return (XML_CONVERT_INPUT_INCOMPLETE, from_cursor, to_cursor);
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
                        return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
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
                        return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                    }
                    let fresh20 = *toP;
                    *toP = (*toP).offset(1);
                    *fresh20 =
                        (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char;
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
            return (XML_CONVERT_INPUT_INCOMPLETE, from_cursor, to_cursor);
        } else {
            return (XML_CONVERT_COMPLETED, from_cursor, to_cursor);
        };
    }
}

fn little2_toUtf16(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_ushort = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
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
            return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
        } else {
            return (res, from_cursor, to_cursor);
        };
    }
}

fn big2_toUtf8(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_char = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
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
                            return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
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
                        return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                    }
                    if (fromLim.offset_from(from) as c_long) < 4 {
                        *fromP = from;
                        return (XML_CONVERT_INPUT_INCOMPLETE, from_cursor, to_cursor);
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
                        return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
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
                        return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                    }
                    let fresh39 = *toP;
                    *toP = (*toP).offset(1);
                    *fresh39 =
                        (lo as c_int >> 6 | (hi as c_int) << 2 | UTF8_cval2 as c_int) as c_char;
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
            return (XML_CONVERT_INPUT_INCOMPLETE, from_cursor, to_cursor);
        } else {
            return (XML_CONVERT_COMPLETED, from_cursor, to_cursor);
        };
    }
}

fn big2_toUtf16(
    _enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_ushort = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
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
            return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
        } else {
            return (res, from_cursor, to_cursor);
        };
    }
}

static little2_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            little2_prologTok as SCANNER,
            little2_contentTok as SCANNER,
            little2_cdataSectionTok as SCANNER,
            little2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            little2_attributeValueTok as SCANNER,
            little2_entityValueTok as SCANNER,
        ],
        functions: &LITTLE2_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static little2_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            little2_prologTok as SCANNER,
            little2_contentTok as SCANNER,
            little2_cdataSectionTok as SCANNER,
            little2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            little2_attributeValueTok as SCANNER,
            little2_entityValueTok as SCANNER,
        ],
        functions: &LITTLE2_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static internal_little2_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            little2_prologTok as SCANNER,
            little2_contentTok as SCANNER,
            little2_cdataSectionTok as SCANNER,
            little2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            little2_attributeValueTok as SCANNER,
            little2_entityValueTok as SCANNER,
        ],
        functions: &LITTLE2_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static internal_little2_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            little2_prologTok as SCANNER,
            little2_contentTok as SCANNER,
            little2_cdataSectionTok as SCANNER,
            little2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            little2_attributeValueTok as SCANNER,
            little2_entityValueTok as SCANNER,
        ],
        functions: &LITTLE2_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static big2_encoding_ns: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            big2_prologTok as SCANNER,
            big2_contentTok as SCANNER,
            big2_cdataSectionTok as SCANNER,
            big2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            big2_attributeValueTok as SCANNER,
            big2_entityValueTok as SCANNER,
        ],
        functions: &BIG2_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

static big2_encoding: normal_encoding = normal_encoding {
    enc: encoding {
        scanners: [
            big2_prologTok as SCANNER,
            big2_contentTok as SCANNER,
            big2_cdataSectionTok as SCANNER,
            big2_ignoreSectionTok as SCANNER,
        ],
        literalScanners: [
            big2_attributeValueTok as SCANNER,
            big2_entityValueTok as SCANNER,
        ],
        functions: &BIG2_ENCODING_FUNCTIONS,
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
    check_functions: &MISSING_NORMAL_ENCODING_CHECK_FUNCTIONS,
};

fn streqci(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    unsafe {
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
}

fn initUpdatePosition(_enc: &ENCODING, input: &[c_char], mut pos: *mut POSITION) {
    normal_updatePosition(&utf8_encoding.enc, input, pos);
}

fn toAscii(enc: &ENCODING, input: &[c_char]) -> c_int {
    unsafe {
        let mut ptr = input.as_ptr();
        let end = ptr.add(input.len());
        let mut buf: [c_char; 1] = [0; 1];
        let buf_start = buf.as_mut_ptr();
        let mut p: *mut c_char = buf_start;
        (_, ptr, p) = (*enc).utf8Convert(enc, ptr, end, p, p.offset(1));
        if p == buf_start {
            return -(1i32);
        } else {
            return buf[0usize] as c_int;
        };
    }
}

fn isSpace(mut c: c_int) -> c_int {
    match c {
        32 | 13 | 10 | 9 => return 1,
        _ => {}
    }
    return 0;
}

fn parsePseudoAttribute(enc: &ENCODING, input: &[c_char]) -> ParsePseudoAttributeResult {
    unsafe {
        let mut ptr = input.as_ptr();
        let mut end = ptr.add(input.len());
        let mut c: c_int = 0;
        let mut open: c_char = 0;
        let mut name: *const c_char = null::<c_char>();
        let mut nameEnd: *const c_char = null::<c_char>();
        let mut val: *const c_char = null::<c_char>();
        if ptr == end {
            return (1, null::<c_char>(), null::<c_char>(), null::<c_char>(), ptr);
        }
        if isSpace(toAscii(enc, c_char_slice_from_ptr_end(ptr, end))) == 0 {
            return (0, name, nameEnd, val, ptr);
        }
        loop {
            ptr = ptr.offset((*enc).minBytesPerChar as isize);
            if !(isSpace(toAscii(enc, c_char_slice_from_ptr_end(ptr, end))) != 0) {
                break;
            }
        }
        if ptr == end {
            return (1, null::<c_char>(), null::<c_char>(), null::<c_char>(), ptr);
        }
        name = ptr;
        loop {
            c = toAscii(enc, c_char_slice_from_ptr_end(ptr, end));
            if c == -(1) {
                return (0, name, nameEnd, val, ptr);
            }
            if c == ASCII_EQUALS {
                nameEnd = ptr;
                break;
            } else if isSpace(c) != 0 {
                nameEnd = ptr;
                loop {
                    ptr = ptr.offset((*enc).minBytesPerChar as isize);
                    c = toAscii(enc, c_char_slice_from_ptr_end(ptr, end));
                    if !(isSpace(c) != 0) {
                        break;
                    }
                }
                if c != ASCII_EQUALS {
                    return (0, name, nameEnd, val, ptr);
                }
                break;
            } else {
                ptr = ptr.offset((*enc).minBytesPerChar as isize);
            }
        }
        if ptr == name {
            return (0, name, nameEnd, val, ptr);
        }
        ptr = ptr.offset((*enc).minBytesPerChar as isize);
        c = toAscii(enc, c_char_slice_from_ptr_end(ptr, end));
        while isSpace(c) != 0 {
            ptr = ptr.offset((*enc).minBytesPerChar as isize);
            c = toAscii(enc, c_char_slice_from_ptr_end(ptr, end));
        }
        if c != ASCII_QUOT && c != ASCII_APOS {
            return (0, name, nameEnd, val, ptr);
        }
        open = c as c_char;
        ptr = ptr.offset((*enc).minBytesPerChar as isize);
        val = ptr;
        loop {
            c = toAscii(enc, c_char_slice_from_ptr_end(ptr, end));
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
                return (0, name, nameEnd, val, ptr);
            }
            ptr = ptr.offset((*enc).minBytesPerChar as isize);
        }
        return (
            1,
            name,
            nameEnd,
            val,
            ptr.offset((*enc).minBytesPerChar as isize),
        );
    }
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

fn doParseXmlDecl(
    mut encodingFinder: Option<fn(&ENCODING, &[c_char]) -> *const ENCODING>,
    mut isGeneralTextEntity: c_int,
    enc: &ENCODING,
    input: &[c_char],
) -> ParseXmlDeclResult {
    unsafe {
        let mut ptr = input.as_ptr();
        let mut end = ptr.add(input.len());
        let mut val: *const c_char = null::<c_char>();
        let mut name: *const c_char = null::<c_char>();
        let mut nameEnd: *const c_char = null::<c_char>();
        let mut badPtr: *const c_char = null::<c_char>();
        let mut versionPtr: *const c_char = null::<c_char>();
        let mut versionEndPtr: *const c_char = null::<c_char>();
        let mut encodingName: *const c_char = null::<c_char>();
        let mut encoding: *const ENCODING = null::<ENCODING>();
        let mut standalone: c_int = -(1);
        let mut ok: c_int = 0;
        ptr = ptr.offset((5i32 * (*enc).minBytesPerChar) as isize);
        end = end.offset(-((2i32 * (*enc).minBytesPerChar) as isize));
        (ok, name, nameEnd, val, ptr) =
            parsePseudoAttribute(enc, c_char_slice_from_ptr_end(ptr, end));
        if ok == 0 || name.is_null() {
            badPtr = ptr;
            return (
                0,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
        if (*enc).nameMatchesAscii(
            enc,
            c_char_slice_from_ptr_end(name, nameEnd),
            &raw const KW_version as *const c_char,
        ) == 0
        {
            if isGeneralTextEntity == 0 {
                badPtr = name;
                return (
                    0,
                    badPtr,
                    versionPtr,
                    versionEndPtr,
                    encodingName,
                    encoding,
                    standalone,
                );
            }
        } else {
            versionPtr = val;
            versionEndPtr = ptr;
            (ok, name, nameEnd, val, ptr) =
                parsePseudoAttribute(enc, c_char_slice_from_ptr_end(ptr, end));
            if ok == 0 {
                badPtr = ptr;
                return (
                    0,
                    badPtr,
                    versionPtr,
                    versionEndPtr,
                    encodingName,
                    encoding,
                    standalone,
                );
            }
            if name.is_null() {
                if isGeneralTextEntity != 0 {
                    badPtr = ptr;
                    return (
                        0,
                        badPtr,
                        versionPtr,
                        versionEndPtr,
                        encodingName,
                        encoding,
                        standalone,
                    );
                }
                return (
                    1,
                    badPtr,
                    versionPtr,
                    versionEndPtr,
                    encodingName,
                    encoding,
                    standalone,
                );
            }
        }
        if (*enc).nameMatchesAscii(
            enc,
            c_char_slice_from_ptr_end(name, nameEnd),
            &raw const KW_encoding as *const c_char,
        ) != 0
        {
            let mut c: c_int = toAscii(enc, c_char_slice_from_ptr_end(val, end));
            if !(ASCII_a_1 <= c && c <= ASCII_z) && !(ASCII_A <= c && c <= ASCII_Z) {
                badPtr = val;
                return (
                    0,
                    badPtr,
                    versionPtr,
                    versionEndPtr,
                    encodingName,
                    encoding,
                    standalone,
                );
            }
            encodingName = val;
            encoding = encodingFinder.expect("non-null function pointer")(
                enc,
                c_char_slice_from_ptr_end(val, ptr.offset(-((*enc).minBytesPerChar as isize))),
            );
            (ok, name, nameEnd, val, ptr) =
                parsePseudoAttribute(enc, c_char_slice_from_ptr_end(ptr, end));
            if ok == 0 {
                badPtr = ptr;
                return (
                    0,
                    badPtr,
                    versionPtr,
                    versionEndPtr,
                    encodingName,
                    encoding,
                    standalone,
                );
            }
            if name.is_null() {
                return (
                    1,
                    badPtr,
                    versionPtr,
                    versionEndPtr,
                    encodingName,
                    encoding,
                    standalone,
                );
            }
        }
        if (*enc).nameMatchesAscii(
            enc,
            c_char_slice_from_ptr_end(name, nameEnd),
            &raw const KW_standalone as *const c_char,
        ) == 0
            || isGeneralTextEntity != 0
        {
            badPtr = name;
            return (
                0,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
        if (*enc).nameMatchesAscii(
            enc,
            c_char_slice_from_ptr_end(val, ptr.offset(-((*enc).minBytesPerChar as isize))),
            &raw const KW_yes as *const c_char,
        ) != 0
        {
            standalone = 1i32;
        } else if (*enc).nameMatchesAscii(
            enc,
            c_char_slice_from_ptr_end(val, ptr.offset(-((*enc).minBytesPerChar as isize))),
            &raw const KW_no as *const c_char,
        ) != 0
        {
            standalone = 0i32;
        } else {
            badPtr = val;
            return (
                0,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
        while isSpace(toAscii(enc, c_char_slice_from_ptr_end(ptr, end))) != 0 {
            ptr = ptr.offset((*enc).minBytesPerChar as isize);
        }
        if ptr != end {
            badPtr = ptr;
            return (
                0,
                badPtr,
                versionPtr,
                versionEndPtr,
                encodingName,
                encoding,
                standalone,
            );
        }
        return (
            1,
            badPtr,
            versionPtr,
            versionEndPtr,
            encodingName,
            encoding,
            standalone,
        );
    }
}

fn checkCharRefNumber(mut result: c_int) -> c_int {
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
pub(crate) fn XmlUtf8Encode(mut c: c_int, mut buf: *mut c_char) -> c_int {
    unsafe {
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
}
pub(crate) fn XmlUtf16Encode(mut charNum: c_int, mut buf: *mut c_ushort) -> c_int {
    unsafe {
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
}
pub(crate) fn XmlSizeOfUnknownEncoding() -> c_int {
    return size_of::<unknown_encoding>() as c_int;
}

fn unknown_isName(enc: &ENCODING, mut p: *const c_char) -> c_int {
    let uenc: &unknown_encoding = as_unknown_encoding(enc);
    let mut c: c_int = uenc.convert.expect("non-null function pointer")(uenc.userData, p);
    if c & !(0xffff) != 0 {
        return 0i32;
    }
    return (namingBitmap
        [(((namePages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
        & (1) << (c & 0xff & 0x1f)) as c_int;
}

fn unknown_isNmstrt(enc: &ENCODING, mut p: *const c_char) -> c_int {
    let uenc: &unknown_encoding = as_unknown_encoding(enc);
    let mut c: c_int = uenc.convert.expect("non-null function pointer")(uenc.userData, p);
    if c & !(0xffff) != 0 {
        return 0i32;
    }
    return (namingBitmap
        [(((nmstrtPages[(c >> 8) as usize] as c_int) << 3) + ((c & 0xff) >> 5)) as usize]
        & (1) << (c & 0xff & 0x1f)) as c_int;
}

fn unknown_isInvalid(enc: &ENCODING, mut p: *const c_char) -> c_int {
    let uenc: &unknown_encoding = as_unknown_encoding(enc);
    let mut c: c_int = uenc.convert.expect("non-null function pointer")(uenc.userData, p);
    return (c & !(0xffff) != 0 || checkCharRefNumber(c) < 0) as c_int;
}

fn unknown_toUtf8(
    enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_char,
    mut toLim: *const c_char,
) -> Utf8ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_char = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
        let uenc: &unknown_encoding = as_unknown_encoding(enc);
        let mut buf: [c_char; 4] = [0; 4];
        loop {
            let mut utf8: *const c_char = null::<c_char>();
            let mut n: c_int = 0;
            if *fromP == fromLim {
                return (XML_CONVERT_COMPLETED, from_cursor, to_cursor);
            }
            utf8 = &raw const *(&raw const uenc.utf8 as *const [c_char; 4])
                .offset(**fromP as c_uchar as isize) as *const c_char;
            let fresh61 = utf8;
            utf8 = utf8.offset(1);
            n = *fresh61 as c_int;
            if n == 0 {
                let mut c: c_int =
                    uenc.convert.expect("non-null function pointer")(uenc.userData, *fromP);
                n = XmlUtf8Encode(c, &raw mut buf as *mut c_char);
                if n as c_long > toLim.offset_from(*toP) as c_long {
                    return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                }
                utf8 = &raw mut buf as *mut c_char;
                *fromP = (*fromP).offset(
                    (as_normal_encoding(enc).type_0[**fromP as c_uchar as usize] as c_int
                        - (BT_LEAD2 as c_int - 2i32)) as isize,
                );
            } else {
                if n as c_long > toLim.offset_from(*toP) as c_long {
                    return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
                }
                *fromP = (*fromP).offset(1);
            }
            memcpy(*toP as *mut c_void, utf8 as *const c_void, n as size_t);
            *toP = (*toP).offset(n as isize);
        }
    }
}

fn unknown_toUtf16(
    enc: &ENCODING,
    fromP: *const c_char,
    mut fromLim: *const c_char,
    toP: *mut c_ushort,
    mut toLim: *const c_ushort,
) -> Utf16ConvertResult {
    unsafe {
        let mut from_cursor: *const c_char = fromP;
        let mut to_cursor: *mut c_ushort = toP;
        let fromP = &mut from_cursor;
        let toP = &mut to_cursor;
        let uenc: &unknown_encoding = as_unknown_encoding(enc);
        while *fromP < fromLim && *toP < toLim as *mut c_ushort {
            let mut c: c_ushort = uenc.utf16[**fromP as c_uchar as usize];
            if c as c_int == 0 {
                c = uenc.convert.expect("non-null function pointer")(uenc.userData, *fromP)
                    as c_ushort;
                *fromP = (*fromP).offset(
                    (as_normal_encoding(enc).type_0[**fromP as c_uchar as usize] as c_int
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
            return (XML_CONVERT_OUTPUT_EXHAUSTED, from_cursor, to_cursor);
        } else {
            return (XML_CONVERT_COMPLETED, from_cursor, to_cursor);
        };
    }
}
pub(crate) fn XmlInitUnknownEncoding(
    mut mem: *mut c_void,
    mut table: *const c_int,
    mut convert: CONVERTER,
    mut userData: *mut c_void,
) -> *mut ENCODING {
    unsafe {
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
                if namingBitmap[(((nmstrtPages[(c >> 8) as usize] as c_int) << 3)
                    + ((c & 0xff) >> 5)) as usize]
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
            (*e).normal.check_functions = &UNKNOWN_NORMAL_ENCODING_CHECK_FUNCTIONS;
        }
        (*e).normal.enc.functions = &UNKNOWN_ENCODING_FUNCTIONS;
        return &raw mut (*e).normal.enc;
    }
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

fn getEncodingIndex(mut name: *const c_char) -> c_int {
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

fn initScan(
    mut encodingTable: *const *const ENCODING,
    enc: &INIT_ENCODING,
    mut state: c_int,
    input: &[c_char],
) -> ScannerResult {
    unsafe {
        let mut next_tok: *const c_char = input.as_ptr();
        let nextTokPtr = &mut next_tok;
        let tok: c_int = 'iife_ret_61: {
            let mut ptr = input.as_ptr();
            let mut end = ptr.add(input.len());
            let mut encPtr: *mut *const ENCODING = null_mut::<*const ENCODING>();
            if ptr >= end {
                break 'iife_ret_61 XML_TOK_NONE_1;
            }
            encPtr = enc.encPtr;
            if ptr.offset(1) == end {
                match enc.initEnc.isUtf16 as c_int {
                    3 | 5 | 4 => break 'iife_ret_61 XML_TOK_PARTIAL_1,
                    _ => {}
                }
                let mut current_block_5: u64;
                match *ptr as c_uchar as c_int {
                    254 | 255 | 239 => {
                        if enc.initEnc.isUtf16 as c_int == ISO_8859_1_ENC
                            && state == XML_CONTENT_STATE
                        {
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
                    _ => break 'iife_ret_61 XML_TOK_PARTIAL_1,
                }
            } else {
                let mut current_block_26: u64;
                match (*ptr.offset(0) as c_uchar as c_int) << 8 | *ptr.offset(1) as c_uchar as c_int
                {
                    65279 => {
                        if !(enc.initEnc.isUtf16 as c_int == ISO_8859_1_ENC
                            && state == XML_CONTENT_STATE)
                        {
                            *nextTokPtr = ptr.offset(2);
                            *encPtr = *encodingTable.offset(UTF_16BE_ENC as isize);
                            break 'iife_ret_61 XML_TOK_BOM_1;
                        }
                    }
                    15360 => {
                        if !((enc.initEnc.isUtf16 as c_int == UTF_16BE_ENC
                            || enc.initEnc.isUtf16 as c_int == UTF_16_ENC)
                            && state == XML_CONTENT_STATE)
                        {
                            *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                            break 'iife_ret_61 ({
                                let (tok_value, next_tok_value) = (**encPtr).scanners
                                    [state as usize](
                                    &**encPtr,
                                    c_char_slice_from_ptr_end(ptr, end),
                                );
                                *nextTokPtr = next_tok_value;
                                tok_value
                            });
                        }
                    }
                    65534 => {
                        if !(enc.initEnc.isUtf16 as c_int == ISO_8859_1_ENC
                            && state == XML_CONTENT_STATE)
                        {
                            *nextTokPtr = ptr.offset(2);
                            *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                            break 'iife_ret_61 XML_TOK_BOM_1;
                        }
                    }
                    61371 => {
                        if state == XML_CONTENT_STATE {
                            let mut e: c_int = enc.initEnc.isUtf16 as c_int;
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
                                    break 'iife_ret_61 XML_TOK_PARTIAL_1;
                                }
                                if *ptr.offset(2) as c_uchar as c_int == 0xbf {
                                    *nextTokPtr = ptr.offset(3);
                                    *encPtr = *encodingTable.offset(UTF_8_ENC as isize);
                                    break 'iife_ret_61 XML_TOK_BOM_1;
                                }
                            }
                        }
                    }
                    _ => {
                        if *ptr.offset(0) as c_int == '\0' as i32 {
                            if !(state == XML_CONTENT_STATE
                                && enc.initEnc.isUtf16 as c_int == UTF_16LE_ENC)
                            {
                                *encPtr = *encodingTable.offset(UTF_16BE_ENC as isize);
                                break 'iife_ret_61 ({
                                    let (tok_value, next_tok_value) = (**encPtr).scanners
                                        [state as usize](
                                        &**encPtr,
                                        c_char_slice_from_ptr_end(ptr, end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                        } else if *ptr.offset(1) as c_int == '\0' as i32 {
                            if !(state == XML_CONTENT_STATE) {
                                *encPtr = *encodingTable.offset(UTF_16LE_ENC as isize);
                                break 'iife_ret_61 ({
                                    let (tok_value, next_tok_value) = (**encPtr).scanners
                                        [state as usize](
                                        &**encPtr,
                                        c_char_slice_from_ptr_end(ptr, end),
                                    );
                                    *nextTokPtr = next_tok_value;
                                    tok_value
                                });
                            }
                        }
                    }
                }
            }
            *encPtr = *encodingTable.offset(enc.initEnc.isUtf16 as c_int as isize);
            break 'iife_ret_61 ({
                let (tok_value, next_tok_value) = (**encPtr).scanners[state as usize](
                    &**encPtr,
                    c_char_slice_from_ptr_end(ptr, end),
                );
                *nextTokPtr = next_tok_value;
                tok_value
            });
        };
        return (tok, next_tok);
    }
}
pub(crate) fn XmlInitUnknownEncodingNS(
    mut mem: *mut c_void,
    mut table: *const c_int,
    mut convert: CONVERTER,
    mut userData: *mut c_void,
) -> *mut ENCODING {
    unsafe {
        let mut enc: *mut ENCODING = XmlInitUnknownEncoding(mem, table, convert, userData);
        if !enc.is_null() {
            (*(enc as *mut normal_encoding)).type_0[ASCII_COLON as usize] = BT_COLON_0 as c_uchar;
        }
        return enc;
    }
}
