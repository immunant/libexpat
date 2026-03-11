use ::core::ffi::{c_char, c_float, c_int, c_long, c_uchar, c_uint, c_ulong, c_ulonglong, c_void};
use ::core::mem::size_of;
use ::core::ptr::{null, null_mut};

use crate::stdlib::{__errno_location, memcpy, memset};

pub mod siphash_h {
    use ::core::ffi::{c_char, c_int, c_uchar, c_void};
    use ::core::mem::size_of;
    use ::core::ptr::null_mut;

    use crate::__stddef_size_t_h::size_t;
    use crate::siphash_h::{siphash, sipkey};
    use crate::stdlib::uint64_t;

    pub(crate) unsafe extern "C" fn sip_tokey(
        mut key: *mut sipkey,
        mut src: *const c_void,
    ) -> *mut sipkey {
        (*key).k[0] = (*(src as *const c_uchar).offset(0) as uint64_t) << 0
            | (*(src as *const c_uchar).offset(1) as uint64_t) << 8
            | (*(src as *const c_uchar).offset(2) as uint64_t) << 16
            | (*(src as *const c_uchar).offset(3) as uint64_t) << 24
            | (*(src as *const c_uchar).offset(4) as uint64_t) << 32
            | (*(src as *const c_uchar).offset(5) as uint64_t) << 40
            | (*(src as *const c_uchar).offset(6) as uint64_t) << 48
            | (*(src as *const c_uchar).offset(7) as uint64_t) << 56;
        (*key).k[1] = (*(src as *const c_uchar).offset(8).offset(0) as uint64_t) << 0
            | (*(src as *const c_uchar).offset(8).offset(1) as uint64_t) << 8
            | (*(src as *const c_uchar).offset(8).offset(2) as uint64_t) << 16
            | (*(src as *const c_uchar).offset(8).offset(3) as uint64_t) << 24
            | (*(src as *const c_uchar).offset(8).offset(4) as uint64_t) << 32
            | (*(src as *const c_uchar).offset(8).offset(5) as uint64_t) << 40
            | (*(src as *const c_uchar).offset(8).offset(6) as uint64_t) << 48
            | (*(src as *const c_uchar).offset(8).offset(7) as uint64_t) << 56;
        return key;
    }

    pub(crate) unsafe extern "C" fn sip_round(mut H: *mut siphash, rounds: c_int) {
        let mut i: c_int = 0;
        i = 0;
        while i < rounds {
            (*H).v0 = (*H).v0.wrapping_add((*H).v1);
            (*H).v1 = (*H).v1 << 13 | (*H).v1 >> 64 - 13;
            (*H).v1 ^= (*H).v0;
            (*H).v0 = (*H).v0 << 32 | (*H).v0 >> 64 - 32;
            (*H).v2 = (*H).v2.wrapping_add((*H).v3);
            (*H).v3 = (*H).v3 << 16 | (*H).v3 >> 64 - 16;
            (*H).v3 ^= (*H).v2;
            (*H).v0 = (*H).v0.wrapping_add((*H).v3);
            (*H).v3 = (*H).v3 << 21 | (*H).v3 >> 64 - 21;
            (*H).v3 ^= (*H).v0;
            (*H).v2 = (*H).v2.wrapping_add((*H).v1);
            (*H).v1 = (*H).v1 << 17 | (*H).v1 >> 64 - 17;
            (*H).v1 ^= (*H).v2;
            (*H).v2 = (*H).v2 << 32 | (*H).v2 >> 64 - 32;
            i += 1;
        }
    }

    pub(crate) unsafe extern "C" fn sip24_init(
        mut H: *mut siphash,
        mut key: *const sipkey,
    ) -> *mut siphash {
        (*H).v0 = ((0x736f6d65u64) << 32 | 0x70736575) ^ (*key).k[0];
        (*H).v1 = ((0x646f7261u64) << 32 | 0x6e646f6d) ^ (*key).k[1];
        (*H).v2 = ((0x6c796765u64) << 32 | 0x6e657261) ^ (*key).k[0];
        (*H).v3 = ((0x74656462u64) << 32 | 0x79746573) ^ (*key).k[1];
        (*H).p = &raw mut (*H).buf as *mut c_uchar;
        (*H).c = 0u64;
        return H;
    }

    pub(crate) unsafe extern "C" fn sip24_update(
        mut H: *mut siphash,
        mut src: *const c_void,
        mut len: size_t,
    ) -> *mut siphash {
        let mut p: *const c_uchar = src as *const c_uchar;
        let mut pe: *const c_uchar = p.offset(len as isize);
        let mut m: uint64_t = 0;
        loop {
            while p < pe
                && (*H).p
                    < (&raw mut (*H).buf as *mut c_uchar).offset(
                        (size_of::<[c_uchar; 8]>()).wrapping_div(size_of::<c_uchar>()) as isize,
                    )
            {
                let fresh20 = p;
                p = p.offset(1);
                let fresh21 = (*H).p;
                (*H).p = (*H).p.offset(1);
                *fresh21 = *fresh20;
            }
            if (*H).p
                < (&raw mut (*H).buf as *mut c_uchar)
                    .offset((size_of::<[c_uchar; 8]>()).wrapping_div(size_of::<c_uchar>()) as isize)
            {
                break;
            }
            m = ((*H).buf[0] as uint64_t) << 0
                | ((*H).buf[1] as uint64_t) << 8
                | ((*H).buf[2] as uint64_t) << 16
                | ((*H).buf[3] as uint64_t) << 24
                | ((*H).buf[4] as uint64_t) << 32
                | ((*H).buf[5] as uint64_t) << 40
                | ((*H).buf[6] as uint64_t) << 48
                | ((*H).buf[7] as uint64_t) << 56;
            (*H).v3 ^= m;
            sip_round(H, 2);
            (*H).v0 ^= m;
            (*H).p = &raw mut (*H).buf as *mut c_uchar;
            (*H).c = (*H).c.wrapping_add(8u64);
            if !(p < pe) {
                break;
            }
        }
        return H;
    }

    pub(crate) unsafe extern "C" fn sip24_final(mut H: *mut siphash) -> uint64_t {
        let left: c_char = (*H).p.offset_from(&raw mut (*H).buf as *mut c_uchar) as c_char;
        let mut b: uint64_t = (*H).c.wrapping_add(left as uint64_t) << 56;
        let mut current_block_6: u64;
        match left as c_int {
            7 => {
                b |= ((*H).buf[6] as uint64_t) << 48;
                current_block_6 = 4021588137456158946;
            }
            6 => {
                current_block_6 = 4021588137456158946;
            }
            5 => {
                current_block_6 = 12485585037491154495;
            }
            4 => {
                current_block_6 = 435354115069985819;
            }
            3 => {
                current_block_6 = 1199690694990637288;
            }
            2 => {
                current_block_6 = 2615438511104190163;
            }
            1 => {
                current_block_6 = 4681268752173749360;
            }
            0 | _ => {
                current_block_6 = 5720623009719927633;
            }
        }
        match current_block_6 {
            4021588137456158946 => {
                b |= ((*H).buf[5] as uint64_t) << 40;
                current_block_6 = 12485585037491154495;
            }
            _ => {}
        }
        match current_block_6 {
            12485585037491154495 => {
                b |= ((*H).buf[4] as uint64_t) << 32;
                current_block_6 = 435354115069985819;
            }
            _ => {}
        }
        match current_block_6 {
            435354115069985819 => {
                b |= ((*H).buf[3] as uint64_t) << 24;
                current_block_6 = 1199690694990637288;
            }
            _ => {}
        }
        match current_block_6 {
            1199690694990637288 => {
                b |= ((*H).buf[2] as uint64_t) << 16;
                current_block_6 = 2615438511104190163;
            }
            _ => {}
        }
        match current_block_6 {
            2615438511104190163 => {
                b |= ((*H).buf[1] as uint64_t) << 8;
                current_block_6 = 4681268752173749360;
            }
            _ => {}
        }
        match current_block_6 {
            4681268752173749360 => {
                b |= ((*H).buf[0usize] as uint64_t) << 0i32;
            }
            _ => {}
        }
        (*H).v3 ^= b;
        sip_round(H, 2);
        (*H).v0 ^= b;
        (*H).v2 ^= 0xffu64;
        sip_round(H, 4);
        return (*H).v0 ^ (*H).v1 ^ (*H).v2 ^ (*H).v3;
    }

    pub(crate) unsafe extern "C" fn siphash24(
        mut src: *const c_void,
        mut len: size_t,
        mut key: *const sipkey,
    ) -> uint64_t {
        let mut state: siphash = siphash {
            v0: 0u64,
            v1: 0u64,
            v2: 0u64,
            v3: 0u64,
            buf: [0u8, 0, 0, 0, 0, 0, 0, 0],
            p: null_mut::<c_uchar>(),
            c: 0u64,
        };
        return sip24_final(sip24_update(sip24_init(&raw mut state, key), src, len));
    }

    pub(crate) unsafe extern "C" fn sip24_valid() -> c_int {
        pub static mut vectors: [[c_uchar; 8]; 64] = [
            [0x31, 0xe, 0xe, 0xdd, 0x47, 0xdb, 0x6f, 0x72],
            [0xfd, 0x67, 0xdc, 0x93, 0xc5, 0x39, 0xf8, 0x74],
            [0x5a, 0x4f, 0xa9, 0xd9, 0x9, 0x80, 0x6c, 0xd],
            [0x2d, 0x7e, 0xfb, 0xd7, 0x96, 0x66, 0x67, 0x85],
            [0xb7, 0x87, 0x71, 0x27, 0xe0, 0x94, 0x27, 0xcf],
            [0x8d, 0xa6, 0x99, 0xcd, 0x64, 0x55, 0x76, 0x18],
            [0xce, 0xe3, 0xfe, 0x58, 0x6e, 0x46, 0xc9, 0xcb],
            [0x37, 0xd1, 0x1, 0x8b, 0xf5, 0, 0x2, 0xab],
            [0x62, 0x24, 0x93, 0x9a, 0x79, 0xf5, 0xf5, 0x93],
            [0xb0, 0xe4, 0xa9, 0xb, 0xdf, 0x82, 0, 0x9e],
            [0xf3, 0xb9, 0xdd, 0x94, 0xc5, 0xbb, 0x5d, 0x7a],
            [0xa7, 0xad, 0x6b, 0x22, 0x46, 0x2f, 0xb3, 0xf4],
            [0xfb, 0xe5, 0xe, 0x86, 0xbc, 0x8f, 0x1e, 0x75],
            [0x90, 0x3d, 0x84, 0xc0, 0x27, 0x56, 0xea, 0x14],
            [0xee, 0xf2, 0x7a, 0x8e, 0x90, 0xca, 0x23, 0xf7],
            [0xe5, 0x45, 0xbe, 0x49, 0x61, 0xca, 0x29, 0xa1],
            [0xdb, 0x9b, 0xc2, 0x57, 0x7f, 0xcc, 0x2a, 0x3f],
            [0x94, 0x47, 0xbe, 0x2c, 0xf5, 0xe9, 0x9a, 0x69],
            [0x9c, 0xd3, 0x8d, 0x96, 0xf0, 0xb3, 0xc1, 0x4b],
            [0xbd, 0x61, 0x79, 0xa7, 0x1d, 0xc9, 0x6d, 0xbb],
            [0x98, 0xee, 0xa2, 0x1a, 0xf2, 0x5c, 0xd6, 0xbe],
            [0xc7, 0x67, 0x3b, 0x2e, 0xb0, 0xcb, 0xf2, 0xd0],
            [0x88, 0x3e, 0xa3, 0xe3, 0x95, 0x67, 0x53, 0x93],
            [0xc8, 0xce, 0x5c, 0xcd, 0x8c, 0x3, 0xc, 0xa8],
            [0x94, 0xaf, 0x49, 0xf6, 0xc6, 0x50, 0xad, 0xb8],
            [0xea, 0xb8, 0x85, 0x8a, 0xde, 0x92, 0xe1, 0xbc],
            [0xf3, 0x15, 0xbb, 0x5b, 0xb8, 0x35, 0xd8, 0x17],
            [0xad, 0xcf, 0x6b, 0x7, 0x63, 0x61, 0x2e, 0x2f],
            [0xa5, 0xc9, 0x1d, 0xa7, 0xac, 0xaa, 0x4d, 0xde],
            [0x71, 0x65, 0x95, 0x87, 0x66, 0x50, 0xa2, 0xa6],
            [0x28, 0xef, 0x49, 0x5c, 0x53, 0xa3, 0x87, 0xad],
            [0x42, 0xc3, 0x41, 0xd8, 0xfa, 0x92, 0xd8, 0x32],
            [0xce, 0x7c, 0xf2, 0x72, 0x2f, 0x51, 0x27, 0x71],
            [0xe3, 0x78, 0x59, 0xf9, 0x46, 0x23, 0xf3, 0xa7],
            [0x38, 0x12, 0x5, 0xbb, 0x1a, 0xb0, 0xe0, 0x12],
            [0xae, 0x97, 0xa1, 0xf, 0xd4, 0x34, 0xe0, 0x15],
            [0xb4, 0xa3, 0x15, 0x8, 0xbe, 0xff, 0x4d, 0x31],
            [0x81, 0x39, 0x62, 0x29, 0xf0, 0x90, 0x79, 0x2],
            [0x4d, 0xc, 0xf4, 0x9e, 0xe5, 0xd4, 0xdc, 0xca],
            [0x5c, 0x73, 0x33, 0x6a, 0x76, 0xd8, 0xbf, 0x9a],
            [0xd0, 0xa7, 0x4, 0x53, 0x6b, 0xa9, 0x3e, 0xe],
            [0x92, 0x59, 0x58, 0xfc, 0xd6, 0x42, 0xc, 0xad],
            [0xa9, 0x15, 0xc2, 0x9b, 0xc8, 0x6, 0x73, 0x18],
            [0x95, 0x2b, 0x79, 0xf3, 0xbc, 0xa, 0xa6, 0xd4],
            [0xf2, 0x1d, 0xf2, 0xe4, 0x1d, 0x45, 0x35, 0xf9],
            [0x87, 0x57, 0x75, 0x19, 0x4, 0x8f, 0x53, 0xa9],
            [0x10, 0xa5, 0x6c, 0xf5, 0xdf, 0xcd, 0x9a, 0xdb],
            [0xeb, 0x75, 0x9, 0x5c, 0xcd, 0x98, 0x6c, 0xd0],
            [0x51, 0xa9, 0xcb, 0x9e, 0xcb, 0xa3, 0x12, 0xe6],
            [0x96, 0xaf, 0xad, 0xfc, 0x2c, 0xe6, 0x66, 0xc7],
            [0x72, 0xfe, 0x52, 0x97, 0x5a, 0x43, 0x64, 0xee],
            [0x5a, 0x16, 0x45, 0xb2, 0x76, 0xd5, 0x92, 0xa1],
            [0xb2, 0x74, 0xcb, 0x8e, 0xbf, 0x87, 0x87, 0xa],
            [0x6f, 0x9b, 0xb4, 0x20, 0x3d, 0xe7, 0xb3, 0x81],
            [0xea, 0xec, 0xb2, 0xa3, 0xb, 0x22, 0xa8, 0x7f],
            [0x99, 0x24, 0xa4, 0x3c, 0xc1, 0x31, 0x57, 0x24],
            [0xbd, 0x83, 0x8d, 0x3a, 0xaf, 0xbf, 0x8d, 0xb7],
            [0xb, 0x1a, 0x2a, 0x32, 0x65, 0xd5, 0x1a, 0xea],
            [0x13, 0x50, 0x79, 0xa3, 0x23, 0x1c, 0xe6, 0x60],
            [0x93, 0x2b, 0x28, 0x46, 0xe4, 0xd7, 0x6, 0x66],
            [0xe1, 0x91, 0x5f, 0x5c, 0xb1, 0xec, 0xa4, 0x6c],
            [0xf3, 0x25, 0x96, 0x5c, 0xa1, 0x6d, 0x62, 0x9f],
            [0x57, 0x5f, 0xf2, 0x8e, 0x60, 0x38, 0x1b, 0xe5],
            [0x72, 0x45, 0x6, 0xeb, 0x4c, 0x32, 0x8a, 0x95],
        ];
        let mut in_0: [c_uchar; 64] = [0; 64];
        let mut k: sipkey = sipkey { k: [0; 2] };
        let mut i: size_t = 0;
        sip_tokey(
            &raw mut k,
            b"\0\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0B\x0C\r\x0E\x0F\0" as *const u8
                as *const c_void,
        );
        i = 0;
        while i < size_of::<[c_uchar; 64]>() {
            in_0[i] = i as c_uchar;
            if siphash24(&raw mut in_0 as *const c_void, i, &raw mut k)
                != (vectors[i][0] as uint64_t) << 0
                    | (vectors[i][1] as uint64_t) << 8
                    | (vectors[i][2] as uint64_t) << 16
                    | (vectors[i][3] as uint64_t) << 24
                    | (vectors[i][4] as uint64_t) << 32
                    | (vectors[i][5] as uint64_t) << 40
                    | (vectors[i][6] as uint64_t) << 48
                    | (vectors[i][7] as uint64_t) << 56
            {
                return 0i32;
            }
            i = i.wrapping_add(1);
        }
        return 1;
    }
}

pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_ptrdiff_t_h::ptrdiff_t;
pub use crate::__stddef_size_t_h::size_t;
pub use crate::ascii_h::ASCII_a;
pub use crate::ascii_h::ASCII_c;
pub use crate::ascii_h::ASCII_e;
pub use crate::ascii_h::ASCII_g;
pub use crate::ascii_h::ASCII_h;
pub use crate::ascii_h::ASCII_l;
pub use crate::ascii_h::ASCII_m;
pub use crate::ascii_h::ASCII_n;
pub use crate::ascii_h::ASCII_o;
pub use crate::ascii_h::ASCII_p;
pub use crate::ascii_h::ASCII_r;
pub use crate::ascii_h::ASCII_s;
pub use crate::ascii_h::ASCII_t;
pub use crate::ascii_h::ASCII_w;
pub use crate::ascii_h::ASCII_x;
pub use crate::ascii_h::ASCII_0;
pub use crate::ascii_h::ASCII_1;
pub use crate::ascii_h::ASCII_2;
pub use crate::ascii_h::ASCII_3;
pub use crate::ascii_h::ASCII_8;
pub use crate::ascii_h::ASCII_9;
pub use crate::ascii_h::ASCII_A;
pub use crate::ascii_h::ASCII_C;
pub use crate::ascii_h::ASCII_COLON;
pub use crate::ascii_h::ASCII_COMMA;
pub use crate::ascii_h::ASCII_D;
pub use crate::ascii_h::ASCII_E;
pub use crate::ascii_h::ASCII_EQUALS;
pub use crate::ascii_h::ASCII_EXCL;
pub use crate::ascii_h::ASCII_F;
pub use crate::ascii_h::ASCII_HASH;
pub use crate::ascii_h::ASCII_I;
pub use crate::ascii_h::ASCII_K;
pub use crate::ascii_h::ASCII_L;
pub use crate::ascii_h::ASCII_LPAREN;
pub use crate::ascii_h::ASCII_M;
pub use crate::ascii_h::ASCII_N;
pub use crate::ascii_h::ASCII_O;
pub use crate::ascii_h::ASCII_PERIOD;
pub use crate::ascii_h::ASCII_PIPE;
pub use crate::ascii_h::ASCII_R;
pub use crate::ascii_h::ASCII_S;
pub use crate::ascii_h::ASCII_SLASH;
pub use crate::ascii_h::ASCII_T;
pub use crate::ascii_h::ASCII_X;
pub use crate::ascii_h::ASCII_Y;
pub use crate::expat_config_h::XML_CONTEXT_BYTES;

pub use crate::stdlib::EINTR;

pub use crate::expat_external_h::XML_Char;
pub use crate::expat_external_h::XML_Index;
pub use crate::expat_external_h::XML_LChar;
pub use crate::expat_external_h::XML_Size;
pub use crate::expat_h::XML_AttlistDeclHandler;
pub use crate::expat_h::XML_Bool;
pub use crate::expat_h::XML_CharacterDataHandler;
pub use crate::expat_h::XML_CommentHandler;
pub use crate::expat_h::XML_Content;
pub use crate::expat_h::XML_Content_Quant;
pub use crate::expat_h::XML_Content_Type;
pub use crate::expat_h::XML_DefaultHandler;
pub use crate::expat_h::XML_ElementDeclHandler;
pub use crate::expat_h::XML_Encoding;
pub use crate::expat_h::XML_EndCdataSectionHandler;
pub use crate::expat_h::XML_EndDoctypeDeclHandler;
pub use crate::expat_h::XML_EndElementHandler;
pub use crate::expat_h::XML_EndNamespaceDeclHandler;
pub use crate::expat_h::XML_EntityDeclHandler;
pub use crate::expat_h::XML_Error;
pub use crate::expat_h::XML_Expat_Version;
pub use crate::expat_h::XML_ExternalEntityRefHandler;
pub use crate::expat_h::XML_Feature;
pub use crate::expat_h::XML_FeatureEnum;
pub use crate::expat_h::XML_Memory_Handling_Suite;
pub use crate::expat_h::XML_NotStandaloneHandler;
pub use crate::expat_h::XML_NotationDeclHandler;
pub use crate::expat_h::XML_ParamEntityParsing;
pub use crate::expat_h::XML_Parser;
pub use crate::expat_h::XML_Parsing;
pub use crate::expat_h::XML_ParsingStatus;
pub use crate::expat_h::XML_ProcessingInstructionHandler;
pub use crate::expat_h::XML_SkippedEntityHandler;
pub use crate::expat_h::XML_StartCdataSectionHandler;
pub use crate::expat_h::XML_StartDoctypeDeclHandler;
pub use crate::expat_h::XML_StartElementHandler;
pub use crate::expat_h::XML_StartNamespaceDeclHandler;
pub use crate::expat_h::XML_Status;
pub use crate::expat_h::XML_UnknownEncodingHandler;
pub use crate::expat_h::XML_UnparsedEntityDeclHandler;
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
pub use crate::expat_h::XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT;
pub use crate::expat_h::XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT;
pub use crate::expat_h::XML_FEATURE_ATTR_INFO;
pub use crate::expat_h::XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT;
pub use crate::expat_h::XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT;
pub use crate::expat_h::XML_FEATURE_CONTEXT_BYTES;
pub use crate::expat_h::XML_FEATURE_DTD;
pub use crate::expat_h::XML_FEATURE_END;
pub use crate::expat_h::XML_FEATURE_GE;
pub use crate::expat_h::XML_FEATURE_LARGE_SIZE;
pub use crate::expat_h::XML_FEATURE_MIN_SIZE;
pub use crate::expat_h::XML_FEATURE_NS;
pub use crate::expat_h::XML_FEATURE_SIZEOF_XML_CHAR;
pub use crate::expat_h::XML_FEATURE_SIZEOF_XML_LCHAR;
pub use crate::expat_h::XML_FEATURE_UNICODE;
pub use crate::expat_h::XML_FEATURE_UNICODE_WCHAR_T;
pub use crate::expat_h::XML_FINISHED;
pub use crate::expat_h::XML_INITIALIZED;
pub use crate::expat_h::XML_MAJOR_VERSION;
pub use crate::expat_h::XML_MICRO_VERSION;
pub use crate::expat_h::XML_MINOR_VERSION;
pub use crate::expat_h::XML_PARAM_ENTITY_PARSING_ALWAYS;
pub use crate::expat_h::XML_PARAM_ENTITY_PARSING_NEVER;
pub use crate::expat_h::XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE;
pub use crate::expat_h::XML_PARSING;
pub use crate::expat_h::XML_STATUS_ERROR;
pub use crate::expat_h::XML_STATUS_OK;
pub use crate::expat_h::XML_STATUS_SUSPENDED;
pub use crate::expat_h::XML_SUSPENDED;
pub use crate::expat_h::XML_TRUE;
pub use crate::internal::__INT_MAX__;

pub use crate::stdlib::O_RDONLY;

pub use crate::internal_h::EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT;
pub use crate::internal_h::EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT;
pub use crate::internal_h::EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT;
pub use crate::internal_h::EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT;
pub use crate::internal_h::EXPAT_MALLOC_ALIGNMENT;
pub use crate::internal_h::EXPAT_MALLOC_PADDING;
pub use crate::limits_h::INT_MAX;
pub use crate::limits_h::UINT_MAX;
pub use crate::siphash_h::siphash;
pub use crate::siphash_h::sipkey;
pub(crate) use crate::src::lib::xmlparse::siphash_h::{sip24_final, sip24_init, sip24_update};
pub use crate::stdbool_h::false_0;
pub use crate::stdbool_h::true_0;
pub use crate::stdlib::getrandom;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::GRND_NONBLOCK;
pub use crate::stdlib::SIZE_MAX;

pub use crate::stdlib::_IO_codecvt;
pub use crate::stdlib::_IO_lock_t;
pub use crate::stdlib::_IO_marker;
pub use crate::stdlib::_IO_wide_data;
pub use crate::stdlib::fprintf;

pub use crate::stdlib::ssize_t;
pub use crate::stdlib::stderr;

pub use crate::stdlib::timeval;
pub use crate::stdlib::_IO_FILE;

pub use crate::src::lib::xmlrole::prolog_state;
pub use crate::src::lib::xmlrole::C2RustUnnamed_0;
pub(crate) use crate::src::lib::xmlrole::XmlPrologStateInit;
pub(crate) use crate::src::lib::xmlrole::XmlPrologStateInitExternalEntity;
pub use crate::src::lib::xmlrole::PROLOG_STATE;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTLIST_ELEMENT_NAME;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTLIST_NONE;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_ENUM_VALUE;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_NAME;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_NOTATION_VALUE;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_CDATA;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_ENTITIES;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_ENTITY;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_ID;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_IDREF;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_IDREFS;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_NMTOKEN;
pub use crate::src::lib::xmlrole::XML_ROLE_ATTRIBUTE_TYPE_NMTOKENS;
pub use crate::src::lib::xmlrole::XML_ROLE_COMMENT;
pub use crate::src::lib::xmlrole::XML_ROLE_CONTENT_ANY;
pub use crate::src::lib::xmlrole::XML_ROLE_CONTENT_ELEMENT;
pub use crate::src::lib::xmlrole::XML_ROLE_CONTENT_ELEMENT_OPT;
pub use crate::src::lib::xmlrole::XML_ROLE_CONTENT_ELEMENT_PLUS;
pub use crate::src::lib::xmlrole::XML_ROLE_CONTENT_ELEMENT_REP;
pub use crate::src::lib::xmlrole::XML_ROLE_CONTENT_EMPTY;
pub use crate::src::lib::xmlrole::XML_ROLE_CONTENT_PCDATA;
pub use crate::src::lib::xmlrole::XML_ROLE_DEFAULT_ATTRIBUTE_VALUE;
pub use crate::src::lib::xmlrole::XML_ROLE_DOCTYPE_CLOSE;
pub use crate::src::lib::xmlrole::XML_ROLE_DOCTYPE_INTERNAL_SUBSET;
pub use crate::src::lib::xmlrole::XML_ROLE_DOCTYPE_NAME;
pub use crate::src::lib::xmlrole::XML_ROLE_DOCTYPE_NONE;
pub use crate::src::lib::xmlrole::XML_ROLE_DOCTYPE_PUBLIC_ID;
pub use crate::src::lib::xmlrole::XML_ROLE_DOCTYPE_SYSTEM_ID;
pub use crate::src::lib::xmlrole::XML_ROLE_ELEMENT_NAME;
pub use crate::src::lib::xmlrole::XML_ROLE_ELEMENT_NONE;
pub use crate::src::lib::xmlrole::XML_ROLE_ENTITY_COMPLETE;
pub use crate::src::lib::xmlrole::XML_ROLE_ENTITY_NONE;
pub use crate::src::lib::xmlrole::XML_ROLE_ENTITY_NOTATION_NAME;
pub use crate::src::lib::xmlrole::XML_ROLE_ENTITY_PUBLIC_ID;
pub use crate::src::lib::xmlrole::XML_ROLE_ENTITY_SYSTEM_ID;
pub use crate::src::lib::xmlrole::XML_ROLE_ENTITY_VALUE;
pub use crate::src::lib::xmlrole::XML_ROLE_ERROR;
pub use crate::src::lib::xmlrole::XML_ROLE_FIXED_ATTRIBUTE_VALUE;
pub use crate::src::lib::xmlrole::XML_ROLE_GENERAL_ENTITY_NAME;
pub use crate::src::lib::xmlrole::XML_ROLE_GROUP_CHOICE;
pub use crate::src::lib::xmlrole::XML_ROLE_GROUP_CLOSE;
pub use crate::src::lib::xmlrole::XML_ROLE_GROUP_CLOSE_OPT;
pub use crate::src::lib::xmlrole::XML_ROLE_GROUP_CLOSE_PLUS;
pub use crate::src::lib::xmlrole::XML_ROLE_GROUP_CLOSE_REP;
pub use crate::src::lib::xmlrole::XML_ROLE_GROUP_OPEN;
pub use crate::src::lib::xmlrole::XML_ROLE_GROUP_SEQUENCE;
pub use crate::src::lib::xmlrole::XML_ROLE_IGNORE_SECT;
pub use crate::src::lib::xmlrole::XML_ROLE_IMPLIED_ATTRIBUTE_VALUE;
pub use crate::src::lib::xmlrole::XML_ROLE_INNER_PARAM_ENTITY_REF;
pub use crate::src::lib::xmlrole::XML_ROLE_INSTANCE_START;
pub use crate::src::lib::xmlrole::XML_ROLE_NONE;
pub use crate::src::lib::xmlrole::XML_ROLE_NOTATION_NAME;
pub use crate::src::lib::xmlrole::XML_ROLE_NOTATION_NONE;
pub use crate::src::lib::xmlrole::XML_ROLE_NOTATION_NO_SYSTEM_ID;
pub use crate::src::lib::xmlrole::XML_ROLE_NOTATION_PUBLIC_ID;
pub use crate::src::lib::xmlrole::XML_ROLE_NOTATION_SYSTEM_ID;
pub use crate::src::lib::xmlrole::XML_ROLE_PARAM_ENTITY_NAME;
pub use crate::src::lib::xmlrole::XML_ROLE_PARAM_ENTITY_REF;
pub use crate::src::lib::xmlrole::XML_ROLE_PI;
pub use crate::src::lib::xmlrole::XML_ROLE_REQUIRED_ATTRIBUTE_VALUE;
pub use crate::src::lib::xmlrole::XML_ROLE_TEXT_DECL;
pub use crate::src::lib::xmlrole::XML_ROLE_XML_DECL;
pub use crate::src::lib::xmltok::encoding;
pub use crate::src::lib::xmltok::position;
pub(crate) use crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding;
pub(crate) use crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS;
pub(crate) use crate::src::lib::xmltok::xmltok_ns_c::XmlInitEncoding;
pub(crate) use crate::src::lib::xmltok::xmltok_ns_c::XmlInitEncodingNS;
pub(crate) use crate::src::lib::xmltok::xmltok_ns_c::XmlParseXmlDecl;
pub(crate) use crate::src::lib::xmltok::xmltok_ns_c::XmlParseXmlDeclNS;
pub use crate::src::lib::xmltok::XML_Convert_Result;
pub(crate) use crate::src::lib::xmltok::XmlInitUnknownEncoding;
pub(crate) use crate::src::lib::xmltok::XmlInitUnknownEncodingNS;
pub(crate) use crate::src::lib::xmltok::XmlSizeOfUnknownEncoding;
pub(crate) use crate::src::lib::xmltok::XmlUtf8Encode;
pub use crate::src::lib::xmltok::ATTRIBUTE;
pub use crate::src::lib::xmltok::CONVERTER;
pub use crate::src::lib::xmltok::ENCODING;
pub use crate::src::lib::xmltok::INIT_ENCODING;
pub use crate::src::lib::xmltok::POSITION;
pub use crate::src::lib::xmltok::SCANNER;
pub use crate::src::lib::xmltok::XML_CONVERT_COMPLETED;
pub use crate::src::lib::xmltok::XML_CONVERT_INPUT_INCOMPLETE;
pub use crate::src::lib::xmltok::XML_CONVERT_OUTPUT_EXHAUSTED;
pub use crate::src::lib::xmltok::XML_TOK_ATTRIBUTE_VALUE_S;
pub use crate::src::lib::xmltok::XML_TOK_BOM;
pub use crate::src::lib::xmltok::XML_TOK_CDATA_SECT_CLOSE;
pub use crate::src::lib::xmltok::XML_TOK_CDATA_SECT_OPEN;
pub use crate::src::lib::xmltok::XML_TOK_CHAR_REF;
pub use crate::src::lib::xmltok::XML_TOK_COMMENT;
pub use crate::src::lib::xmltok::XML_TOK_DATA_CHARS;
pub use crate::src::lib::xmltok::XML_TOK_DATA_NEWLINE;
pub use crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_NO_ATTS;
pub use crate::src::lib::xmltok::XML_TOK_EMPTY_ELEMENT_WITH_ATTS;
pub use crate::src::lib::xmltok::XML_TOK_END_TAG;
pub use crate::src::lib::xmltok::XML_TOK_ENTITY_REF;
pub use crate::src::lib::xmltok::XML_TOK_IGNORE_SECT;
pub use crate::src::lib::xmltok::XML_TOK_INSTANCE_START;
pub use crate::src::lib::xmltok::XML_TOK_INVALID;
pub use crate::src::lib::xmltok::XML_TOK_NONE;
pub use crate::src::lib::xmltok::XML_TOK_PARAM_ENTITY_REF;
pub use crate::src::lib::xmltok::XML_TOK_PARTIAL;
pub use crate::src::lib::xmltok::XML_TOK_PARTIAL_CHAR;
pub use crate::src::lib::xmltok::XML_TOK_PI;
pub use crate::src::lib::xmltok::XML_TOK_PROLOG_S;
pub use crate::src::lib::xmltok::XML_TOK_START_TAG_NO_ATTS;
pub use crate::src::lib::xmltok::XML_TOK_START_TAG_WITH_ATTS;
pub use crate::src::lib::xmltok::XML_TOK_TRAILING_CR;
pub use crate::src::lib::xmltok::XML_TOK_TRAILING_RSQB;
pub use crate::src::lib::xmltok::XML_TOK_XML_DECL;
pub use crate::stdlib::__off64_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__pid_t;
pub use crate::stdlib::__ssize_t;
pub use crate::stdlib::__suseconds_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::__uint64_t;

pub use crate::stdlib::FILE;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct XML_ParserStruct {
    pub m_userData: *mut c_void,
    pub m_handlerArg: *mut c_void,
    pub m_buffer: *mut c_char,
    pub m_mem: XML_Memory_Handling_Suite,
    pub m_bufferPtr: *const c_char,
    pub m_bufferEnd: *mut c_char,
    pub m_bufferLim: *const c_char,
    pub m_parseEndByteIndex: XML_Index,
    pub m_parseEndPtr: *const c_char,
    pub m_partialTokenBytesBefore: size_t,
    pub m_reparseDeferralEnabled: XML_Bool,
    pub m_lastBufferRequestSize: c_int,
    pub m_dataBuf: *mut XML_Char,
    pub m_dataBufEnd: *mut XML_Char,
    pub m_startElementHandler: XML_StartElementHandler,
    pub m_endElementHandler: XML_EndElementHandler,
    pub m_characterDataHandler: XML_CharacterDataHandler,
    pub m_processingInstructionHandler: XML_ProcessingInstructionHandler,
    pub m_commentHandler: XML_CommentHandler,
    pub m_startCdataSectionHandler: XML_StartCdataSectionHandler,
    pub m_endCdataSectionHandler: XML_EndCdataSectionHandler,
    pub m_defaultHandler: XML_DefaultHandler,
    pub m_startDoctypeDeclHandler: XML_StartDoctypeDeclHandler,
    pub m_endDoctypeDeclHandler: XML_EndDoctypeDeclHandler,
    pub m_unparsedEntityDeclHandler: XML_UnparsedEntityDeclHandler,
    pub m_notationDeclHandler: XML_NotationDeclHandler,
    pub m_startNamespaceDeclHandler: XML_StartNamespaceDeclHandler,
    pub m_endNamespaceDeclHandler: XML_EndNamespaceDeclHandler,
    pub m_notStandaloneHandler: XML_NotStandaloneHandler,
    pub m_externalEntityRefHandler: XML_ExternalEntityRefHandler,
    pub m_externalEntityRefHandlerArg: XML_Parser,
    pub m_skippedEntityHandler: XML_SkippedEntityHandler,
    pub m_unknownEncodingHandler: XML_UnknownEncodingHandler,
    pub m_elementDeclHandler: XML_ElementDeclHandler,
    pub m_attlistDeclHandler: XML_AttlistDeclHandler,
    pub m_entityDeclHandler: XML_EntityDeclHandler,
    pub m_xmlDeclHandler: XML_XmlDeclHandler,
    pub m_encoding: *const ENCODING,
    pub m_initEncoding: INIT_ENCODING,
    pub m_internalEncoding: *const ENCODING,
    pub m_protocolEncodingName: *const XML_Char,
    pub m_ns: XML_Bool,
    pub m_ns_triplets: XML_Bool,
    pub m_unknownEncodingMem: *mut c_void,
    pub m_unknownEncodingData: *mut c_void,
    pub m_unknownEncodingHandlerData: *mut c_void,
    pub m_unknownEncodingRelease: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
    pub m_prologState: PROLOG_STATE,
    pub m_processor: Option<Processor>,
    pub m_errorCode: XML_Error,
    pub m_eventPtr: *const c_char,
    pub m_eventEndPtr: *const c_char,
    pub m_positionPtr: *const c_char,
    pub m_openInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeInternalEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_openAttributeEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeAttributeEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_openValueEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_freeValueEntities: *mut OPEN_INTERNAL_ENTITY,
    pub m_defaultExpandInternalEntities: XML_Bool,
    pub m_tagLevel: c_int,
    pub m_declEntity: *mut ENTITY,
    pub m_doctypeName: *const XML_Char,
    pub m_doctypeSysid: *const XML_Char,
    pub m_doctypePubid: *const XML_Char,
    pub m_declAttributeType: *const XML_Char,
    pub m_declNotationName: *const XML_Char,
    pub m_declNotationPublicId: *const XML_Char,
    pub m_declElementType: *mut ELEMENT_TYPE,
    pub m_declAttributeId: *mut ATTRIBUTE_ID,
    pub m_declAttributeIsCdata: XML_Bool,
    pub m_declAttributeIsId: XML_Bool,
    pub m_dtd: *mut DTD,
    pub m_curBase: *const XML_Char,
    pub m_tagStack: *mut TAG,
    pub m_freeTagList: *mut TAG,
    pub m_inheritedBindings: *mut BINDING,
    pub m_freeBindingList: *mut BINDING,
    pub m_attsSize: c_int,
    pub m_nSpecifiedAtts: c_int,
    pub m_idAttIndex: c_int,
    pub m_atts: *mut ATTRIBUTE,
    pub m_nsAtts: *mut NS_ATT,
    pub m_nsAttsVersion: c_ulong,
    pub m_nsAttsPower: c_uchar,
    pub m_position: POSITION,
    pub m_tempPool: STRING_POOL,
    pub m_temp2Pool: STRING_POOL,
    pub m_groupConnector: *mut c_char,
    pub m_groupSize: c_uint,
    pub m_namespaceSeparator: XML_Char,
    pub m_parentParser: XML_Parser,
    pub m_parsingStatus: XML_ParsingStatus,
    pub m_isParamEntity: XML_Bool,
    pub m_useForeignDTD: XML_Bool,
    pub m_paramEntityParsing: XML_ParamEntityParsing,
    pub m_hash_secret_salt: c_ulong,
    pub m_accounting: ACCOUNTING,
    pub m_alloc_tracker: MALLOC_TRACKER,
    pub m_entity_stats: ENTITY_STATS,
    pub m_reenter: XML_Bool,
}

pub type ENTITY_STATS = entity_stats;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct entity_stats {
    pub countEverOpened: c_uint,
    pub currentDepth: c_uint,
    pub maximumDepthSeen: c_uint,
    pub debugLevel: c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct MALLOC_TRACKER {
    pub bytesAllocated: XmlBigCount,
    pub peakBytesAllocated: XmlBigCount,
    pub debugLevel: c_ulong,
    pub maximumAmplificationFactor: c_float,
    pub activationThresholdBytes: XmlBigCount,
}

pub type XmlBigCount = c_ulonglong;

pub type ACCOUNTING = accounting;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct accounting {
    pub countBytesDirect: XmlBigCount,
    pub countBytesIndirect: XmlBigCount,
    pub debugLevel: c_ulong,
    pub maximumAmplificationFactor: c_float,
    pub activationThresholdBytes: c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct STRING_POOL {
    pub blocks: *mut BLOCK,
    pub freeBlocks: *mut BLOCK,
    pub end: *const XML_Char,
    pub ptr: *mut XML_Char,
    pub start: *mut XML_Char,
    pub parser: XML_Parser,
}

pub type BLOCK = block;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct block {
    pub next: *mut block,
    pub size: c_int,
    pub s: [XML_Char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct NS_ATT {
    pub version: c_ulong,
    pub hash: c_ulong,
    pub uriName: *const XML_Char,
}

pub type BINDING = binding;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct binding {
    pub prefix: *mut prefix,
    pub nextTagBinding: *mut binding,
    pub prevPrefixBinding: *mut binding,
    pub attId: *const attribute_id,
    pub uri: *mut XML_Char,
    pub uriLen: c_int,
    pub uriAlloc: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct attribute_id {
    pub name: *mut XML_Char,
    pub prefix: *mut PREFIX,
    pub maybeTokenized: XML_Bool,
    pub xmlns: XML_Bool,
}

pub type PREFIX = prefix;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct prefix {
    pub name: *const XML_Char,
    pub binding: *mut BINDING,
}

pub type TAG = tag;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct tag {
    pub parent: *mut tag,
    pub rawName: *const c_char,
    pub rawNameLength: c_int,
    pub name: TAG_NAME,
    pub buf: C2RustUnnamed_1,
    pub bufEnd: *mut c_char,
    pub bindings: *mut BINDING,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub union C2RustUnnamed_1 {
    pub raw: *mut c_char,
    pub str_0: *mut XML_Char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct TAG_NAME {
    pub str_0: *const XML_Char,
    pub localPart: *const XML_Char,
    pub prefix: *const XML_Char,
    pub strLen: c_int,
    pub uriLen: c_int,
    pub prefixLen: c_int,
}
#[repr(C)]
pub struct HASH_TABLE {
    pub entries: std::collections::HashMap<Vec<XML_Char>, *mut NAMED>,
    pub parser: XML_Parser,
}
#[repr(C)]

pub struct DTD {
    pub generalEntities: HASH_TABLE,
    pub elementTypes: HASH_TABLE,
    pub attributeIds: HASH_TABLE,
    pub prefixes: HASH_TABLE,
    pub pool: STRING_POOL,
    pub entityValuePool: STRING_POOL,
    pub keepProcessing: XML_Bool,
    pub hasParamEntityRefs: XML_Bool,
    pub standalone: XML_Bool,
    pub paramEntityRead: XML_Bool,
    pub paramEntities: HASH_TABLE,
    pub defaultPrefix: PREFIX,
    pub in_eldecl: XML_Bool,
    pub scaffold: *mut CONTENT_SCAFFOLD,
    pub contentStringLen: c_uint,
    pub scaffSize: c_uint,
    pub scaffCount: c_uint,
    pub scaffLevel: c_int,
    pub scaffIndex: *mut c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct CONTENT_SCAFFOLD {
    pub type_0: XML_Content_Type,
    pub quant: XML_Content_Quant,
    pub name: *const XML_Char,
    pub firstchild: c_int,
    pub lastchild: c_int,
    pub childcnt: c_int,
    pub nextsib: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NAMED {
    pub name: KEY,
}

pub type KEY = *const XML_Char;

pub type ATTRIBUTE_ID = attribute_id;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ELEMENT_TYPE {
    pub name: *const XML_Char,
    pub prefix: *mut PREFIX,
    pub idAtt: *const ATTRIBUTE_ID,
    pub nDefaultAtts: c_int,
    pub allocDefaultAtts: c_int,
    pub defaultAtts: *mut DEFAULT_ATTRIBUTE,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct DEFAULT_ATTRIBUTE {
    pub id: *const ATTRIBUTE_ID,
    pub isCdata: XML_Bool,
    pub value: *const XML_Char,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct ENTITY {
    pub name: *const XML_Char,
    pub textPtr: *const XML_Char,
    pub textLen: c_int,
    pub processed: c_int,
    pub systemId: *const XML_Char,
    pub base: *const XML_Char,
    pub publicId: *const XML_Char,
    pub notation: *const XML_Char,
    pub open: XML_Bool,
    pub hasMore: XML_Bool,
    pub is_param: XML_Bool,
    pub is_internal: XML_Bool,
}

pub type OPEN_INTERNAL_ENTITY = open_internal_entity;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct open_internal_entity {
    pub internalEventPtr: *const c_char,
    pub internalEventEndPtr: *const c_char,
    pub next: *mut open_internal_entity,
    pub entity: *mut ENTITY,
    pub startTagLevel: c_int,
    pub betweenDecl: XML_Bool,
    pub type_0: EntityType,
}

pub type EntityType = c_uint;

pub const ENTITY_VALUE: EntityType = 2;

pub const ENTITY_ATTRIBUTE: EntityType = 1;

pub const ENTITY_INTERNAL: EntityType = 0;

pub type Processor =
    unsafe extern "C" fn(XML_Parser, *const c_char, *const c_char, *mut *const c_char) -> XML_Error;
#[derive(Copy, Clone)]
#[repr(C)]

pub struct HASH_TABLE_ITER {
    pub p: *mut *mut NAMED,
    pub end: *mut *mut NAMED,
    pub table: *const HASH_TABLE,
    pub index: size_t,
}

pub type XML_Account = c_uint;

pub const XML_ACCOUNT_NONE: XML_Account = 2;

pub const XML_ACCOUNT_ENTITY_EXPANSION: XML_Account = 1;

pub const XML_ACCOUNT_DIRECT: XML_Account = 0;

pub type ICHAR = c_char;
static mut xmlLen: c_int = 0;
static mut xmlnsLen: c_int = 0;

pub const INIT_TAG_BUF_SIZE: c_int = 32;

pub const INIT_DATA_BUF_SIZE: c_int = 1024;

pub const INIT_ATTS_SIZE: c_int = 16;

pub const INIT_ATTS_VERSION: c_uint = 0xffffffff;

pub const INIT_BLOCK_SIZE: c_int = 1024;

pub const INIT_BUFFER_SIZE: c_int = 1024;

pub const EXPAND_SPARE: c_int = 24;

pub const INIT_SCAFFOLD_ELEMENTS: c_int = 32;
#[cfg(feature = "xml-testing")]
#[no_mangle]
pub static mut g_reparseDeferralEnabledDefault: XML_Bool = XML_TRUE;
#[cfg(not(feature = "xml-testing"))]
#[no_mangle]
pub static g_reparseDeferralEnabledDefault: XML_Bool = XML_TRUE;
#[cfg(feature = "xml-testing")]
#[no_mangle]
pub static mut g_bytesScanned: c_uint = 0;

unsafe extern "C" fn expat_heap_stat(
    mut rootParser: XML_Parser,
    mut operator: c_char,
    mut absDiff: XmlBigCount,
    mut newTotal: XmlBigCount,
    mut peakTotal: XmlBigCount,
    mut sourceLine: c_int,
) {
    let amplification: c_float =
        newTotal as c_float / (*rootParser).m_accounting.countBytesDirect as c_float;
    fprintf(
        stderr,
        b"expat: Allocations(%p): Direct %10llu, allocated %c%10llu to %10llu (%10llu peak), amplification %8.2f (xmlparse.c:%d)\n\0"
            as *const u8 as *const c_char,
        rootParser as *mut c_void,
        (*rootParser).m_accounting.countBytesDirect,
        operator as c_int,
        absDiff,
        newTotal,
        peakTotal,
        amplification as ::core::ffi::c_double,
        sourceLine,
    );
}

unsafe extern "C" fn expat_heap_increase_tolerable(
    mut rootParser: XML_Parser,
    mut increase: XmlBigCount,
    mut sourceLine: c_int,
) -> bool {
    assert!(!rootParser.is_null(), "rootParser != NULL");
    assert!(increase > 0u64, "increase > 0");
    let mut newTotal: XmlBigCount = 0;
    let mut tolerable: bool = true_0 != 0;
    if (-(1i32) as XmlBigCount).wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
        < increase
    {
        tolerable = false_0 != 0;
    } else {
        newTotal = (*rootParser)
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_add(increase);
        if newTotal >= (*rootParser).m_alloc_tracker.activationThresholdBytes {
            assert!(newTotal > 0, "newTotal > 0");
            let amplification: c_float =
                newTotal as c_float / (*rootParser).m_accounting.countBytesDirect as c_float;
            if amplification > (*rootParser).m_alloc_tracker.maximumAmplificationFactor {
                tolerable = false_0 != 0;
            }
        }
    }
    if !tolerable && (*rootParser).m_alloc_tracker.debugLevel >= 1u64 {
        expat_heap_stat(
            rootParser,
            '+' as c_char,
            increase,
            newTotal,
            newTotal,
            sourceLine,
        );
    }
    return tolerable;
}
#[cfg_attr(feature = "xml-testing", no_mangle)]
pub unsafe extern "C" fn expat_malloc(
    mut parser: XML_Parser,
    mut size: size_t,
    mut sourceLine: c_int,
) -> *mut c_void {
    if (SIZE_MAX as size_t).wrapping_sub(size)
        < (size_of::<size_t>()).wrapping_add(EXPAT_MALLOC_PADDING)
    {
        return NULL;
    }
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "rootParser->m_parentParser == NULL"
    );
    let bytesToAllocate: size_t = (size_of::<size_t>())
        .wrapping_add(EXPAT_MALLOC_PADDING)
        .wrapping_add(size);
    if (-(1i32) as XmlBigCount).wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
        < bytesToAllocate as XmlBigCount
    {
        return NULL;
    }
    if !expat_heap_increase_tolerable(rootParser, bytesToAllocate as XmlBigCount, sourceLine) {
        return NULL;
    }
    let mallocedPtr: *mut c_void = (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(bytesToAllocate);
    if mallocedPtr.is_null() {
        return NULL;
    }
    *(mallocedPtr as *mut size_t) = size;
    (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_add(bytesToAllocate as XmlBigCount);
    if (*rootParser).m_alloc_tracker.debugLevel >= 2 {
        if (*rootParser).m_alloc_tracker.bytesAllocated
            > (*rootParser).m_alloc_tracker.peakBytesAllocated
        {
            (*rootParser).m_alloc_tracker.peakBytesAllocated =
                (*rootParser).m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            rootParser,
            '+' as c_char,
            bytesToAllocate as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    return (mallocedPtr as *mut c_char)
        .offset(size_of::<size_t>() as isize)
        .offset(EXPAT_MALLOC_PADDING as isize) as *mut c_void;
}
#[cfg_attr(feature = "xml-testing", no_mangle)]
pub unsafe extern "C" fn expat_free(
    mut parser: XML_Parser,
    mut ptr: *mut c_void,
    mut sourceLine: c_int,
) {
    assert!(!parser.is_null(), "parser != NULL");
    if ptr.is_null() {
        return;
    }
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "rootParser->m_parentParser == NULL"
    );
    let mallocedPtr: *mut c_void = (ptr as *mut c_char)
        .offset(-(EXPAT_MALLOC_PADDING as isize))
        .offset(-(size_of::<size_t>() as isize)) as *mut c_void;
    let bytesAllocated: size_t = (size_of::<size_t>())
        .wrapping_add(EXPAT_MALLOC_PADDING)
        .wrapping_add(*(mallocedPtr as *mut size_t));
    assert!(
        (*rootParser).m_alloc_tracker.bytesAllocated >= bytesAllocated as XmlBigCount,
        "rootParser->m_alloc_tracker.bytesAllocated >= bytesAllocated"
    );
    (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_sub(bytesAllocated as XmlBigCount);
    if (*rootParser).m_alloc_tracker.debugLevel >= 2 {
        expat_heap_stat(
            rootParser,
            '-' as c_char,
            bytesAllocated as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(mallocedPtr);
}
#[cfg_attr(feature = "xml-testing", no_mangle)]
pub unsafe extern "C" fn expat_realloc(
    mut parser: XML_Parser,
    mut ptr: *mut c_void,
    mut size: size_t,
    mut sourceLine: c_int,
) -> *mut c_void {
    assert!(!parser.is_null(), "parser != NULL");
    if ptr.is_null() {
        return expat_malloc(parser, size, sourceLine);
    }
    if size == 0usize {
        expat_free(parser, ptr, sourceLine);
        return NULL;
    }
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "rootParser->m_parentParser == NULL"
    );
    let mut mallocedPtr: *mut c_void = (ptr as *mut c_char)
        .offset(-(EXPAT_MALLOC_PADDING as isize))
        .offset(-(size_of::<size_t>() as isize))
        as *mut c_void;
    let prevSize: size_t = *(mallocedPtr as *mut size_t);
    let isIncrease: bool = size > prevSize;
    let absDiff: size_t = if size > prevSize {
        size.wrapping_sub(prevSize)
    } else {
        prevSize.wrapping_sub(size)
    };
    if isIncrease {
        if !expat_heap_increase_tolerable(rootParser, absDiff as XmlBigCount, sourceLine) {
            return NULL;
        }
    }
    assert!(
        (18446744073709551615 as usize)
            .wrapping_sub(size_of::<size_t>())
            .wrapping_sub((size_of::<::core::ffi::c_longlong>()).wrapping_sub(size_of::<size_t>()))
            >= size,
        "SIZE_MAX - sizeof(size_t) - EXPAT_MALLOC_PADDING >= size"
    );
    mallocedPtr = (*parser)
        .m_mem
        .realloc_fcn
        .expect("non-null function pointer")(
        mallocedPtr,
        (size_of::<size_t>())
            .wrapping_add(EXPAT_MALLOC_PADDING)
            .wrapping_add(size),
    );
    if mallocedPtr.is_null() {
        return NULL;
    }
    if isIncrease {
        assert!(
            (-(1i32) as XmlBigCount).wrapping_sub((*rootParser).m_alloc_tracker.bytesAllocated)
                >= absDiff as XmlBigCount,
            "(XmlBigCount)-1 - rootParser->m_alloc_tracker.bytesAllocated >= absDiff"
        );
        (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_add(absDiff as XmlBigCount);
    } else {
        assert!(
            (*rootParser).m_alloc_tracker.bytesAllocated >= absDiff as XmlBigCount,
            "rootParser->m_alloc_tracker.bytesAllocated >= absDiff"
        );
        (*rootParser).m_alloc_tracker.bytesAllocated = (*rootParser)
            .m_alloc_tracker
            .bytesAllocated
            .wrapping_sub(absDiff as XmlBigCount);
    }
    if (*rootParser).m_alloc_tracker.debugLevel >= 2 {
        if (*rootParser).m_alloc_tracker.bytesAllocated
            > (*rootParser).m_alloc_tracker.peakBytesAllocated
        {
            (*rootParser).m_alloc_tracker.peakBytesAllocated =
                (*rootParser).m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            rootParser,
            (if isIncrease as c_int != 0 {
                '+' as i32
            } else {
                '-' as i32
            }) as c_char,
            absDiff as XmlBigCount,
            (*rootParser).m_alloc_tracker.bytesAllocated,
            (*rootParser).m_alloc_tracker.peakBytesAllocated,
            sourceLine,
        );
    }
    *(mallocedPtr as *mut size_t) = size;
    return (mallocedPtr as *mut c_char)
        .offset(size_of::<size_t>() as isize)
        .offset(EXPAT_MALLOC_PADDING as isize) as *mut c_void;
}
#[no_mangle]

pub unsafe extern "C" fn XML_ParserCreate(mut encodingName: *const XML_Char) -> XML_Parser {
    return XML_ParserCreate_MM(
        encodingName,
        null::<XML_Memory_Handling_Suite>(),
        null::<XML_Char>(),
    );
}
#[no_mangle]

pub unsafe extern "C" fn XML_ParserCreateNS(
    mut encodingName: *const XML_Char,
    mut nsSep: XML_Char,
) -> XML_Parser {
    let mut tmp: [XML_Char; 2] = [nsSep, 0];
    return XML_ParserCreate_MM(
        encodingName,
        null::<XML_Memory_Handling_Suite>(),
        &raw mut tmp as *mut XML_Char,
    );
}

static mut implicitContext: [XML_Char; 41] = [
    ASCII_x as XML_Char,
    ASCII_m as XML_Char,
    ASCII_l as XML_Char,
    ASCII_EQUALS as XML_Char,
    ASCII_h as XML_Char,
    ASCII_t as XML_Char,
    ASCII_t as XML_Char,
    ASCII_p as XML_Char,
    ASCII_COLON as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_w as XML_Char,
    ASCII_w as XML_Char,
    ASCII_w as XML_Char,
    ASCII_PERIOD as XML_Char,
    ASCII_w as XML_Char,
    ASCII_3 as XML_Char,
    ASCII_PERIOD as XML_Char,
    ASCII_o as XML_Char,
    ASCII_r as XML_Char,
    ASCII_g as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_X as XML_Char,
    ASCII_M as XML_Char,
    ASCII_L as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_1 as XML_Char,
    ASCII_9 as XML_Char,
    ASCII_9 as XML_Char,
    ASCII_8 as XML_Char,
    ASCII_SLASH as XML_Char,
    ASCII_n as XML_Char,
    ASCII_a as XML_Char,
    ASCII_m as XML_Char,
    ASCII_e as XML_Char,
    ASCII_s as XML_Char,
    ASCII_p as XML_Char,
    ASCII_a as XML_Char,
    ASCII_c as XML_Char,
    ASCII_e as XML_Char,
    '\0' as XML_Char,
];

unsafe extern "C" fn writeRandomBytes_getrandom_nonblock(
    mut target: *mut c_void,
    mut count: size_t,
) -> c_int {
    let mut success: c_int = 0;
    let mut bytesWrittenTotal: size_t = 0;
    let getrandomFlags: c_uint = GRND_NONBLOCK as c_uint;
    loop {
        let currentTarget: *mut c_void =
            (target as *mut c_char).offset(bytesWrittenTotal as isize) as *mut c_void;
        let bytesToWrite: size_t = count.wrapping_sub(bytesWrittenTotal);
        assert!(
            bytesToWrite <= 2147483647i32 as size_t,
            "bytesToWrite <= INT_MAX"
        );
        let bytesWrittenMore: c_int =
            getrandom(currentTarget, bytesToWrite, getrandomFlags) as c_int;
        if bytesWrittenMore > 0 {
            bytesWrittenTotal = bytesWrittenTotal.wrapping_add(bytesWrittenMore as size_t);
            if bytesWrittenTotal >= count {
                success = 1i32;
            }
        }
        if !(success == 0 && *__errno_location() == EINTR) {
            break;
        }
    }
    return success;
}

unsafe extern "C" fn writeRandomBytes_dev_urandom(
    mut target: *mut c_void,
    mut count: size_t,
) -> c_int {
    let mut success: c_int = 0;
    let mut bytesWrittenTotal: size_t = 0;
    let fd: c_int = crate::stdlib::open(b"/dev/urandom\0" as *const u8 as *const c_char, O_RDONLY);
    if fd < 0 {
        return 0i32;
    }
    loop {
        let currentTarget: *mut c_void =
            (target as *mut c_char).offset(bytesWrittenTotal as isize) as *mut c_void;
        let bytesToWrite: size_t = count.wrapping_sub(bytesWrittenTotal);
        let bytesWrittenMore: ssize_t = crate::stdlib::read(fd, currentTarget, bytesToWrite);
        if bytesWrittenMore > 0 {
            bytesWrittenTotal = bytesWrittenTotal.wrapping_add(bytesWrittenMore as size_t);
            if bytesWrittenTotal >= count {
                success = 1i32;
            }
        }
        if !(success == 0 && *__errno_location() == EINTR) {
            break;
        }
    }
    crate::stdlib::close(fd);
    return success;
}

unsafe extern "C" fn gather_time_entropy() -> c_ulong {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut gettimeofday_res: c_int = 0;
    gettimeofday_res = crate::stdlib::gettimeofday(&raw mut tv, NULL);
    assert!(gettimeofday_res == 0, "gettimeofday_res == 0");
    return tv.tv_usec as c_ulong;
}

unsafe extern "C" fn ENTROPY_DEBUG(mut label: *const c_char, mut entropy: c_ulong) -> c_ulong {
    if getDebugLevel(b"EXPAT_ENTROPY_DEBUG\0" as *const u8 as *const c_char, 0) >= 1 {
        fprintf(
            stderr,
            b"expat: Entropy: %s --> 0x%0*lx (%lu bytes)\n\0" as *const u8 as *const c_char,
            label,
            size_of::<c_ulong>() as c_int * 2i32,
            entropy,
            size_of::<c_ulong>() as c_ulong,
        );
    }
    return entropy;
}

unsafe extern "C" fn generate_hash_secret_salt(mut _parser: XML_Parser) -> c_ulong {
    let mut entropy: c_ulong = 0;
    if writeRandomBytes_getrandom_nonblock(&raw mut entropy as *mut c_void, size_of::<c_ulong>())
        != 0
    {
        return ENTROPY_DEBUG(b"getrandom\0" as *const u8 as *const c_char, entropy);
    }
    if writeRandomBytes_dev_urandom(&raw mut entropy as *mut c_void, size_of::<c_ulong>()) != 0 {
        return ENTROPY_DEBUG(b"/dev/urandom\0" as *const u8 as *const c_char, entropy);
    }
    entropy = gather_time_entropy();
    entropy ^= crate::stdlib::getpid() as c_ulong;
    if size_of::<c_ulong>() == 4 {
        return ENTROPY_DEBUG(
            b"fallback(4)\0" as *const u8 as *const c_char,
            entropy.wrapping_mul(2147483647u64),
        );
    } else {
        return ENTROPY_DEBUG(
            b"fallback(8)\0" as *const u8 as *const c_char,
            entropy.wrapping_mul(2305843009213693951u64),
        );
    };
}

unsafe extern "C" fn get_hash_secret_salt(mut parser: XML_Parser) -> c_ulong {
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    return (*rootParser).m_hash_secret_salt;
}

unsafe extern "C" fn callProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let have_now: size_t = (if !end.is_null() && !start.is_null() {
        end.offset_from(start) as c_long
    } else {
        0
    }) as size_t;
    if (*parser).m_reparseDeferralEnabled as c_int != 0
        && (*parser).m_parsingStatus.finalBuffer == 0
    {
        let had_before: size_t = (*parser).m_partialTokenBytesBefore;
        let mut available_buffer: size_t =
            (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as c_long
            } else {
                0
            }) as size_t;
        available_buffer = available_buffer.wrapping_sub(if available_buffer < 1024 {
            available_buffer
        } else {
            1024usize
        });
        available_buffer = available_buffer.wrapping_add(
            (if !(*parser).m_bufferLim.is_null() && !(*parser).m_bufferEnd.is_null() {
                (*parser).m_bufferLim.offset_from((*parser).m_bufferEnd) as c_long
            } else {
                0
            }) as size_t,
        );
        let enough: bool = have_now >= (2usize).wrapping_mul(had_before)
            || (*parser).m_lastBufferRequestSize as size_t > available_buffer;
        if !enough {
            *endPtr = start;
            return XML_ERROR_NONE;
        }
    }
    #[cfg(feature = "xml-testing")]
    {
        g_bytesScanned = g_bytesScanned.wrapping_add(have_now as c_uint);
    }
    let mut ret: XML_Error = XML_ERROR_NONE;
    *endPtr = start;
    loop {
        ret =
            (*parser).m_processor.expect("non-null function pointer")(parser, *endPtr, end, endPtr);
        if (*parser).m_parsingStatus.parsing != XML_PARSING {
            (*parser).m_reenter = XML_FALSE;
        }
        if (*parser).m_reenter == 0 {
            break;
        }
        (*parser).m_reenter = XML_FALSE;
        if ret != XML_ERROR_NONE {
            return ret;
        }
    }
    if ret == XML_ERROR_NONE {
        if *endPtr == start {
            (*parser).m_partialTokenBytesBefore = have_now;
        } else {
            (*parser).m_partialTokenBytesBefore = 0usize;
        }
    }
    return ret;
}

unsafe extern "C" fn startParsing(mut parser: XML_Parser) -> XML_Bool {
    if (*parser).m_hash_secret_salt == 0u64 {
        (*parser).m_hash_secret_salt = generate_hash_secret_salt(parser);
    }
    if (*parser).m_ns != 0 {
        return setContext(parser, &raw const implicitContext as *const XML_Char);
    }
    return XML_TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn XML_ParserCreate_MM(
    mut encodingName: *const XML_Char,
    mut memsuite: *const XML_Memory_Handling_Suite,
    mut nameSep: *const XML_Char,
) -> XML_Parser {
    return parserCreate(
        encodingName,
        memsuite,
        nameSep,
        null_mut::<DTD>(),
        null_mut::<XML_ParserStruct>(),
    );
}

unsafe extern "C" fn parserCreate(
    mut encodingName: *const XML_Char,
    mut memsuite: *const XML_Memory_Handling_Suite,
    mut nameSep: *const XML_Char,
    mut dtd: *mut DTD,
    mut parentParser: XML_Parser,
) -> XML_Parser {
    let mut parser: XML_Parser = null_mut::<XML_ParserStruct>();
    let increase: size_t = (size_of::<size_t>())
        .wrapping_add(EXPAT_MALLOC_PADDING)
        .wrapping_add(size_of::<XML_ParserStruct>());
    if !parentParser.is_null() {
        let rootParser: XML_Parser = getRootParserOf(parentParser, null_mut::<c_uint>());
        if !expat_heap_increase_tolerable(rootParser, increase as XmlBigCount, 1354) {
            return null_mut::<XML_ParserStruct>();
        }
    }
    if !memsuite.is_null() {
        let mut mtemp: *mut XML_Memory_Handling_Suite = null_mut::<XML_Memory_Handling_Suite>();
        let sizeAndParser: *mut c_void = (*memsuite).malloc_fcn.expect("non-null function pointer")(
            (size_of::<size_t>())
                .wrapping_add(EXPAT_MALLOC_PADDING)
                .wrapping_add(size_of::<XML_ParserStruct>()),
        );
        if !sizeAndParser.is_null() {
            *(sizeAndParser as *mut size_t) = size_of::<XML_ParserStruct>();
            parser = (sizeAndParser as *mut c_char)
                .offset(size_of::<size_t>() as isize)
                .offset(EXPAT_MALLOC_PADDING as isize) as XML_Parser;
            mtemp = &raw const (*parser).m_mem as *mut XML_Memory_Handling_Suite;
            (*mtemp).malloc_fcn = (*memsuite).malloc_fcn;
            (*mtemp).realloc_fcn = (*memsuite).realloc_fcn;
            (*mtemp).free_fcn = (*memsuite).free_fcn;
        }
    } else {
        let mut mtemp_0: *mut XML_Memory_Handling_Suite = null_mut::<XML_Memory_Handling_Suite>();
        let sizeAndParser_0: *mut c_void = crate::stdlib::malloc(
            (size_of::<size_t>())
                .wrapping_add(EXPAT_MALLOC_PADDING)
                .wrapping_add(size_of::<XML_ParserStruct>()),
        );
        if !sizeAndParser_0.is_null() {
            *(sizeAndParser_0 as *mut size_t) = size_of::<XML_ParserStruct>();
            parser = (sizeAndParser_0 as *mut c_char)
                .offset(size_of::<size_t>() as isize)
                .offset(EXPAT_MALLOC_PADDING as isize) as XML_Parser;
            mtemp_0 = &raw const (*parser).m_mem as *mut XML_Memory_Handling_Suite;
            (*mtemp_0).malloc_fcn =
                Some(crate::stdlib::malloc as unsafe extern "C" fn(size_t) -> *mut c_void);
            (*mtemp_0).realloc_fcn = Some(
                crate::stdlib::realloc as unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void,
            );
            (*mtemp_0).free_fcn =
                Some(crate::stdlib::free as unsafe extern "C" fn(*mut c_void) -> ());
        }
    }
    if parser.is_null() {
        return parser;
    }
    memset(
        &raw mut (*parser).m_alloc_tracker as *mut c_void,
        0,
        size_of::<MALLOC_TRACKER>(),
    );
    if parentParser.is_null() {
        (*parser).m_alloc_tracker.debugLevel =
            getDebugLevel(b"EXPAT_MALLOC_DEBUG\0" as *const u8 as *const c_char, 0);
        (*parser).m_alloc_tracker.maximumAmplificationFactor =
            EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT;
        (*parser).m_alloc_tracker.activationThresholdBytes =
            EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT as XmlBigCount;
        (*parser).m_parentParser = null_mut::<XML_ParserStruct>();
        (*parser).m_accounting.countBytesDirect = 0u64;
    } else {
        (*parser).m_parentParser = parentParser;
    }
    let rootParser_0: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(
        (*rootParser_0).m_parentParser.is_null(),
        "rootParser->m_parentParser == NULL"
    );
    assert!(
        (18446744073709551615u64).wrapping_sub((*rootParser_0).m_alloc_tracker.bytesAllocated)
            >= increase as XmlBigCount,
        "SIZE_MAX - rootParser->m_alloc_tracker.bytesAllocated >= increase"
    );
    (*rootParser_0).m_alloc_tracker.bytesAllocated = (*rootParser_0)
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_add(increase as XmlBigCount);
    if (*rootParser_0).m_alloc_tracker.debugLevel >= 2 {
        if (*rootParser_0).m_alloc_tracker.bytesAllocated
            > (*rootParser_0).m_alloc_tracker.peakBytesAllocated
        {
            (*rootParser_0).m_alloc_tracker.peakBytesAllocated =
                (*rootParser_0).m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            rootParser_0,
            '+' as c_char,
            increase as XmlBigCount,
            (*rootParser_0).m_alloc_tracker.bytesAllocated,
            (*rootParser_0).m_alloc_tracker.peakBytesAllocated,
            1439i32,
        );
    }
    (*parser).m_buffer = null_mut::<c_char>();
    (*parser).m_bufferLim = null::<c_char>();
    (*parser).m_attsSize = INIT_ATTS_SIZE;
    (*parser).m_atts = expat_malloc(
        parser,
        ((*parser).m_attsSize as size_t).wrapping_mul(size_of::<ATTRIBUTE>()),
        1449,
    ) as *mut ATTRIBUTE;
    if (*parser).m_atts.is_null() {
        expat_free(parser, parser as *mut c_void, 1451);
        return null_mut::<XML_ParserStruct>();
    }
    (*parser).m_dataBuf = expat_malloc(
        parser,
        (1024usize).wrapping_mul(size_of::<XML_Char>()),
        1462,
    ) as *mut XML_Char;
    if (*parser).m_dataBuf.is_null() {
        expat_free(parser, (*parser).m_atts as *mut c_void, 1464);
        expat_free(parser, parser as *mut c_void, 1468);
        return null_mut::<XML_ParserStruct>();
    }
    (*parser).m_dataBufEnd = (*parser).m_dataBuf.offset(INIT_DATA_BUF_SIZE as isize);
    if !dtd.is_null() {
        (*parser).m_dtd = dtd;
    } else {
        (*parser).m_dtd = dtdCreate(parser);
        if (*parser).m_dtd.is_null() {
            expat_free(parser, (*parser).m_dataBuf as *mut c_void, 1478);
            expat_free(parser, (*parser).m_atts as *mut c_void, 1479);
            expat_free(parser, parser as *mut c_void, 1483);
            return null_mut::<XML_ParserStruct>();
        }
    }
    (*parser).m_freeBindingList = null_mut::<BINDING>();
    (*parser).m_freeTagList = null_mut::<TAG>();
    (*parser).m_freeInternalEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_freeAttributeEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_freeValueEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_groupSize = 0;
    (*parser).m_groupConnector = null_mut::<c_char>();
    (*parser).m_unknownEncodingHandler = None;
    (*parser).m_unknownEncodingHandlerData = NULL;
    (*parser).m_namespaceSeparator = ASCII_EXCL as XML_Char;
    (*parser).m_ns = XML_FALSE;
    (*parser).m_ns_triplets = XML_FALSE;
    (*parser).m_nsAtts = null_mut::<NS_ATT>();
    (*parser).m_nsAttsVersion = 0;
    (*parser).m_nsAttsPower = 0;
    (*parser).m_protocolEncodingName = null::<XML_Char>();
    poolInit(&raw mut (*parser).m_tempPool, parser);
    poolInit(&raw mut (*parser).m_temp2Pool, parser);
    parserInit(parser, encodingName);
    if !encodingName.is_null() && (*parser).m_protocolEncodingName.is_null() {
        if !dtd.is_null() {
            (*parser).m_dtd = null_mut::<DTD>();
        }
        XML_ParserFree(parser);
        return null_mut::<XML_ParserStruct>();
    }
    if !nameSep.is_null() {
        (*parser).m_ns = XML_TRUE;
        (*parser).m_internalEncoding = XmlGetUtf8InternalEncodingNS();
        (*parser).m_namespaceSeparator = *nameSep;
    } else {
        (*parser).m_internalEncoding = XmlGetUtf8InternalEncoding();
    }
    return parser;
}

unsafe extern "C" fn parserInit(mut parser: XML_Parser, mut encodingName: *const XML_Char) {
    (*parser).m_processor = Some(
        prologInitProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    XmlPrologStateInit(&raw mut (*parser).m_prologState);
    if !encodingName.is_null() {
        (*parser).m_protocolEncodingName = copyString(encodingName, parser);
    }
    (*parser).m_curBase = null::<XML_Char>();
    XmlInitEncoding(
        &raw mut (*parser).m_initEncoding,
        &raw mut (*parser).m_encoding,
        null::<c_char>(),
    );
    (*parser).m_userData = NULL;
    (*parser).m_handlerArg = NULL;
    (*parser).m_startElementHandler = None;
    (*parser).m_endElementHandler = None;
    (*parser).m_characterDataHandler = None;
    (*parser).m_processingInstructionHandler = None;
    (*parser).m_commentHandler = None;
    (*parser).m_startCdataSectionHandler = None;
    (*parser).m_endCdataSectionHandler = None;
    (*parser).m_defaultHandler = None;
    (*parser).m_startDoctypeDeclHandler = None;
    (*parser).m_endDoctypeDeclHandler = None;
    (*parser).m_unparsedEntityDeclHandler = None;
    (*parser).m_notationDeclHandler = None;
    (*parser).m_startNamespaceDeclHandler = None;
    (*parser).m_endNamespaceDeclHandler = None;
    (*parser).m_notStandaloneHandler = None;
    (*parser).m_externalEntityRefHandler = None;
    (*parser).m_externalEntityRefHandlerArg = parser;
    (*parser).m_skippedEntityHandler = None;
    (*parser).m_elementDeclHandler = None;
    (*parser).m_attlistDeclHandler = None;
    (*parser).m_entityDeclHandler = None;
    (*parser).m_xmlDeclHandler = None;
    (*parser).m_bufferPtr = (*parser).m_buffer;
    (*parser).m_bufferEnd = (*parser).m_buffer;
    (*parser).m_parseEndByteIndex = 0i64;
    (*parser).m_parseEndPtr = null::<c_char>();
    (*parser).m_partialTokenBytesBefore = 0usize;
    (*parser).m_reparseDeferralEnabled = g_reparseDeferralEnabledDefault;
    (*parser).m_lastBufferRequestSize = 0;
    (*parser).m_declElementType = null_mut::<ELEMENT_TYPE>();
    (*parser).m_declAttributeId = null_mut::<ATTRIBUTE_ID>();
    (*parser).m_declEntity = null_mut::<ENTITY>();
    (*parser).m_doctypeName = null::<XML_Char>();
    (*parser).m_doctypeSysid = null::<XML_Char>();
    (*parser).m_doctypePubid = null::<XML_Char>();
    (*parser).m_declAttributeType = null::<XML_Char>();
    (*parser).m_declNotationName = null::<XML_Char>();
    (*parser).m_declNotationPublicId = null::<XML_Char>();
    (*parser).m_declAttributeIsCdata = XML_FALSE;
    (*parser).m_declAttributeIsId = XML_FALSE;
    memset(
        &raw mut (*parser).m_position as *mut c_void,
        0,
        size_of::<POSITION>(),
    );
    (*parser).m_errorCode = XML_ERROR_NONE;
    (*parser).m_eventPtr = null::<c_char>();
    (*parser).m_eventEndPtr = null::<c_char>();
    (*parser).m_positionPtr = null::<c_char>();
    (*parser).m_openInternalEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_openAttributeEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_openValueEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    (*parser).m_defaultExpandInternalEntities = XML_TRUE;
    (*parser).m_tagLevel = 0;
    (*parser).m_tagStack = null_mut::<TAG>();
    (*parser).m_inheritedBindings = null_mut::<BINDING>();
    (*parser).m_nSpecifiedAtts = 0;
    (*parser).m_unknownEncodingMem = NULL;
    (*parser).m_unknownEncodingRelease = None;
    (*parser).m_unknownEncodingData = NULL;
    (*parser).m_parsingStatus.parsing = XML_INITIALIZED;
    (*parser).m_reenter = XML_FALSE;
    (*parser).m_isParamEntity = XML_FALSE;
    (*parser).m_useForeignDTD = XML_FALSE;
    (*parser).m_paramEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    (*parser).m_hash_secret_salt = 0u64;
    memset(
        &raw mut (*parser).m_accounting as *mut c_void,
        0,
        size_of::<ACCOUNTING>(),
    );
    (*parser).m_accounting.debugLevel =
        getDebugLevel(b"EXPAT_ACCOUNTING_DEBUG\0" as *const u8 as *const c_char, 0);
    (*parser).m_accounting.maximumAmplificationFactor =
        EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT;
    (*parser).m_accounting.activationThresholdBytes =
        EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT as c_ulonglong;
    memset(
        &raw mut (*parser).m_entity_stats as *mut c_void,
        0,
        size_of::<ENTITY_STATS>(),
    );
    (*parser).m_entity_stats.debugLevel =
        getDebugLevel(b"EXPAT_ENTITY_DEBUG\0" as *const u8 as *const c_char, 0);
}

unsafe extern "C" fn moveToFreeBindingList(mut parser: XML_Parser, mut bindings: *mut BINDING) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        bindings = (*bindings).nextTagBinding;
        (*b).nextTagBinding = (*parser).m_freeBindingList;
        (*parser).m_freeBindingList = b;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_ParserReset(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Bool {
    let mut tStk: *mut TAG = null_mut::<TAG>();
    let mut openEntityList: *mut OPEN_INTERNAL_ENTITY = null_mut::<OPEN_INTERNAL_ENTITY>();
    if parser.is_null() {
        return XML_FALSE;
    }
    if !(*parser).m_parentParser.is_null() {
        return XML_FALSE;
    }
    tStk = (*parser).m_tagStack;
    while !tStk.is_null() {
        let mut tag: *mut TAG = tStk;
        tStk = (*tStk).parent;
        (*tag).parent = (*parser).m_freeTagList;
        moveToFreeBindingList(parser, (*tag).bindings);
        (*tag).bindings = null_mut::<BINDING>();
        (*parser).m_freeTagList = tag;
    }
    openEntityList = (*parser).m_openInternalEntities;
    while !openEntityList.is_null() {
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity).next;
        (*openEntity).next = (*parser).m_freeInternalEntities;
        (*parser).m_freeInternalEntities = openEntity;
    }
    openEntityList = (*parser).m_openAttributeEntities;
    while !openEntityList.is_null() {
        let mut openEntity_0: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity_0).next;
        (*openEntity_0).next = (*parser).m_freeAttributeEntities;
        (*parser).m_freeAttributeEntities = openEntity_0;
    }
    openEntityList = (*parser).m_openValueEntities;
    while !openEntityList.is_null() {
        let mut openEntity_1: *mut OPEN_INTERNAL_ENTITY = openEntityList;
        openEntityList = (*openEntity_1).next;
        (*openEntity_1).next = (*parser).m_freeValueEntities;
        (*parser).m_freeValueEntities = openEntity_1;
    }
    moveToFreeBindingList(parser, (*parser).m_inheritedBindings);
    expat_free(parser, (*parser).m_unknownEncodingMem, 1686);
    if (*parser).m_unknownEncodingRelease.is_some() {
        (*parser)
            .m_unknownEncodingRelease
            .expect("non-null function pointer")((*parser).m_unknownEncodingData);
    }
    poolClear(&raw mut (*parser).m_tempPool);
    poolClear(&raw mut (*parser).m_temp2Pool);
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut c_void,
        1691,
    );
    (*parser).m_protocolEncodingName = null::<XML_Char>();
    parserInit(parser, encodingName);
    dtdReset((*parser).m_dtd, parser);
    return XML_TRUE;
}

unsafe extern "C" fn parserBusy(mut parser: XML_Parser) -> XML_Bool {
    match (*parser).m_parsingStatus.parsing {
        1 | 3 => return XML_TRUE,
        0 | 2 | _ => return XML_FALSE,
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEncoding(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if parserBusy(parser) != 0 {
        return XML_STATUS_ERROR;
    }
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut c_void,
        1723,
    );
    if encodingName.is_null() {
        (*parser).m_protocolEncodingName = null::<XML_Char>();
    } else {
        (*parser).m_protocolEncodingName = copyString(encodingName, parser);
        if (*parser).m_protocolEncodingName.is_null() {
            return XML_STATUS_ERROR;
        }
    }
    return XML_STATUS_OK;
}
#[no_mangle]

pub unsafe extern "C" fn XML_ExternalEntityParserCreate(
    mut oldParser: XML_Parser,
    mut context: *const XML_Char,
    mut encodingName: *const XML_Char,
) -> XML_Parser {
    let mut parser: XML_Parser = oldParser;
    let mut newDtd: *mut DTD = null_mut::<DTD>();
    let mut oldDtd: *mut DTD = null_mut::<DTD>();
    let mut oldStartElementHandler: XML_StartElementHandler = None;
    let mut oldEndElementHandler: XML_EndElementHandler = None;
    let mut oldCharacterDataHandler: XML_CharacterDataHandler = None;
    let mut oldProcessingInstructionHandler: XML_ProcessingInstructionHandler = None;
    let mut oldCommentHandler: XML_CommentHandler = None;
    let mut oldStartCdataSectionHandler: XML_StartCdataSectionHandler = None;
    let mut oldEndCdataSectionHandler: XML_EndCdataSectionHandler = None;
    let mut oldDefaultHandler: XML_DefaultHandler = None;
    let mut oldUnparsedEntityDeclHandler: XML_UnparsedEntityDeclHandler = None;
    let mut oldNotationDeclHandler: XML_NotationDeclHandler = None;
    let mut oldStartNamespaceDeclHandler: XML_StartNamespaceDeclHandler = None;
    let mut oldEndNamespaceDeclHandler: XML_EndNamespaceDeclHandler = None;
    let mut oldNotStandaloneHandler: XML_NotStandaloneHandler = None;
    let mut oldExternalEntityRefHandler: XML_ExternalEntityRefHandler = None;
    let mut oldSkippedEntityHandler: XML_SkippedEntityHandler = None;
    let mut oldUnknownEncodingHandler: XML_UnknownEncodingHandler = None;
    let mut oldUnknownEncodingHandlerData: *mut c_void = null_mut::<c_void>();
    let mut oldElementDeclHandler: XML_ElementDeclHandler = None;
    let mut oldAttlistDeclHandler: XML_AttlistDeclHandler = None;
    let mut oldEntityDeclHandler: XML_EntityDeclHandler = None;
    let mut oldXmlDeclHandler: XML_XmlDeclHandler = None;
    let mut oldDeclElementType: *mut ELEMENT_TYPE = null_mut::<ELEMENT_TYPE>();
    let mut oldUserData: *mut c_void = null_mut::<c_void>();
    let mut oldHandlerArg: *mut c_void = null_mut::<c_void>();
    let mut oldDefaultExpandInternalEntities: XML_Bool = 0;
    let mut oldExternalEntityRefHandlerArg: XML_Parser = null_mut::<XML_ParserStruct>();
    let mut oldParamEntityParsing: XML_ParamEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    let mut oldInEntityValue: c_int = 0;
    let mut oldns_triplets: XML_Bool = 0;
    let mut oldhash_secret_salt: c_ulong = 0;
    let mut oldReparseDeferralEnabled: XML_Bool = 0;
    if oldParser.is_null() {
        return null_mut::<XML_ParserStruct>();
    }
    oldDtd = (*parser).m_dtd;
    oldStartElementHandler = (*parser).m_startElementHandler;
    oldEndElementHandler = (*parser).m_endElementHandler;
    oldCharacterDataHandler = (*parser).m_characterDataHandler;
    oldProcessingInstructionHandler = (*parser).m_processingInstructionHandler;
    oldCommentHandler = (*parser).m_commentHandler;
    oldStartCdataSectionHandler = (*parser).m_startCdataSectionHandler;
    oldEndCdataSectionHandler = (*parser).m_endCdataSectionHandler;
    oldDefaultHandler = (*parser).m_defaultHandler;
    oldUnparsedEntityDeclHandler = (*parser).m_unparsedEntityDeclHandler;
    oldNotationDeclHandler = (*parser).m_notationDeclHandler;
    oldStartNamespaceDeclHandler = (*parser).m_startNamespaceDeclHandler;
    oldEndNamespaceDeclHandler = (*parser).m_endNamespaceDeclHandler;
    oldNotStandaloneHandler = (*parser).m_notStandaloneHandler;
    oldExternalEntityRefHandler = (*parser).m_externalEntityRefHandler;
    oldSkippedEntityHandler = (*parser).m_skippedEntityHandler;
    oldUnknownEncodingHandler = (*parser).m_unknownEncodingHandler;
    oldUnknownEncodingHandlerData = (*parser).m_unknownEncodingHandlerData;
    oldElementDeclHandler = (*parser).m_elementDeclHandler;
    oldAttlistDeclHandler = (*parser).m_attlistDeclHandler;
    oldEntityDeclHandler = (*parser).m_entityDeclHandler;
    oldXmlDeclHandler = (*parser).m_xmlDeclHandler;
    oldDeclElementType = (*parser).m_declElementType;
    oldUserData = (*parser).m_userData;
    oldHandlerArg = (*parser).m_handlerArg;
    oldDefaultExpandInternalEntities = (*parser).m_defaultExpandInternalEntities;
    oldExternalEntityRefHandlerArg = (*parser).m_externalEntityRefHandlerArg;
    oldParamEntityParsing = (*parser).m_paramEntityParsing;
    oldInEntityValue = (*parser).m_prologState.inEntityValue;
    oldns_triplets = (*parser).m_ns_triplets;
    oldhash_secret_salt = (*parser).m_hash_secret_salt;
    oldReparseDeferralEnabled = (*parser).m_reparseDeferralEnabled;
    if context.is_null() {
        newDtd = oldDtd;
    }
    if (*parser).m_ns != 0 {
        let mut tmp: [XML_Char; 2] = [(*parser).m_namespaceSeparator, 0];
        parser = parserCreate(
            encodingName,
            &raw const (*parser).m_mem,
            &raw mut tmp as *mut XML_Char,
            newDtd,
            oldParser,
        );
    } else {
        parser = parserCreate(
            encodingName,
            &raw const (*parser).m_mem,
            null::<XML_Char>(),
            newDtd,
            oldParser,
        );
    }
    if parser.is_null() {
        return null_mut::<XML_ParserStruct>();
    }
    (*parser).m_startElementHandler = oldStartElementHandler;
    (*parser).m_endElementHandler = oldEndElementHandler;
    (*parser).m_characterDataHandler = oldCharacterDataHandler;
    (*parser).m_processingInstructionHandler = oldProcessingInstructionHandler;
    (*parser).m_commentHandler = oldCommentHandler;
    (*parser).m_startCdataSectionHandler = oldStartCdataSectionHandler;
    (*parser).m_endCdataSectionHandler = oldEndCdataSectionHandler;
    (*parser).m_defaultHandler = oldDefaultHandler;
    (*parser).m_unparsedEntityDeclHandler = oldUnparsedEntityDeclHandler;
    (*parser).m_notationDeclHandler = oldNotationDeclHandler;
    (*parser).m_startNamespaceDeclHandler = oldStartNamespaceDeclHandler;
    (*parser).m_endNamespaceDeclHandler = oldEndNamespaceDeclHandler;
    (*parser).m_notStandaloneHandler = oldNotStandaloneHandler;
    (*parser).m_externalEntityRefHandler = oldExternalEntityRefHandler;
    (*parser).m_skippedEntityHandler = oldSkippedEntityHandler;
    (*parser).m_unknownEncodingHandler = oldUnknownEncodingHandler;
    (*parser).m_unknownEncodingHandlerData = oldUnknownEncodingHandlerData;
    (*parser).m_elementDeclHandler = oldElementDeclHandler;
    (*parser).m_attlistDeclHandler = oldAttlistDeclHandler;
    (*parser).m_entityDeclHandler = oldEntityDeclHandler;
    (*parser).m_xmlDeclHandler = oldXmlDeclHandler;
    (*parser).m_declElementType = oldDeclElementType;
    (*parser).m_userData = oldUserData;
    if oldUserData == oldHandlerArg {
        (*parser).m_handlerArg = (*parser).m_userData;
    } else {
        (*parser).m_handlerArg = parser as *mut c_void;
    }
    if oldExternalEntityRefHandlerArg != oldParser {
        (*parser).m_externalEntityRefHandlerArg = oldExternalEntityRefHandlerArg;
    }
    (*parser).m_defaultExpandInternalEntities = oldDefaultExpandInternalEntities;
    (*parser).m_ns_triplets = oldns_triplets;
    (*parser).m_hash_secret_salt = oldhash_secret_salt;
    (*parser).m_reparseDeferralEnabled = oldReparseDeferralEnabled;
    (*parser).m_parentParser = oldParser;
    (*parser).m_paramEntityParsing = oldParamEntityParsing;
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    if !context.is_null() {
        if dtdCopy(oldParser, (*parser).m_dtd, oldDtd, parser) == 0
            || setContext(parser, context) == 0
        {
            XML_ParserFree(parser);
            return null_mut::<XML_ParserStruct>();
        }
        (*parser).m_processor = Some(
            externalEntityInitProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
    } else {
        (*parser).m_isParamEntity = XML_TRUE;
        XmlPrologStateInitExternalEntity(&raw mut (*parser).m_prologState);
        (*parser).m_processor = Some(
            externalParEntInitProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
    }
    return parser;
}

unsafe extern "C" fn destroyBindings(mut bindings: *mut BINDING, mut parser: XML_Parser) {
    loop {
        let mut b: *mut BINDING = bindings;
        if b.is_null() {
            break;
        }
        bindings = (*b).nextTagBinding;
        expat_free(parser, (*b).uri as *mut c_void, 1919);
        expat_free(parser, b as *mut c_void, 1920);
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_ParserFree(mut parser: XML_Parser) {
    let mut tagList: *mut TAG = null_mut::<TAG>();
    let mut entityList: *mut OPEN_INTERNAL_ENTITY = null_mut::<OPEN_INTERNAL_ENTITY>();
    if parser.is_null() {
        return;
    }
    tagList = (*parser).m_tagStack;
    loop {
        let mut p: *mut TAG = null_mut::<TAG>();
        if tagList.is_null() {
            if (*parser).m_freeTagList.is_null() {
                break;
            }
            tagList = (*parser).m_freeTagList;
            (*parser).m_freeTagList = null_mut::<TAG>();
        }
        p = tagList;
        tagList = (*tagList).parent;
        expat_free(parser, (*p).buf.raw as *mut c_void, 1942);
        destroyBindings((*p).bindings, parser);
        expat_free(parser, p as *mut c_void, 1944);
    }
    entityList = (*parser).m_openInternalEntities;
    loop {
        let mut openEntity: *mut OPEN_INTERNAL_ENTITY = null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if (*parser).m_freeInternalEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeInternalEntities;
            (*parser).m_freeInternalEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity = entityList;
        entityList = (*entityList).next;
        expat_free(parser, openEntity as *mut c_void, 1958);
    }
    entityList = (*parser).m_openAttributeEntities;
    loop {
        let mut openEntity_0: *mut OPEN_INTERNAL_ENTITY = null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if (*parser).m_freeAttributeEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeAttributeEntities;
            (*parser).m_freeAttributeEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity_0 = entityList;
        entityList = (*entityList).next;
        expat_free(parser, openEntity_0 as *mut c_void, 1972);
    }
    entityList = (*parser).m_openValueEntities;
    loop {
        let mut openEntity_1: *mut OPEN_INTERNAL_ENTITY = null_mut::<OPEN_INTERNAL_ENTITY>();
        if entityList.is_null() {
            if (*parser).m_freeValueEntities.is_null() {
                break;
            }
            entityList = (*parser).m_freeValueEntities;
            (*parser).m_freeValueEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        openEntity_1 = entityList;
        entityList = (*entityList).next;
        expat_free(parser, openEntity_1 as *mut c_void, 1986);
    }
    destroyBindings((*parser).m_freeBindingList, parser);
    destroyBindings((*parser).m_inheritedBindings, parser);
    poolDestroy(&raw mut (*parser).m_tempPool);
    poolDestroy(&raw mut (*parser).m_temp2Pool);
    expat_free(
        parser,
        (*parser).m_protocolEncodingName as *mut c_void,
        1992,
    );
    if (*parser).m_isParamEntity == 0 && !(*parser).m_dtd.is_null() {
        dtdDestroy(
            (*parser).m_dtd,
            (*parser).m_parentParser.is_null() as XML_Bool,
            parser,
        );
    }
    expat_free(parser, (*parser).m_atts as *mut c_void, 2002);
    expat_free(parser, (*parser).m_groupConnector as *mut c_void, 2006);
    (*parser).m_mem.free_fcn.expect("non-null function pointer")((*parser).m_buffer as *mut c_void);
    expat_free(parser, (*parser).m_dataBuf as *mut c_void, 2011);
    expat_free(parser, (*parser).m_nsAtts as *mut c_void, 2012);
    expat_free(parser, (*parser).m_unknownEncodingMem, 2013);
    if (*parser).m_unknownEncodingRelease.is_some() {
        (*parser)
            .m_unknownEncodingRelease
            .expect("non-null function pointer")((*parser).m_unknownEncodingData);
    }
    expat_free(parser, parser as *mut c_void, 2016);
}
#[no_mangle]

pub unsafe extern "C" fn XML_UseParserAsHandlerArg(mut parser: XML_Parser) {
    if !parser.is_null() {
        (*parser).m_handlerArg = parser as *mut c_void;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_UseForeignDTD(
    mut parser: XML_Parser,
    mut useDTD: XML_Bool,
) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    if parserBusy(parser) != 0 {
        return XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
    }
    (*parser).m_useForeignDTD = useDTD;
    return XML_ERROR_NONE;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetReturnNSTriplet(mut parser: XML_Parser, mut do_nst: c_int) {
    if parser.is_null() {
        return;
    }
    if parserBusy(parser) != 0 {
        return;
    }
    (*parser).m_ns_triplets = (if do_nst != 0 {
        XML_TRUE as c_int
    } else {
        XML_FALSE as c_int
    }) as XML_Bool;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetUserData(mut parser: XML_Parser, mut p: *mut c_void) {
    if parser.is_null() {
        return;
    }
    if (*parser).m_handlerArg == (*parser).m_userData {
        (*parser).m_userData = p;
        (*parser).m_handlerArg = (*parser).m_userData;
    } else {
        (*parser).m_userData = p;
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetBase(mut parser: XML_Parser, mut p: *const XML_Char) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if !p.is_null() {
        p = poolCopyString(&raw mut (*(*parser).m_dtd).pool, p);
        if p.is_null() {
            return XML_STATUS_ERROR;
        }
        (*parser).m_curBase = p;
    } else {
        (*parser).m_curBase = null::<XML_Char>();
    }
    return XML_STATUS_OK;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetBase(mut parser: XML_Parser) -> *const XML_Char {
    if parser.is_null() {
        return null::<XML_Char>();
    }
    return (*parser).m_curBase;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetSpecifiedAttributeCount(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return -(1i32);
    }
    return (*parser).m_nSpecifiedAtts;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetIdAttributeIndex(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return -(1i32);
    }
    return (*parser).m_idAttIndex;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetElementHandler(
    mut parser: XML_Parser,
    mut start: XML_StartElementHandler,
    mut end: XML_EndElementHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startElementHandler = start;
    (*parser).m_endElementHandler = end;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetStartElementHandler(
    mut parser: XML_Parser,
    mut start: XML_StartElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_startElementHandler = start;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEndElementHandler(
    mut parser: XML_Parser,
    mut end: XML_EndElementHandler,
) {
    if !parser.is_null() {
        (*parser).m_endElementHandler = end;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetCharacterDataHandler(
    mut parser: XML_Parser,
    mut handler: XML_CharacterDataHandler,
) {
    if !parser.is_null() {
        (*parser).m_characterDataHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetProcessingInstructionHandler(
    mut parser: XML_Parser,
    mut handler: XML_ProcessingInstructionHandler,
) {
    if !parser.is_null() {
        (*parser).m_processingInstructionHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetCommentHandler(
    mut parser: XML_Parser,
    mut handler: XML_CommentHandler,
) {
    if !parser.is_null() {
        (*parser).m_commentHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetCdataSectionHandler(
    mut parser: XML_Parser,
    mut start: XML_StartCdataSectionHandler,
    mut end: XML_EndCdataSectionHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startCdataSectionHandler = start;
    (*parser).m_endCdataSectionHandler = end;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetStartCdataSectionHandler(
    mut parser: XML_Parser,
    mut start: XML_StartCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_startCdataSectionHandler = start;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEndCdataSectionHandler(
    mut parser: XML_Parser,
    mut end: XML_EndCdataSectionHandler,
) {
    if !parser.is_null() {
        (*parser).m_endCdataSectionHandler = end;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetDefaultHandler(
    mut parser: XML_Parser,
    mut handler: XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_defaultHandler = handler;
    (*parser).m_defaultExpandInternalEntities = XML_FALSE;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetDefaultHandlerExpand(
    mut parser: XML_Parser,
    mut handler: XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_defaultHandler = handler;
    (*parser).m_defaultExpandInternalEntities = XML_TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartDoctypeDeclHandler,
    mut end: XML_EndDoctypeDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startDoctypeDeclHandler = start;
    (*parser).m_endDoctypeDeclHandler = end;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetStartDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_startDoctypeDeclHandler = start;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEndDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut end: XML_EndDoctypeDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_endDoctypeDeclHandler = end;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetUnparsedEntityDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_UnparsedEntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_unparsedEntityDeclHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetNotationDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_NotationDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_notationDeclHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartNamespaceDeclHandler,
    mut end: XML_EndNamespaceDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_startNamespaceDeclHandler = start;
    (*parser).m_endNamespaceDeclHandler = end;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetStartNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_startNamespaceDeclHandler = start;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEndNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut end: XML_EndNamespaceDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_endNamespaceDeclHandler = end;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetNotStandaloneHandler(
    mut parser: XML_Parser,
    mut handler: XML_NotStandaloneHandler,
) {
    if !parser.is_null() {
        (*parser).m_notStandaloneHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetExternalEntityRefHandler(
    mut parser: XML_Parser,
    mut handler: XML_ExternalEntityRefHandler,
) {
    if !parser.is_null() {
        (*parser).m_externalEntityRefHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetExternalEntityRefHandlerArg(
    mut parser: XML_Parser,
    mut arg: *mut c_void,
) {
    if parser.is_null() {
        return;
    }
    if !arg.is_null() {
        (*parser).m_externalEntityRefHandlerArg = arg as XML_Parser;
    } else {
        (*parser).m_externalEntityRefHandlerArg = parser;
    };
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetSkippedEntityHandler(
    mut parser: XML_Parser,
    mut handler: XML_SkippedEntityHandler,
) {
    if !parser.is_null() {
        (*parser).m_skippedEntityHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetUnknownEncodingHandler(
    mut parser: XML_Parser,
    mut handler: XML_UnknownEncodingHandler,
    mut data: *mut c_void,
) {
    if parser.is_null() {
        return;
    }
    (*parser).m_unknownEncodingHandler = handler;
    (*parser).m_unknownEncodingHandlerData = data;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetElementDeclHandler(
    mut parser: XML_Parser,
    mut eldecl: XML_ElementDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_elementDeclHandler = eldecl;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetAttlistDeclHandler(
    mut parser: XML_Parser,
    mut attdecl: XML_AttlistDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_attlistDeclHandler = attdecl;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetEntityDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_EntityDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_entityDeclHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetXmlDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_XmlDeclHandler,
) {
    if !parser.is_null() {
        (*parser).m_xmlDeclHandler = handler;
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetParamEntityParsing(
    mut parser: XML_Parser,
    mut peParsing: XML_ParamEntityParsing,
) -> c_int {
    if parser.is_null() {
        return 0i32;
    }
    if parserBusy(parser) != 0 {
        return 0i32;
    }
    (*parser).m_paramEntityParsing = peParsing;
    return 1;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetHashSalt(mut parser: XML_Parser, mut hash_salt: c_ulong) -> c_int {
    if parser.is_null() {
        return 0i32;
    }
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    if parserBusy(rootParser) != 0 {
        return 0i32;
    }
    (*rootParser).m_hash_secret_salt = hash_salt;
    return 1;
}
#[no_mangle]

pub unsafe extern "C" fn XML_Parse(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut len: c_int,
    mut isFinal: c_int,
) -> XML_Status {
    if parser.is_null() || len < 0 || s.is_null() && len != 0 {
        if !parser.is_null() {
            (*parser).m_errorCode = XML_ERROR_INVALID_ARGUMENT;
        }
        return XML_STATUS_ERROR;
    }
    match (*parser).m_parsingStatus.parsing {
        3 => {
            (*parser).m_errorCode = XML_ERROR_SUSPENDED;
            return XML_STATUS_ERROR;
        }
        2 => {
            (*parser).m_errorCode = XML_ERROR_FINISHED;
            return XML_STATUS_ERROR;
        }
        0 => {
            if (*parser).m_parentParser.is_null() && startParsing(parser) == 0 {
                (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
                return XML_STATUS_ERROR;
            }
        }
        _ => {}
    }
    (*parser).m_parsingStatus.parsing = XML_PARSING;
    let mut buff: *mut c_void = XML_GetBuffer(parser, len);
    if buff.is_null() {
        return XML_STATUS_ERROR;
    }
    if len > 0 {
        assert!(!s.is_null(), "s != NULL");
        memcpy(buff, s as *const c_void, len as size_t);
    }
    return XML_ParseBuffer(parser, len, isFinal);
}
#[no_mangle]

pub unsafe extern "C" fn XML_ParseBuffer(
    mut parser: XML_Parser,
    mut len: c_int,
    mut isFinal: c_int,
) -> XML_Status {
    let mut start: *const c_char = null::<c_char>();
    let mut result: XML_Status = XML_STATUS_OK;
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if len < 0 {
        (*parser).m_errorCode = XML_ERROR_INVALID_ARGUMENT;
        return XML_STATUS_ERROR;
    }
    match (*parser).m_parsingStatus.parsing {
        3 => {
            (*parser).m_errorCode = XML_ERROR_SUSPENDED;
            return XML_STATUS_ERROR;
        }
        2 => {
            (*parser).m_errorCode = XML_ERROR_FINISHED;
            return XML_STATUS_ERROR;
        }
        0 => {
            if (*parser).m_bufferPtr.is_null() {
                (*parser).m_errorCode = XML_ERROR_NO_BUFFER;
                return XML_STATUS_ERROR;
            }
            if (*parser).m_parentParser.is_null() && startParsing(parser) == 0 {
                (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
                return XML_STATUS_ERROR;
            }
        }
        _ => {}
    }
    (*parser).m_parsingStatus.parsing = XML_PARSING;
    start = (*parser).m_bufferPtr;
    (*parser).m_positionPtr = start;
    (*parser).m_bufferEnd = (*parser).m_bufferEnd.offset(len as isize);
    (*parser).m_parseEndPtr = (*parser).m_bufferEnd;
    (*parser).m_parseEndByteIndex += len as XML_Index;
    (*parser).m_parsingStatus.finalBuffer = isFinal as XML_Bool;
    (*parser).m_errorCode = callProcessor(
        parser,
        start,
        (*parser).m_parseEndPtr,
        &raw mut (*parser).m_bufferPtr,
    );
    if (*parser).m_errorCode != XML_ERROR_NONE {
        (*parser).m_eventEndPtr = (*parser).m_eventPtr;
        (*parser).m_processor = Some(
            errorProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
        return XML_STATUS_ERROR;
    } else {
        match (*parser).m_parsingStatus.parsing {
            3 => {
                result = XML_STATUS_SUSPENDED;
            }
            0 | 1 => {
                if isFinal != 0 {
                    (*parser).m_parsingStatus.parsing = XML_FINISHED;
                    return result;
                }
            }
            _ => {}
        }
    }
    (*(*parser).m_encoding)
        .updatePosition
        .expect("non-null function pointer")(
        (*parser).m_encoding,
        (*parser).m_positionPtr,
        (*parser).m_bufferPtr,
        &raw mut (*parser).m_position,
    );
    (*parser).m_positionPtr = (*parser).m_bufferPtr;
    return result;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetBuffer(mut parser: XML_Parser, mut len: c_int) -> *mut c_void {
    if parser.is_null() {
        return NULL;
    }
    if len < 0 {
        (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
        return NULL;
    }
    match (*parser).m_parsingStatus.parsing {
        3 => {
            (*parser).m_errorCode = XML_ERROR_SUSPENDED;
            return NULL;
        }
        2 => {
            (*parser).m_errorCode = XML_ERROR_FINISHED;
            return NULL;
        }
        _ => {}
    }
    (*parser).m_lastBufferRequestSize = len;
    if len as c_long
        > (if !(*parser).m_bufferLim.is_null() && !(*parser).m_bufferEnd.is_null() {
            (*parser).m_bufferLim.offset_from((*parser).m_bufferEnd) as c_long
        } else {
            0
        })
        || (*parser).m_buffer.is_null()
    {
        let mut keep: c_int = 0;
        let mut neededSize: c_int = (len as c_uint).wrapping_add(
            (if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                (*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr) as c_long
            } else {
                0
            }) as c_uint,
        ) as c_int;
        if neededSize < 0 {
            (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
            return NULL;
        }
        keep = (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
            (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as c_long
        } else {
            0
        }) as c_int;
        if keep > XML_CONTEXT_BYTES {
            keep = XML_CONTEXT_BYTES;
        }
        if keep > INT_MAX - neededSize {
            (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
            return NULL;
        }
        neededSize += keep;
        if !(*parser).m_buffer.is_null()
            && !(*parser).m_bufferPtr.is_null()
            && neededSize as c_long
                <= (if !(*parser).m_bufferLim.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser).m_bufferLim.offset_from((*parser).m_buffer) as c_long
                } else {
                    0
                })
        {
            if (keep as c_long)
                < (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as c_long
                } else {
                    0
                })
            {
                let mut offset: c_int =
                    (if !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() {
                        (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as c_long
                    } else {
                        0
                    }) as c_int
                        - keep;
                crate::stdlib::memmove(
                    (*parser).m_buffer as *mut c_void,
                    (*parser).m_buffer.offset(offset as isize) as *const c_void,
                    ((*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr) as c_long
                        + keep as c_long) as size_t,
                );
                (*parser).m_bufferEnd = (*parser).m_bufferEnd.offset(-(offset as isize));
                (*parser).m_bufferPtr = (*parser).m_bufferPtr.offset(-(offset as isize));
            }
        } else {
            let mut newBuf: *mut c_char = null_mut::<c_char>();
            let mut bufferSize: c_int =
                (if !(*parser).m_bufferLim.is_null() && !(*parser).m_buffer.is_null() {
                    (*parser).m_bufferLim.offset_from((*parser).m_buffer) as c_long
                } else {
                    0
                }) as c_int;
            if bufferSize == 0 {
                bufferSize = INIT_BUFFER_SIZE;
            }
            loop {
                bufferSize = (2u32).wrapping_mul(bufferSize as c_uint) as c_int;
                if !(bufferSize < neededSize && bufferSize > 0) {
                    break;
                }
            }
            if bufferSize <= 0 {
                (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
                return NULL;
            }
            newBuf = (*parser)
                .m_mem
                .malloc_fcn
                .expect("non-null function pointer")(bufferSize as size_t)
                as *mut c_char;
            if newBuf.is_null() {
                (*parser).m_errorCode = XML_ERROR_NO_MEMORY;
                return NULL;
            }
            (*parser).m_bufferLim = newBuf.offset(bufferSize as isize);
            if !(*parser).m_bufferPtr.is_null() {
                memcpy(
                    newBuf as *mut c_void,
                    (*parser).m_bufferPtr.offset(-keep as isize) as *const c_void,
                    ((if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                        (*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr) as c_long
                    } else {
                        0
                    }) + keep as c_long) as size_t,
                );
                (*parser).m_mem.free_fcn.expect("non-null function pointer")(
                    (*parser).m_buffer as *mut c_void,
                );
                (*parser).m_buffer = newBuf;
                (*parser).m_bufferEnd = (*parser)
                    .m_buffer
                    .offset(
                        (if !(*parser).m_bufferEnd.is_null() && !(*parser).m_bufferPtr.is_null() {
                            (*parser).m_bufferEnd.offset_from((*parser).m_bufferPtr) as c_long
                        } else {
                            0
                        }) as isize,
                    )
                    .offset(keep as isize);
                (*parser).m_bufferPtr = (*parser).m_buffer.offset(keep as isize);
            } else {
                (*parser).m_bufferEnd = newBuf;
                (*parser).m_buffer = newBuf;
                (*parser).m_bufferPtr = (*parser).m_buffer;
            }
        }
        (*parser).m_eventEndPtr = null::<c_char>();
        (*parser).m_eventPtr = (*parser).m_eventEndPtr;
        (*parser).m_positionPtr = null::<c_char>();
    }
    return (*parser).m_bufferEnd as *mut c_void;
}

unsafe extern "C" fn triggerReenter(mut parser: XML_Parser) {
    (*parser).m_reenter = XML_TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn XML_StopParser(
    mut parser: XML_Parser,
    mut resumable: XML_Bool,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    match (*parser).m_parsingStatus.parsing {
        0 => {
            (*parser).m_errorCode = XML_ERROR_NOT_STARTED;
            return XML_STATUS_ERROR;
        }
        3 => {
            if resumable != 0 {
                (*parser).m_errorCode = XML_ERROR_SUSPENDED;
                return XML_STATUS_ERROR;
            }
            (*parser).m_parsingStatus.parsing = XML_FINISHED;
        }
        2 => {
            (*parser).m_errorCode = XML_ERROR_FINISHED;
            return XML_STATUS_ERROR;
        }
        1 => {
            if resumable != 0 {
                if (*parser).m_isParamEntity != 0 {
                    (*parser).m_errorCode = XML_ERROR_SUSPEND_PE;
                    return XML_STATUS_ERROR;
                }
                (*parser).m_parsingStatus.parsing = XML_SUSPENDED;
            } else {
                (*parser).m_parsingStatus.parsing = XML_FINISHED;
            }
        }
        _ => {
            assert!(false, "0");
        }
    }
    return XML_STATUS_OK;
}
#[no_mangle]

pub unsafe extern "C" fn XML_ResumeParser(mut parser: XML_Parser) -> XML_Status {
    let mut result: XML_Status = XML_STATUS_OK;
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if (*parser).m_parsingStatus.parsing != XML_SUSPENDED {
        (*parser).m_errorCode = XML_ERROR_NOT_SUSPENDED;
        return XML_STATUS_ERROR;
    }
    (*parser).m_parsingStatus.parsing = XML_PARSING;
    (*parser).m_errorCode = callProcessor(
        parser,
        (*parser).m_bufferPtr,
        (*parser).m_parseEndPtr,
        &raw mut (*parser).m_bufferPtr,
    );
    if (*parser).m_errorCode != XML_ERROR_NONE {
        (*parser).m_eventEndPtr = (*parser).m_eventPtr;
        (*parser).m_processor = Some(
            errorProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
        return XML_STATUS_ERROR;
    } else {
        match (*parser).m_parsingStatus.parsing {
            3 => {
                result = XML_STATUS_SUSPENDED;
            }
            0 | 1 => {
                if (*parser).m_parsingStatus.finalBuffer != 0 {
                    (*parser).m_parsingStatus.parsing = XML_FINISHED;
                    return result;
                }
            }
            _ => {}
        }
    }
    (*(*parser).m_encoding)
        .updatePosition
        .expect("non-null function pointer")(
        (*parser).m_encoding,
        (*parser).m_positionPtr,
        (*parser).m_bufferPtr,
        &raw mut (*parser).m_position,
    );
    (*parser).m_positionPtr = (*parser).m_bufferPtr;
    return result;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetParsingStatus(
    mut parser: XML_Parser,
    mut status: *mut XML_ParsingStatus,
) {
    if parser.is_null() {
        return;
    }
    assert!(!status.is_null(), "status != NULL");
    *status = (*parser).m_parsingStatus;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetErrorCode(mut parser: XML_Parser) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    return (*parser).m_errorCode;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetCurrentByteIndex(mut parser: XML_Parser) -> XML_Index {
    if parser.is_null() {
        return -1i64;
    }
    if !(*parser).m_eventPtr.is_null() {
        return (*parser).m_parseEndByteIndex
            - (*parser).m_parseEndPtr.offset_from((*parser).m_eventPtr) as c_long;
    }
    return -1i64;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetCurrentByteCount(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return 0i32;
    }
    if !(*parser).m_eventEndPtr.is_null() && !(*parser).m_eventPtr.is_null() {
        return (*parser).m_eventEndPtr.offset_from((*parser).m_eventPtr) as c_int;
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetInputContext(
    mut parser: XML_Parser,
    mut offset: *mut c_int,
    mut size: *mut c_int,
) -> *const c_char {
    if parser.is_null() {
        return null::<c_char>();
    }
    if !(*parser).m_eventPtr.is_null() && !(*parser).m_buffer.is_null() {
        if !offset.is_null() {
            *offset = (*parser).m_eventPtr.offset_from((*parser).m_buffer) as c_int;
        }
        if !size.is_null() {
            *size = (*parser).m_bufferEnd.offset_from((*parser).m_buffer) as c_int;
        }
        return (*parser).m_buffer;
    }
    return null::<c_char>();
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetCurrentLineNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0u64;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding)
            .updatePosition
            .expect("non-null function pointer")(
            (*parser).m_encoding,
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &raw mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr;
    }
    return (*parser).m_position.lineNumber.wrapping_add(1u64);
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetCurrentColumnNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0u64;
    }
    if !(*parser).m_eventPtr.is_null() && (*parser).m_eventPtr >= (*parser).m_positionPtr {
        (*(*parser).m_encoding)
            .updatePosition
            .expect("non-null function pointer")(
            (*parser).m_encoding,
            (*parser).m_positionPtr,
            (*parser).m_eventPtr,
            &raw mut (*parser).m_position,
        );
        (*parser).m_positionPtr = (*parser).m_eventPtr;
    }
    return (*parser).m_position.columnNumber;
}
#[no_mangle]

pub unsafe extern "C" fn XML_FreeContentModel(mut parser: XML_Parser, mut model: *mut XML_Content) {
    if parser.is_null() {
        return;
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(model as *mut c_void);
}
#[no_mangle]

pub unsafe extern "C" fn XML_MemMalloc(mut parser: XML_Parser, mut size: size_t) -> *mut c_void {
    if parser.is_null() {
        return NULL;
    }
    return (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(size);
}
#[no_mangle]

pub unsafe extern "C" fn XML_MemRealloc(
    mut parser: XML_Parser,
    mut ptr: *mut c_void,
    mut size: size_t,
) -> *mut c_void {
    if parser.is_null() {
        return NULL;
    }
    return (*parser)
        .m_mem
        .realloc_fcn
        .expect("non-null function pointer")(ptr, size);
}
#[no_mangle]

pub unsafe extern "C" fn XML_MemFree(mut parser: XML_Parser, mut ptr: *mut c_void) {
    if parser.is_null() {
        return;
    }
    (*parser).m_mem.free_fcn.expect("non-null function pointer")(ptr);
}
#[no_mangle]

pub unsafe extern "C" fn XML_DefaultCurrent(mut parser: XML_Parser) {
    if parser.is_null() {
        return;
    }
    if (*parser).m_defaultHandler.is_some() {
        if !(*parser).m_openInternalEntities.is_null() {
            reportDefault(
                parser,
                (*parser).m_internalEncoding,
                (*(*parser).m_openInternalEntities).internalEventPtr,
                (*(*parser).m_openInternalEntities).internalEventEndPtr,
            );
        } else {
            reportDefault(
                parser,
                (*parser).m_encoding,
                (*parser).m_eventPtr,
                (*parser).m_eventEndPtr,
            );
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn XML_ErrorString(mut code: XML_Error) -> *const XML_LChar {
    match code {
        0 => return null::<XML_LChar>(),
        1 => return b"out of memory\0" as *const u8 as *const XML_LChar,
        2 => return b"syntax error\0" as *const u8 as *const XML_LChar,
        3 => return b"no element found\0" as *const u8 as *const XML_LChar,
        4 => return b"not well-formed (invalid token)\0" as *const u8 as *const XML_LChar,
        5 => return b"unclosed token\0" as *const u8 as *const XML_LChar,
        6 => return b"partial character\0" as *const u8 as *const XML_LChar,
        7 => return b"mismatched tag\0" as *const u8 as *const XML_LChar,
        8 => return b"duplicate attribute\0" as *const u8 as *const XML_LChar,
        9 => return b"junk after document element\0" as *const u8 as *const XML_LChar,
        10 => {
            return b"illegal parameter entity reference\0" as *const u8 as *const XML_LChar;
        }
        11 => return b"undefined entity\0" as *const u8 as *const XML_LChar,
        12 => return b"recursive entity reference\0" as *const u8 as *const XML_LChar,
        13 => return b"asynchronous entity\0" as *const u8 as *const XML_LChar,
        14 => {
            return b"reference to invalid character number\0" as *const u8 as *const XML_LChar;
        }
        15 => return b"reference to binary entity\0" as *const u8 as *const XML_LChar,
        16 => {
            return b"reference to external entity in attribute\0" as *const u8 as *const XML_LChar;
        }
        17 => {
            return b"XML or text declaration not at start of entity\0" as *const u8
                as *const XML_LChar;
        }
        18 => return b"unknown encoding\0" as *const u8 as *const XML_LChar,
        19 => {
            return b"encoding specified in XML declaration is incorrect\0" as *const u8
                as *const XML_LChar;
        }
        20 => return b"unclosed CDATA section\0" as *const u8 as *const XML_LChar,
        21 => {
            return b"error in processing external entity reference\0" as *const u8
                as *const XML_LChar;
        }
        22 => return b"document is not standalone\0" as *const u8 as *const XML_LChar,
        23 => {
            return b"unexpected parser state - please send a bug report\0" as *const u8
                as *const XML_LChar;
        }
        24 => {
            return b"entity declared in parameter entity\0" as *const u8 as *const XML_LChar;
        }
        25 => {
            return b"requested feature requires XML_DTD support in Expat\0" as *const u8
                as *const XML_LChar;
        }
        26 => {
            return b"cannot change setting once parsing has begun\0" as *const u8
                as *const XML_LChar;
        }
        27 => return b"unbound prefix\0" as *const u8 as *const XML_LChar,
        28 => return b"must not undeclare prefix\0" as *const u8 as *const XML_LChar,
        29 => {
            return b"incomplete markup in parameter entity\0" as *const u8 as *const XML_LChar;
        }
        30 => {
            return b"XML declaration not well-formed\0" as *const u8 as *const XML_LChar;
        }
        31 => {
            return b"text declaration not well-formed\0" as *const u8 as *const XML_LChar;
        }
        32 => {
            return b"illegal character(s) in public id\0" as *const u8 as *const XML_LChar;
        }
        33 => return b"parser suspended\0" as *const u8 as *const XML_LChar,
        34 => return b"parser not suspended\0" as *const u8 as *const XML_LChar,
        35 => return b"parsing aborted\0" as *const u8 as *const XML_LChar,
        36 => return b"parsing finished\0" as *const u8 as *const XML_LChar,
        37 => {
            return b"cannot suspend in external parameter entity\0" as *const u8
                as *const XML_LChar;
        }
        38 => {
            return b"reserved prefix (xml) must not be undeclared or bound to another namespace name\0"
                as *const u8 as *const XML_LChar;
        }
        39 => {
            return b"reserved prefix (xmlns) must not be declared or undeclared\0" as *const u8
                as *const XML_LChar;
        }
        40 => {
            return b"prefix must not be bound to one of the reserved namespace names\0" as *const u8
                as *const XML_LChar;
        }
        41 => return b"invalid argument\0" as *const u8 as *const XML_LChar,
        42 => {
            return b"a successful prior call to function XML_GetBuffer is required\0" as *const u8
                as *const XML_LChar;
        }
        43 => {
            return b"limit on input amplification factor (from DTD and entities) breached\0"
                as *const u8 as *const XML_LChar;
        }
        44 => return b"parser not started\0" as *const u8 as *const XML_LChar,
        _ => {}
    }
    return null::<XML_LChar>();
}
#[no_mangle]

pub unsafe extern "C" fn XML_ExpatVersion() -> *const XML_LChar {
    return b"expat_2.7.4\0" as *const u8 as *const XML_LChar;
}
#[no_mangle]

pub unsafe extern "C" fn XML_ExpatVersionInfo() -> XML_Expat_Version {
    let mut version: XML_Expat_Version = XML_Expat_Version {
        major: 0,
        minor: 0,
        micro: 0,
    };
    version.major = XML_MAJOR_VERSION;
    version.minor = XML_MINOR_VERSION;
    version.micro = XML_MICRO_VERSION;
    return version;
}
#[no_mangle]

pub unsafe extern "C" fn XML_GetFeatureList() -> *const XML_Feature {
    static mut features: [XML_Feature; 11] = [
        XML_Feature {
            feature: XML_FEATURE_SIZEOF_XML_CHAR,
            name: b"sizeof(XML_Char)\0" as *const u8 as *const XML_LChar,
            value: size_of::<XML_Char>() as c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_SIZEOF_XML_LCHAR,
            name: b"sizeof(XML_LChar)\0" as *const u8 as *const XML_LChar,
            value: size_of::<XML_LChar>() as c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_DTD,
            name: b"XML_DTD\0" as *const u8 as *const XML_LChar,
            value: 0i64,
        },
        XML_Feature {
            feature: XML_FEATURE_CONTEXT_BYTES,
            name: b"XML_CONTEXT_BYTES\0" as *const u8 as *const XML_LChar,
            value: XML_CONTEXT_BYTES as c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_NS,
            name: b"XML_NS\0" as *const u8 as *const XML_LChar,
            value: 0i64,
        },
        XML_Feature {
            feature: XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT,
            name: b"XML_BLAP_MAX_AMP\0" as *const u8 as *const XML_LChar,
            value: EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT as c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT,
            name: b"XML_BLAP_ACT_THRES\0" as *const u8 as *const XML_LChar,
            value: EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT as c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_GE,
            name: b"XML_GE\0" as *const u8 as *const XML_LChar,
            value: 0i64,
        },
        XML_Feature {
            feature: XML_FEATURE_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT,
            name: b"XML_AT_MAX_AMP\0" as *const u8 as *const XML_LChar,
            value: EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT as c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT,
            name: b"XML_AT_ACT_THRES\0" as *const u8 as *const XML_LChar,
            value: EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT as c_long,
        },
        XML_Feature {
            feature: XML_FEATURE_END,
            name: null::<XML_LChar>(),
            value: 0i64,
        },
    ];
    return &raw const features as *const XML_Feature;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
    mut parser: XML_Parser,
    mut maximumAmplificationFactor: c_float,
) -> XML_Bool {
    if parser.is_null()
        || !(*parser).m_parentParser.is_null()
        || maximumAmplificationFactor.is_nan() as i32 != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return XML_FALSE;
    }
    (*parser).m_accounting.maximumAmplificationFactor = maximumAmplificationFactor;
    return XML_TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
    mut parser: XML_Parser,
    mut activationThresholdBytes: c_ulonglong,
) -> XML_Bool {
    if parser.is_null() || !(*parser).m_parentParser.is_null() {
        return XML_FALSE;
    }
    (*parser).m_accounting.activationThresholdBytes = activationThresholdBytes;
    return XML_TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetAllocTrackerMaximumAmplification(
    mut parser: XML_Parser,
    mut maximumAmplificationFactor: c_float,
) -> XML_Bool {
    if parser.is_null()
        || !(*parser).m_parentParser.is_null()
        || maximumAmplificationFactor.is_nan() as i32 != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return XML_FALSE;
    }
    (*parser).m_alloc_tracker.maximumAmplificationFactor = maximumAmplificationFactor;
    return XML_TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetAllocTrackerActivationThreshold(
    mut parser: XML_Parser,
    mut activationThresholdBytes: c_ulonglong,
) -> XML_Bool {
    if parser.is_null() || !(*parser).m_parentParser.is_null() {
        return XML_FALSE;
    }
    (*parser).m_alloc_tracker.activationThresholdBytes = activationThresholdBytes;
    return XML_TRUE;
}
#[no_mangle]

pub unsafe extern "C" fn XML_SetReparseDeferralEnabled(
    mut parser: XML_Parser,
    mut enabled: XML_Bool,
) -> XML_Bool {
    if !parser.is_null()
        && (enabled as c_int == XML_TRUE as c_int || enabled as c_int == XML_FALSE as c_int)
    {
        (*parser).m_reparseDeferralEnabled = enabled;
        return XML_TRUE;
    }
    return XML_FALSE;
}

unsafe extern "C" fn storeRawNames(mut parser: XML_Parser) -> XML_Bool {
    let mut tag: *mut TAG = (*parser).m_tagStack;
    while !tag.is_null() {
        let mut bufSize: size_t = 0;
        let mut nameLen: size_t =
            (size_of::<XML_Char>()).wrapping_mul(((*tag).name.strLen + 1) as size_t);
        let mut rawNameLen: size_t = 0;
        let mut rawNameBuf: *mut c_char = (*tag).buf.raw.offset(nameLen as isize);
        if (*tag).rawName == rawNameBuf as *const c_char {
            break;
        }
        rawNameLen = ((*tag).rawNameLength as usize)
            .wrapping_add((size_of::<XML_Char>()).wrapping_sub(1usize))
            & !(size_of::<XML_Char>()).wrapping_sub(1usize);
        if rawNameLen > (INT_MAX as size_t).wrapping_sub(nameLen) {
            return XML_FALSE;
        }
        bufSize = nameLen.wrapping_add(rawNameLen);
        if bufSize > (*tag).bufEnd.offset_from((*tag).buf.raw) as size_t {
            let mut temp: *mut c_char =
                expat_realloc(parser, (*tag).buf.raw as *mut c_void, bufSize, 3151) as *mut c_char;
            if temp.is_null() {
                return XML_FALSE;
            }
            if (*tag).name.str_0 == (*tag).buf.str_0 as *const XML_Char {
                (*tag).name.str_0 = temp;
            }
            if !(*tag).name.localPart.is_null() {
                (*tag).name.localPart =
                    (temp).offset((*tag).name.localPart.offset_from((*tag).buf.str_0));
            }
            (*tag).buf.raw = temp;
            (*tag).bufEnd = temp.offset(bufSize as isize);
            rawNameBuf = temp.offset(nameLen as isize);
        }
        memcpy(
            rawNameBuf as *mut c_void,
            (*tag).rawName as *const c_void,
            (*tag).rawNameLength as size_t,
        );
        (*tag).rawName = rawNameBuf;
        tag = (*tag).parent;
    }
    return XML_TRUE;
}

unsafe extern "C" fn contentProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = doContent(
        parser,
        if !(*parser).m_parentParser.is_null() {
            1
        } else {
            0
        },
        (*parser).m_encoding,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
        XML_ACCOUNT_DIRECT,
    );
    if result == XML_ERROR_NONE {
        if storeRawNames(parser) == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}

unsafe extern "C" fn externalEntityInitProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = initializeEncoding(parser);
    if result != XML_ERROR_NONE {
        return result;
    }
    (*parser).m_processor = Some(
        externalEntityInitProcessor2
            as unsafe extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    return externalEntityInitProcessor2(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityInitProcessor2(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = start;
    let mut tok: c_int = (*(*parser).m_encoding).scanners[1].expect("non-null function pointer")(
        (*parser).m_encoding,
        start,
        end,
        &raw mut next,
    );
    match tok {
        XML_TOK_BOM => {
            if accountingDiffTolerated(parser, tok, start, next, 3208, XML_ACCOUNT_DIRECT) == 0 {
                accountingOnAbort(parser);
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            if next == end && (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = next;
                return XML_ERROR_NONE;
            }
            start = next;
        }
        XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return XML_ERROR_UNCLOSED_TOKEN;
        }
        XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            (*parser).m_eventPtr = start;
            return XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(
        externalEntityInitProcessor3
            as unsafe extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    return externalEntityInitProcessor3(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityInitProcessor3(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut tok: c_int = 0;
    let mut next: *const c_char = start;
    (*parser).m_eventPtr = start;
    tok = (*(*parser).m_encoding).scanners[1].expect("non-null function pointer")(
        (*parser).m_encoding,
        start,
        end,
        &raw mut next,
    );
    (*parser).m_eventEndPtr = next;
    match tok {
        XML_TOK_XML_DECL => {
            let mut result: XML_Error = XML_ERROR_NONE;
            result = processXmlDecl(parser, 1, start, next);
            if result != XML_ERROR_NONE {
                return result;
            }
            match (*parser).m_parsingStatus.parsing {
                3 => {
                    *endPtr = next;
                    return XML_ERROR_NONE;
                }
                2 => return XML_ERROR_ABORTED,
                1 => {
                    if (*parser).m_reenter != 0 {
                        return XML_ERROR_UNEXPECTED_STATE;
                    }
                }
                _ => {}
            }
            start = next;
        }
        XML_TOK_PARTIAL => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_UNCLOSED_TOKEN;
        }
        XML_TOK_PARTIAL_CHAR => {
            if (*parser).m_parsingStatus.finalBuffer == 0 {
                *endPtr = start;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    (*parser).m_processor = Some(
        externalEntityContentProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    (*parser).m_tagLevel = 1;
    return externalEntityContentProcessor(parser, start, end, endPtr);
}

unsafe extern "C" fn externalEntityContentProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = doContent(
        parser,
        1,
        (*parser).m_encoding,
        start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
        XML_ACCOUNT_ENTITY_EXPANSION,
    );
    if result == XML_ERROR_NONE {
        if storeRawNames(parser) == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    return result;
}

unsafe extern "C" fn doContent(
    mut parser: XML_Parser,
    mut startTagLevel: c_int,
    mut enc: *const ENCODING,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
    mut account: XML_Account,
) -> XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut eventPP: *mut *const c_char = null_mut::<*const c_char>();
    let mut eventEndPP: *mut *const c_char = null_mut::<*const c_char>();
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    loop {
        let mut next: *const c_char = s;
        let mut tok: c_int =
            (*enc).scanners[1].expect("non-null function pointer")(enc, s, end, &raw mut next);
        let mut accountAfter: *const c_char =
            if tok == XML_TOK_TRAILING_RSQB || tok == XML_TOK_TRAILING_CR {
                if haveMore as c_int != 0 {
                    s
                } else {
                    end
                }
            } else {
                next
            };
        if accountingDiffTolerated(parser, tok, s, accountAfter, 3337, account) == 0 {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        *eventEndPP = next;
        let mut current_block_281: u64;
        match tok {
            XML_TOK_TRAILING_CR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                *eventEndPP = end;
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c: XML_Char = 0xa;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut c,
                        1i32,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                if startTagLevel == 0 {
                    return XML_ERROR_NO_ELEMENTS;
                }
                if (*parser).m_tagLevel != startTagLevel {
                    return XML_ERROR_ASYNC_ENTITY;
                }
                *nextPtr = end;
                return XML_ERROR_NONE;
            }
            XML_TOK_NONE => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                if startTagLevel > 0 {
                    if (*parser).m_tagLevel != startTagLevel {
                        return XML_ERROR_ASYNC_ENTITY;
                    }
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_NO_ELEMENTS;
            }
            XML_TOK_INVALID => {
                *eventPP = next;
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_TOKEN;
            }
            XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            XML_TOK_ENTITY_REF => {
                let mut name: *const XML_Char = null::<XML_Char>();
                let mut entity: *mut ENTITY = null_mut::<ENTITY>();
                let mut ch: XML_Char = (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(
                    enc,
                    s.offset((*enc).minBytesPerChar as isize),
                    next.offset(-((*enc).minBytesPerChar as isize)),
                ) as XML_Char;
                if ch != 0 {
                    accountingDiffTolerated(
                        parser,
                        tok,
                        &raw mut ch,
                        (&raw mut ch).offset(size_of::<XML_Char>() as isize),
                        3403,
                        XML_ACCOUNT_ENTITY_EXPANSION,
                    );
                    if (*parser).m_characterDataHandler.is_some() {
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            &raw mut ch,
                            1i32,
                        );
                    } else if (*parser).m_defaultHandler.is_some() {
                        reportDefault(parser, enc, s, next);
                    }
                } else {
                    name = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    entity =
                        lookup(parser, &raw mut (*dtd).generalEntities, name, 0) as *mut ENTITY;
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    if (*dtd).hasParamEntityRefs == 0 || (*dtd).standalone as c_int != 0 {
                        if entity.is_null() {
                            return XML_ERROR_UNDEFINED_ENTITY;
                        } else if (*entity).is_internal == 0 {
                            return XML_ERROR_ENTITY_DECLARED_IN_PE;
                        }
                        current_block_281 = 3546145585875536353;
                    } else if entity.is_null() {
                        if (*parser).m_skippedEntityHandler.is_some() {
                            (*parser)
                                .m_skippedEntityHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                name,
                                0i32,
                            );
                        } else if (*parser).m_defaultHandler.is_some() {
                            reportDefault(parser, enc, s, next);
                        }
                        current_block_281 = 1957216233951053322;
                    } else {
                        current_block_281 = 3546145585875536353;
                    }
                    match current_block_281 {
                        1957216233951053322 => {}
                        _ => {
                            if (*entity).open != 0 {
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                return XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if !(*entity).textPtr.is_null() {
                                let mut result: XML_Error = XML_ERROR_NONE;
                                if (*parser).m_defaultExpandInternalEntities == 0 {
                                    if (*parser).m_skippedEntityHandler.is_some() {
                                        (*parser)
                                            .m_skippedEntityHandler
                                            .expect("non-null function pointer")(
                                            (*parser).m_handlerArg,
                                            (*entity).name,
                                            0i32,
                                        );
                                    } else if (*parser).m_defaultHandler.is_some() {
                                        reportDefault(parser, enc, s, next);
                                    }
                                } else {
                                    result =
                                        processEntity(parser, entity, XML_FALSE, ENTITY_INTERNAL);
                                    if result != XML_ERROR_NONE {
                                        return result;
                                    }
                                }
                            } else if (*parser).m_externalEntityRefHandler.is_some() {
                                let mut context: *const XML_Char = null::<XML_Char>();
                                (*entity).open = XML_TRUE;
                                context = getContext(parser);
                                (*entity).open = XML_FALSE;
                                if context.is_null() {
                                    return XML_ERROR_NO_MEMORY;
                                }
                                if (*parser)
                                    .m_externalEntityRefHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_externalEntityRefHandlerArg,
                                    context,
                                    (*entity).base,
                                    (*entity).systemId,
                                    (*entity).publicId,
                                ) == 0
                                {
                                    return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                }
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
                            } else if (*parser).m_defaultHandler.is_some() {
                                reportDefault(parser, enc, s, next);
                            }
                        }
                    }
                }
            }
            XML_TOK_START_TAG_NO_ATTS | XML_TOK_START_TAG_WITH_ATTS => {
                let mut tag: *mut TAG = null_mut::<TAG>();
                let mut result_0: XML_Error = XML_ERROR_NONE;
                let mut toPtr: *mut XML_Char = null_mut::<XML_Char>();
                if !(*parser).m_freeTagList.is_null() {
                    tag = (*parser).m_freeTagList;
                    (*parser).m_freeTagList = (*(*parser).m_freeTagList).parent;
                } else {
                    tag = expat_malloc(parser, size_of::<TAG>(), 3477) as *mut TAG;
                    if tag.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*tag).buf.raw = expat_malloc(parser, 32, 3480) as *mut c_char;
                    if (*tag).buf.raw.is_null() {
                        expat_free(parser, tag as *mut c_void, 3482);
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*tag).bufEnd = (*tag).buf.raw.offset(INIT_TAG_BUF_SIZE as isize);
                }
                (*tag).bindings = null_mut::<BINDING>();
                (*tag).parent = (*parser).m_tagStack;
                (*parser).m_tagStack = tag;
                (*tag).name.localPart = null::<XML_Char>();
                (*tag).name.prefix = null::<XML_Char>();
                (*tag).rawName = s.offset((*enc).minBytesPerChar as isize);
                (*tag).rawNameLength =
                    (*enc).nameLength.expect("non-null function pointer")(enc, (*tag).rawName);
                (*parser).m_tagLevel += 1;
                let mut rawNameEnd: *const c_char =
                    (*tag).rawName.offset((*tag).rawNameLength as isize);
                let mut fromPtr: *const c_char = (*tag).rawName;
                toPtr = (*tag).buf.str_0;
                loop {
                    let mut convLen: c_int = 0;
                    let convert_res: XML_Convert_Result =
                        (*enc).utf8Convert.expect("non-null function pointer")(
                            enc,
                            &raw mut fromPtr,
                            rawNameEnd,
                            &raw mut toPtr,
                            ((*tag).bufEnd).offset(-(1)),
                        );
                    convLen = toPtr.offset_from((*tag).buf.str_0) as c_int;
                    if fromPtr >= rawNameEnd || convert_res == XML_CONVERT_INPUT_INCOMPLETE {
                        (*tag).name.strLen = convLen;
                        break;
                    } else {
                        if (SIZE_MAX as size_t).wrapping_div(2usize)
                            < (*tag).bufEnd.offset_from((*tag).buf.raw) as size_t
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        let bufSize: size_t = ((*tag).bufEnd.offset_from((*tag).buf.raw) as size_t)
                            .wrapping_mul(2usize);
                        let mut temp: *mut c_char =
                            expat_realloc(parser, (*tag).buf.raw as *mut c_void, bufSize, 3514)
                                as *mut c_char;
                        if temp.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*tag).buf.raw = temp;
                        (*tag).bufEnd = temp.offset(bufSize as isize);
                        toPtr = (temp).offset(convLen as isize);
                    }
                }
                (*tag).name.str_0 = (*tag).buf.str_0;
                *toPtr = '\0' as XML_Char;
                result_0 = storeAtts(
                    parser,
                    enc,
                    s,
                    &raw mut (*tag).name,
                    &raw mut (*tag).bindings,
                    account,
                );
                if result_0 as u64 != 0 {
                    return result_0;
                }
                if (*parser).m_startElementHandler.is_some() {
                    (*parser)
                        .m_startElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*tag).name.str_0,
                        (*parser).m_atts as *mut *const XML_Char,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&raw mut (*parser).m_tempPool);
            }
            XML_TOK_EMPTY_ELEMENT_NO_ATTS | XML_TOK_EMPTY_ELEMENT_WITH_ATTS => {
                let mut rawName: *const c_char = s.offset((*enc).minBytesPerChar as isize);
                let mut result_1: XML_Error = XML_ERROR_NONE;
                let mut bindings: *mut BINDING = null_mut::<BINDING>();
                let mut noElmHandlers: XML_Bool = XML_TRUE;
                let mut name_0: TAG_NAME = TAG_NAME {
                    str_0: null::<XML_Char>(),
                    localPart: null::<XML_Char>(),
                    prefix: null::<XML_Char>(),
                    strLen: 0,
                    uriLen: 0,
                    prefixLen: 0,
                };
                name_0.str_0 = poolStoreString(
                    &raw mut (*parser).m_tempPool,
                    enc,
                    rawName,
                    rawName.offset((*enc).nameLength.expect("non-null function pointer")(
                        enc, rawName,
                    ) as isize),
                );
                if name_0.str_0.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                result_1 = storeAtts(
                    parser,
                    enc,
                    s,
                    &raw mut name_0,
                    &raw mut bindings,
                    XML_ACCOUNT_NONE,
                );
                if result_1 != XML_ERROR_NONE {
                    freeBindings(parser, bindings);
                    return result_1;
                }
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                if (*parser).m_startElementHandler.is_some() {
                    (*parser)
                        .m_startElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        name_0.str_0,
                        (*parser).m_atts as *mut *const XML_Char,
                    );
                    noElmHandlers = XML_FALSE;
                }
                if (*parser).m_endElementHandler.is_some() {
                    if (*parser).m_startElementHandler.is_some() {
                        *eventPP = *eventEndPP;
                    }
                    (*parser)
                        .m_endElementHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg, name_0.str_0
                    );
                    noElmHandlers = XML_FALSE;
                }
                if noElmHandlers as c_int != 0 && (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&raw mut (*parser).m_tempPool);
                freeBindings(parser, bindings);
                if (*parser).m_tagLevel == 0 && (*parser).m_parsingStatus.parsing != XML_FINISHED {
                    if (*parser).m_parsingStatus.parsing == XML_SUSPENDED
                        || (*parser).m_parsingStatus.parsing == XML_PARSING
                            && (*parser).m_reenter as c_int != 0
                    {
                        (*parser).m_processor = Some(
                            epilogProcessor
                                as unsafe extern "C" fn(
                                    XML_Parser,
                                    *const c_char,
                                    *const c_char,
                                    *mut *const c_char,
                                )
                                    -> XML_Error,
                        );
                    } else {
                        return epilogProcessor(parser, next, end, nextPtr);
                    }
                }
            }
            XML_TOK_END_TAG => {
                if (*parser).m_tagLevel == startTagLevel {
                    return XML_ERROR_ASYNC_ENTITY;
                } else {
                    let mut len: c_int = 0;
                    let mut rawName_0: *const c_char = null::<c_char>();
                    let mut tag_0: *mut TAG = (*parser).m_tagStack;
                    rawName_0 = s.offset(((*enc).minBytesPerChar * 2i32) as isize);
                    len = (*enc).nameLength.expect("non-null function pointer")(enc, rawName_0);
                    if len != (*tag_0).rawNameLength
                        || crate::stdlib::memcmp(
                            (*tag_0).rawName as *const c_void,
                            rawName_0 as *const c_void,
                            len as size_t,
                        ) != 0
                    {
                        *eventPP = rawName_0;
                        return XML_ERROR_TAG_MISMATCH;
                    }
                    (*parser).m_tagStack = (*tag_0).parent;
                    (*tag_0).parent = (*parser).m_freeTagList;
                    (*parser).m_freeTagList = tag_0;
                    (*parser).m_tagLevel -= 1;
                    if (*parser).m_endElementHandler.is_some() {
                        let mut localPart: *const XML_Char = null::<XML_Char>();
                        let mut prefix: *const XML_Char = null::<XML_Char>();
                        let mut uri: *mut XML_Char = null_mut::<XML_Char>();
                        localPart = (*tag_0).name.localPart;
                        if (*parser).m_ns as c_int != 0 && !localPart.is_null() {
                            uri = ((*tag_0).name.str_0 as *mut XML_Char)
                                .offset((*tag_0).name.uriLen as isize);
                            while *localPart != 0 {
                                let fresh22 = localPart;
                                localPart = localPart.offset(1);
                                let fresh23 = uri;
                                uri = uri.offset(1);
                                *fresh23 = *fresh22;
                            }
                            prefix = (*tag_0).name.prefix;
                            if (*parser).m_ns_triplets as c_int != 0 && !prefix.is_null() {
                                let fresh24 = uri;
                                uri = uri.offset(1);
                                *fresh24 = (*parser).m_namespaceSeparator;
                                while *prefix != 0 {
                                    let fresh25 = prefix;
                                    prefix = prefix.offset(1);
                                    let fresh26 = uri;
                                    uri = uri.offset(1);
                                    *fresh26 = *fresh25;
                                }
                            }
                            *uri = '\0' as XML_Char;
                        }
                        (*parser)
                            .m_endElementHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*tag_0).name.str_0,
                        );
                    } else if (*parser).m_defaultHandler.is_some() {
                        reportDefault(parser, enc, s, next);
                    }
                    while !(*tag_0).bindings.is_null() {
                        let mut b: *mut BINDING = (*tag_0).bindings;
                        if (*parser).m_endNamespaceDeclHandler.is_some() {
                            (*parser)
                                .m_endNamespaceDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*b).prefix).name,
                            );
                        }
                        (*tag_0).bindings = (*(*tag_0).bindings).nextTagBinding;
                        (*b).nextTagBinding = (*parser).m_freeBindingList;
                        (*parser).m_freeBindingList = b;
                        (*(*b).prefix).binding = (*b).prevPrefixBinding;
                    }
                    if (*parser).m_tagLevel == 0
                        && (*parser).m_parsingStatus.parsing != XML_FINISHED
                    {
                        if (*parser).m_parsingStatus.parsing == XML_SUSPENDED
                            || (*parser).m_parsingStatus.parsing == XML_PARSING
                                && (*parser).m_reenter as c_int != 0
                        {
                            (*parser).m_processor = Some(
                                epilogProcessor
                                    as unsafe extern "C" fn(
                                        XML_Parser,
                                        *const c_char,
                                        *const c_char,
                                        *mut *const c_char,
                                    )
                                        -> XML_Error,
                            );
                        } else {
                            return epilogProcessor(parser, next, end, nextPtr);
                        }
                    }
                }
            }
            XML_TOK_CHAR_REF => {
                let mut n: c_int = (*enc).charRefNumber.expect("non-null function pointer")(enc, s);
                if n < 0 {
                    return XML_ERROR_BAD_CHAR_REF;
                }
                if (*parser).m_characterDataHandler.is_some() {
                    let mut buf: [XML_Char; 4] = [0; 4];
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut buf as *mut XML_Char,
                        XmlUtf8Encode(n, &raw mut buf as *mut c_char),
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_XML_DECL => return XML_ERROR_MISPLACED_XML_PI,
            XML_TOK_DATA_NEWLINE => {
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c_0: XML_Char = 0xa;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut c_0,
                        1i32,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_CDATA_SECT_OPEN => {
                let mut result_2: XML_Error = XML_ERROR_NONE;
                if (*parser).m_startCdataSectionHandler.is_some() {
                    (*parser)
                        .m_startCdataSectionHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                } else if 0 != 0 && (*parser).m_characterDataHandler.is_some() {
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_dataBuf,
                        0i32,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                result_2 =
                    doCdataSection(parser, enc, &raw mut next, end, nextPtr, haveMore, account);
                if result_2 != XML_ERROR_NONE {
                    return result_2;
                } else if next.is_null() {
                    (*parser).m_processor = Some(
                        cdataSectionProcessor
                            as unsafe extern "C" fn(
                                XML_Parser,
                                *const c_char,
                                *const c_char,
                                *mut *const c_char,
                            ) -> XML_Error,
                    );
                    return result_2;
                }
            }
            XML_TOK_TRAILING_RSQB => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                if (*parser).m_characterDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf;
                        (*enc).utf8Convert.expect("non-null function pointer")(
                            enc,
                            &raw mut s,
                            end,
                            &raw mut dataPtr,
                            (*parser).m_dataBufEnd,
                        );
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*parser).m_dataBuf,
                            dataPtr.offset_from((*parser).m_dataBuf) as c_int,
                        );
                    } else {
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s,
                            (end).offset_from(s) as c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                if startTagLevel == 0 {
                    *eventPP = end;
                    return XML_ERROR_NO_ELEMENTS;
                }
                if (*parser).m_tagLevel != startTagLevel {
                    *eventPP = end;
                    return XML_ERROR_ASYNC_ENTITY;
                }
                *nextPtr = end;
                return XML_ERROR_NONE;
            }
            XML_TOK_DATA_CHARS => {
                let mut charDataHandler: XML_CharacterDataHandler =
                    (*parser).m_characterDataHandler;
                if charDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        loop {
                            let mut dataPtr_0: *mut ICHAR = (*parser).m_dataBuf;
                            let convert_res_0: XML_Convert_Result =
                                (*enc).utf8Convert.expect("non-null function pointer")(
                                    enc,
                                    &raw mut s,
                                    next,
                                    &raw mut dataPtr_0,
                                    (*parser).m_dataBufEnd,
                                );
                            *eventEndPP = s;
                            charDataHandler.expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*parser).m_dataBuf,
                                dataPtr_0.offset_from((*parser).m_dataBuf) as c_int,
                            );
                            if convert_res_0 == XML_CONVERT_COMPLETED
                                || convert_res_0 == XML_CONVERT_INPUT_INCOMPLETE
                            {
                                break;
                            }
                            *eventPP = s;
                        }
                    } else {
                        charDataHandler.expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s,
                            (next).offset_from(s) as c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_PI => {
                if reportProcessingInstruction(parser, enc, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            XML_TOK_COMMENT => {
                if reportComment(parser, enc, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            _ => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
        }
        match (*parser).m_parsingStatus.parsing {
            3 => {
                *eventPP = next;
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => {
                *eventPP = next;
                return XML_ERROR_ABORTED;
            }
            1 => {
                if (*parser).m_reenter != 0 {
                    *nextPtr = next;
                    return XML_ERROR_NONE;
                }
            }
            _ => {}
        }
        s = next;
        *eventPP = s;
    }
}

unsafe extern "C" fn freeBindings(mut parser: XML_Parser, mut bindings: *mut BINDING) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        if (*parser).m_endNamespaceDeclHandler.is_some() {
            (*parser)
                .m_endNamespaceDeclHandler
                .expect("non-null function pointer")(
                (*parser).m_handlerArg, (*(*b).prefix).name
            );
        }
        bindings = (*bindings).nextTagBinding;
        (*b).nextTagBinding = (*parser).m_freeBindingList;
        (*parser).m_freeBindingList = b;
        (*(*b).prefix).binding = (*b).prevPrefixBinding;
    }
}

unsafe extern "C" fn storeAtts(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut attStr: *const c_char,
    mut tagNamePtr: *mut TAG_NAME,
    mut bindingsPtr: *mut *mut BINDING,
    mut account: XML_Account,
) -> XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut elementType: *mut ELEMENT_TYPE = null_mut::<ELEMENT_TYPE>();
    let mut nDefaultAtts: c_int = 0;
    let mut appAtts: *mut *const XML_Char = null_mut::<*const XML_Char>();
    let mut attIndex: c_int = 0;
    let mut prefixLen: c_int = 0;
    let mut i: c_int = 0;
    let mut n: c_int = 0;
    let mut uri: *mut XML_Char = null_mut::<XML_Char>();
    let mut nPrefixes: c_int = 0;
    let mut binding: *mut BINDING = null_mut::<BINDING>();
    let mut localPart: *const XML_Char = null::<XML_Char>();
    elementType =
        lookup(parser, &raw mut (*dtd).elementTypes, (*tagNamePtr).str_0, 0) as *mut ELEMENT_TYPE;
    if elementType.is_null() {
        let mut name: *const XML_Char = poolCopyString(&raw mut (*dtd).pool, (*tagNamePtr).str_0);
        if name.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        elementType = lookup(
            parser,
            &raw mut (*dtd).elementTypes,
            name,
            size_of::<ELEMENT_TYPE>(),
        ) as *mut ELEMENT_TYPE;
        if elementType.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        if (*parser).m_ns as c_int != 0 && setElementTypePrefix(parser, elementType) == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    nDefaultAtts = (*elementType).nDefaultAtts;
    n = (*enc).getAtts.expect("non-null function pointer")(
        enc,
        attStr,
        (*parser).m_attsSize,
        (*parser).m_atts,
    );
    if n > INT_MAX - nDefaultAtts {
        return XML_ERROR_NO_MEMORY;
    }
    if n + nDefaultAtts > (*parser).m_attsSize {
        let mut oldAttsSize: c_int = (*parser).m_attsSize;
        let mut temp: *mut ATTRIBUTE = null_mut::<ATTRIBUTE>();
        if nDefaultAtts > INT_MAX - INIT_ATTS_SIZE || n > INT_MAX - (nDefaultAtts + INIT_ATTS_SIZE)
        {
            return XML_ERROR_NO_MEMORY;
        }
        (*parser).m_attsSize = n + nDefaultAtts + INIT_ATTS_SIZE;
        temp = expat_realloc(
            parser,
            (*parser).m_atts as *mut c_void,
            ((*parser).m_attsSize as size_t).wrapping_mul(size_of::<ATTRIBUTE>()),
            3894,
        ) as *mut ATTRIBUTE;
        if temp.is_null() {
            (*parser).m_attsSize = oldAttsSize;
            return XML_ERROR_NO_MEMORY;
        }
        (*parser).m_atts = temp;
        if n > oldAttsSize {
            (*enc).getAtts.expect("non-null function pointer")(enc, attStr, n, (*parser).m_atts);
        }
    }
    appAtts = (*parser).m_atts as *mut *const XML_Char;
    i = 0;
    while i < n {
        let mut currAtt: *mut ATTRIBUTE = (*parser).m_atts.offset(i as isize);
        let mut attId: *mut ATTRIBUTE_ID = getAttributeId(
            parser,
            enc,
            (*currAtt).name,
            (*currAtt)
                .name
                .offset(
                    (*enc).nameLength.expect("non-null function pointer")(enc, (*currAtt).name)
                        as isize,
                ),
        );
        if attId.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        if *(*attId).name.offset(-1) != 0 {
            if enc == (*parser).m_encoding {
                (*parser).m_eventPtr = (*(*parser).m_atts.offset(i as isize)).name;
            }
            return XML_ERROR_DUPLICATE_ATTRIBUTE;
        }
        *(*attId).name.offset(-1) = 1i8;
        let fresh27 = attIndex;
        attIndex = attIndex + 1;
        let ref mut fresh28 = *appAtts.offset(fresh27 as isize);
        *fresh28 = (*attId).name;
        if (*(*parser).m_atts.offset(i as isize)).normalized == 0 {
            let mut result: XML_Error = XML_ERROR_NONE;
            let mut isCdata: XML_Bool = XML_TRUE;
            if (*attId).maybeTokenized != 0 {
                let mut j: c_int = 0;
                j = 0;
                while j < nDefaultAtts {
                    if attId
                        == (*(*elementType).defaultAtts.offset(j as isize)).id as *mut ATTRIBUTE_ID
                    {
                        isCdata = (*(*elementType).defaultAtts.offset(j as isize)).isCdata;
                        break;
                    } else {
                        j += 1;
                    }
                }
            }
            result = storeAttributeValue(
                parser,
                enc,
                isCdata,
                (*(*parser).m_atts.offset(i as isize)).valuePtr,
                (*(*parser).m_atts.offset(i as isize)).valueEnd,
                &raw mut (*parser).m_tempPool,
                account,
            );
            if result as u64 != 0 {
                return result;
            }
            let ref mut fresh29 = *appAtts.offset(attIndex as isize);
            *fresh29 = (*parser).m_tempPool.start;
            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
        } else {
            let ref mut fresh30 = *appAtts.offset(attIndex as isize);
            *fresh30 = poolStoreString(
                &raw mut (*parser).m_tempPool,
                enc,
                (*(*parser).m_atts.offset(i as isize)).valuePtr,
                (*(*parser).m_atts.offset(i as isize)).valueEnd,
            );
            if (*appAtts.offset(attIndex as isize)).is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
        }
        if !(*attId).prefix.is_null() {
            if (*attId).xmlns != 0 {
                let mut result_0: XML_Error = addBinding(
                    parser,
                    (*attId).prefix,
                    attId,
                    *appAtts.offset(attIndex as isize),
                    bindingsPtr,
                );
                if result_0 as u64 != 0 {
                    return result_0;
                }
                attIndex -= 1;
            } else {
                attIndex += 1;
                nPrefixes += 1;
                *(*attId).name.offset(-1isize) = 2i8;
            }
        } else {
            attIndex += 1;
        }
        i += 1;
    }
    (*parser).m_nSpecifiedAtts = attIndex;
    if !(*elementType).idAtt.is_null() && *(*(*elementType).idAtt).name.offset(-1) as c_int != 0 {
        i = 0;
        while i < attIndex {
            if *appAtts.offset(i as isize) == (*(*elementType).idAtt).name as *const XML_Char {
                (*parser).m_idAttIndex = i;
                break;
            } else {
                i += 2i32;
            }
        }
    } else {
        (*parser).m_idAttIndex = -(1i32);
    }
    i = 0;
    while i < nDefaultAtts {
        let mut da: *const DEFAULT_ATTRIBUTE = (*elementType).defaultAtts.offset(i as isize);
        if *(*(*da).id).name.offset(-1) == 0 && !(*da).value.is_null() {
            if !(*(*da).id).prefix.is_null() {
                if (*(*da).id).xmlns != 0 {
                    let mut result_1: XML_Error = addBinding(
                        parser,
                        (*(*da).id).prefix,
                        (*da).id,
                        (*da).value,
                        bindingsPtr,
                    );
                    if result_1 as u64 != 0 {
                        return result_1;
                    }
                } else {
                    *(*(*da).id).name.offset(-1) = 2i8;
                    nPrefixes += 1;
                    let fresh31 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut fresh32 = *appAtts.offset(fresh31 as isize);
                    *fresh32 = (*(*da).id).name;
                    let fresh33 = attIndex;
                    attIndex = attIndex + 1;
                    let ref mut fresh34 = *appAtts.offset(fresh33 as isize);
                    *fresh34 = (*da).value;
                }
            } else {
                *(*(*da).id).name.offset(-1) = 1i8;
                let fresh35 = attIndex;
                attIndex = attIndex + 1;
                let ref mut fresh36 = *appAtts.offset(fresh35 as isize);
                *fresh36 = (*(*da).id).name;
                let fresh37 = attIndex;
                attIndex = attIndex + 1;
                let ref mut fresh38 = *appAtts.offset(fresh37 as isize);
                *fresh38 = (*da).value;
            }
        }
        i += 1;
    }
    let ref mut fresh39 = *appAtts.offset(attIndex as isize);
    *fresh39 = null::<XML_Char>();
    i = 0;
    if nPrefixes != 0 {
        let mut j_0: c_uint = 0;
        let mut version: c_ulong = (*parser).m_nsAttsVersion;
        if (*parser).m_nsAttsPower as usize >= (size_of::<c_uint>()).wrapping_mul(8usize) {
            return XML_ERROR_NO_MEMORY;
        }
        let mut nsAttsSize: c_uint = (1) << (*parser).m_nsAttsPower as c_int;
        let mut oldNsAttsPower: c_uchar = (*parser).m_nsAttsPower;
        if nPrefixes << 1 >> (*parser).m_nsAttsPower as c_int != 0 {
            let mut temp_0: *mut NS_ATT = null_mut::<NS_ATT>();
            loop {
                let fresh40 = (*parser).m_nsAttsPower;
                (*parser).m_nsAttsPower = (*parser).m_nsAttsPower.wrapping_add(1);
                if !(nPrefixes >> fresh40 as c_int != 0) {
                    break;
                }
            }
            if ((*parser).m_nsAttsPower as c_int) < 3 {
                (*parser).m_nsAttsPower = 3u8;
            }
            if (*parser).m_nsAttsPower as usize >= (size_of::<c_uint>()).wrapping_mul(8usize) {
                (*parser).m_nsAttsPower = oldNsAttsPower;
                return XML_ERROR_NO_MEMORY;
            }
            nsAttsSize = (1) << (*parser).m_nsAttsPower as c_int;
            temp_0 = expat_realloc(
                parser,
                (*parser).m_nsAtts as *mut c_void,
                (nsAttsSize as size_t).wrapping_mul(size_of::<NS_ATT>()),
                4089,
            ) as *mut NS_ATT;
            if temp_0.is_null() {
                (*parser).m_nsAttsPower = oldNsAttsPower;
                return XML_ERROR_NO_MEMORY;
            }
            (*parser).m_nsAtts = temp_0;
            version = 0u64;
        }
        if version == 0 {
            version = INIT_ATTS_VERSION as c_ulong;
            j_0 = nsAttsSize;
            while j_0 != 0 {
                j_0 = j_0.wrapping_sub(1);
                (*(*parser).m_nsAtts.offset(j_0 as isize)).version = version;
            }
        }
        version = version.wrapping_sub(1);
        (*parser).m_nsAttsVersion = version;
        while i < attIndex {
            let mut s: *const XML_Char = *appAtts.offset(i as isize);
            if *s.offset(-1) as c_int == 2 {
                let mut id: *mut ATTRIBUTE_ID = null_mut::<ATTRIBUTE_ID>();
                let mut b: *const BINDING = null::<BINDING>();
                let mut uriHash: c_ulong = 0;
                let mut sip_state: siphash = siphash {
                    v0: 0,
                    v1: 0,
                    v2: 0,
                    v3: 0,
                    buf: [0; 8],
                    p: null_mut::<c_uchar>(),
                    c: 0,
                };
                let mut sip_key: sipkey = sipkey { k: [0; 2] };
                copy_salt_to_sipkey(parser, &raw mut sip_key);
                sip24_init(&raw mut sip_state, &raw mut sip_key);
                *(s as *mut XML_Char).offset(-1) = 0i8;
                id = lookup(parser, &raw mut (*dtd).attributeIds, s, 0) as *mut ATTRIBUTE_ID;
                if id.is_null() || (*id).prefix.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                b = (*(*id).prefix).binding;
                if b.is_null() {
                    return XML_ERROR_UNBOUND_PREFIX;
                }
                j_0 = 0;
                while j_0 < (*b).uriLen as c_uint {
                    let c: XML_Char = *(*b).uri.offset(j_0 as isize);
                    if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                        && poolGrow(&raw mut (*parser).m_tempPool) == 0
                    {
                        0
                    } else {
                        let fresh41 = (*parser).m_tempPool.ptr;
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                        *fresh41 = c;
                        1
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                sip24_update(
                    &raw mut sip_state,
                    (*b).uri as *const c_void,
                    ((*b).uriLen as size_t).wrapping_mul(size_of::<XML_Char>()),
                );
                loop {
                    let fresh42 = s;
                    s = s.offset(1);
                    if !(*fresh42 as c_int != 0x3a) {
                        break;
                    }
                }
                sip24_update(
                    &raw mut sip_state,
                    s as *const c_void,
                    keylen(s).wrapping_mul(size_of::<XML_Char>()),
                );
                loop {
                    if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                        && poolGrow(&raw mut (*parser).m_tempPool) == 0
                    {
                        0
                    } else {
                        let fresh43 = (*parser).m_tempPool.ptr;
                        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                        *fresh43 = *s;
                        1
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                    let fresh44 = s;
                    s = s.offset(1);
                    if !(*fresh44 != 0) {
                        break;
                    }
                }
                uriHash = sip24_final(&raw mut sip_state);
                let mut step: c_uchar = 0;
                let mut mask: c_ulong = nsAttsSize.wrapping_sub(1u32) as c_ulong;
                j_0 = (uriHash & mask) as c_uint;
                while (*(*parser).m_nsAtts.offset(j_0 as isize)).version == version {
                    if uriHash == (*(*parser).m_nsAtts.offset(j_0 as isize)).hash {
                        let mut s1: *const XML_Char = (*parser).m_tempPool.start;
                        let mut s2: *const XML_Char =
                            (*(*parser).m_nsAtts.offset(j_0 as isize)).uriName;
                        while *s1 as c_int == *s2 as c_int && *s1 as c_int != 0 {
                            s1 = s1.offset(1);
                            s2 = s2.offset(1);
                        }
                        if *s1 as c_int == 0 {
                            return XML_ERROR_DUPLICATE_ATTRIBUTE;
                        }
                    }
                    if step == 0 {
                        step = ((uriHash & !mask) >> (*parser).m_nsAttsPower as c_int - 1i32
                            & mask >> 2i32
                            | 1u64) as c_uchar;
                    }
                    if j_0 < step as c_uint {
                        j_0 = j_0.wrapping_add(nsAttsSize.wrapping_sub(step as c_uint));
                    } else {
                        j_0 = j_0.wrapping_sub(step as c_uint);
                    };
                }
                if (*parser).m_ns_triplets != 0 {
                    *(*parser).m_tempPool.ptr.offset(-1) = (*parser).m_namespaceSeparator;
                    s = (*(*b).prefix).name;
                    loop {
                        if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                            && poolGrow(&raw mut (*parser).m_tempPool) == 0
                        {
                            0
                        } else {
                            let fresh45 = (*parser).m_tempPool.ptr;
                            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                            *fresh45 = *s;
                            1
                        } == 0
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        let fresh46 = s;
                        s = s.offset(1);
                        if !(*fresh46 != 0) {
                            break;
                        }
                    }
                }
                s = (*parser).m_tempPool.start;
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                let ref mut fresh47 = *appAtts.offset(i as isize);
                *fresh47 = s;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).version = version;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).hash = uriHash;
                let ref mut fresh48 = (*(*parser).m_nsAtts.offset(j_0 as isize)).uriName;
                *fresh48 = s;
                nPrefixes -= 1;
                if nPrefixes == 0 {
                    i += 2;
                    break;
                }
            } else {
                *(s as *mut XML_Char).offset(-1isize) = 0i8;
            }
            i += 2;
        }
    }
    while i < attIndex {
        *(*appAtts.offset(i as isize) as *mut XML_Char).offset(-1) = 0i8;
        i += 2;
    }
    binding = *bindingsPtr;
    while !binding.is_null() {
        *(*(*binding).attId).name.offset(-1) = 0i8;
        binding = (*binding).nextTagBinding;
    }
    if (*parser).m_ns == 0 {
        return XML_ERROR_NONE;
    }
    if !(*elementType).prefix.is_null() {
        binding = (*(*elementType).prefix).binding;
        if binding.is_null() {
            return XML_ERROR_UNBOUND_PREFIX;
        }
        localPart = (*tagNamePtr).str_0;
        loop {
            let fresh49 = localPart;
            localPart = localPart.offset(1);
            if !(*fresh49 as c_int != 0x3a) {
                break;
            }
        }
    } else if !(*dtd).defaultPrefix.binding.is_null() {
        binding = (*dtd).defaultPrefix.binding;
        localPart = (*tagNamePtr).str_0;
    } else {
        return XML_ERROR_NONE;
    }
    prefixLen = 0;
    if (*parser).m_ns_triplets as c_int != 0 && !(*(*binding).prefix).name.is_null() {
        loop {
            let fresh50 = prefixLen;
            prefixLen = prefixLen + 1;
            if !(*(*(*binding).prefix).name.offset(fresh50 as isize) != 0) {
                break;
            }
        }
    }
    (*tagNamePtr).localPart = localPart;
    (*tagNamePtr).uriLen = (*binding).uriLen;
    (*tagNamePtr).prefix = (*(*binding).prefix).name;
    (*tagNamePtr).prefixLen = prefixLen;
    i = 0;
    loop {
        let fresh51 = i;
        i = i + 1;
        if !(*localPart.offset(fresh51 as isize) != 0) {
            break;
        }
    }
    if (*binding).uriLen > INT_MAX - prefixLen || i > INT_MAX - ((*binding).uriLen + prefixLen) {
        return XML_ERROR_NO_MEMORY;
    }
    n = i + (*binding).uriLen + prefixLen;
    if n > (*binding).uriAlloc {
        let mut p: *mut TAG = null_mut::<TAG>();
        if n > INT_MAX - EXPAND_SPARE {
            return XML_ERROR_NO_MEMORY;
        }
        uri = expat_malloc(
            parser,
            ((n + 24) as size_t).wrapping_mul(size_of::<XML_Char>()),
            4270,
        ) as *mut XML_Char;
        if uri.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        (*binding).uriAlloc = n + EXPAND_SPARE;
        memcpy(
            uri as *mut c_void,
            (*binding).uri as *const c_void,
            ((*binding).uriLen as size_t).wrapping_mul(size_of::<XML_Char>()),
        );
        p = (*parser).m_tagStack;
        while !p.is_null() {
            if (*p).name.str_0 == (*binding).uri as *const XML_Char {
                (*p).name.str_0 = uri;
            }
            p = (*p).parent;
        }
        expat_free(parser, (*binding).uri as *mut c_void, 4278);
        (*binding).uri = uri;
    }
    uri = (*binding).uri.offset((*binding).uriLen as isize);
    memcpy(
        uri as *mut c_void,
        localPart as *const c_void,
        (i as size_t).wrapping_mul(size_of::<XML_Char>()),
    );
    if prefixLen != 0 {
        uri = uri.offset((i - 1) as isize);
        *uri = (*parser).m_namespaceSeparator;
        memcpy(
            uri.offset(1isize) as *mut c_void,
            (*(*binding).prefix).name as *const c_void,
            (prefixLen as size_t).wrapping_mul(size_of::<XML_Char>()),
        );
    }
    (*tagNamePtr).str_0 = (*binding).uri;
    return XML_ERROR_NONE;
}

unsafe extern "C" fn is_rfc3986_uri_char(mut candidate: XML_Char) -> XML_Bool {
    match candidate as c_int {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104
        | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118
        | 119 | 120 | 121 | 122 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 37 | 45
        | 46 | 95 | 126 | 58 | 47 | 63 | 35 | 91 | 93 | 64 | 33 | 36 | 38 | 39 | 40 | 41 | 42
        | 43 | 44 | 59 | 61 => return XML_TRUE,
        _ => return XML_FALSE,
    };
}

unsafe extern "C" fn addBinding(
    mut parser: XML_Parser,
    mut prefix: *mut PREFIX,
    mut attId: *const ATTRIBUTE_ID,
    mut uri: *const XML_Char,
    mut bindingsPtr: *mut *mut BINDING,
) -> XML_Error {
    static mut xmlNamespace: [XML_Char; 37] = [
        ASCII_h as XML_Char,
        ASCII_t as XML_Char,
        ASCII_t as XML_Char,
        ASCII_p as XML_Char,
        ASCII_COLON as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_w as XML_Char,
        ASCII_3 as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_o as XML_Char,
        ASCII_r as XML_Char,
        ASCII_g as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_X as XML_Char,
        ASCII_M as XML_Char,
        ASCII_L as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_1 as XML_Char,
        ASCII_9 as XML_Char,
        ASCII_9 as XML_Char,
        ASCII_8 as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_n as XML_Char,
        ASCII_a as XML_Char,
        ASCII_m as XML_Char,
        ASCII_e as XML_Char,
        ASCII_s as XML_Char,
        ASCII_p as XML_Char,
        ASCII_a as XML_Char,
        ASCII_c as XML_Char,
        ASCII_e as XML_Char,
        '\0' as XML_Char,
    ];
    static mut xmlnsNamespace: [XML_Char; 30] = [
        ASCII_h as XML_Char,
        ASCII_t as XML_Char,
        ASCII_t as XML_Char,
        ASCII_p as XML_Char,
        ASCII_COLON as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_w as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_w as XML_Char,
        ASCII_3 as XML_Char,
        ASCII_PERIOD as XML_Char,
        ASCII_o as XML_Char,
        ASCII_r as XML_Char,
        ASCII_g as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_2 as XML_Char,
        ASCII_0 as XML_Char,
        ASCII_0 as XML_Char,
        ASCII_0 as XML_Char,
        ASCII_SLASH as XML_Char,
        ASCII_x as XML_Char,
        ASCII_m as XML_Char,
        ASCII_l as XML_Char,
        ASCII_n as XML_Char,
        ASCII_s as XML_Char,
        ASCII_SLASH as XML_Char,
        '\0' as XML_Char,
    ];
    let mut mustBeXML: XML_Bool = XML_FALSE;
    let mut isXML: XML_Bool = XML_TRUE;
    let mut isXMLNS: XML_Bool = XML_TRUE;
    let mut b: *mut BINDING = null_mut::<BINDING>();
    let mut len: c_int = 0;
    if *uri as c_int == '\0' as i32 && !(*prefix).name.is_null() {
        return XML_ERROR_UNDECLARING_PREFIX;
    }
    if !(*prefix).name.is_null()
        && *(*prefix).name.offset(0) as c_int == 0x78
        && *(*prefix).name.offset(1) as c_int == 0x6d
        && *(*prefix).name.offset(2) as c_int == 0x6c
    {
        if *(*prefix).name.offset(3) as c_int == 0x6e
            && *(*prefix).name.offset(4) as c_int == 0x73
            && *(*prefix).name.offset(5) as c_int == '\0' as i32
        {
            return XML_ERROR_RESERVED_PREFIX_XMLNS;
        }
        if *(*prefix).name.offset(3) as c_int == '\0' as i32 {
            mustBeXML = XML_TRUE;
        }
    }
    len = 0;
    while *uri.offset(len as isize) != 0 {
        if isXML as c_int != 0
            && (len > xmlLen
                || *uri.offset(len as isize) as c_int != xmlNamespace[len as usize] as c_int)
        {
            isXML = XML_FALSE;
        }
        if mustBeXML == 0
            && isXMLNS as c_int != 0
            && (len > xmlnsLen
                || *uri.offset(len as isize) as c_int != xmlnsNamespace[len as usize] as c_int)
        {
            isXMLNS = XML_FALSE;
        }
        if (*parser).m_ns as c_int != 0
            && *uri.offset(len as isize) as c_int == (*parser).m_namespaceSeparator as c_int
            && is_rfc3986_uri_char(*uri.offset(len as isize)) == 0
        {
            return XML_ERROR_SYNTAX;
        }
        len += 1;
    }
    isXML = (isXML as c_int != 0 && len == xmlLen) as XML_Bool;
    isXMLNS = (isXMLNS as c_int != 0 && len == xmlnsLen) as XML_Bool;
    if mustBeXML as c_int != isXML as c_int {
        return (if mustBeXML as c_int != 0 {
            XML_ERROR_RESERVED_PREFIX_XML as c_int
        } else {
            XML_ERROR_RESERVED_NAMESPACE_URI as c_int
        }) as XML_Error;
    }
    if isXMLNS != 0 {
        return XML_ERROR_RESERVED_NAMESPACE_URI;
    }
    if (*parser).m_namespaceSeparator != 0 {
        len += 1;
    }
    if !(*parser).m_freeBindingList.is_null() {
        b = (*parser).m_freeBindingList;
        if len > (*b).uriAlloc {
            if len > INT_MAX - EXPAND_SPARE {
                return XML_ERROR_NO_MEMORY;
            }
            let mut temp: *mut XML_Char = expat_realloc(
                parser,
                (*b).uri as *mut c_void,
                (size_of::<XML_Char>()).wrapping_mul((len + 24) as size_t),
                4517,
            ) as *mut XML_Char;
            if temp.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            (*b).uri = temp;
            (*b).uriAlloc = len + EXPAND_SPARE;
        }
        (*parser).m_freeBindingList = (*b).nextTagBinding;
    } else {
        b = expat_malloc(parser, size_of::<BINDING>(), 4525) as *mut BINDING;
        if b.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        if len > INT_MAX - EXPAND_SPARE {
            return XML_ERROR_NO_MEMORY;
        }
        (*b).uri = expat_malloc(
            parser,
            (size_of::<XML_Char>()).wrapping_mul((len + 24) as size_t),
            4543,
        ) as *mut XML_Char;
        if (*b).uri.is_null() {
            expat_free(parser, b as *mut c_void, 4545);
            return XML_ERROR_NO_MEMORY;
        }
        (*b).uriAlloc = len + EXPAND_SPARE;
    }
    (*b).uriLen = len;
    memcpy(
        (*b).uri as *mut c_void,
        uri as *const c_void,
        (len as size_t).wrapping_mul(size_of::<XML_Char>()),
    );
    if (*parser).m_namespaceSeparator != 0 {
        *(*b).uri.offset((len - 1i32) as isize) = (*parser).m_namespaceSeparator;
    }
    (*b).prefix = prefix;
    (*b).attId = attId;
    (*b).prevPrefixBinding = (*prefix).binding;
    if *uri as c_int == '\0' as i32 && prefix == &raw mut (*(*parser).m_dtd).defaultPrefix {
        (*prefix).binding = null_mut::<BINDING>();
    } else {
        (*prefix).binding = b;
    }
    (*b).nextTagBinding = *bindingsPtr;
    *bindingsPtr = b;
    if !attId.is_null() && (*parser).m_startNamespaceDeclHandler.is_some() {
        (*parser)
            .m_startNamespaceDeclHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            (*prefix).name,
            if !(*prefix).binding.is_null() {
                uri
            } else {
                null::<XML_Char>()
            },
        );
    }
    return XML_ERROR_NONE;
}

unsafe extern "C" fn cdataSectionProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = doCdataSection(
        parser,
        (*parser).m_encoding,
        &raw mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
        XML_ACCOUNT_DIRECT,
    );
    if result != XML_ERROR_NONE {
        return result;
    }
    if !start.is_null() {
        if !(*parser).m_parentParser.is_null() {
            (*parser).m_processor = Some(
                externalEntityContentProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            );
            return externalEntityContentProcessor(parser, start, end, endPtr);
        } else {
            (*parser).m_processor = Some(
                contentProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            );
            return contentProcessor(parser, start, end, endPtr);
        }
    }
    return result;
}

unsafe extern "C" fn doCdataSection(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut startPtr: *mut *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
    mut account: XML_Account,
) -> XML_Error {
    let mut s: *const c_char = *startPtr;
    let mut eventPP: *mut *const c_char = null_mut::<*const c_char>();
    let mut eventEndPP: *mut *const c_char = null_mut::<*const c_char>();
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    *startPtr = null::<c_char>();
    loop {
        let mut next: *const c_char = s;
        let mut tok: c_int =
            (*enc).scanners[2].expect("non-null function pointer")(enc, s, end, &raw mut next);
        if accountingDiffTolerated(parser, tok, s, next, 4619, account) == 0 {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        *eventEndPP = next;
        match tok {
            XML_TOK_CDATA_SECT_CLOSE => {
                if (*parser).m_endCdataSectionHandler.is_some() {
                    (*parser)
                        .m_endCdataSectionHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                } else if 0 != 0 && (*parser).m_characterDataHandler.is_some() {
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_dataBuf,
                        0i32,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                *startPtr = next;
                *nextPtr = next;
                if (*parser).m_parsingStatus.parsing == XML_FINISHED {
                    return XML_ERROR_ABORTED;
                } else {
                    return XML_ERROR_NONE;
                }
            }
            XML_TOK_DATA_NEWLINE => {
                if (*parser).m_characterDataHandler.is_some() {
                    let mut c: XML_Char = 0xa;
                    (*parser)
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        &raw mut c,
                        1i32,
                    );
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_DATA_CHARS => {
                let mut charDataHandler: XML_CharacterDataHandler =
                    (*parser).m_characterDataHandler;
                if charDataHandler.is_some() {
                    if (*enc).isUtf8 == 0 {
                        loop {
                            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf;
                            let convert_res: XML_Convert_Result =
                                (*enc).utf8Convert.expect("non-null function pointer")(
                                    enc,
                                    &raw mut s,
                                    next,
                                    &raw mut dataPtr,
                                    (*parser).m_dataBufEnd,
                                );
                            *eventEndPP = next;
                            charDataHandler.expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*parser).m_dataBuf,
                                dataPtr.offset_from((*parser).m_dataBuf) as c_int,
                            );
                            if convert_res == XML_CONVERT_COMPLETED
                                || convert_res == XML_CONVERT_INPUT_INCOMPLETE
                            {
                                break;
                            }
                            *eventPP = s;
                        }
                    } else {
                        charDataHandler.expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            s,
                            (next).offset_from(s) as c_int,
                        );
                    }
                } else if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_INVALID => {
                *eventPP = next;
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            XML_TOK_PARTIAL | XML_TOK_NONE => {
                if haveMore != 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_CDATA_SECTION;
            }
            _ => {
                *eventPP = next;
                return XML_ERROR_UNEXPECTED_STATE;
            }
        }
        match (*parser).m_parsingStatus.parsing {
            3 => {
                *eventPP = next;
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => {
                *eventPP = next;
                return XML_ERROR_ABORTED;
            }
            1 => {
                if (*parser).m_reenter != 0 {
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }
            _ => {}
        }
        s = next;
        *eventPP = s;
    }
}

unsafe extern "C" fn ignoreSectionProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = doIgnoreSection(
        parser,
        (*parser).m_encoding,
        &raw mut start,
        end,
        endPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
    );
    if result != XML_ERROR_NONE {
        return result;
    }
    if !start.is_null() {
        (*parser).m_processor = Some(
            prologProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
        return prologProcessor(parser, start, end, endPtr);
    }
    return result;
}

unsafe extern "C" fn doIgnoreSection(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut startPtr: *mut *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
) -> XML_Error {
    let mut next: *const c_char = *startPtr;
    let mut tok: c_int = 0;
    let mut s: *const c_char = *startPtr;
    let mut eventPP: *mut *const c_char = null_mut::<*const c_char>();
    let mut eventEndPP: *mut *const c_char = null_mut::<*const c_char>();
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        *eventPP = s;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    *startPtr = null::<c_char>();
    tok = (*enc).scanners[3].expect("non-null function pointer")(enc, s, end, &raw mut next);
    if accountingDiffTolerated(parser, tok, s, next, 4778, XML_ACCOUNT_DIRECT) == 0 {
        accountingOnAbort(parser);
        return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    *eventEndPP = next;
    match tok {
        XML_TOK_IGNORE_SECT => {
            if (*parser).m_defaultHandler.is_some() {
                reportDefault(parser, enc, s, next);
            }
            *startPtr = next;
            *nextPtr = next;
            if (*parser).m_parsingStatus.parsing == XML_FINISHED {
                return XML_ERROR_ABORTED;
            } else {
                return XML_ERROR_NONE;
            }
        }
        XML_TOK_INVALID => {
            *eventPP = next;
            return XML_ERROR_INVALID_TOKEN;
        }
        XML_TOK_PARTIAL_CHAR => {
            if haveMore != 0 {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_PARTIAL_CHAR;
        }
        XML_TOK_PARTIAL | XML_TOK_NONE => {
            if haveMore != 0 {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            return XML_ERROR_SYNTAX;
        }
        _ => {
            *eventPP = next;
            return XML_ERROR_UNEXPECTED_STATE;
        }
    };
}

unsafe extern "C" fn initializeEncoding(mut parser: XML_Parser) -> XML_Error {
    let mut s: *const c_char = null::<c_char>();
    s = (*parser).m_protocolEncodingName;
    if if (*parser).m_ns as c_int != 0 {
        Some(
            XmlInitEncodingNS
                as unsafe extern "C" fn(
                    *mut INIT_ENCODING,
                    *mut *const ENCODING,
                    *const c_char,
                ) -> c_int,
        )
    } else {
        Some(
            XmlInitEncoding
                as unsafe extern "C" fn(
                    *mut INIT_ENCODING,
                    *mut *const ENCODING,
                    *const c_char,
                ) -> c_int,
        )
    }
    .expect("non-null function pointer")(
        &raw mut (*parser).m_initEncoding,
        &raw mut (*parser).m_encoding,
        s,
    ) != 0
    {
        return XML_ERROR_NONE;
    }
    return handleUnknownEncoding(parser, (*parser).m_protocolEncodingName);
}

unsafe extern "C" fn processXmlDecl(
    mut parser: XML_Parser,
    mut isGeneralTextEntity: c_int,
    mut s: *const c_char,
    mut next: *const c_char,
) -> XML_Error {
    let mut encodingName: *const c_char = null::<c_char>();
    let mut storedEncName: *const XML_Char = null::<XML_Char>();
    let mut newEncoding: *const ENCODING = null::<ENCODING>();
    let mut version: *const c_char = null::<c_char>();
    let mut versionend: *const c_char = null::<c_char>();
    let mut storedversion: *const XML_Char = null::<XML_Char>();
    let mut standalone: c_int = -(1);
    if accountingDiffTolerated(parser, XML_TOK_XML_DECL, s, next, 4870, XML_ACCOUNT_DIRECT) == 0 {
        accountingOnAbort(parser);
        return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    if if (*parser).m_ns as c_int != 0 {
        Some(
            XmlParseXmlDeclNS
                as unsafe extern "C" fn(
                    c_int,
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                    *mut *const c_char,
                    *mut *const c_char,
                    *mut *const c_char,
                    *mut *const ENCODING,
                    *mut c_int,
                ) -> c_int,
        )
    } else {
        Some(
            XmlParseXmlDecl
                as unsafe extern "C" fn(
                    c_int,
                    *const ENCODING,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                    *mut *const c_char,
                    *mut *const c_char,
                    *mut *const c_char,
                    *mut *const ENCODING,
                    *mut c_int,
                ) -> c_int,
        )
    }
    .expect("non-null function pointer")(
        isGeneralTextEntity,
        (*parser).m_encoding,
        s,
        next,
        &raw mut (*parser).m_eventPtr,
        &raw mut version,
        &raw mut versionend,
        &raw mut encodingName,
        &raw mut newEncoding,
        &raw mut standalone,
    ) == 0
    {
        if isGeneralTextEntity != 0 {
            return XML_ERROR_TEXT_DECL;
        } else {
            return XML_ERROR_XML_DECL;
        }
    }
    if isGeneralTextEntity == 0 && standalone == 1 {
        (*(*parser).m_dtd).standalone = XML_TRUE;
        if (*parser).m_paramEntityParsing == XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE {
            (*parser).m_paramEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
        }
    }
    if (*parser).m_xmlDeclHandler.is_some() {
        if !encodingName.is_null() {
            storedEncName = poolStoreString(
                &raw mut (*parser).m_temp2Pool,
                (*parser).m_encoding,
                encodingName,
                encodingName.offset((*(*parser).m_encoding)
                    .nameLength
                    .expect("non-null function pointer")(
                    (*parser).m_encoding, encodingName
                ) as isize),
            );
            if storedEncName.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            (*parser).m_temp2Pool.start = (*parser).m_temp2Pool.ptr;
        }
        if !version.is_null() {
            storedversion = poolStoreString(
                &raw mut (*parser).m_temp2Pool,
                (*parser).m_encoding,
                version,
                versionend.offset(-((*(*parser).m_encoding).minBytesPerChar as isize)),
            );
            if storedversion.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
        }
        (*parser)
            .m_xmlDeclHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            storedversion,
            storedEncName,
            standalone,
        );
    } else if (*parser).m_defaultHandler.is_some() {
        reportDefault(parser, (*parser).m_encoding, s, next);
    }
    if (*parser).m_protocolEncodingName.is_null() {
        if !newEncoding.is_null() {
            if (*newEncoding).minBytesPerChar != (*(*parser).m_encoding).minBytesPerChar
                || (*newEncoding).minBytesPerChar == 2 && newEncoding != (*parser).m_encoding
            {
                (*parser).m_eventPtr = encodingName;
                return XML_ERROR_INCORRECT_ENCODING;
            }
            (*parser).m_encoding = newEncoding;
        } else if !encodingName.is_null() {
            let mut result: XML_Error = XML_ERROR_NONE;
            if storedEncName.is_null() {
                storedEncName = poolStoreString(
                    &raw mut (*parser).m_temp2Pool,
                    (*parser).m_encoding,
                    encodingName,
                    encodingName.offset((*(*parser).m_encoding)
                        .nameLength
                        .expect("non-null function pointer")(
                        (*parser).m_encoding, encodingName
                    ) as isize),
                );
                if storedEncName.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            result = handleUnknownEncoding(parser, storedEncName);
            poolClear(&raw mut (*parser).m_temp2Pool);
            if result == XML_ERROR_UNKNOWN_ENCODING {
                (*parser).m_eventPtr = encodingName;
            }
            return result;
        }
    }
    if !storedEncName.is_null() || !storedversion.is_null() {
        poolClear(&raw mut (*parser).m_temp2Pool);
    }
    return XML_ERROR_NONE;
}

unsafe extern "C" fn handleUnknownEncoding(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Error {
    if (*parser).m_unknownEncodingHandler.is_some() {
        let mut info: XML_Encoding = XML_Encoding {
            map: [0; 256],
            data: null_mut::<c_void>(),
            convert: None,
            release: None,
        };
        let mut i: c_int = 0;
        i = 0;
        while i < 256 {
            info.map[i as usize] = -(1);
            i += 1;
        }
        info.convert = None;
        info.data = NULL;
        info.release = None;
        if (*parser)
            .m_unknownEncodingHandler
            .expect("non-null function pointer")(
            (*parser).m_unknownEncodingHandlerData,
            encodingName,
            &raw mut info,
        ) != 0
        {
            let mut enc: *mut ENCODING = null_mut::<ENCODING>();
            (*parser).m_unknownEncodingMem =
                expat_malloc(parser, XmlSizeOfUnknownEncoding() as size_t, 4963);
            if (*parser).m_unknownEncodingMem.is_null() {
                if info.release.is_some() {
                    info.release.expect("non-null function pointer")(info.data);
                }
                return XML_ERROR_NO_MEMORY;
            }
            enc = if (*parser).m_ns as c_int != 0 {
                Some(
                    XmlInitUnknownEncodingNS
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_int,
                            CONVERTER,
                            *mut c_void,
                        ) -> *mut ENCODING,
                )
            } else {
                Some(
                    XmlInitUnknownEncoding
                        as unsafe extern "C" fn(
                            *mut c_void,
                            *const c_int,
                            CONVERTER,
                            *mut c_void,
                        ) -> *mut ENCODING,
                )
            }
            .expect("non-null function pointer")(
                (*parser).m_unknownEncodingMem,
                &raw mut info.map as *mut c_int,
                info.convert,
                info.data,
            );
            if !enc.is_null() {
                (*parser).m_unknownEncodingData = info.data;
                (*parser).m_unknownEncodingRelease = info.release;
                (*parser).m_encoding = enc;
                return XML_ERROR_NONE;
            }
        }
        if info.release.is_some() {
            info.release.expect("non-null function pointer")(info.data);
        }
    }
    return XML_ERROR_UNKNOWN_ENCODING;
}

unsafe extern "C" fn prologInitProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = initializeEncoding(parser);
    if result != XML_ERROR_NONE {
        return result;
    }
    (*parser).m_processor = Some(
        prologProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    return prologProcessor(parser, s, end, nextPtr);
}

unsafe extern "C" fn externalParEntInitProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = initializeEncoding(parser);
    if result != XML_ERROR_NONE {
        return result;
    }
    (*(*parser).m_dtd).paramEntityRead = XML_TRUE;
    if (*parser).m_prologState.inEntityValue != 0 {
        (*parser).m_processor = Some(
            entityValueInitProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
        return entityValueInitProcessor(parser, s, end, nextPtr);
    } else {
        (*parser).m_processor = Some(
            externalParEntProcessor
                as unsafe extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
        return externalParEntProcessor(parser, s, end, nextPtr);
    };
}

unsafe extern "C" fn entityValueInitProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut tok: c_int = 0;
    let mut start: *const c_char = s;
    let mut next: *const c_char = start;
    (*parser).m_eventPtr = start;
    loop {
        tok = (*(*parser).m_encoding).scanners[0].expect("non-null function pointer")(
            (*parser).m_encoding,
            start,
            end,
            &raw mut next,
        );
        (*parser).m_eventEndPtr = next;
        if tok <= 0 {
            if (*parser).m_parsingStatus.finalBuffer == 0 && tok != XML_TOK_INVALID {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            match tok {
                XML_TOK_INVALID => return XML_ERROR_INVALID_TOKEN,
                XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
                XML_TOK_NONE | _ => {}
            }
            return storeEntityValue(
                parser,
                (*parser).m_encoding,
                s,
                end,
                XML_ACCOUNT_DIRECT,
                null_mut::<*const c_char>(),
            );
        } else if tok == XML_TOK_XML_DECL {
            let mut result: XML_Error = XML_ERROR_NONE;
            result = processXmlDecl(parser, 0, start, next);
            if result != XML_ERROR_NONE {
                return result;
            }
            if (*parser).m_parsingStatus.parsing == XML_FINISHED {
                return XML_ERROR_ABORTED;
            }
            *nextPtr = next;
            (*parser).m_processor = Some(
                entityValueProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            );
            return entityValueProcessor(parser, next, end, nextPtr);
        } else if tok == XML_TOK_BOM {
            if accountingDiffTolerated(parser, tok, s, next, 5077, XML_ACCOUNT_DIRECT) == 0 {
                accountingOnAbort(parser);
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            *nextPtr = next;
            s = next;
        } else if tok == XML_TOK_INSTANCE_START {
            *nextPtr = next;
            return XML_ERROR_SYNTAX;
        }
        start = next;
        (*parser).m_eventPtr = start;
    }
}

unsafe extern "C" fn externalParEntProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = s;
    let mut tok: c_int = 0;
    tok = (*(*parser).m_encoding).scanners[0].expect("non-null function pointer")(
        (*parser).m_encoding,
        s,
        end,
        &raw mut next,
    );
    if tok <= 0 {
        if (*parser).m_parsingStatus.finalBuffer == 0 && tok != XML_TOK_INVALID {
            *nextPtr = s;
            return XML_ERROR_NONE;
        }
        match tok {
            XML_TOK_INVALID => return XML_ERROR_INVALID_TOKEN,
            XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
            XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
            XML_TOK_NONE | _ => {}
        }
    } else if tok == XML_TOK_BOM {
        if accountingDiffTolerated(parser, tok, s, next, 5130, XML_ACCOUNT_DIRECT) == 0 {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        s = next;
        tok = (*(*parser).m_encoding).scanners[0usize].expect("non-null function pointer")(
            (*parser).m_encoding,
            s,
            end,
            &raw mut next,
        );
    }
    (*parser).m_processor = Some(
        prologProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    return doProlog(
        parser,
        (*parser).m_encoding,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
        XML_TRUE,
        XML_ACCOUNT_DIRECT,
    );
}

unsafe extern "C" fn entityValueProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut start: *const c_char = s;
    let mut next: *const c_char = s;
    let mut enc: *const ENCODING = (*parser).m_encoding;
    let mut tok: c_int = 0;
    loop {
        tok =
            (*enc).scanners[0].expect("non-null function pointer")(enc, start, end, &raw mut next);
        if tok <= 0 {
            if (*parser).m_parsingStatus.finalBuffer == 0 && tok != XML_TOK_INVALID {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            match tok {
                XML_TOK_INVALID => return XML_ERROR_INVALID_TOKEN,
                XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,
                XML_TOK_NONE | _ => {}
            }
            return storeEntityValue(
                parser,
                enc,
                s,
                end,
                XML_ACCOUNT_DIRECT,
                null_mut::<*const c_char>(),
            );
        }
        start = next;
    }
}

unsafe extern "C" fn prologProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = s;
    let mut tok: c_int = (*(*parser).m_encoding).scanners[0].expect("non-null function pointer")(
        (*parser).m_encoding,
        s,
        end,
        &raw mut next,
    );
    return doProlog(
        parser,
        (*parser).m_encoding,
        s,
        end,
        tok,
        next,
        nextPtr,
        ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
        XML_TRUE,
        XML_ACCOUNT_DIRECT,
    );
}

unsafe extern "C" fn doProlog(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut s: *const c_char,
    mut end: *const c_char,
    mut tok: c_int,
    mut next: *const c_char,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
    mut allowClosingDoctype: XML_Bool,
    mut account: XML_Account,
) -> XML_Error {
    let mut current_block: u64;
    static mut externalSubsetName: [XML_Char; 2] = [ASCII_HASH as XML_Char, '\0' as XML_Char];
    static mut atypeCDATA: [XML_Char; 6] = [
        ASCII_C as XML_Char,
        ASCII_D as XML_Char,
        ASCII_A as XML_Char,
        ASCII_T as XML_Char,
        ASCII_A as XML_Char,
        '\0' as XML_Char,
    ];
    static mut atypeID: [XML_Char; 3] =
        [ASCII_I as XML_Char, ASCII_D as XML_Char, '\0' as XML_Char];
    static mut atypeIDREF: [XML_Char; 6] = [
        ASCII_I as XML_Char,
        ASCII_D as XML_Char,
        ASCII_R as XML_Char,
        ASCII_E as XML_Char,
        ASCII_F as XML_Char,
        '\0' as XML_Char,
    ];
    static mut atypeIDREFS: [XML_Char; 7] = [
        ASCII_I as XML_Char,
        ASCII_D as XML_Char,
        ASCII_R as XML_Char,
        ASCII_E as XML_Char,
        ASCII_F as XML_Char,
        ASCII_S as XML_Char,
        '\0' as XML_Char,
    ];
    static mut atypeENTITY: [XML_Char; 7] = [
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        ASCII_T as XML_Char,
        ASCII_I as XML_Char,
        ASCII_T as XML_Char,
        ASCII_Y as XML_Char,
        '\0' as XML_Char,
    ];
    static mut atypeENTITIES: [XML_Char; 9] = [
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        ASCII_T as XML_Char,
        ASCII_I as XML_Char,
        ASCII_T as XML_Char,
        ASCII_I as XML_Char,
        ASCII_E as XML_Char,
        ASCII_S as XML_Char,
        '\0' as XML_Char,
    ];
    static mut atypeNMTOKEN: [XML_Char; 8] = [
        ASCII_N as XML_Char,
        ASCII_M as XML_Char,
        ASCII_T as XML_Char,
        ASCII_O as XML_Char,
        ASCII_K as XML_Char,
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        '\0' as XML_Char,
    ];
    static mut atypeNMTOKENS: [XML_Char; 9] = [
        ASCII_N as XML_Char,
        ASCII_M as XML_Char,
        ASCII_T as XML_Char,
        ASCII_O as XML_Char,
        ASCII_K as XML_Char,
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        ASCII_S as XML_Char,
        '\0' as XML_Char,
    ];
    static mut notationPrefix: [XML_Char; 10] = [
        ASCII_N as XML_Char,
        ASCII_O as XML_Char,
        ASCII_T as XML_Char,
        ASCII_A as XML_Char,
        ASCII_T as XML_Char,
        ASCII_I as XML_Char,
        ASCII_O as XML_Char,
        ASCII_N as XML_Char,
        ASCII_LPAREN as XML_Char,
        '\0' as XML_Char,
    ];
    static mut enumValueSep: [XML_Char; 2] = [ASCII_PIPE as XML_Char, '\0' as XML_Char];
    static mut enumValueStart: [XML_Char; 2] = [ASCII_LPAREN as XML_Char, '\0' as XML_Char];
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut eventPP: *mut *const c_char = null_mut::<*const c_char>();
    let mut eventEndPP: *mut *const c_char = null_mut::<*const c_char>();
    let mut quant: XML_Content_Quant = XML_CQUANT_NONE;
    if enc == (*parser).m_encoding {
        eventPP = &raw mut (*parser).m_eventPtr;
        eventEndPP = &raw mut (*parser).m_eventEndPtr;
    } else {
        eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
    }
    loop {
        let mut role: c_int = 0;
        let mut handleDefault: XML_Bool = XML_TRUE;
        *eventPP = s;
        *eventEndPP = next;
        if tok <= 0 {
            if haveMore as c_int != 0 && tok != XML_TOK_INVALID {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            match tok {
                XML_TOK_INVALID => {
                    *eventPP = next;
                    return XML_ERROR_INVALID_TOKEN;
                }
                XML_TOK_PARTIAL => return XML_ERROR_UNCLOSED_TOKEN,
                XML_TOK_PARTIAL_CHAR => return XML_ERROR_PARTIAL_CHAR,

                -15 => {
                    tok = -tok;
                }
                XML_TOK_NONE => {
                    if enc != (*parser).m_encoding
                        && (*(*parser).m_openInternalEntities).betweenDecl == 0
                    {
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    if (*parser).m_isParamEntity as c_int != 0 || enc != (*parser).m_encoding {
                        if (*parser)
                            .m_prologState
                            .handler
                            .expect("non-null function pointer")(
                            &raw mut (*parser).m_prologState,
                            -(4),
                            end,
                            end,
                            enc,
                        ) == XML_ROLE_ERROR
                        {
                            return XML_ERROR_INCOMPLETE_PE;
                        }
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    return XML_ERROR_NO_ELEMENTS;
                }
                _ => {
                    tok = -tok;
                    next = end;
                }
            }
        }
        role = (*parser)
            .m_prologState
            .handler
            .expect("non-null function pointer")(
            &raw mut (*parser).m_prologState,
            tok,
            s,
            next,
            enc,
        );
        match role {
            2 | 1 | 57 => {}
            _ => {
                if accountingDiffTolerated(parser, tok, s, next, 5301, account) == 0 {
                    accountingOnAbort(parser);
                    return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
                }
            }
        }
        match role {
            1 => {
                let mut result: XML_Error = processXmlDecl(parser, 0, s, next);
                if result != XML_ERROR_NONE {
                    return result;
                }
                enc = (*parser).m_encoding;
                handleDefault = XML_FALSE;
                current_block = 8258632986558375165;
            }
            4 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser).m_doctypeName =
                        poolStoreString(&raw mut (*parser).m_tempPool, enc, s, next);
                    if (*parser).m_doctypeName.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    (*parser).m_doctypePubid = null::<XML_Char>();
                    handleDefault = XML_FALSE;
                }
                (*parser).m_doctypeSysid = null::<XML_Char>();
                current_block = 8258632986558375165;
            }
            7 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser)
                        .m_startDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_doctypeName,
                        (*parser).m_doctypeSysid,
                        (*parser).m_doctypePubid,
                        1,
                    );
                    (*parser).m_doctypeName = null::<XML_Char>();
                    poolClear(&raw mut (*parser).m_tempPool);
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            57 => {
                let mut result_0: XML_Error = processXmlDecl(parser, 1, s, next);
                if result_0 != XML_ERROR_NONE {
                    return result_0;
                }
                enc = (*parser).m_encoding;
                handleDefault = XML_FALSE;
                current_block = 8258632986558375165;
            }
            6 => {
                (*parser).m_useForeignDTD = XML_FALSE;
                (*parser).m_declEntity = lookup(
                    parser,
                    &raw mut (*dtd).paramEntities,
                    &raw const externalSubsetName as KEY,
                    size_of::<ENTITY>(),
                ) as *mut ENTITY;
                if (*parser).m_declEntity.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                (*dtd).hasParamEntityRefs = XML_TRUE;
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    let mut pubId: *mut XML_Char = null_mut::<XML_Char>();
                    if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP)
                        == 0
                    {
                        return XML_ERROR_PUBLICID;
                    }
                    pubId = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if pubId.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    normalizePublicId(pubId);
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    (*parser).m_doctypePubid = pubId;
                    handleDefault = XML_FALSE;
                    current_block = 13941306361429013238;
                } else {
                    current_block = 6873921596653269498;
                }
            }
            14 => {
                current_block = 6873921596653269498;
            }
            8 => {
                if allowClosingDoctype as c_int != XML_TRUE as c_int {
                    return XML_ERROR_INVALID_TOKEN;
                }
                if !(*parser).m_doctypeName.is_null() {
                    (*parser)
                        .m_startDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_doctypeName,
                        (*parser).m_doctypeSysid,
                        (*parser).m_doctypePubid,
                        0,
                    );
                    poolClear(&raw mut (*parser).m_tempPool);
                    handleDefault = XML_FALSE;
                }
                if !(*parser).m_doctypeSysid.is_null() || (*parser).m_useForeignDTD as c_int != 0 {
                    let mut hadParamEntityRefs: XML_Bool = (*dtd).hasParamEntityRefs;
                    (*dtd).hasParamEntityRefs = XML_TRUE;
                    if (*parser).m_paramEntityParsing != 0
                        && (*parser).m_externalEntityRefHandler.is_some()
                    {
                        let mut entity: *mut ENTITY = lookup(
                            parser,
                            &raw mut (*dtd).paramEntities,
                            &raw const externalSubsetName as KEY,
                            size_of::<ENTITY>(),
                        ) as *mut ENTITY;
                        if entity.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        if (*parser).m_useForeignDTD != 0 {
                            (*entity).base = (*parser).m_curBase;
                        }
                        (*dtd).paramEntityRead = XML_FALSE;
                        if (*parser)
                            .m_externalEntityRefHandler
                            .expect("non-null function pointer")(
                            (*parser).m_externalEntityRefHandlerArg,
                            null::<XML_Char>(),
                            (*entity).base,
                            (*entity).systemId,
                            (*entity).publicId,
                        ) == 0
                        {
                            return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                        }
                        if (*dtd).paramEntityRead != 0 {
                            if (*dtd).standalone == 0
                                && (*parser).m_notStandaloneHandler.is_some()
                                && (*parser)
                                    .m_notStandaloneHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_handlerArg
                                ) == 0
                            {
                                return XML_ERROR_NOT_STANDALONE;
                            }
                        } else if (*parser).m_doctypeSysid.is_null() {
                            (*dtd).hasParamEntityRefs = hadParamEntityRefs;
                        }
                    }
                    (*parser).m_useForeignDTD = XML_FALSE;
                }
                if (*parser).m_endDoctypeDeclHandler.is_some() {
                    (*parser)
                        .m_endDoctypeDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    );
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            2 => {
                if (*parser).m_useForeignDTD != 0 {
                    let mut hadParamEntityRefs_0: XML_Bool = (*dtd).hasParamEntityRefs;
                    (*dtd).hasParamEntityRefs = XML_TRUE;
                    if (*parser).m_paramEntityParsing != 0
                        && (*parser).m_externalEntityRefHandler.is_some()
                    {
                        let mut entity_0: *mut ENTITY = lookup(
                            parser,
                            &raw mut (*dtd).paramEntities,
                            &raw const externalSubsetName as KEY,
                            size_of::<ENTITY>(),
                        ) as *mut ENTITY;
                        if entity_0.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*entity_0).base = (*parser).m_curBase;
                        (*dtd).paramEntityRead = XML_FALSE;
                        if (*parser)
                            .m_externalEntityRefHandler
                            .expect("non-null function pointer")(
                            (*parser).m_externalEntityRefHandlerArg,
                            null::<XML_Char>(),
                            (*entity_0).base,
                            (*entity_0).systemId,
                            (*entity_0).publicId,
                        ) == 0
                        {
                            return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                        }
                        if (*dtd).paramEntityRead != 0 {
                            if (*dtd).standalone == 0
                                && (*parser).m_notStandaloneHandler.is_some()
                                && (*parser)
                                    .m_notStandaloneHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_handlerArg
                                ) == 0
                            {
                                return XML_ERROR_NOT_STANDALONE;
                            }
                        } else {
                            (*dtd).hasParamEntityRefs = hadParamEntityRefs_0;
                        }
                    }
                }
                (*parser).m_processor = Some(
                    contentProcessor
                        as unsafe extern "C" fn(
                            XML_Parser,
                            *const c_char,
                            *const c_char,
                            *mut *const c_char,
                        ) -> XML_Error,
                );
                return contentProcessor(parser, s, end, nextPtr);
            }
            34 => {
                (*parser).m_declElementType = getElementType(parser, enc, s, next);
                if (*parser).m_declElementType.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                current_block = 14779040008015901923;
            }
            22 => {
                (*parser).m_declAttributeId = getAttributeId(parser, enc, s, next);
                if (*parser).m_declAttributeId.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                (*parser).m_declAttributeIsCdata = XML_FALSE;
                (*parser).m_declAttributeType = null::<XML_Char>();
                (*parser).m_declAttributeIsId = XML_FALSE;
                current_block = 14779040008015901923;
            }
            23 => {
                (*parser).m_declAttributeIsCdata = XML_TRUE;
                (*parser).m_declAttributeType = &raw const atypeCDATA as *const XML_Char;
                current_block = 14779040008015901923;
            }
            24 => {
                (*parser).m_declAttributeIsId = XML_TRUE;
                (*parser).m_declAttributeType = &raw const atypeID as *const XML_Char;
                current_block = 14779040008015901923;
            }
            25 => {
                (*parser).m_declAttributeType = &raw const atypeIDREF as *const XML_Char;
                current_block = 14779040008015901923;
            }
            26 => {
                (*parser).m_declAttributeType = &raw const atypeIDREFS as *const XML_Char;
                current_block = 14779040008015901923;
            }
            27 => {
                (*parser).m_declAttributeType = &raw const atypeENTITY as *const XML_Char;
                current_block = 14779040008015901923;
            }
            28 => {
                (*parser).m_declAttributeType = &raw const atypeENTITIES as *const XML_Char;
                current_block = 14779040008015901923;
            }
            29 => {
                (*parser).m_declAttributeType = &raw const atypeNMTOKEN as *const XML_Char;
                current_block = 14779040008015901923;
            }
            30 => {
                (*parser).m_declAttributeType = &raw const atypeNMTOKENS as *const XML_Char;
                current_block = 14779040008015901923;
            }
            31 | 32 => {
                if (*dtd).keepProcessing as c_int != 0 && (*parser).m_attlistDeclHandler.is_some() {
                    let mut prefix: *const XML_Char = null::<XML_Char>();
                    if !(*parser).m_declAttributeType.is_null() {
                        prefix = &raw const enumValueSep as *const XML_Char;
                    } else {
                        prefix = if role == XML_ROLE_ATTRIBUTE_NOTATION_VALUE {
                            &raw const notationPrefix as *const XML_Char
                        } else {
                            &raw const enumValueStart as *const XML_Char
                        };
                    }
                    if poolAppendString(&raw mut (*parser).m_tempPool, prefix).is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if poolAppend(&raw mut (*parser).m_tempPool, enc, s, next).is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            35 | 36 => {
                if (*dtd).keepProcessing != 0 {
                    if defineAttribute(
                        (*parser).m_declElementType,
                        (*parser).m_declAttributeId,
                        (*parser).m_declAttributeIsCdata,
                        (*parser).m_declAttributeIsId,
                        null::<XML_Char>(),
                        parser,
                    ) == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if (*parser).m_attlistDeclHandler.is_some()
                        && !(*parser).m_declAttributeType.is_null()
                    {
                        if *(*parser).m_declAttributeType as c_int == 0x28
                            || *(*parser).m_declAttributeType as c_int == 0x4e
                                && *(*parser).m_declAttributeType.offset(1) as c_int == 0x4f
                        {
                            if (if (*parser).m_tempPool.ptr
                                == (*parser).m_tempPool.end as *mut XML_Char
                                && poolGrow(&raw mut (*parser).m_tempPool) == 0
                            {
                                0
                            } else {
                                let fresh1 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *fresh1 = 0x29i8;
                                1
                            }) == 0
                                || (if (*parser).m_tempPool.ptr
                                    == (*parser).m_tempPool.end as *mut XML_Char
                                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                                {
                                    0
                                } else {
                                    let fresh2 = (*parser).m_tempPool.ptr;
                                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                    *fresh2 = '\0' as XML_Char;
                                    1
                                }) == 0
                            {
                                return XML_ERROR_NO_MEMORY;
                            }
                            (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                        }
                        *eventEndPP = s;
                        (*parser)
                            .m_attlistDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            (*(*parser).m_declAttributeId).name,
                            (*parser).m_declAttributeType,
                            null::<XML_Char>(),
                            (role == XML_ROLE_REQUIRED_ATTRIBUTE_VALUE) as c_int,
                        );
                        handleDefault = XML_FALSE;
                    }
                }
                poolClear(&raw mut (*parser).m_tempPool);
                current_block = 8258632986558375165;
            }
            37 | 38 => {
                if (*dtd).keepProcessing != 0 {
                    let mut attVal: *const XML_Char = null::<XML_Char>();
                    let mut result_1: XML_Error = storeAttributeValue(
                        parser,
                        enc,
                        (*parser).m_declAttributeIsCdata,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                        &raw mut (*dtd).pool,
                        XML_ACCOUNT_NONE,
                    );
                    if result_1 as u64 != 0 {
                        return result_1;
                    }
                    attVal = (*dtd).pool.start;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if defineAttribute(
                        (*parser).m_declElementType,
                        (*parser).m_declAttributeId,
                        (*parser).m_declAttributeIsCdata,
                        XML_FALSE,
                        attVal,
                        parser,
                    ) == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if (*parser).m_attlistDeclHandler.is_some()
                        && !(*parser).m_declAttributeType.is_null()
                    {
                        if *(*parser).m_declAttributeType as c_int == 0x28
                            || *(*parser).m_declAttributeType as c_int == 0x4e
                                && *(*parser).m_declAttributeType.offset(1) as c_int == 0x4f
                        {
                            if (if (*parser).m_tempPool.ptr
                                == (*parser).m_tempPool.end as *mut XML_Char
                                && poolGrow(&raw mut (*parser).m_tempPool) == 0
                            {
                                0
                            } else {
                                let fresh3 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *fresh3 = 0x29i8;
                                1
                            }) == 0
                                || (if (*parser).m_tempPool.ptr
                                    == (*parser).m_tempPool.end as *mut XML_Char
                                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                                {
                                    0
                                } else {
                                    let fresh4 = (*parser).m_tempPool.ptr;
                                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                    *fresh4 = '\0' as XML_Char;
                                    1
                                }) == 0
                            {
                                return XML_ERROR_NO_MEMORY;
                            }
                            (*parser).m_declAttributeType = (*parser).m_tempPool.start;
                            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                        }
                        *eventEndPP = s;
                        (*parser)
                            .m_attlistDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            (*(*parser).m_declAttributeId).name,
                            (*parser).m_declAttributeType,
                            attVal,
                            (role == XML_ROLE_FIXED_ATTRIBUTE_VALUE) as c_int,
                        );
                        poolClear(&raw mut (*parser).m_tempPool);
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            12 => {
                if (*dtd).keepProcessing != 0 {
                    let mut result_2: XML_Error = callStoreEntityValue(
                        parser,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                        XML_ACCOUNT_NONE,
                    );
                    if !(*parser).m_declEntity.is_null() {
                        (*(*parser).m_declEntity).textPtr = (*dtd).entityValuePool.start;
                        (*(*parser).m_declEntity).textLen = (*dtd)
                            .entityValuePool
                            .ptr
                            .offset_from((*dtd).entityValuePool.start)
                            as c_int;
                        (*dtd).entityValuePool.start = (*dtd).entityValuePool.ptr;
                        if (*parser).m_entityDeclHandler.is_some() {
                            *eventEndPP = s;
                            (*parser)
                                .m_entityDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*parser).m_declEntity).name,
                                (*(*parser).m_declEntity).is_param as c_int,
                                (*(*parser).m_declEntity).textPtr,
                                (*(*parser).m_declEntity).textLen,
                                (*parser).m_curBase,
                                null::<XML_Char>(),
                                null::<XML_Char>(),
                                null::<XML_Char>(),
                            );
                            handleDefault = XML_FALSE;
                        }
                    } else {
                        (*dtd).entityValuePool.ptr = (*dtd).entityValuePool.start;
                    }
                    if result_2 != XML_ERROR_NONE {
                        return result_2;
                    }
                }
                current_block = 8258632986558375165;
            }
            5 => {
                (*parser).m_useForeignDTD = XML_FALSE;
                (*dtd).hasParamEntityRefs = XML_TRUE;
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    (*parser).m_doctypeSysid = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if (*parser).m_doctypeSysid.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = XML_FALSE;
                } else {
                    (*parser).m_doctypeSysid = &raw const externalSubsetName as *const XML_Char;
                }
                if (*dtd).standalone == 0
                    && (*parser).m_paramEntityParsing as u64 == 0
                    && (*parser).m_notStandaloneHandler.is_some()
                    && (*parser)
                        .m_notStandaloneHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg
                    ) == 0
                {
                    return XML_ERROR_NOT_STANDALONE;
                }
                if (*parser).m_declEntity.is_null() {
                    (*parser).m_declEntity = lookup(
                        parser,
                        &raw mut (*dtd).paramEntities,
                        &raw const externalSubsetName as KEY,
                        size_of::<ENTITY>(),
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*(*parser).m_declEntity).publicId = null::<XML_Char>();
                }
                current_block = 14343490084333691418;
            }
            13 => {
                current_block = 14343490084333691418;
            }
            15 => {
                if (*dtd).keepProcessing as c_int != 0
                    && !(*parser).m_declEntity.is_null()
                    && (*parser).m_entityDeclHandler.is_some()
                {
                    *eventEndPP = s;
                    (*parser)
                        .m_entityDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*(*parser).m_declEntity).name,
                        (*(*parser).m_declEntity).is_param as c_int,
                        null::<XML_Char>(),
                        0,
                        (*(*parser).m_declEntity).base,
                        (*(*parser).m_declEntity).systemId,
                        (*(*parser).m_declEntity).publicId,
                        null::<XML_Char>(),
                    );
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            16 => {
                if (*dtd).keepProcessing as c_int != 0 && !(*parser).m_declEntity.is_null() {
                    (*(*parser).m_declEntity).notation =
                        poolStoreString(&raw mut (*dtd).pool, enc, s, next);
                    if (*(*parser).m_declEntity).notation.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if (*parser).m_unparsedEntityDeclHandler.is_some() {
                        *eventEndPP = s;
                        (*parser)
                            .m_unparsedEntityDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declEntity).name,
                            (*(*parser).m_declEntity).base,
                            (*(*parser).m_declEntity).systemId,
                            (*(*parser).m_declEntity).publicId,
                            (*(*parser).m_declEntity).notation,
                        );
                        handleDefault = XML_FALSE;
                    } else if (*parser).m_entityDeclHandler.is_some() {
                        *eventEndPP = s;
                        (*parser)
                            .m_entityDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declEntity).name,
                            0,
                            null::<XML_Char>(),
                            0,
                            (*(*parser).m_declEntity).base,
                            (*(*parser).m_declEntity).systemId,
                            (*(*parser).m_declEntity).publicId,
                            (*(*parser).m_declEntity).notation,
                        );
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            9 => {
                if (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(enc, s, next)
                    != 0
                {
                    (*parser).m_declEntity = null_mut::<ENTITY>();
                } else if (*dtd).keepProcessing != 0 {
                    let mut name: *const XML_Char =
                        poolStoreString(&raw mut (*dtd).pool, enc, s, next);
                    if name.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declEntity = lookup(
                        parser,
                        &raw mut (*dtd).generalEntities,
                        name,
                        size_of::<ENTITY>(),
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if (*(*parser).m_declEntity).name != name {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        (*parser).m_declEntity = null_mut::<ENTITY>();
                    } else {
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        (*(*parser).m_declEntity).publicId = null::<XML_Char>();
                        (*(*parser).m_declEntity).is_param = XML_FALSE;
                        (*(*parser).m_declEntity).is_internal =
                            !(!(*parser).m_parentParser.is_null()
                                || !(*parser).m_openInternalEntities.is_null())
                                as XML_Bool;
                        if (*parser).m_entityDeclHandler.is_some() {
                            handleDefault = XML_FALSE;
                        }
                    }
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    (*parser).m_declEntity = null_mut::<ENTITY>();
                }
                current_block = 8258632986558375165;
            }
            10 => {
                if (*dtd).keepProcessing != 0 {
                    let mut name_0: *const XML_Char =
                        poolStoreString(&raw mut (*dtd).pool, enc, s, next);
                    if name_0.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_declEntity = lookup(
                        parser,
                        &raw mut (*dtd).paramEntities,
                        name_0,
                        size_of::<ENTITY>(),
                    ) as *mut ENTITY;
                    if (*parser).m_declEntity.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    if (*(*parser).m_declEntity).name != name_0 {
                        (*dtd).pool.ptr = (*dtd).pool.start;
                        (*parser).m_declEntity = null_mut::<ENTITY>();
                    } else {
                        (*dtd).pool.start = (*dtd).pool.ptr;
                        (*(*parser).m_declEntity).publicId = null::<XML_Char>();
                        (*(*parser).m_declEntity).is_param = XML_TRUE;
                        (*(*parser).m_declEntity).is_internal =
                            !(!(*parser).m_parentParser.is_null()
                                || !(*parser).m_openInternalEntities.is_null())
                                as XML_Bool;
                        if (*parser).m_entityDeclHandler.is_some() {
                            handleDefault = XML_FALSE;
                        }
                    }
                } else {
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    (*parser).m_declEntity = null_mut::<ENTITY>();
                }
                current_block = 8258632986558375165;
            }
            18 => {
                (*parser).m_declNotationPublicId = null::<XML_Char>();
                (*parser).m_declNotationName = null::<XML_Char>();
                if (*parser).m_notationDeclHandler.is_some() {
                    (*parser).m_declNotationName =
                        poolStoreString(&raw mut (*parser).m_tempPool, enc, s, next);
                    if (*parser).m_declNotationName.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            21 => {
                if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP) == 0
                {
                    return XML_ERROR_PUBLICID;
                }
                if !(*parser).m_declNotationName.is_null() {
                    let mut tem_0: *mut XML_Char = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if tem_0.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    normalizePublicId(tem_0);
                    (*parser).m_declNotationPublicId = tem_0;
                    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            19 => {
                if !(*parser).m_declNotationName.is_null()
                    && (*parser).m_notationDeclHandler.is_some()
                {
                    let mut systemId: *const XML_Char = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if systemId.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    *eventEndPP = s;
                    (*parser)
                        .m_notationDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_declNotationName,
                        (*parser).m_curBase,
                        systemId,
                        (*parser).m_declNotationPublicId,
                    );
                    handleDefault = XML_FALSE;
                }
                poolClear(&raw mut (*parser).m_tempPool);
                current_block = 8258632986558375165;
            }
            20 => {
                if !(*parser).m_declNotationPublicId.is_null()
                    && (*parser).m_notationDeclHandler.is_some()
                {
                    *eventEndPP = s;
                    (*parser)
                        .m_notationDeclHandler
                        .expect("non-null function pointer")(
                        (*parser).m_handlerArg,
                        (*parser).m_declNotationName,
                        (*parser).m_curBase,
                        null::<XML_Char>(),
                        (*parser).m_declNotationPublicId,
                    );
                    handleDefault = XML_FALSE;
                }
                poolClear(&raw mut (*parser).m_tempPool);
                current_block = 8258632986558375165;
            }
            -1 => match tok {
                XML_TOK_PARAM_ENTITY_REF => return XML_ERROR_PARAM_ENTITY_REF,
                XML_TOK_XML_DECL => return XML_ERROR_MISPLACED_XML_PI,
                _ => return XML_ERROR_SYNTAX,
            },
            58 => {
                let mut result_3: XML_Error = XML_ERROR_NONE;
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                handleDefault = XML_FALSE;
                result_3 = doIgnoreSection(parser, enc, &raw mut next, end, nextPtr, haveMore);
                if result_3 != XML_ERROR_NONE {
                    return result_3;
                } else if next.is_null() {
                    (*parser).m_processor = Some(
                        ignoreSectionProcessor
                            as unsafe extern "C" fn(
                                XML_Parser,
                                *const c_char,
                                *const c_char,
                                *mut *const c_char,
                            ) -> XML_Error,
                    );
                    return result_3;
                }
                current_block = 8258632986558375165;
            }
            44 => {
                if (*parser).m_prologState.level >= (*parser).m_groupSize {
                    if (*parser).m_groupSize != 0 {
                        if (*parser).m_groupSize > (-(1i32) as c_uint).wrapping_div(2u32) {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*parser).m_groupSize = (*parser).m_groupSize.wrapping_mul(2u32);
                        let new_connector: *mut c_char = expat_realloc(
                            parser,
                            (*parser).m_groupConnector as *mut c_void,
                            (*parser).m_groupSize as size_t,
                            5915,
                        ) as *mut c_char;
                        if new_connector.is_null() {
                            (*parser).m_groupSize = (*parser).m_groupSize.wrapping_div(2u32);
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*parser).m_groupConnector = new_connector;
                        if !(*dtd).scaffIndex.is_null() {
                            let new_scaff_index: *mut c_int = expat_realloc(
                                parser,
                                (*dtd).scaffIndex as *mut c_void,
                                ((*parser).m_groupSize as size_t).wrapping_mul(size_of::<c_int>()),
                                5936,
                            )
                                as *mut c_int;
                            if new_scaff_index.is_null() {
                                (*parser).m_groupSize = (*parser).m_groupSize.wrapping_div(2u32);
                                return XML_ERROR_NO_MEMORY;
                            }
                            (*dtd).scaffIndex = new_scaff_index;
                        }
                    } else {
                        (*parser).m_groupSize = 32u32;
                        (*parser).m_groupConnector =
                            expat_malloc(parser, (*parser).m_groupSize as size_t, 5944)
                                as *mut c_char;
                        if (*parser).m_groupConnector.is_null() {
                            (*parser).m_groupSize = 0u32;
                            return XML_ERROR_NO_MEMORY;
                        }
                    }
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) = 0i8;
                if (*dtd).in_eldecl != 0 {
                    let mut myindex: c_int = nextScaffoldPart(parser);
                    if myindex < 0 {
                        return XML_ERROR_NO_MEMORY;
                    }
                    assert!(!(*dtd).scaffIndex.is_null(), "dtd->scaffIndex != NULL");
                    *(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) = myindex;
                    (*dtd).scaffLevel += 1;
                    (*(*dtd).scaffold.offset(myindex as isize)).type_0 = XML_CTYPE_SEQ;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            50 => {
                if *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) as c_int
                    == ASCII_PIPE
                {
                    return XML_ERROR_SYNTAX;
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) = ASCII_COMMA as c_char;
                if (*dtd).in_eldecl as c_int != 0 && (*parser).m_elementDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            49 => {
                if *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) as c_int
                    == ASCII_COMMA
                {
                    return XML_ERROR_SYNTAX;
                }
                if (*dtd).in_eldecl as c_int != 0
                    && *(*parser)
                        .m_groupConnector
                        .offset((*parser).m_prologState.level as isize)
                        == 0
                    && (*(*dtd).scaffold.offset(
                        *(*dtd).scaffIndex.offset(((*dtd).scaffLevel - 1) as isize) as isize,
                    ))
                    .type_0
                        != XML_CTYPE_MIXED
                {
                    (*(*dtd).scaffold.offset(
                        *(*dtd).scaffIndex.offset(((*dtd).scaffLevel - 1) as isize) as isize,
                    ))
                    .type_0 = XML_CTYPE_CHOICE;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                }
                *(*parser)
                    .m_groupConnector
                    .offset((*parser).m_prologState.level as isize) = ASCII_PIPE as c_char;
                current_block = 8258632986558375165;
            }
            60 | 59 => {
                (*dtd).hasParamEntityRefs = XML_TRUE;
                if (*parser).m_paramEntityParsing as u64 == 0 {
                    (*dtd).keepProcessing = (*dtd).standalone;
                    current_block = 16953886395775657100;
                } else {
                    let mut name_1: *const XML_Char = null::<XML_Char>();
                    let mut entity_1: *mut ENTITY = null_mut::<ENTITY>();
                    name_1 = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name_1.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    entity_1 =
                        lookup(parser, &raw mut (*dtd).paramEntities, name_1, 0) as *mut ENTITY;
                    (*dtd).pool.ptr = (*dtd).pool.start;
                    if (*parser).m_prologState.documentEntity != 0
                        && (if (*dtd).standalone as c_int != 0 {
                            (*parser).m_openInternalEntities.is_null() as c_int
                        } else {
                            ((*dtd).hasParamEntityRefs == 0) as c_int
                        }) != 0
                    {
                        if entity_1.is_null() {
                            return XML_ERROR_UNDEFINED_ENTITY;
                        } else if (*entity_1).is_internal == 0 {
                            return XML_ERROR_ENTITY_DECLARED_IN_PE;
                        }
                        current_block = 11938645146649090955;
                    } else if entity_1.is_null() {
                        (*dtd).keepProcessing = (*dtd).standalone;
                        if role == XML_ROLE_PARAM_ENTITY_REF
                            && (*parser).m_skippedEntityHandler.is_some()
                        {
                            (*parser)
                                .m_skippedEntityHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                name_1,
                                1,
                            );
                            handleDefault = XML_FALSE;
                        }
                        current_block = 8258632986558375165;
                    } else {
                        current_block = 11938645146649090955;
                    }
                    match current_block {
                        8258632986558375165 => {}
                        _ => {
                            if (*entity_1).open != 0 {
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity_1).textPtr.is_null() {
                                let mut result_4: XML_Error = XML_ERROR_NONE;
                                let mut betweenDecl: XML_Bool =
                                    (if role == XML_ROLE_PARAM_ENTITY_REF {
                                        XML_TRUE as c_int
                                    } else {
                                        XML_FALSE as c_int
                                    }) as XML_Bool;
                                result_4 =
                                    processEntity(parser, entity_1, betweenDecl, ENTITY_INTERNAL);
                                if result_4 != XML_ERROR_NONE {
                                    return result_4;
                                }
                                handleDefault = XML_FALSE;
                                current_block = 8258632986558375165;
                            } else if (*parser).m_externalEntityRefHandler.is_some() {
                                (*dtd).paramEntityRead = XML_FALSE;
                                (*entity_1).open = XML_TRUE;
                                entityTrackingOnOpen(parser, entity_1, 6057);
                                if (*parser)
                                    .m_externalEntityRefHandler
                                    .expect("non-null function pointer")(
                                    (*parser).m_externalEntityRefHandlerArg,
                                    null::<XML_Char>(),
                                    (*entity_1).base,
                                    (*entity_1).systemId,
                                    (*entity_1).publicId,
                                ) == 0
                                {
                                    entityTrackingOnClose(parser, entity_1, 6061);
                                    (*entity_1).open = XML_FALSE;
                                    return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                }
                                entityTrackingOnClose(parser, entity_1, 6065);
                                (*entity_1).open = XML_FALSE;
                                handleDefault = XML_FALSE;
                                if (*dtd).paramEntityRead == 0 {
                                    (*dtd).keepProcessing = (*dtd).standalone;
                                    current_block = 8258632986558375165;
                                } else {
                                    current_block = 16953886395775657100;
                                }
                            } else {
                                (*dtd).keepProcessing = (*dtd).standalone;
                                current_block = 8258632986558375165;
                            }
                        }
                    }
                }
                match current_block {
                    8258632986558375165 => {}
                    _ => {
                        if (*dtd).standalone == 0
                            && (*parser).m_notStandaloneHandler.is_some()
                            && (*parser)
                                .m_notStandaloneHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg
                            ) == 0
                        {
                            return XML_ERROR_NOT_STANDALONE;
                        }
                        current_block = 8258632986558375165;
                    }
                }
            }
            40 => {
                if (*parser).m_elementDeclHandler.is_some() {
                    (*parser).m_declElementType = getElementType(parser, enc, s, next);
                    if (*parser).m_declElementType.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).scaffLevel = 0;
                    (*dtd).scaffCount = 0;
                    (*dtd).in_eldecl = XML_TRUE;
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            41 | 42 => {
                if (*dtd).in_eldecl != 0 {
                    if (*parser).m_elementDeclHandler.is_some() {
                        let mut content: *mut XML_Content = (*parser)
                            .m_mem
                            .malloc_fcn
                            .expect("non-null function pointer")(
                            size_of::<XML_Content>()
                        )
                            as *mut XML_Content;
                        if content.is_null() {
                            return XML_ERROR_NO_MEMORY;
                        }
                        (*content).quant = XML_CQUANT_NONE;
                        (*content).name = null_mut::<XML_Char>();
                        (*content).numchildren = 0;
                        (*content).children = null_mut::<XML_Content>();
                        (*content).type_0 = (if role == XML_ROLE_CONTENT_ANY {
                            XML_CTYPE_ANY as c_int
                        } else {
                            XML_CTYPE_EMPTY as c_int
                        }) as XML_Content_Type;
                        *eventEndPP = s;
                        (*parser)
                            .m_elementDeclHandler
                            .expect("non-null function pointer")(
                            (*parser).m_handlerArg,
                            (*(*parser).m_declElementType).name,
                            content,
                        );
                        handleDefault = XML_FALSE;
                    }
                    (*dtd).in_eldecl = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            43 => {
                if (*dtd).in_eldecl != 0 {
                    (*(*dtd).scaffold.offset(
                        *(*dtd).scaffIndex.offset(((*dtd).scaffLevel - 1) as isize) as isize,
                    ))
                    .type_0 = XML_CTYPE_MIXED;
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            51 => {
                quant = XML_CQUANT_NONE;
                current_block = 403054792318898984;
            }
            53 => {
                quant = XML_CQUANT_OPT;
                current_block = 403054792318898984;
            }
            52 => {
                quant = XML_CQUANT_REP;
                current_block = 403054792318898984;
            }
            54 => {
                quant = XML_CQUANT_PLUS;
                current_block = 403054792318898984;
            }
            45 => {
                quant = XML_CQUANT_NONE;
                current_block = 16394788973656955466;
            }
            47 => {
                quant = XML_CQUANT_OPT;
                current_block = 16394788973656955466;
            }
            46 => {
                quant = XML_CQUANT_REP;
                current_block = 16394788973656955466;
            }
            48 => {
                quant = XML_CQUANT_PLUS;
                current_block = 16394788973656955466;
            }
            55 => {
                if reportProcessingInstruction(parser, enc, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
                handleDefault = XML_FALSE;
                current_block = 8258632986558375165;
            }
            56 => {
                if reportComment(parser, enc, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
                handleDefault = XML_FALSE;
                current_block = 8258632986558375165;
            }
            0 => {
                match tok {
                    XML_TOK_BOM => {
                        handleDefault = XML_FALSE;
                    }
                    _ => {}
                }
                current_block = 8258632986558375165;
            }
            3 => {
                if (*parser).m_startDoctypeDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            11 => {
                if (*dtd).keepProcessing as c_int != 0 && (*parser).m_entityDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            17 => {
                if (*parser).m_notationDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            33 => {
                if (*dtd).keepProcessing as c_int != 0 && (*parser).m_attlistDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            39 => {
                if (*parser).m_elementDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            _ => {
                current_block = 8258632986558375165;
            }
        }
        match current_block {
            6873921596653269498 => {
                if (*enc).isPublicId.expect("non-null function pointer")(enc, s, next, eventPP) == 0
                {
                    return XML_ERROR_PUBLICID;
                }
                current_block = 13941306361429013238;
            }
            14343490084333691418 => {
                if (*dtd).keepProcessing as c_int != 0 && !(*parser).m_declEntity.is_null() {
                    (*(*parser).m_declEntity).systemId = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if (*(*parser).m_declEntity).systemId.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*(*parser).m_declEntity).base = (*parser).m_curBase;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if (*parser).m_entityDeclHandler.is_some() && role == XML_ROLE_ENTITY_SYSTEM_ID
                    {
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            14779040008015901923 => {
                if (*dtd).keepProcessing as c_int != 0 && (*parser).m_attlistDeclHandler.is_some() {
                    handleDefault = XML_FALSE;
                }
                current_block = 8258632986558375165;
            }
            403054792318898984 => {
                if (*dtd).in_eldecl != 0 {
                    let mut el: *mut ELEMENT_TYPE = null_mut::<ELEMENT_TYPE>();
                    let mut name_2: *const XML_Char = null::<XML_Char>();
                    let mut nameLen: size_t = 0;
                    let mut nxt: *const c_char = if quant == XML_CQUANT_NONE {
                        next
                    } else {
                        next.offset(-((*enc).minBytesPerChar as isize))
                    };
                    let mut myindex_0: c_int = nextScaffoldPart(parser);
                    if myindex_0 < 0 {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*(*dtd).scaffold.offset(myindex_0 as isize)).type_0 = XML_CTYPE_NAME;
                    (*(*dtd).scaffold.offset(myindex_0 as isize)).quant = quant;
                    el = getElementType(parser, enc, s, nxt);
                    if el.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    name_2 = (*el).name;
                    let ref mut fresh5 = (*(*dtd).scaffold.offset(myindex_0 as isize)).name;
                    *fresh5 = name_2;
                    nameLen = 0;
                    loop {
                        let fresh6 = nameLen;
                        nameLen = nameLen.wrapping_add(1);
                        if !(*name_2.offset(fresh6 as isize) != 0) {
                            break;
                        }
                    }
                    if nameLen > UINT_MAX.wrapping_sub((*dtd).contentStringLen) as size_t {
                        return XML_ERROR_NO_MEMORY;
                    }
                    (*dtd).contentStringLen =
                        (*dtd).contentStringLen.wrapping_add(nameLen as c_uint);
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                }
                current_block = 8258632986558375165;
            }
            16394788973656955466 => {
                if (*dtd).in_eldecl != 0 {
                    if (*parser).m_elementDeclHandler.is_some() {
                        handleDefault = XML_FALSE;
                    }
                    (*dtd).scaffLevel -= 1;
                    (*(*dtd)
                        .scaffold
                        .offset(*(*dtd).scaffIndex.offset((*dtd).scaffLevel as isize) as isize))
                    .quant = quant;
                    if (*dtd).scaffLevel == 0 {
                        if handleDefault == 0 {
                            let mut model: *mut XML_Content = build_model(parser);
                            if model.is_null() {
                                return XML_ERROR_NO_MEMORY;
                            }
                            *eventEndPP = s;
                            (*parser)
                                .m_elementDeclHandler
                                .expect("non-null function pointer")(
                                (*parser).m_handlerArg,
                                (*(*parser).m_declElementType).name,
                                model,
                            );
                        }
                        (*dtd).in_eldecl = XML_FALSE;
                        (*dtd).contentStringLen = 0u32;
                    }
                }
                current_block = 8258632986558375165;
            }
            _ => {}
        }
        match current_block {
            13941306361429013238 => {
                if (*dtd).keepProcessing as c_int != 0 && !(*parser).m_declEntity.is_null() {
                    let mut tem: *mut XML_Char = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if tem.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    normalizePublicId(tem);
                    (*(*parser).m_declEntity).publicId = tem;
                    (*dtd).pool.start = (*dtd).pool.ptr;
                    if (*parser).m_entityDeclHandler.is_some() && role == XML_ROLE_ENTITY_PUBLIC_ID
                    {
                        handleDefault = XML_FALSE;
                    }
                }
            }
            _ => {}
        }
        if handleDefault as c_int != 0 && (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, s, next);
        }
        match (*parser).m_parsingStatus.parsing {
            3 => {
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => return XML_ERROR_ABORTED,
            1 => {
                if (*parser).m_reenter != 0 {
                    *nextPtr = next;
                    return XML_ERROR_NONE;
                }
            }
            _ => {}
        }
        s = next;
        tok = (*enc).scanners[0].expect("non-null function pointer")(enc, s, end, &raw mut next);
    }
}

unsafe extern "C" fn epilogProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    (*parser).m_processor = Some(
        epilogProcessor
            as unsafe extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    (*parser).m_eventPtr = s;
    loop {
        let mut next: *const c_char = null::<c_char>();
        let mut tok: c_int = (*(*parser).m_encoding).scanners[0]
            .expect("non-null function pointer")(
            (*parser).m_encoding, s, end, &raw mut next
        );
        if accountingDiffTolerated(parser, tok, s, next, 6279, XML_ACCOUNT_DIRECT) == 0 {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        (*parser).m_eventEndPtr = next;
        match tok {
            -15 => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, (*parser).m_encoding, s, next);
                    if (*parser).m_parsingStatus.parsing == XML_FINISHED {
                        return XML_ERROR_ABORTED;
                    }
                }
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            XML_TOK_NONE => {
                *nextPtr = s;
                return XML_ERROR_NONE;
            }
            XML_TOK_PROLOG_S => {
                if (*parser).m_defaultHandler.is_some() {
                    reportDefault(parser, (*parser).m_encoding, s, next);
                }
            }
            XML_TOK_PI => {
                if reportProcessingInstruction(parser, (*parser).m_encoding, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            XML_TOK_COMMENT => {
                if reportComment(parser, (*parser).m_encoding, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            XML_TOK_INVALID => {
                (*parser).m_eventPtr = next;
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_TOKEN;
            }
            XML_TOK_PARTIAL_CHAR => {
                if (*parser).m_parsingStatus.finalBuffer == 0 {
                    *nextPtr = s;
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            _ => return XML_ERROR_JUNK_AFTER_DOC_ELEMENT,
        }
        match (*parser).m_parsingStatus.parsing {
            3 => {
                (*parser).m_eventPtr = next;
                *nextPtr = next;
                return XML_ERROR_NONE;
            }
            2 => {
                (*parser).m_eventPtr = next;
                return XML_ERROR_ABORTED;
            }
            1 => {
                if (*parser).m_reenter != 0 {
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }
            _ => {}
        }
        s = next;
        (*parser).m_eventPtr = s;
    }
}

unsafe extern "C" fn processEntity(
    mut parser: XML_Parser,
    mut entity: *mut ENTITY,
    mut betweenDecl: XML_Bool,
    mut type_0: EntityType,
) -> XML_Error {
    let mut openEntity: *mut OPEN_INTERNAL_ENTITY = null_mut::<OPEN_INTERNAL_ENTITY>();
    let mut openEntityList: *mut *mut OPEN_INTERNAL_ENTITY =
        null_mut::<*mut OPEN_INTERNAL_ENTITY>();
    let mut freeEntityList: *mut *mut OPEN_INTERNAL_ENTITY =
        null_mut::<*mut OPEN_INTERNAL_ENTITY>();
    match type_0 {
        0 => {
            (*parser).m_processor = Some(
                internalEntityProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            );
            openEntityList = &raw mut (*parser).m_openInternalEntities;
            freeEntityList = &raw mut (*parser).m_freeInternalEntities;
        }
        1 => {
            openEntityList = &raw mut (*parser).m_openAttributeEntities;
            freeEntityList = &raw mut (*parser).m_freeAttributeEntities;
        }
        2 => {
            openEntityList = &raw mut (*parser).m_openValueEntities;
            freeEntityList = &raw mut (*parser).m_freeValueEntities;
        }
        _ => {
            assert!(false, "0");
        }
    }
    if !(*freeEntityList).is_null() {
        openEntity = *freeEntityList;
        *freeEntityList = (*openEntity).next;
    } else {
        openEntity = expat_malloc(parser, size_of::<OPEN_INTERNAL_ENTITY>(), 6382)
            as *mut OPEN_INTERNAL_ENTITY;
        if openEntity.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
    }
    (*entity).open = XML_TRUE;
    (*entity).hasMore = XML_TRUE;
    entityTrackingOnOpen(parser, entity, 6389);
    (*entity).processed = 0;
    (*openEntity).next = *openEntityList;
    *openEntityList = openEntity;
    (*openEntity).entity = entity;
    (*openEntity).type_0 = type_0;
    (*openEntity).startTagLevel = (*parser).m_tagLevel;
    (*openEntity).betweenDecl = betweenDecl;
    (*openEntity).internalEventPtr = null::<c_char>();
    (*openEntity).internalEventEndPtr = null::<c_char>();
    if type_0 == ENTITY_INTERNAL {
        triggerReenter(parser);
    }
    return XML_ERROR_NONE;
}

unsafe extern "C" fn internalEntityProcessor(
    mut parser: XML_Parser,
    mut _s: *const c_char,
    mut _end: *const c_char,
    mut _nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut entity: *mut ENTITY = null_mut::<ENTITY>();
    let mut textStart: *const c_char = null::<c_char>();
    let mut textEnd: *const c_char = null::<c_char>();
    let mut next: *const c_char = null::<c_char>();
    let mut result: XML_Error = XML_ERROR_NONE;
    let mut openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openInternalEntities;
    if openEntity.is_null() {
        return XML_ERROR_UNEXPECTED_STATE;
    }
    entity = (*openEntity).entity;
    if (*entity).hasMore != 0 {
        textStart = ((*entity).textPtr).offset((*entity).processed as isize);
        textEnd = (*entity).textPtr.offset((*entity).textLen as isize);
        next = textStart;
        if (*entity).is_param != 0 {
            let mut tok: c_int = (*(*parser).m_internalEncoding).scanners[0]
                .expect("non-null function pointer")(
                (*parser).m_internalEncoding,
                textStart,
                textEnd,
                &raw mut next,
            );
            result = doProlog(
                parser,
                (*parser).m_internalEncoding,
                textStart,
                textEnd,
                tok,
                next,
                &raw mut next,
                XML_FALSE,
                XML_FALSE,
                XML_ACCOUNT_ENTITY_EXPANSION,
            );
        } else {
            result = doContent(
                parser,
                (*openEntity).startTagLevel,
                (*parser).m_internalEncoding,
                textStart,
                textEnd,
                &raw mut next,
                XML_FALSE,
                XML_ACCOUNT_ENTITY_EXPANSION,
            );
        }
        if result != XML_ERROR_NONE {
            return result;
        }
        if textEnd != next
            && ((*parser).m_parsingStatus.parsing == XML_SUSPENDED
                || (*parser).m_parsingStatus.parsing == XML_PARSING
                    && (*parser).m_reenter as c_int != 0)
        {
            (*entity).processed = next.offset_from((*entity).textPtr) as c_int;
            return result;
        }
        (*entity).hasMore = XML_FALSE;
        if (*entity).is_param == 0 && (*openEntity).startTagLevel != (*parser).m_tagLevel {
            return XML_ERROR_ASYNC_ENTITY;
        }
        triggerReenter(parser);
        return result;
    }
    entityTrackingOnClose(parser, entity, 6470);
    assert!(
        (*parser).m_openInternalEntities == openEntity,
        "parser->m_openInternalEntities == openEntity"
    );
    (*entity).open = XML_FALSE;
    (*parser).m_openInternalEntities = (*(*parser).m_openInternalEntities).next;
    (*openEntity).next = (*parser).m_freeInternalEntities;
    (*parser).m_freeInternalEntities = openEntity;
    if (*parser).m_openInternalEntities.is_null() {
        (*parser).m_processor = if (*entity).is_param as c_int != 0 {
            Some(
                prologProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            )
        } else {
            Some(
                contentProcessor
                    as unsafe extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            )
        };
    }
    triggerReenter(parser);
    return XML_ERROR_NONE;
}

unsafe extern "C" fn errorProcessor(
    mut parser: XML_Parser,
    mut _s: *const c_char,
    mut _end: *const c_char,
    mut _nextPtr: *mut *const c_char,
) -> XML_Error {
    return (*parser).m_errorCode;
}

unsafe extern "C" fn storeAttributeValue(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut isCdata: XML_Bool,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut pool: *mut STRING_POOL,
    mut account: XML_Account,
) -> XML_Error {
    let mut next: *const c_char = ptr;
    let mut result: XML_Error = XML_ERROR_NONE;
    loop {
        if (*parser).m_openAttributeEntities.is_null() {
            result = appendAttributeValue(
                parser,
                enc,
                isCdata,
                next,
                end,
                pool,
                account,
                &raw mut next,
            );
        } else {
            let openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openAttributeEntities;
            if openEntity.is_null() {
                return XML_ERROR_UNEXPECTED_STATE;
            }
            let entity: *mut ENTITY = (*openEntity).entity;
            let textStart: *const c_char = ((*entity).textPtr).offset((*entity).processed as isize);
            let textEnd: *const c_char = (*entity).textPtr.offset((*entity).textLen as isize);
            let mut nextInEntity: *const c_char = textStart;
            if (*entity).hasMore != 0 {
                result = appendAttributeValue(
                    parser,
                    (*parser).m_internalEncoding,
                    isCdata,
                    textStart,
                    textEnd,
                    pool,
                    XML_ACCOUNT_ENTITY_EXPANSION,
                    &raw mut nextInEntity,
                );
                if result != XML_ERROR_NONE {
                    break;
                }
                if textEnd != nextInEntity {
                    (*entity).processed = nextInEntity.offset_from((*entity).textPtr) as c_int;
                    continue;
                } else {
                    (*entity).hasMore = XML_FALSE;
                    continue;
                }
            } else {
                entityTrackingOnClose(parser, entity, 6547);
                assert!(
                    (*parser).m_openAttributeEntities == openEntity,
                    "parser->m_openAttributeEntities == openEntity"
                );
                (*entity).open = XML_FALSE;
                (*parser).m_openAttributeEntities = (*(*parser).m_openAttributeEntities).next;
                (*openEntity).next = (*parser).m_freeAttributeEntities;
                (*parser).m_freeAttributeEntities = openEntity;
            }
        }
        if result != 0 || (*parser).m_openAttributeEntities.is_null() && end == next {
            break;
        }
    }
    if result as u64 != 0 {
        return result;
    }
    if isCdata == 0
        && (*pool).ptr.offset_from((*pool).start) as c_long != 0
        && *(*pool).ptr.offset(-1) as c_int == 0x20
    {
        (*pool).ptr = (*pool).ptr.offset(-1);
    }
    if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
        0
    } else {
        let fresh55 = (*pool).ptr;
        (*pool).ptr = (*pool).ptr.offset(1);
        *fresh55 = '\0' as XML_Char;
        1
    } == 0
    {
        return XML_ERROR_NO_MEMORY;
    }
    return XML_ERROR_NONE;
}

unsafe extern "C" fn appendAttributeValue(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut isCdata: XML_Bool,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut pool: *mut STRING_POOL,
    mut account: XML_Account,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let dtd: *mut DTD = (*parser).m_dtd;
    loop {
        let mut next: *const c_char = ptr;
        let mut tok: c_int = (*enc).literalScanners[0].expect("non-null function pointer")(
            enc,
            ptr,
            end,
            &raw mut next,
        );
        if accountingDiffTolerated(parser, tok, ptr, next, 6591, account) == 0 {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        let mut current_block_70: u64;
        match tok {
            XML_TOK_NONE => {
                if !nextPtr.is_null() {
                    *nextPtr = next;
                }
                return XML_ERROR_NONE;
            }
            XML_TOK_INVALID => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = next;
                }
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = ptr;
                }
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_CHAR_REF => {
                let mut buf: [XML_Char; 4] = [0; 4];
                let mut i: c_int = 0;
                let mut n: c_int =
                    (*enc).charRefNumber.expect("non-null function pointer")(enc, ptr);
                if n < 0 {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = ptr;
                    }
                    return XML_ERROR_BAD_CHAR_REF;
                }
                if isCdata == 0
                    && n == 0x20
                    && ((*pool).ptr.offset_from((*pool).start) as c_long == 0
                        || *(*pool).ptr.offset(-1) as c_int == 0x20)
                {
                    current_block_70 = 18038362259723567392;
                } else {
                    n = XmlUtf8Encode(n, &raw mut buf as *mut c_char);
                    i = 0;
                    while i < n {
                        if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
                            0
                        } else {
                            let fresh56 = (*pool).ptr;
                            (*pool).ptr = (*pool).ptr.offset(1);
                            *fresh56 = buf[i as usize];
                            1
                        } == 0
                        {
                            return XML_ERROR_NO_MEMORY;
                        }
                        i += 1;
                    }
                    current_block_70 = 18038362259723567392;
                }
            }
            XML_TOK_DATA_CHARS => {
                if poolAppend(pool, enc, ptr, next).is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                current_block_70 = 18038362259723567392;
            }
            XML_TOK_TRAILING_CR => {
                next = ptr.offset((*enc).minBytesPerChar as isize);
                current_block_70 = 1987954931741999833;
            }
            XML_TOK_ATTRIBUTE_VALUE_S | XML_TOK_DATA_NEWLINE => {
                current_block_70 = 1987954931741999833;
            }
            XML_TOK_ENTITY_REF => {
                let mut name: *const XML_Char = null::<XML_Char>();
                let mut entity: *mut ENTITY = null_mut::<ENTITY>();
                let mut checkEntityDecl: bool = false;
                let mut ch: XML_Char = (*enc)
                    .predefinedEntityName
                    .expect("non-null function pointer")(
                    enc,
                    ptr.offset((*enc).minBytesPerChar as isize),
                    next.offset(-((*enc).minBytesPerChar as isize)),
                ) as XML_Char;
                if ch != 0 {
                    accountingDiffTolerated(
                        parser,
                        tok,
                        &raw mut ch,
                        (&raw mut ch).offset(size_of::<XML_Char>() as isize),
                        6663,
                        XML_ACCOUNT_ENTITY_EXPANSION,
                    );
                    if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
                        0
                    } else {
                        let fresh58 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *fresh58 = ch;
                        1
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                } else {
                    name = poolStoreString(
                        &raw mut (*parser).m_temp2Pool,
                        enc,
                        ptr.offset((*enc).minBytesPerChar as isize),
                        next.offset(-((*enc).minBytesPerChar as isize)),
                    );
                    if name.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    entity =
                        lookup(parser, &raw mut (*dtd).generalEntities, name, 0) as *mut ENTITY;
                    (*parser).m_temp2Pool.ptr = (*parser).m_temp2Pool.start;
                    if pool == &raw mut (*dtd).pool {
                        checkEntityDecl = (*parser).m_prologState.documentEntity != 0
                            && (if (*dtd).standalone as c_int != 0 {
                                (*parser).m_openInternalEntities.is_null() as c_int
                            } else {
                                ((*dtd).hasParamEntityRefs == 0) as c_int
                            }) != 0;
                    } else {
                        checkEntityDecl =
                            (*dtd).hasParamEntityRefs == 0 || (*dtd).standalone as c_int != 0;
                    }
                    if checkEntityDecl {
                        if entity.is_null() {
                            return XML_ERROR_UNDEFINED_ENTITY;
                        } else if (*entity).is_internal == 0 {
                            return XML_ERROR_ENTITY_DECLARED_IN_PE;
                        }
                        current_block_70 = 13678349939556791712;
                    } else if entity.is_null() {
                        current_block_70 = 18038362259723567392;
                    } else {
                        current_block_70 = 13678349939556791712;
                    }
                    match current_block_70 {
                        18038362259723567392 => {}
                        _ => {
                            if (*entity).open != 0 {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr;
                                }
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !(*entity).notation.is_null() {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr;
                                }
                                return XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if (*entity).textPtr.is_null() {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = ptr;
                                }
                                return XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
                            } else {
                                let mut result: XML_Error = XML_ERROR_NONE;
                                result = processEntity(parser, entity, XML_FALSE, ENTITY_ATTRIBUTE);
                                if result == XML_ERROR_NONE && !nextPtr.is_null() {
                                    *nextPtr = next;
                                }
                                return result;
                            }
                        }
                    }
                }
                current_block_70 = 18038362259723567392;
            }
            _ => {
                if enc == (*parser).m_encoding {
                    (*parser).m_eventPtr = ptr;
                }
                return XML_ERROR_UNEXPECTED_STATE;
            }
        }
        match current_block_70 {
            1987954931741999833 => {
                if !(isCdata == 0
                    && ((*pool).ptr.offset_from((*pool).start) as c_long == 0
                        || *(*pool).ptr.offset(-1) as c_int == 0x20))
                {
                    if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
                        0
                    } else {
                        let fresh57 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *fresh57 = 0x20i8;
                        1
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                }
            }
            _ => {}
        }
        ptr = next;
    }
}

unsafe extern "C" fn storeEntityValue(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut entityTextPtr: *const c_char,
    mut entityTextEnd: *const c_char,
    mut account: XML_Account,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut current_block: u64;
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut pool: *mut STRING_POOL = &raw mut (*dtd).entityValuePool;
    let mut result: XML_Error = XML_ERROR_NONE;
    let mut oldInEntityValue: c_int = (*parser).m_prologState.inEntityValue;
    (*parser).m_prologState.inEntityValue = 1;
    if (*pool).blocks.is_null() {
        if poolGrow(pool) == 0 {
            return XML_ERROR_NO_MEMORY;
        }
    }
    let mut next: *const c_char = null::<c_char>();
    's_35: loop {
        next = entityTextPtr;
        let mut tok: c_int = (*enc).literalScanners[1].expect("non-null function pointer")(
            enc,
            entityTextPtr,
            entityTextEnd,
            &raw mut next,
        );
        if accountingDiffTolerated(parser, tok, entityTextPtr, next, 6798, account) == 0 {
            accountingOnAbort(parser);
            result = XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            break;
        } else {
            match tok {
                XML_TOK_PARAM_ENTITY_REF => {
                    if (*parser).m_isParamEntity as c_int != 0 || enc != (*parser).m_encoding {
                        let mut name: *const XML_Char = null::<XML_Char>();
                        let mut entity: *mut ENTITY = null_mut::<ENTITY>();
                        name = poolStoreString(
                            &raw mut (*parser).m_tempPool,
                            enc,
                            entityTextPtr.offset((*enc).minBytesPerChar as isize),
                            next.offset(-((*enc).minBytesPerChar as isize)),
                        );
                        if name.is_null() {
                            result = XML_ERROR_NO_MEMORY;
                            break;
                        } else {
                            entity = lookup(parser, &raw mut (*dtd).paramEntities, name, 0)
                                as *mut ENTITY;
                            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
                            if entity.is_null() {
                                (*dtd).keepProcessing = (*dtd).standalone;
                                break;
                            } else if (*entity).open as c_int != 0
                                || entity == (*parser).m_declEntity
                            {
                                if enc == (*parser).m_encoding {
                                    (*parser).m_eventPtr = entityTextPtr;
                                }
                                result = XML_ERROR_RECURSIVE_ENTITY_REF;
                                break;
                            } else if !(*entity).systemId.is_null() {
                                if (*parser).m_externalEntityRefHandler.is_some() {
                                    (*dtd).paramEntityRead = XML_FALSE;
                                    (*entity).open = XML_TRUE;
                                    entityTrackingOnOpen(parser, entity, 6840);
                                    if (*parser)
                                        .m_externalEntityRefHandler
                                        .expect("non-null function pointer")(
                                        (*parser).m_externalEntityRefHandlerArg,
                                        null::<XML_Char>(),
                                        (*entity).base,
                                        (*entity).systemId,
                                        (*entity).publicId,
                                    ) == 0
                                    {
                                        entityTrackingOnClose(parser, entity, 6844);
                                        (*entity).open = XML_FALSE;
                                        result = XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                        break;
                                    } else {
                                        entityTrackingOnClose(parser, entity, 6849);
                                        (*entity).open = XML_FALSE;
                                        if (*dtd).paramEntityRead == 0 {
                                            (*dtd).keepProcessing = (*dtd).standalone;
                                        }
                                    }
                                } else {
                                    (*dtd).keepProcessing = (*dtd).standalone;
                                }
                            } else {
                                result = processEntity(parser, entity, XML_FALSE, ENTITY_VALUE);
                                break;
                            }
                        }
                    } else {
                        (*parser).m_eventPtr = entityTextPtr;
                        result = XML_ERROR_PARAM_ENTITY_REF;
                        break;
                    }
                    current_block = 5028470053297453708;
                }
                XML_TOK_NONE => {
                    result = XML_ERROR_NONE;
                    break;
                }
                XML_TOK_ENTITY_REF | XML_TOK_DATA_CHARS => {
                    if poolAppend(pool, enc, entityTextPtr, next).is_null() {
                        result = XML_ERROR_NO_MEMORY;
                        break;
                    } else {
                        current_block = 5028470053297453708;
                    }
                }
                XML_TOK_TRAILING_CR => {
                    next = entityTextPtr.offset((*enc).minBytesPerChar as isize);
                    current_block = 14913579936405700701;
                }
                XML_TOK_DATA_NEWLINE => {
                    current_block = 14913579936405700701;
                }
                XML_TOK_CHAR_REF => {
                    let mut buf: [XML_Char; 4] = [0; 4];
                    let mut i: c_int = 0;
                    let mut n: c_int = (*enc).charRefNumber.expect("non-null function pointer")(
                        enc,
                        entityTextPtr,
                    );
                    if n < 0 {
                        if enc == (*parser).m_encoding {
                            (*parser).m_eventPtr = entityTextPtr;
                        }
                        result = XML_ERROR_BAD_CHAR_REF;
                        break;
                    } else {
                        n = XmlUtf8Encode(n, &raw mut buf as *mut c_char);
                        i = 0;
                        while i < n {
                            if (*pool).end == (*pool).ptr as *const XML_Char && poolGrow(pool) == 0
                            {
                                result = XML_ERROR_NO_MEMORY;
                                break 's_35;
                            } else {
                                let fresh73 = (*pool).ptr;
                                (*pool).ptr = (*pool).ptr.offset(1);
                                *fresh73 = buf[i as usize];
                                i += 1;
                            }
                        }
                    }
                    current_block = 5028470053297453708;
                }
                XML_TOK_PARTIAL => {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = entityTextPtr;
                    }
                    result = XML_ERROR_INVALID_TOKEN;
                    break;
                }
                XML_TOK_INVALID => {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = next;
                    }
                    result = XML_ERROR_INVALID_TOKEN;
                    break;
                }
                _ => {
                    if enc == (*parser).m_encoding {
                        (*parser).m_eventPtr = entityTextPtr;
                    }
                    result = XML_ERROR_UNEXPECTED_STATE;
                    break;
                }
            }
            match current_block {
                14913579936405700701 => {
                    if (*pool).end == (*pool).ptr as *const XML_Char && poolGrow(pool) == 0 {
                        result = XML_ERROR_NO_MEMORY;
                        break;
                    } else {
                        let fresh72 = (*pool).ptr;
                        (*pool).ptr = (*pool).ptr.offset(1);
                        *fresh72 = 0xai8;
                    }
                }
                _ => {}
            }
            entityTextPtr = next;
        }
    }
    (*parser).m_prologState.inEntityValue = oldInEntityValue;
    if !nextPtr.is_null() {
        *nextPtr = next;
    }
    return result;
}

unsafe extern "C" fn callStoreEntityValue(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut entityTextPtr: *const c_char,
    mut entityTextEnd: *const c_char,
    mut account: XML_Account,
) -> XML_Error {
    let mut next: *const c_char = entityTextPtr;
    let mut result: XML_Error = XML_ERROR_NONE;
    loop {
        if (*parser).m_openValueEntities.is_null() {
            result = storeEntityValue(parser, enc, next, entityTextEnd, account, &raw mut next);
        } else {
            let openEntity: *mut OPEN_INTERNAL_ENTITY = (*parser).m_openValueEntities;
            if openEntity.is_null() {
                return XML_ERROR_UNEXPECTED_STATE;
            }
            let entity: *mut ENTITY = (*openEntity).entity;
            let textStart: *const c_char = ((*entity).textPtr).offset((*entity).processed as isize);
            let textEnd: *const c_char = (*entity).textPtr.offset((*entity).textLen as isize);
            let mut nextInEntity: *const c_char = textStart;
            if (*entity).hasMore != 0 {
                result = storeEntityValue(
                    parser,
                    (*parser).m_internalEncoding,
                    textStart,
                    textEnd,
                    XML_ACCOUNT_ENTITY_EXPANSION,
                    &raw mut nextInEntity,
                );
                if result != XML_ERROR_NONE {
                    break;
                }
                if textEnd != nextInEntity {
                    (*entity).processed = nextInEntity.offset_from((*entity).textPtr) as c_int;
                    continue;
                } else {
                    (*entity).hasMore = XML_FALSE;
                    continue;
                }
            } else {
                entityTrackingOnClose(parser, entity, 6998);
                assert!(
                    (*parser).m_openValueEntities == openEntity,
                    "parser->m_openValueEntities == openEntity"
                );
                (*entity).open = XML_FALSE;
                (*parser).m_openValueEntities = (*(*parser).m_openValueEntities).next;
                (*openEntity).next = (*parser).m_freeValueEntities;
                (*parser).m_freeValueEntities = openEntity;
            }
        }
        if result != 0 || (*parser).m_openValueEntities.is_null() && entityTextEnd == next {
            break;
        }
    }
    return result;
}

unsafe extern "C" fn normalizeLines(mut s: *mut XML_Char) {
    let mut p: *mut XML_Char = null_mut::<XML_Char>();
    loop {
        if *s as c_int == '\0' as i32 {
            return;
        }
        if *s as c_int == 0xd {
            break;
        }
        s = s.offset(1);
    }
    p = s;
    loop {
        if *s as c_int == 0xd {
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = 0xai8;
            s = s.offset(1);
            if *s as c_int == 0xa {
                s = s.offset(1);
            }
        } else {
            let fresh8 = s;
            s = s.offset(1);
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = *fresh8;
        }
        if !(*s != 0) {
            break;
        }
    }
    *p = '\0' as XML_Char;
}

unsafe extern "C" fn reportProcessingInstruction(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut start: *const c_char,
    mut end: *const c_char,
) -> c_int {
    let mut target: *const XML_Char = null::<XML_Char>();
    let mut data: *mut XML_Char = null_mut::<XML_Char>();
    let mut tem: *const c_char = null::<c_char>();
    if (*parser).m_processingInstructionHandler.is_none() {
        if (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, start, end);
        }
        return 1i32;
    }
    start = start.offset(((*enc).minBytesPerChar * 2i32) as isize);
    tem = start.offset((*enc).nameLength.expect("non-null function pointer")(enc, start) as isize);
    target = poolStoreString(&raw mut (*parser).m_tempPool, enc, start, tem);
    if target.is_null() {
        return 0i32;
    }
    (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
    data = poolStoreString(
        &raw mut (*parser).m_tempPool,
        enc,
        (*enc).skipS.expect("non-null function pointer")(enc, tem),
        end.offset(-(((*enc).minBytesPerChar * 2i32) as isize)),
    );
    if data.is_null() {
        return 0i32;
    }
    normalizeLines(data);
    (*parser)
        .m_processingInstructionHandler
        .expect("non-null function pointer")((*parser).m_handlerArg, target, data);
    poolClear(&raw mut (*parser).m_tempPool);
    return 1;
}

unsafe extern "C" fn reportComment(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut start: *const c_char,
    mut end: *const c_char,
) -> c_int {
    let mut data: *mut XML_Char = null_mut::<XML_Char>();
    if (*parser).m_commentHandler.is_none() {
        if (*parser).m_defaultHandler.is_some() {
            reportDefault(parser, enc, start, end);
        }
        return 1i32;
    }
    data = poolStoreString(
        &raw mut (*parser).m_tempPool,
        enc,
        start.offset(((*enc).minBytesPerChar * 4i32) as isize),
        end.offset(-(((*enc).minBytesPerChar * 3i32) as isize)),
    );
    if data.is_null() {
        return 0i32;
    }
    normalizeLines(data);
    (*parser)
        .m_commentHandler
        .expect("non-null function pointer")((*parser).m_handlerArg, data);
    poolClear(&raw mut (*parser).m_tempPool);
    return 1;
}

unsafe extern "C" fn reportDefault(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut s: *const c_char,
    mut end: *const c_char,
) {
    if (*enc).isUtf8 == 0 {
        let mut convert_res: XML_Convert_Result = XML_CONVERT_COMPLETED;
        let mut eventPP: *mut *const c_char = null_mut::<*const c_char>();
        let mut eventEndPP: *mut *const c_char = null_mut::<*const c_char>();
        if enc == (*parser).m_encoding {
            eventPP = &raw mut (*parser).m_eventPtr;
            eventEndPP = &raw mut (*parser).m_eventEndPtr;
        } else {
            eventPP = &raw mut (*(*parser).m_openInternalEntities).internalEventPtr;
            eventEndPP = &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr;
        }
        loop {
            let mut dataPtr: *mut ICHAR = (*parser).m_dataBuf;
            convert_res = (*enc).utf8Convert.expect("non-null function pointer")(
                enc,
                &raw mut s,
                end,
                &raw mut dataPtr,
                (*parser).m_dataBufEnd,
            );
            *eventEndPP = s;
            (*parser)
                .m_defaultHandler
                .expect("non-null function pointer")(
                (*parser).m_handlerArg,
                (*parser).m_dataBuf,
                dataPtr.offset_from((*parser).m_dataBuf) as c_int,
            );
            *eventPP = s;
            if !(convert_res != XML_CONVERT_COMPLETED
                && convert_res != XML_CONVERT_INPUT_INCOMPLETE)
            {
                break;
            }
        }
    } else {
        (*parser)
            .m_defaultHandler
            .expect("non-null function pointer")(
            (*parser).m_handlerArg,
            s,
            (end).offset_from(s) as c_int,
        );
    };
}

unsafe extern "C" fn defineAttribute(
    mut type_0: *mut ELEMENT_TYPE,
    mut attId: *mut ATTRIBUTE_ID,
    mut isCdata: XML_Bool,
    mut isId: XML_Bool,
    mut value: *const XML_Char,
    mut parser: XML_Parser,
) -> c_int {
    let mut att: *mut DEFAULT_ATTRIBUTE = null_mut::<DEFAULT_ATTRIBUTE>();
    if !value.is_null() || isId as c_int != 0 {
        let mut i: c_int = 0;
        i = 0;
        while i < (*type_0).nDefaultAtts {
            if attId == (*(*type_0).defaultAtts.offset(i as isize)).id as *mut ATTRIBUTE_ID {
                return 1i32;
            }
            i += 1;
        }
        if isId as c_int != 0 && (*type_0).idAtt.is_null() && (*attId).xmlns == 0 {
            (*type_0).idAtt = attId;
        }
    }
    if (*type_0).nDefaultAtts == (*type_0).allocDefaultAtts {
        if (*type_0).allocDefaultAtts == 0 {
            (*type_0).allocDefaultAtts = 8;
            (*type_0).defaultAtts = expat_malloc(
                parser,
                ((*type_0).allocDefaultAtts as size_t).wrapping_mul(size_of::<DEFAULT_ATTRIBUTE>()),
                7182,
            ) as *mut DEFAULT_ATTRIBUTE;
            if (*type_0).defaultAtts.is_null() {
                (*type_0).allocDefaultAtts = 0;
                return 0i32;
            }
        } else {
            let mut temp: *mut DEFAULT_ATTRIBUTE = null_mut::<DEFAULT_ATTRIBUTE>();
            if (*type_0).allocDefaultAtts > INT_MAX / 2 {
                return 0i32;
            }
            let mut count: c_int = (*type_0).allocDefaultAtts * 2;
            temp = expat_realloc(
                parser,
                (*type_0).defaultAtts as *mut c_void,
                (count as size_t).wrapping_mul(size_of::<DEFAULT_ATTRIBUTE>()),
                7208,
            ) as *mut DEFAULT_ATTRIBUTE;
            if temp.is_null() {
                return 0i32;
            }
            (*type_0).allocDefaultAtts = count;
            (*type_0).defaultAtts = temp;
        }
    }
    att = (*type_0)
        .defaultAtts
        .offset((*type_0).nDefaultAtts as isize);
    (*att).id = attId;
    (*att).value = value;
    (*att).isCdata = isCdata;
    if isCdata == 0 {
        (*attId).maybeTokenized = XML_TRUE;
    }
    (*type_0).nDefaultAtts += 1;
    return 1;
}

unsafe extern "C" fn setElementTypePrefix(
    mut parser: XML_Parser,
    mut elementType: *mut ELEMENT_TYPE,
) -> c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut name: *const XML_Char = null::<XML_Char>();
    name = (*elementType).name;
    while *name != 0 {
        if *name as c_int == 0x3a {
            let mut prefix: *mut PREFIX = null_mut::<PREFIX>();
            let mut s: *const XML_Char = null::<XML_Char>();
            s = (*elementType).name;
            while s != name {
                if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                    && poolGrow(&raw mut (*dtd).pool) == 0
                {
                    0
                } else {
                    let fresh15 = (*dtd).pool.ptr;
                    (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                    *fresh15 = *s;
                    1
                } == 0
                {
                    return 0i32;
                }
                s = s.offset(1);
            }
            if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                && poolGrow(&raw mut (*dtd).pool) == 0
            {
                0
            } else {
                let fresh16 = (*dtd).pool.ptr;
                (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                *fresh16 = '\0' as XML_Char;
                1
            } == 0
            {
                return 0i32;
            }
            prefix = lookup(
                parser,
                &raw mut (*dtd).prefixes,
                (*dtd).pool.start as KEY,
                size_of::<PREFIX>(),
            ) as *mut PREFIX;
            if prefix.is_null() {
                return 0i32;
            }
            if (*prefix).name == (*dtd).pool.start as *const XML_Char {
                (*dtd).pool.start = (*dtd).pool.ptr;
            } else {
                (*dtd).pool.ptr = (*dtd).pool.start;
            }
            (*elementType).prefix = prefix;
            break;
        } else {
            name = name.offset(1);
        }
    }
    return 1;
}

unsafe extern "C" fn getAttributeId(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut start: *const c_char,
    mut end: *const c_char,
) -> *mut ATTRIBUTE_ID {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut id: *mut ATTRIBUTE_ID = null_mut::<ATTRIBUTE_ID>();
    let mut name: *const XML_Char = null::<XML_Char>();
    if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char && poolGrow(&raw mut (*dtd).pool) == 0
    {
        0
    } else {
        let fresh52 = (*dtd).pool.ptr;
        (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
        *fresh52 = '\0' as XML_Char;
        1
    } == 0
    {
        return null_mut::<ATTRIBUTE_ID>();
    }
    name = poolStoreString(&raw mut (*dtd).pool, enc, start, end);
    if name.is_null() {
        return null_mut::<ATTRIBUTE_ID>();
    }
    name = name.offset(1);
    id = lookup(
        parser,
        &raw mut (*dtd).attributeIds,
        name,
        size_of::<ATTRIBUTE_ID>(),
    ) as *mut ATTRIBUTE_ID;
    if id.is_null() {
        return null_mut::<ATTRIBUTE_ID>();
    }
    if (*id).name != name as *mut XML_Char {
        (*dtd).pool.ptr = (*dtd).pool.start;
    } else {
        (*dtd).pool.start = (*dtd).pool.ptr;
        if !((*parser).m_ns == 0) {
            if *name.offset(0) as c_int == 0x78
                && *name.offset(1) as c_int == 0x6d
                && *name.offset(2) as c_int == 0x6c
                && *name.offset(3) as c_int == 0x6e
                && *name.offset(4) as c_int == 0x73
                && (*name.offset(5) as c_int == '\0' as i32 || *name.offset(5) as c_int == 0x3a)
            {
                if *name.offset(5) as c_int == '\0' as i32 {
                    (*id).prefix = &raw mut (*dtd).defaultPrefix;
                } else {
                    (*id).prefix = lookup(
                        parser,
                        &raw mut (*dtd).prefixes,
                        name.offset(6isize),
                        size_of::<PREFIX>(),
                    ) as *mut PREFIX;
                }
                (*id).xmlns = XML_TRUE;
            } else {
                let mut i: c_int = 0;
                i = 0;
                while *name.offset(i as isize) != 0 {
                    if *name.offset(i as isize) as c_int == 0x3a {
                        let mut j: c_int = 0;
                        j = 0;
                        while j < i {
                            if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                                && poolGrow(&raw mut (*dtd).pool) == 0
                            {
                                0
                            } else {
                                let fresh53 = (*dtd).pool.ptr;
                                (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                                *fresh53 = *name.offset(j as isize);
                                1
                            } == 0
                            {
                                return null_mut::<ATTRIBUTE_ID>();
                            }
                            j += 1;
                        }
                        if if (*dtd).pool.ptr == (*dtd).pool.end as *mut XML_Char
                            && poolGrow(&raw mut (*dtd).pool) == 0
                        {
                            0
                        } else {
                            let fresh54 = (*dtd).pool.ptr;
                            (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1);
                            *fresh54 = '\0' as XML_Char;
                            1
                        } == 0
                        {
                            return null_mut::<ATTRIBUTE_ID>();
                        }
                        (*id).prefix = lookup(
                            parser,
                            &raw mut (*dtd).prefixes,
                            (*dtd).pool.start as KEY,
                            size_of::<PREFIX>(),
                        ) as *mut PREFIX;
                        if (*id).prefix.is_null() {
                            return null_mut::<ATTRIBUTE_ID>();
                        }
                        if (*(*id).prefix).name == (*dtd).pool.start as *const XML_Char {
                            (*dtd).pool.start = (*dtd).pool.ptr;
                        } else {
                            (*dtd).pool.ptr = (*dtd).pool.start;
                        }
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }
    return id;
}

unsafe extern "C" fn getContext(mut parser: XML_Parser) -> *const XML_Char {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: null_mut::<*mut NAMED>(),
        end: null_mut::<*mut NAMED>(),
        table: null::<HASH_TABLE>(),
        index: 0 as size_t,
    };
    let mut needSep: XML_Bool = XML_FALSE;
    if !(*dtd).defaultPrefix.binding.is_null() {
        let mut i: c_int = 0;
        let mut len: c_int = 0;
        if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
            && poolGrow(&raw mut (*parser).m_tempPool) == 0
        {
            0
        } else {
            let fresh61 = (*parser).m_tempPool.ptr;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
            *fresh61 = 0x3di8;
            1
        } == 0
        {
            return null::<XML_Char>();
        }
        len = (*(*dtd).defaultPrefix.binding).uriLen;
        if (*parser).m_namespaceSeparator != 0 {
            len -= 1;
        }
        i = 0;
        while i < len {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0
            } else {
                let fresh62 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh62 = *(*(*dtd).defaultPrefix.binding).uri.offset(i as isize);
                1
            } == 0
            {
                return null::<XML_Char>();
            }
            i += 1;
        }
        needSep = XML_TRUE;
    }
    hashTableIterInit(&raw mut iter, &raw mut (*dtd).prefixes);
    loop {
        let mut i_0: c_int = 0;
        let mut len_0: c_int = 0;
        let mut s: *const XML_Char = null::<XML_Char>();
        let mut prefix: *mut PREFIX = hashTableIterNext(&raw mut iter) as *mut PREFIX;
        if prefix.is_null() {
            break;
        }
        if (*prefix).binding.is_null() {
            continue;
        }
        if needSep as c_int != 0
            && (if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0
            } else {
                let fresh63 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh63 = 0xci8;
                1
            }) == 0
        {
            return null::<XML_Char>();
        }
        s = (*prefix).name;
        while *s != 0 {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0
            } else {
                let fresh64 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh64 = *s;
                1
            } == 0
            {
                return null::<XML_Char>();
            }
            s = s.offset(1);
        }
        if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
            && poolGrow(&raw mut (*parser).m_tempPool) == 0
        {
            0
        } else {
            let fresh65 = (*parser).m_tempPool.ptr;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
            *fresh65 = 0x3di8;
            1
        } == 0
        {
            return null::<XML_Char>();
        }
        len_0 = (*(*prefix).binding).uriLen;
        if (*parser).m_namespaceSeparator != 0 {
            len_0 -= 1;
        }
        i_0 = 0;
        while i_0 < len_0 {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0
            } else {
                let fresh66 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh66 = *(*(*prefix).binding).uri.offset(i_0 as isize);
                1
            } == 0
            {
                return null::<XML_Char>();
            }
            i_0 += 1;
        }
        needSep = XML_TRUE;
    }
    hashTableIterInit(&raw mut iter, &raw mut (*dtd).generalEntities);
    loop {
        let mut s_0: *const XML_Char = null::<XML_Char>();
        let mut e: *mut ENTITY = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if e.is_null() {
            break;
        }
        if (*e).open == 0 {
            continue;
        }
        if needSep as c_int != 0
            && (if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0
            } else {
                let fresh67 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh67 = 0xci8;
                1
            }) == 0
        {
            return null::<XML_Char>();
        }
        s_0 = (*e).name;
        while *s_0 != 0 {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0
            } else {
                let fresh68 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh68 = *s_0;
                1
            } == 0
            {
                return null::<XML_Char>();
            }
            s_0 = s_0.offset(1);
        }
        needSep = XML_TRUE;
    }
    if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
        && poolGrow(&raw mut (*parser).m_tempPool) == 0
    {
        0
    } else {
        let fresh69 = (*parser).m_tempPool.ptr;
        (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
        *fresh69 = '\0' as XML_Char;
        1
    } == 0
    {
        return null::<XML_Char>();
    }
    return (*parser).m_tempPool.start;
}

unsafe extern "C" fn setContext(mut parser: XML_Parser, mut context: *const XML_Char) -> XML_Bool {
    if context.is_null() {
        return XML_FALSE;
    }
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut s: *const XML_Char = context;
    while *context as c_int != '\0' as i32 {
        if *s as c_int == 0xc || *s as c_int == '\0' as i32 {
            let mut e: *mut ENTITY = null_mut::<ENTITY>();
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0
            } else {
                let fresh76 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh76 = '\0' as XML_Char;
                1
            } == 0
            {
                return XML_FALSE;
            }
            e = lookup(
                parser,
                &raw mut (*dtd).generalEntities,
                (*parser).m_tempPool.start as KEY,
                0,
            ) as *mut ENTITY;
            if !e.is_null() {
                (*e).open = XML_TRUE;
            }
            if *s as c_int != '\0' as i32 {
                s = s.offset(1);
            }
            context = s;
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
        } else if *s as c_int == 0x3d {
            let mut prefix: *mut PREFIX = null_mut::<PREFIX>();
            if (*parser)
                .m_tempPool
                .ptr
                .offset_from((*parser).m_tempPool.start) as c_long
                == 0
            {
                prefix = &raw mut (*dtd).defaultPrefix;
            } else {
                if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                {
                    0
                } else {
                    let fresh77 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *fresh77 = '\0' as XML_Char;
                    1
                } == 0
                {
                    return XML_FALSE;
                }
                prefix = lookup(
                    parser,
                    &raw mut (*dtd).prefixes,
                    (*parser).m_tempPool.start as KEY,
                    size_of::<PREFIX>(),
                ) as *mut PREFIX;
                if prefix.is_null() {
                    return XML_FALSE;
                }
                if (*prefix).name == (*parser).m_tempPool.start as *const XML_Char {
                    (*prefix).name = poolCopyString(&raw mut (*dtd).pool, (*prefix).name);
                    if (*prefix).name.is_null() {
                        return XML_FALSE;
                    }
                }
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
            }
            context = s.offset(1);
            while *context as c_int != 0xc && *context as c_int != '\0' as i32 {
                if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                    && poolGrow(&raw mut (*parser).m_tempPool) == 0
                {
                    0
                } else {
                    let fresh78 = (*parser).m_tempPool.ptr;
                    (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                    *fresh78 = *context;
                    1
                } == 0
                {
                    return XML_FALSE;
                }
                context = context.offset(1);
            }
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0
            } else {
                let fresh79 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh79 = '\0' as XML_Char;
                1
            } == 0
            {
                return XML_FALSE;
            }
            if addBinding(
                parser,
                prefix,
                null::<ATTRIBUTE_ID>(),
                (*parser).m_tempPool.start,
                &raw mut (*parser).m_inheritedBindings,
            ) != XML_ERROR_NONE
            {
                return XML_FALSE;
            }
            (*parser).m_tempPool.ptr = (*parser).m_tempPool.start;
            if *context as c_int != '\0' as i32 {
                context = context.offset(1);
            }
            s = context;
        } else {
            if if (*parser).m_tempPool.ptr == (*parser).m_tempPool.end as *mut XML_Char
                && poolGrow(&raw mut (*parser).m_tempPool) == 0
            {
                0
            } else {
                let fresh80 = (*parser).m_tempPool.ptr;
                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                *fresh80 = *s;
                1
            } == 0
            {
                return XML_FALSE;
            }
            s = s.offset(1);
        }
    }
    return XML_TRUE;
}

unsafe extern "C" fn normalizePublicId(mut publicId: *mut XML_Char) {
    let mut p: *mut XML_Char = publicId;
    let mut s: *mut XML_Char = null_mut::<XML_Char>();
    s = publicId;
    while *s != 0 {
        match *s as c_int {
            32 | 13 | 10 => {
                if p != publicId && *p.offset(-1) as c_int != 0x20 {
                    let fresh70 = p;
                    p = p.offset(1);
                    *fresh70 = 0x20i8;
                }
            }
            _ => {
                let fresh71 = p;
                p = p.offset(1);
                *fresh71 = *s;
            }
        }
        s = s.offset(1);
    }
    if p != publicId && *p.offset(-1) as c_int == 0x20 {
        p = p.offset(-1);
    }
    *p = '\0' as XML_Char;
}

unsafe extern "C" fn dtdCreate(mut parser: XML_Parser) -> *mut DTD {
    let mut p: *mut DTD = expat_malloc(parser, size_of::<DTD>(), 7500) as *mut DTD;
    if p.is_null() {
        return p;
    }
    poolInit(&raw mut (*p).pool, parser);
    poolInit(&raw mut (*p).entityValuePool, parser);
    hashTableInit(&raw mut (*p).generalEntities, parser);
    hashTableInit(&raw mut (*p).elementTypes, parser);
    hashTableInit(&raw mut (*p).attributeIds, parser);
    hashTableInit(&raw mut (*p).prefixes, parser);
    (*p).paramEntityRead = XML_FALSE;
    hashTableInit(&raw mut (*p).paramEntities, parser);
    (*p).defaultPrefix.name = null::<XML_Char>();
    (*p).defaultPrefix.binding = null_mut::<BINDING>();
    (*p).in_eldecl = XML_FALSE;
    (*p).scaffIndex = null_mut::<c_int>();
    (*p).scaffold = null_mut::<CONTENT_SCAFFOLD>();
    (*p).scaffLevel = 0;
    (*p).scaffSize = 0;
    (*p).scaffCount = 0;
    (*p).contentStringLen = 0;
    (*p).keepProcessing = XML_TRUE;
    (*p).hasParamEntityRefs = XML_FALSE;
    (*p).standalone = XML_FALSE;
    return p;
}

unsafe extern "C" fn dtdReset(mut p: *mut DTD, mut parser: XML_Parser) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: null_mut::<*mut NAMED>(),
        end: null_mut::<*mut NAMED>(),
        table: null::<HASH_TABLE>(),
        index: 0 as size_t,
    };
    hashTableIterInit(&raw mut iter, &raw mut (*p).elementTypes);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 {
            expat_free(parser, (*e).defaultAtts as *mut c_void, 7539i32);
        }
    }
    hashTableClear(&raw mut (*p).generalEntities);
    (*p).paramEntityRead = XML_FALSE;
    hashTableClear(&raw mut (*p).paramEntities);
    hashTableClear(&raw mut (*p).elementTypes);
    hashTableClear(&raw mut (*p).attributeIds);
    hashTableClear(&raw mut (*p).prefixes);
    poolClear(&raw mut (*p).pool);
    poolClear(&raw mut (*p).entityValuePool);
    (*p).defaultPrefix.name = null::<XML_Char>();
    (*p).defaultPrefix.binding = null_mut::<BINDING>();
    (*p).in_eldecl = XML_FALSE;
    expat_free(parser, (*p).scaffIndex as *mut c_void, 7556);
    (*p).scaffIndex = null_mut::<c_int>();
    expat_free(parser, (*p).scaffold as *mut c_void, 7558);
    (*p).scaffold = null_mut::<CONTENT_SCAFFOLD>();
    (*p).scaffLevel = 0;
    (*p).scaffSize = 0u32;
    (*p).scaffCount = 0u32;
    (*p).contentStringLen = 0u32;
    (*p).keepProcessing = XML_TRUE;
    (*p).hasParamEntityRefs = XML_FALSE;
    (*p).standalone = XML_FALSE;
}

unsafe extern "C" fn dtdDestroy(
    mut p: *mut DTD,
    mut isDocEntity: XML_Bool,
    mut parser: XML_Parser,
) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: null_mut::<*mut NAMED>(),
        end: null_mut::<*mut NAMED>(),
        table: null::<HASH_TABLE>(),
        index: 0 as size_t,
    };
    hashTableIterInit(&raw mut iter, &raw mut (*p).elementTypes);
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if (*e).allocDefaultAtts != 0 {
            expat_free(parser, (*e).defaultAtts as *mut c_void, 7580i32);
        }
    }
    hashTableDestroy(&raw mut (*p).generalEntities);
    hashTableDestroy(&raw mut (*p).paramEntities);
    hashTableDestroy(&raw mut (*p).elementTypes);
    hashTableDestroy(&raw mut (*p).attributeIds);
    hashTableDestroy(&raw mut (*p).prefixes);
    poolDestroy(&raw mut (*p).pool);
    poolDestroy(&raw mut (*p).entityValuePool);
    if isDocEntity != 0 {
        expat_free(parser, (*p).scaffIndex as *mut c_void, 7592);
        expat_free(parser, (*p).scaffold as *mut c_void, 7593i32);
    }
    expat_free(parser, p as *mut c_void, 7595);
}

unsafe extern "C" fn dtdCopy(
    mut oldParser: XML_Parser,
    mut newDtd: *mut DTD,
    mut oldDtd: *const DTD,
    mut parser: XML_Parser,
) -> c_int {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: null_mut::<*mut NAMED>(),
        end: null_mut::<*mut NAMED>(),
        table: null::<HASH_TABLE>(),
        index: 0 as size_t,
    };
    hashTableIterInit(&raw mut iter, &raw const (*oldDtd).prefixes);
    loop {
        let mut name: *const XML_Char = null::<XML_Char>();
        let mut oldP: *const PREFIX = hashTableIterNext(&raw mut iter) as *mut PREFIX;
        if oldP.is_null() {
            break;
        }
        name = poolCopyString(&raw mut (*newDtd).pool, (*oldP).name);
        if name.is_null() {
            return 0i32;
        }
        if lookup(
            oldParser,
            &raw mut (*newDtd).prefixes,
            name,
            size_of::<PREFIX>(),
        )
        .is_null()
        {
            return 0i32;
        }
    }
    hashTableIterInit(&raw mut iter, &raw const (*oldDtd).attributeIds);
    loop {
        let mut newA: *mut ATTRIBUTE_ID = null_mut::<ATTRIBUTE_ID>();
        let mut name_0: *const XML_Char = null::<XML_Char>();
        let mut oldA: *const ATTRIBUTE_ID = hashTableIterNext(&raw mut iter) as *mut ATTRIBUTE_ID;
        if oldA.is_null() {
            break;
        }
        if if (*newDtd).pool.ptr == (*newDtd).pool.end as *mut XML_Char
            && poolGrow(&raw mut (*newDtd).pool) == 0
        {
            0
        } else {
            let fresh81 = (*newDtd).pool.ptr;
            (*newDtd).pool.ptr = (*newDtd).pool.ptr.offset(1);
            *fresh81 = '\0' as XML_Char;
            1
        } == 0
        {
            return 0i32;
        }
        name_0 = poolCopyString(&raw mut (*newDtd).pool, (*oldA).name);
        if name_0.is_null() {
            return 0i32;
        }
        name_0 = name_0.offset(1);
        newA = lookup(
            oldParser,
            &raw mut (*newDtd).attributeIds,
            name_0,
            size_of::<ATTRIBUTE_ID>(),
        ) as *mut ATTRIBUTE_ID;
        if newA.is_null() {
            return 0i32;
        }
        (*newA).maybeTokenized = (*oldA).maybeTokenized;
        if !(*oldA).prefix.is_null() {
            (*newA).xmlns = (*oldA).xmlns;
            if (*oldA).prefix == &raw const (*oldDtd).defaultPrefix as *mut PREFIX {
                (*newA).prefix = &raw mut (*newDtd).defaultPrefix;
            } else {
                (*newA).prefix = lookup(
                    oldParser,
                    &raw mut (*newDtd).prefixes,
                    (*(*oldA).prefix).name,
                    0usize,
                ) as *mut PREFIX;
            }
        }
    }
    hashTableIterInit(&raw mut iter, &raw const (*oldDtd).elementTypes);
    loop {
        let mut i: c_int = 0;
        let mut newE: *mut ELEMENT_TYPE = null_mut::<ELEMENT_TYPE>();
        let mut name_1: *const XML_Char = null::<XML_Char>();
        let mut oldE: *const ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if oldE.is_null() {
            break;
        }
        name_1 = poolCopyString(&raw mut (*newDtd).pool, (*oldE).name);
        if name_1.is_null() {
            return 0i32;
        }
        newE = lookup(
            oldParser,
            &raw mut (*newDtd).elementTypes,
            name_1,
            size_of::<ELEMENT_TYPE>(),
        ) as *mut ELEMENT_TYPE;
        if newE.is_null() {
            return 0i32;
        }
        if (*oldE).nDefaultAtts != 0 {
            (*newE).defaultAtts = expat_malloc(
                parser,
                ((*oldE).nDefaultAtts as size_t).wrapping_mul(size_of::<DEFAULT_ATTRIBUTE>()),
                7683,
            ) as *mut DEFAULT_ATTRIBUTE;
            if (*newE).defaultAtts.is_null() {
                return 0i32;
            }
        }
        if !(*oldE).idAtt.is_null() {
            (*newE).idAtt = lookup(
                oldParser,
                &raw mut (*newDtd).attributeIds,
                (*(*oldE).idAtt).name as KEY,
                0usize,
            ) as *mut ATTRIBUTE_ID;
        }
        (*newE).nDefaultAtts = (*oldE).nDefaultAtts;
        (*newE).allocDefaultAtts = (*newE).nDefaultAtts;
        if !(*oldE).prefix.is_null() {
            (*newE).prefix = lookup(
                oldParser,
                &raw mut (*newDtd).prefixes,
                (*(*oldE).prefix).name,
                0usize,
            ) as *mut PREFIX;
        }
        i = 0;
        while i < (*newE).nDefaultAtts {
            let ref mut fresh82 = (*(*newE).defaultAtts.offset(i as isize)).id;
            *fresh82 = lookup(
                oldParser,
                &raw mut (*newDtd).attributeIds,
                (*(*(*oldE).defaultAtts.offset(i as isize)).id).name as KEY,
                0,
            ) as *mut ATTRIBUTE_ID;
            (*(*newE).defaultAtts.offset(i as isize)).isCdata =
                (*(*oldE).defaultAtts.offset(i as isize)).isCdata;
            if !(*(*oldE).defaultAtts.offset(i as isize)).value.is_null() {
                let ref mut fresh83 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *fresh83 = poolCopyString(
                    &raw mut (*newDtd).pool,
                    (*(*oldE).defaultAtts.offset(i as isize)).value,
                );
                if (*(*newE).defaultAtts.offset(i as isize)).value.is_null() {
                    return 0i32;
                }
            } else {
                let ref mut fresh84 = (*(*newE).defaultAtts.offset(i as isize)).value;
                *fresh84 = null::<XML_Char>();
            }
            i += 1;
        }
    }
    if copyEntityTable(
        oldParser,
        &raw mut (*newDtd).generalEntities,
        &raw mut (*newDtd).pool,
        &raw const (*oldDtd).generalEntities,
    ) == 0
    {
        return 0i32;
    }
    if copyEntityTable(
        oldParser,
        &raw mut (*newDtd).paramEntities,
        &raw mut (*newDtd).pool,
        &raw const (*oldDtd).paramEntities,
    ) == 0
    {
        return 0i32;
    }
    (*newDtd).paramEntityRead = (*oldDtd).paramEntityRead;
    (*newDtd).keepProcessing = (*oldDtd).keepProcessing;
    (*newDtd).hasParamEntityRefs = (*oldDtd).hasParamEntityRefs;
    (*newDtd).standalone = (*oldDtd).standalone;
    (*newDtd).in_eldecl = (*oldDtd).in_eldecl;
    (*newDtd).scaffold = (*oldDtd).scaffold;
    (*newDtd).contentStringLen = (*oldDtd).contentStringLen;
    (*newDtd).scaffSize = (*oldDtd).scaffSize;
    (*newDtd).scaffLevel = (*oldDtd).scaffLevel;
    (*newDtd).scaffIndex = (*oldDtd).scaffIndex;
    return 1;
}

unsafe extern "C" fn copyEntityTable(
    mut oldParser: XML_Parser,
    mut newTable: *mut HASH_TABLE,
    mut newPool: *mut STRING_POOL,
    mut oldTable: *const HASH_TABLE,
) -> c_int {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: null_mut::<*mut NAMED>(),
        end: null_mut::<*mut NAMED>(),
        table: null::<HASH_TABLE>(),
        index: 0 as size_t,
    };
    let mut cachedOldBase: *const XML_Char = null::<XML_Char>();
    let mut cachedNewBase: *const XML_Char = null::<XML_Char>();
    hashTableIterInit(&raw mut iter, oldTable);
    loop {
        let mut newE: *mut ENTITY = null_mut::<ENTITY>();
        let mut name: *const XML_Char = null::<XML_Char>();
        let mut oldE: *const ENTITY = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if oldE.is_null() {
            break;
        }
        name = poolCopyString(newPool, (*oldE).name);
        if name.is_null() {
            return 0i32;
        }
        newE = lookup(oldParser, newTable, name, size_of::<ENTITY>()) as *mut ENTITY;
        if newE.is_null() {
            return 0i32;
        }
        if !(*oldE).systemId.is_null() {
            let mut tem: *const XML_Char = poolCopyString(newPool, (*oldE).systemId);
            if tem.is_null() {
                return 0i32;
            }
            (*newE).systemId = tem;
            if !(*oldE).base.is_null() {
                if (*oldE).base == cachedOldBase {
                    (*newE).base = cachedNewBase;
                } else {
                    cachedOldBase = (*oldE).base;
                    tem = poolCopyString(newPool, cachedOldBase);
                    if tem.is_null() {
                        return 0i32;
                    }
                    (*newE).base = tem;
                    cachedNewBase = (*newE).base;
                }
            }
            if !(*oldE).publicId.is_null() {
                tem = poolCopyString(newPool, (*oldE).publicId);
                if tem.is_null() {
                    return 0i32;
                }
                (*newE).publicId = tem;
            }
        } else {
            let mut tem_0: *const XML_Char =
                poolCopyStringN(newPool, (*oldE).textPtr, (*oldE).textLen);
            if tem_0.is_null() {
                return 0i32;
            }
            (*newE).textPtr = tem_0;
            (*newE).textLen = (*oldE).textLen;
        }
        if !(*oldE).notation.is_null() {
            let mut tem_1: *const XML_Char = poolCopyString(newPool, (*oldE).notation);
            if tem_1.is_null() {
                return 0i32;
            }
            (*newE).notation = tem_1;
        }
        (*newE).is_param = (*oldE).is_param;
        (*newE).is_internal = (*oldE).is_internal;
    }
    return 1;
}

pub const INIT_POWER: c_int = 6;

unsafe extern "C" fn keyeq(mut s1: KEY, mut s2: KEY) -> XML_Bool {
    while *s1 as c_int == *s2 as c_int {
        if *s1 as c_int == 0 {
            return XML_TRUE;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    }
    return XML_FALSE;
}

unsafe extern "C" fn keylen(mut s: KEY) -> size_t {
    let mut len: size_t = 0;
    while *s != 0 {
        s = s.offset(1);
        len = len.wrapping_add(1);
    }
    return len;
}

unsafe extern "C" fn copy_salt_to_sipkey(mut parser: XML_Parser, mut key: *mut sipkey) {
    (*key).k[0] = 0u64;
    (*key).k[1] = get_hash_secret_salt(parser);
}

unsafe extern "C" fn hash(mut parser: XML_Parser, mut s: KEY) -> c_ulong {
    let mut state: siphash = siphash {
        v0: 0,
        v1: 0,
        v2: 0,
        v3: 0,
        buf: [0; 8],
        p: null_mut::<c_uchar>(),
        c: 0,
    };
    let mut key: sipkey = sipkey { k: [0; 2] };
    copy_salt_to_sipkey(parser, &raw mut key);
    sip24_init(&raw mut state, &raw mut key);
    sip24_update(
        &raw mut state,
        s as *const c_void,
        keylen(s).wrapping_mul(size_of::<XML_Char>()),
    );
    return sip24_final(&raw mut state);
}

unsafe extern "C" fn lookup(
    mut parser: XML_Parser,
    mut table: *mut HASH_TABLE,
    mut name: KEY,
    mut createSize: size_t,
) -> *mut NAMED {
    let _ = parser;
    let mut key: Vec<XML_Char> = Vec::new();
    let mut key_cursor: KEY = name;
    while *key_cursor != 0 {
        key.push(*key_cursor);
        key_cursor = key_cursor.offset(1);
    }
    if let Some(&entry) = (&(*table).entries).get(&key) {
        return entry;
    }
    if createSize == 0 {
        return null_mut::<NAMED>();
    }
    let table_parser = (*table).parser;
    let mut entry: *mut NAMED = expat_malloc(table_parser, createSize, 7914 as c_int) as *mut NAMED;
    if entry.is_null() {
        return null_mut::<NAMED>();
    }
    memset(entry as *mut c_void, 0 as c_int, createSize);
    (*entry).name = name;
    (&mut (*table).entries).insert(key, entry);
    return entry;
}

unsafe extern "C" fn hashTableClear(mut table: *mut HASH_TABLE) {
    let parser = (*table).parser;
    for (_, entry) in (&mut (*table).entries).drain() {
        expat_free(parser, entry as *mut c_void, 7927 as c_int);
    }
}

unsafe extern "C" fn hashTableDestroy(mut table: *mut HASH_TABLE) {
    hashTableClear(table);
    ::core::ptr::drop_in_place(table);
}

unsafe extern "C" fn hashTableInit(mut p: *mut HASH_TABLE, mut parser: XML_Parser) {
    ::core::ptr::write(
        p,
        HASH_TABLE {
            entries: std::collections::HashMap::new(),
            parser,
        },
    );
}

unsafe extern "C" fn hashTableIterInit(
    mut iter: *mut HASH_TABLE_ITER,
    mut table: *const HASH_TABLE,
) {
    (*iter).p = null_mut::<*mut NAMED>();
    (*iter).end = null_mut::<*mut NAMED>();
    (*iter).table = table;
    (*iter).index = 0 as size_t;
}

unsafe extern "C" fn hashTableIterNext(mut iter: *mut HASH_TABLE_ITER) -> *mut NAMED {
    if (*iter).table.is_null() {
        return null_mut::<NAMED>();
    }
    let mut table: &HASH_TABLE = &*(*iter).table;
    if (*iter).index >= table.entries.len() {
        return null_mut::<NAMED>();
    }
    let current_index: size_t = (*iter).index;
    (*iter).index = (*iter).index.wrapping_add(1);
    if let Some(&entry) = table.entries.values().nth(current_index) {
        return entry;
    }
    return null_mut::<NAMED>();
}

unsafe extern "C" fn poolInit(mut pool: *mut STRING_POOL, mut parser: XML_Parser) {
    (*pool).blocks = null_mut::<BLOCK>();
    (*pool).freeBlocks = null_mut::<BLOCK>();
    (*pool).start = null_mut::<XML_Char>();
    (*pool).ptr = null_mut::<XML_Char>();
    (*pool).end = null::<XML_Char>();
    (*pool).parser = parser;
}

unsafe extern "C" fn poolClear(mut pool: *mut STRING_POOL) {
    if (*pool).freeBlocks.is_null() {
        (*pool).freeBlocks = (*pool).blocks;
    } else {
        let mut p: *mut BLOCK = (*pool).blocks;
        while !p.is_null() {
            let mut tem: *mut BLOCK = (*p).next;
            (*p).next = (*pool).freeBlocks;
            (*pool).freeBlocks = p;
            p = tem;
        }
    }
    (*pool).blocks = null_mut::<BLOCK>();
    (*pool).start = null_mut::<XML_Char>();
    (*pool).ptr = null_mut::<XML_Char>();
    (*pool).end = null::<XML_Char>();
}

unsafe extern "C" fn poolDestroy(mut pool: *mut STRING_POOL) {
    let mut p: *mut BLOCK = (*pool).blocks;
    while !p.is_null() {
        let mut tem: *mut BLOCK = (*p).next;
        expat_free((*pool).parser, p as *mut c_void, 8000);
        p = tem;
    }
    p = (*pool).freeBlocks;
    while !p.is_null() {
        let mut tem_0: *mut BLOCK = (*p).next;
        expat_free((*pool).parser, p as *mut c_void, 8006);
        p = tem_0;
    }
}

unsafe extern "C" fn poolAppend(
    mut pool: *mut STRING_POOL,
    mut enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
) -> *mut XML_Char {
    if (*pool).ptr.is_null() && poolGrow(pool) == 0 {
        return null_mut::<XML_Char>();
    }
    loop {
        let convert_res: XML_Convert_Result =
            (*enc).utf8Convert.expect("non-null function pointer")(
                enc,
                &raw mut ptr,
                end,
                &raw mut (*pool).ptr,
                (*pool).end,
            );
        if convert_res == XML_CONVERT_COMPLETED || convert_res == XML_CONVERT_INPUT_INCOMPLETE {
            break;
        }
        if poolGrow(pool) == 0 {
            return null_mut::<XML_Char>();
        }
    }
    return (*pool).start;
}

unsafe extern "C" fn poolCopyString(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
) -> *const XML_Char {
    loop {
        if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
            0
        } else {
            let fresh59 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh59 = *s;
            1
        } == 0
        {
            return null::<XML_Char>();
        }
        let fresh60 = s;
        s = s.offset(1);
        if !(*fresh60 != 0) {
            break;
        }
    }
    s = (*pool).start;
    (*pool).start = (*pool).ptr;
    return s;
}

unsafe extern "C" fn poolCopyStringN(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
    mut n: c_int,
) -> *const XML_Char {
    if (*pool).ptr.is_null() && poolGrow(pool) == 0 {
        return null::<XML_Char>();
    }
    while n > 0 {
        if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
            0
        } else {
            let fresh85 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh85 = *s;
            1
        } == 0
        {
            return null::<XML_Char>();
        }
        n -= 1;
        s = s.offset(1);
    }
    s = (*pool).start;
    (*pool).start = (*pool).ptr;
    return s;
}

unsafe extern "C" fn poolAppendString(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
) -> *const XML_Char {
    while *s != 0 {
        if if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
            0
        } else {
            let fresh74 = (*pool).ptr;
            (*pool).ptr = (*pool).ptr.offset(1);
            *fresh74 = *s;
            1
        } == 0
        {
            return null::<XML_Char>();
        }
        s = s.offset(1);
    }
    return (*pool).start;
}

unsafe extern "C" fn poolStoreString(
    mut pool: *mut STRING_POOL,
    mut enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
) -> *mut XML_Char {
    if poolAppend(pool, enc, ptr, end).is_null() {
        return null_mut::<XML_Char>();
    }
    if (*pool).ptr == (*pool).end as *mut XML_Char && poolGrow(pool) == 0 {
        return null_mut::<XML_Char>();
    }
    let fresh10 = (*pool).ptr;
    (*pool).ptr = (*pool).ptr.offset(1);
    *fresh10 = 0i8;
    return (*pool).start;
}

unsafe extern "C" fn poolBytesToAllocateFor(mut blockSize: c_int) -> size_t {
    let stretch: size_t = size_of::<XML_Char>();
    if blockSize <= 0 {
        return 0usize;
    }
    if blockSize > (INT_MAX as size_t).wrapping_div(stretch) as c_int {
        return 0usize;
    }
    let stretchedBlockSize: c_int = blockSize * stretch as c_int;
    let bytesToAllocate: c_int =
        (12u64).wrapping_add(stretchedBlockSize as c_uint as c_ulong) as c_int;
    if bytesToAllocate < 0 {
        return 0usize;
    }
    return bytesToAllocate as size_t;
}

unsafe extern "C" fn poolGrow(mut pool: *mut STRING_POOL) -> XML_Bool {
    if !(*pool).freeBlocks.is_null() {
        if (*pool).start.is_null() {
            (*pool).blocks = (*pool).freeBlocks;
            (*pool).freeBlocks = (*(*pool).freeBlocks).next;
            (*(*pool).blocks).next = null_mut::<block>();
            (*pool).start = &raw mut (*(*pool).blocks).s as *mut XML_Char;
            (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize);
            (*pool).ptr = (*pool).start;
            return XML_TRUE;
        }
        if ((*pool).end.offset_from((*pool).start) as c_long) < (*(*pool).freeBlocks).size as c_long
        {
            let mut tem: *mut BLOCK = (*(*pool).freeBlocks).next;
            (*(*pool).freeBlocks).next = (*pool).blocks;
            (*pool).blocks = (*pool).freeBlocks;
            (*pool).freeBlocks = tem;
            memcpy(
                &raw mut (*(*pool).blocks).s as *mut c_void,
                (*pool).start as *const c_void,
                ((*pool).end.offset_from((*pool).start) as size_t)
                    .wrapping_mul(size_of::<XML_Char>()),
            );
            (*pool).ptr = (&raw mut (*(*pool).blocks).s as *mut XML_Char)
                .offset((*pool).ptr.offset_from((*pool).start));
            (*pool).start = &raw mut (*(*pool).blocks).s as *mut XML_Char;
            (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize);
            return XML_TRUE;
        }
    }
    if !(*pool).blocks.is_null() && (*pool).start == &raw mut (*(*pool).blocks).s as *mut XML_Char {
        let mut temp: *mut BLOCK = null_mut::<BLOCK>();
        let mut blockSize: c_int =
            ((*pool).end.offset_from((*pool).start) as c_uint).wrapping_mul(2u32) as c_int;
        let mut bytesToAllocate: size_t = 0;
        let offsetInsideBlock: ptrdiff_t = (*pool).ptr.offset_from((*pool).start);
        if blockSize < 0 {
            return XML_FALSE;
        }
        bytesToAllocate = poolBytesToAllocateFor(blockSize);
        if bytesToAllocate == 0 {
            return XML_FALSE;
        }
        temp = expat_realloc(
            (*pool).parser,
            (*pool).blocks as *mut c_void,
            bytesToAllocate,
            8161,
        ) as *mut BLOCK;
        if temp.is_null() {
            return XML_FALSE;
        }
        (*pool).blocks = temp;
        (*(*pool).blocks).size = blockSize;
        (*pool).ptr = (&raw mut (*(*pool).blocks).s as *mut XML_Char).offset(offsetInsideBlock);
        (*pool).start = &raw mut (*(*pool).blocks).s as *mut XML_Char;
        (*pool).end = (*pool).start.offset(blockSize as isize);
    } else {
        let mut tem_0: *mut BLOCK = null_mut::<BLOCK>();
        let mut blockSize_0: c_int = (*pool).end.offset_from((*pool).start) as c_int;
        let mut bytesToAllocate_0: size_t = 0;
        if blockSize_0 < 0 {
            return XML_FALSE;
        }
        if blockSize_0 < INIT_BLOCK_SIZE {
            blockSize_0 = INIT_BLOCK_SIZE;
        } else {
            if ((blockSize_0 as c_uint).wrapping_mul(2u32) as c_int) < 0 {
                return XML_FALSE;
            }
            blockSize_0 *= 2i32;
        }
        bytesToAllocate_0 = poolBytesToAllocateFor(blockSize_0);
        if bytesToAllocate_0 == 0 {
            return XML_FALSE;
        }
        tem_0 = expat_malloc((*pool).parser, bytesToAllocate_0, 8201) as *mut BLOCK;
        if tem_0.is_null() {
            return XML_FALSE;
        }
        (*tem_0).size = blockSize_0;
        (*tem_0).next = (*pool).blocks;
        (*pool).blocks = tem_0;
        if (*pool).ptr != (*pool).start {
            memcpy(
                &raw mut (*tem_0).s as *mut c_void,
                (*pool).start as *const c_void,
                ((*pool).ptr.offset_from((*pool).start) as size_t)
                    .wrapping_mul(size_of::<XML_Char>()),
            );
        }
        (*pool).ptr =
            (&raw mut (*tem_0).s as *mut XML_Char).offset((*pool).ptr.offset_from((*pool).start));
        (*pool).start = &raw mut (*tem_0).s as *mut XML_Char;
        (*pool).end = (&raw mut (*tem_0).s as *mut XML_Char).offset(blockSize_0 as isize);
    }
    return XML_TRUE;
}

unsafe extern "C" fn nextScaffoldPart(mut parser: XML_Parser) -> c_int {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut me: *mut CONTENT_SCAFFOLD = null_mut::<CONTENT_SCAFFOLD>();
    let mut next: c_int = 0;
    if (*dtd).scaffIndex.is_null() {
        (*dtd).scaffIndex = expat_malloc(
            parser,
            ((*parser).m_groupSize as size_t).wrapping_mul(size_of::<c_int>()),
            8232,
        ) as *mut c_int;
        if (*dtd).scaffIndex.is_null() {
            return -(1i32);
        }
        *(*dtd).scaffIndex.offset(0isize) = 0i32;
    }
    if (*dtd).scaffCount > INT_MAX as c_uint {
        return -(1i32);
    }
    if (*dtd).scaffCount >= (*dtd).scaffSize {
        let mut temp: *mut CONTENT_SCAFFOLD = null_mut::<CONTENT_SCAFFOLD>();
        if !(*dtd).scaffold.is_null() {
            if (*dtd).scaffSize > UINT_MAX.wrapping_div(2u32) {
                return -(1i32);
            }
            temp = expat_realloc(
                parser,
                (*dtd).scaffold as *mut c_void,
                ((*dtd).scaffSize.wrapping_mul(2u32) as size_t)
                    .wrapping_mul(size_of::<CONTENT_SCAFFOLD>()),
                8261,
            ) as *mut CONTENT_SCAFFOLD;
            if temp.is_null() {
                return -(1i32);
            }
            (*dtd).scaffSize = (*dtd).scaffSize.wrapping_mul(2u32);
        } else {
            temp = expat_malloc(
                parser,
                (32usize).wrapping_mul(size_of::<CONTENT_SCAFFOLD>()),
                8266,
            ) as *mut CONTENT_SCAFFOLD;
            if temp.is_null() {
                return -(1i32);
            }
            (*dtd).scaffSize = INIT_SCAFFOLD_ELEMENTS as c_uint;
        }
        (*dtd).scaffold = temp;
    }
    let fresh14 = (*dtd).scaffCount;
    (*dtd).scaffCount = (*dtd).scaffCount.wrapping_add(1);
    next = fresh14 as c_int;
    me = (*dtd).scaffold.offset(next as isize);
    if (*dtd).scaffLevel != 0 {
        let mut parent: *mut CONTENT_SCAFFOLD = (*dtd)
            .scaffold
            .offset(*(*dtd).scaffIndex.offset(((*dtd).scaffLevel - 1) as isize) as isize);
        if (*parent).lastchild != 0 {
            (*(*dtd).scaffold.offset((*parent).lastchild as isize)).nextsib = next;
        }
        if (*parent).childcnt == 0 {
            (*parent).firstchild = next;
        }
        (*parent).lastchild = next;
        (*parent).childcnt += 1;
    }
    (*me).nextsib = 0;
    (*me).childcnt = (*me).nextsib;
    (*me).lastchild = (*me).childcnt;
    (*me).firstchild = (*me).lastchild;
    return next;
}

unsafe extern "C" fn build_model(mut parser: XML_Parser) -> *mut XML_Content {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut ret: *mut XML_Content = null_mut::<XML_Content>();
    let mut str: *mut XML_Char = null_mut::<XML_Char>();
    if ((*dtd).scaffCount as usize).wrapping_mul(size_of::<XML_Content>())
        > (SIZE_MAX as usize)
            .wrapping_sub(((*dtd).contentStringLen as usize).wrapping_mul(size_of::<XML_Char>()))
    {
        return null_mut::<XML_Content>();
    }
    let allocsize: size_t = ((*dtd).scaffCount as size_t)
        .wrapping_mul(size_of::<XML_Content>())
        .wrapping_add(((*dtd).contentStringLen as size_t).wrapping_mul(size_of::<XML_Char>()));
    ret = (*parser)
        .m_mem
        .malloc_fcn
        .expect("non-null function pointer")(allocsize) as *mut XML_Content;
    if ret.is_null() {
        return null_mut::<XML_Content>();
    }
    let mut dest: *mut XML_Content = ret;
    let destLimit: *mut XML_Content = ret.offset((*dtd).scaffCount as isize);
    let mut jobDest: *mut XML_Content = ret;
    str = ret.offset((*dtd).scaffCount as isize) as *mut XML_Char;
    let fresh11 = jobDest;
    jobDest = jobDest.offset(1);
    (*fresh11).numchildren = 0u32;
    while dest < destLimit {
        let src_node: c_int = (*dest).numchildren as c_int;
        (*dest).type_0 = (*(*dtd).scaffold.offset(src_node as isize)).type_0;
        (*dest).quant = (*(*dtd).scaffold.offset(src_node as isize)).quant;
        if (*dest).type_0 == XML_CTYPE_NAME {
            let mut src: *const XML_Char = null::<XML_Char>();
            (*dest).name = str;
            src = (*(*dtd).scaffold.offset(src_node as isize)).name;
            loop {
                let fresh12 = str;
                str = str.offset(1);
                *fresh12 = *src;
                if *src == 0 {
                    break;
                }
                src = src.offset(1);
            }
            (*dest).numchildren = 0;
            (*dest).children = null_mut::<XML_Content>();
        } else {
            let mut i: c_uint = 0;
            let mut cn: c_int = 0;
            (*dest).name = null_mut::<XML_Char>();
            (*dest).numchildren = (*(*dtd).scaffold.offset(src_node as isize)).childcnt as c_uint;
            (*dest).children = jobDest;
            i = 0;
            cn = (*(*dtd).scaffold.offset(src_node as isize)).firstchild;
            while i < (*dest).numchildren {
                let fresh13 = jobDest;
                jobDest = jobDest.offset(1);
                (*fresh13).numchildren = cn as c_uint;
                i = i.wrapping_add(1);
                cn = (*(*dtd).scaffold.offset(cn as isize)).nextsib;
            }
        }
        dest = dest.offset(1);
    }
    return ret;
}

unsafe extern "C" fn getElementType(
    mut parser: XML_Parser,
    mut enc: *const ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
) -> *mut ELEMENT_TYPE {
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut name: *const XML_Char = poolStoreString(&raw mut (*dtd).pool, enc, ptr, end);
    let mut ret: *mut ELEMENT_TYPE = null_mut::<ELEMENT_TYPE>();
    if name.is_null() {
        return null_mut::<ELEMENT_TYPE>();
    }
    ret = lookup(
        parser,
        &raw mut (*dtd).elementTypes,
        name,
        size_of::<ELEMENT_TYPE>(),
    ) as *mut ELEMENT_TYPE;
    if ret.is_null() {
        return null_mut::<ELEMENT_TYPE>();
    }
    if (*ret).name != name {
        (*dtd).pool.ptr = (*dtd).pool.start;
    } else {
        (*dtd).pool.start = (*dtd).pool.ptr;
        if setElementTypePrefix(parser, ret) == 0 {
            return null_mut::<ELEMENT_TYPE>();
        }
    }
    return ret;
}

unsafe extern "C" fn copyString(mut s: *const XML_Char, mut parser: XML_Parser) -> *mut XML_Char {
    let mut charsRequired: size_t = 0;
    let mut result: *mut XML_Char = null_mut::<XML_Char>();
    while *s.offset(charsRequired as isize) as c_int != 0 {
        charsRequired = charsRequired.wrapping_add(1);
    }
    charsRequired = charsRequired.wrapping_add(1);
    result = expat_malloc(
        parser,
        charsRequired.wrapping_mul(size_of::<XML_Char>()),
        8456,
    ) as *mut XML_Char;
    if result.is_null() {
        return null_mut::<XML_Char>();
    }
    memcpy(
        result as *mut c_void,
        s as *const c_void,
        charsRequired.wrapping_mul(size_of::<XML_Char>()),
    );
    return result;
}

unsafe extern "C" fn accountingGetCurrentAmplification(mut rootParser: XML_Parser) -> c_float {
    let lenOfShortestInclude: size_t = (size_of::<[c_char; 23]>()).wrapping_sub(1usize);
    let countBytesOutput: XmlBigCount = (*rootParser)
        .m_accounting
        .countBytesDirect
        .wrapping_add((*rootParser).m_accounting.countBytesIndirect);
    let amplificationFactor: c_float = if (*rootParser).m_accounting.countBytesDirect != 0 {
        countBytesOutput as c_float / (*rootParser).m_accounting.countBytesDirect as c_float
    } else {
        (lenOfShortestInclude as XmlBigCount)
            .wrapping_add((*rootParser).m_accounting.countBytesIndirect) as c_float
            / lenOfShortestInclude as c_float
    };
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    return amplificationFactor;
}

unsafe extern "C" fn accountingReportStats(
    mut originParser: XML_Parser,
    mut epilog: *const c_char,
) {
    let rootParser: XML_Parser = getRootParserOf(originParser, null_mut::<c_uint>());
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    if (*rootParser).m_accounting.debugLevel == 0 {
        return;
    }
    let amplificationFactor: c_float = accountingGetCurrentAmplification(rootParser);
    fprintf(
        stderr,
        b"expat: Accounting(%p): Direct %10llu, indirect %10llu, amplification %8.2f%s\0"
            as *const u8 as *const c_char,
        rootParser as *mut c_void,
        (*rootParser).m_accounting.countBytesDirect,
        (*rootParser).m_accounting.countBytesIndirect,
        amplificationFactor as ::core::ffi::c_double,
        epilog,
    );
}

unsafe extern "C" fn accountingOnAbort(mut originParser: XML_Parser) {
    accountingReportStats(originParser, b" ABORTING\n\0" as *const u8 as *const c_char);
}

unsafe extern "C" fn accountingReportDiff(
    mut rootParser: XML_Parser,
    mut levelsAwayFromRootParser: c_uint,
    mut before: *const c_char,
    mut after: *const c_char,
    mut bytesMore: ptrdiff_t,
    mut source_line: c_int,
    mut account: XML_Account,
) {
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    fprintf(
        stderr,
        b" (+%6ld bytes %s|%u, xmlparse.c:%d) %*s\"\0" as *const u8 as *const c_char,
        bytesMore,
        if account == XML_ACCOUNT_DIRECT {
            b"DIR\0" as *const u8 as *const c_char
        } else {
            b"EXP\0" as *const u8 as *const c_char
        },
        levelsAwayFromRootParser,
        source_line,
        10i32,
        b"\0" as *const u8 as *const c_char,
    );
    let ellipis: [c_char; 5] = ::core::mem::transmute::<[u8; 5], [c_char; 5]>(*b"[..]\0");
    let ellipsisLength: size_t = (size_of::<[c_char; 5]>()).wrapping_sub(1usize);
    let contextLength: c_uint = 10;
    let mut walker: *const c_char = before;
    if (*rootParser).m_accounting.debugLevel >= 3u64
        || after.offset_from(before)
            <= (contextLength as size_t)
                .wrapping_add(ellipsisLength)
                .wrapping_add(contextLength as size_t) as ptrdiff_t
    {
        while walker < after {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const c_char,
                unsignedCharToPrintable(*walker.offset(0isize) as c_uchar),
            );
            walker = walker.offset(1);
        }
    } else {
        while walker < before.offset(contextLength as isize) {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const c_char,
                unsignedCharToPrintable(*walker.offset(0isize) as c_uchar),
            );
            walker = walker.offset(1);
        }
        fprintf(stderr, &raw const ellipis as *const c_char);
        walker = after.offset(-(contextLength as isize));
        while walker < after {
            fprintf(
                stderr,
                b"%s\0" as *const u8 as *const c_char,
                unsignedCharToPrintable(*walker.offset(0isize) as c_uchar),
            );
            walker = walker.offset(1);
        }
    }
    fprintf(stderr, b"\"\n\0" as *const u8 as *const c_char);
}

unsafe extern "C" fn accountingDiffTolerated(
    mut originParser: XML_Parser,
    mut tok: c_int,
    mut before: *const c_char,
    mut after: *const c_char,
    mut source_line: c_int,
    mut account: XML_Account,
) -> XML_Bool {
    match tok {
        XML_TOK_INVALID | XML_TOK_PARTIAL | XML_TOK_PARTIAL_CHAR | XML_TOK_NONE => {
            return XML_TRUE;
        }
        _ => {}
    }
    if account == XML_ACCOUNT_NONE {
        return XML_TRUE;
    }
    let mut levelsAwayFromRootParser: c_uint = 0;
    let rootParser: XML_Parser = getRootParserOf(originParser, &raw mut levelsAwayFromRootParser);
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    let isDirect: c_int = (account == XML_ACCOUNT_DIRECT && originParser == rootParser) as c_int;
    let bytesMore: ptrdiff_t = after.offset_from(before);
    let additionTarget: *mut XmlBigCount = if isDirect != 0 {
        &raw mut (*rootParser).m_accounting.countBytesDirect
    } else {
        &raw mut (*rootParser).m_accounting.countBytesIndirect
    };
    if *additionTarget > (-(1i32) as XmlBigCount).wrapping_sub(bytesMore as XmlBigCount) {
        return XML_FALSE;
    }
    *additionTarget = (*additionTarget).wrapping_add(bytesMore as XmlBigCount);
    let countBytesOutput: XmlBigCount = (*rootParser)
        .m_accounting
        .countBytesDirect
        .wrapping_add((*rootParser).m_accounting.countBytesIndirect);
    let amplificationFactor: c_float = accountingGetCurrentAmplification(rootParser);
    let tolerated: XML_Bool = (countBytesOutput
        < (*rootParser).m_accounting.activationThresholdBytes
        || amplificationFactor <= (*rootParser).m_accounting.maximumAmplificationFactor)
        as XML_Bool;
    if (*rootParser).m_accounting.debugLevel >= 2 {
        accountingReportStats(rootParser, b"\0" as *const u8 as *const c_char);
        accountingReportDiff(
            rootParser,
            levelsAwayFromRootParser,
            before,
            after,
            bytesMore,
            source_line,
            account,
        );
    }
    return tolerated;
}
#[no_mangle]

pub unsafe extern "C" fn testingAccountingGetCountBytesDirect(
    mut parser: XML_Parser,
) -> c_ulonglong {
    if parser.is_null() {
        return 0u64;
    }
    return (*parser).m_accounting.countBytesDirect;
}
#[no_mangle]

pub unsafe extern "C" fn testingAccountingGetCountBytesIndirect(
    mut parser: XML_Parser,
) -> c_ulonglong {
    if parser.is_null() {
        return 0u64;
    }
    return (*parser).m_accounting.countBytesIndirect;
}

unsafe extern "C" fn entityTrackingReportStats(
    mut rootParser: XML_Parser,
    mut entity: *mut ENTITY,
    mut action: *const c_char,
    mut sourceLine: c_int,
) {
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    if (*rootParser).m_entity_stats.debugLevel == 0u64 {
        return;
    }
    let entityName: *const c_char = (*entity).name;
    fprintf(
        stderr,
        b"expat: Entities(%p): Count %9u, depth %2u/%2u %*s%s%s; %s length %d (xmlparse.c:%d)\n\0"
            as *const u8 as *const c_char,
        rootParser as *mut c_void,
        (*rootParser).m_entity_stats.countEverOpened,
        (*rootParser).m_entity_stats.currentDepth,
        (*rootParser).m_entity_stats.maximumDepthSeen,
        ((*rootParser).m_entity_stats.currentDepth as c_int - 1i32) * 2i32,
        b"\0" as *const u8 as *const c_char,
        if (*entity).is_param as c_int != 0 {
            b"%\0" as *const u8 as *const c_char
        } else {
            b"&\0" as *const u8 as *const c_char
        },
        entityName,
        action,
        (*entity).textLen,
        sourceLine,
    );
}

unsafe extern "C" fn entityTrackingOnOpen(
    mut originParser: XML_Parser,
    mut entity: *mut ENTITY,
    mut sourceLine: c_int,
) {
    let rootParser: XML_Parser = getRootParserOf(originParser, null_mut::<c_uint>());
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    (*rootParser).m_entity_stats.countEverOpened =
        (*rootParser).m_entity_stats.countEverOpened.wrapping_add(1);
    (*rootParser).m_entity_stats.currentDepth =
        (*rootParser).m_entity_stats.currentDepth.wrapping_add(1);
    if (*rootParser).m_entity_stats.currentDepth > (*rootParser).m_entity_stats.maximumDepthSeen {
        (*rootParser).m_entity_stats.maximumDepthSeen = (*rootParser)
            .m_entity_stats
            .maximumDepthSeen
            .wrapping_add(1);
    }
    entityTrackingReportStats(
        rootParser,
        entity,
        b"OPEN \0" as *const u8 as *const c_char,
        sourceLine,
    );
}

unsafe extern "C" fn entityTrackingOnClose(
    mut originParser: XML_Parser,
    mut entity: *mut ENTITY,
    mut sourceLine: c_int,
) {
    let rootParser: XML_Parser = getRootParserOf(originParser, null_mut::<c_uint>());
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    entityTrackingReportStats(
        rootParser,
        entity,
        b"CLOSE\0" as *const u8 as *const c_char,
        sourceLine,
    );
    (*rootParser).m_entity_stats.currentDepth =
        (*rootParser).m_entity_stats.currentDepth.wrapping_sub(1);
}

unsafe extern "C" fn getRootParserOf(
    mut parser: XML_Parser,
    mut outLevelDiff: *mut c_uint,
) -> XML_Parser {
    let mut rootParser: XML_Parser = parser;
    let mut stepsTakenUpwards: c_uint = 0;
    while !(*rootParser).m_parentParser.is_null() {
        rootParser = (*rootParser).m_parentParser;
        stepsTakenUpwards = stepsTakenUpwards.wrapping_add(1);
    }
    assert!(
        (*rootParser).m_parentParser.is_null(),
        "! rootParser->m_parentParser"
    );
    if !outLevelDiff.is_null() {
        *outLevelDiff = stepsTakenUpwards;
    }
    return rootParser;
}
#[no_mangle]

pub unsafe extern "C" fn unsignedCharToPrintable(mut c: c_uchar) -> *const c_char {
    match c as c_int {
        0 => return b"\\0\0" as *const u8 as *const c_char,
        1 => return b"\\x1\0" as *const u8 as *const c_char,
        2 => return b"\\x2\0" as *const u8 as *const c_char,
        3 => return b"\\x3\0" as *const u8 as *const c_char,
        4 => return b"\\x4\0" as *const u8 as *const c_char,
        5 => return b"\\x5\0" as *const u8 as *const c_char,
        6 => return b"\\x6\0" as *const u8 as *const c_char,
        7 => return b"\\x7\0" as *const u8 as *const c_char,
        8 => return b"\\x8\0" as *const u8 as *const c_char,
        9 => return b"\\t\0" as *const u8 as *const c_char,
        10 => return b"\\n\0" as *const u8 as *const c_char,
        11 => return b"\\xB\0" as *const u8 as *const c_char,
        12 => return b"\\xC\0" as *const u8 as *const c_char,
        13 => return b"\\r\0" as *const u8 as *const c_char,
        14 => return b"\\xE\0" as *const u8 as *const c_char,
        15 => return b"\\xF\0" as *const u8 as *const c_char,
        16 => return b"\\x10\0" as *const u8 as *const c_char,
        17 => return b"\\x11\0" as *const u8 as *const c_char,
        18 => return b"\\x12\0" as *const u8 as *const c_char,
        19 => return b"\\x13\0" as *const u8 as *const c_char,
        20 => return b"\\x14\0" as *const u8 as *const c_char,
        21 => return b"\\x15\0" as *const u8 as *const c_char,
        22 => return b"\\x16\0" as *const u8 as *const c_char,
        23 => return b"\\x17\0" as *const u8 as *const c_char,
        24 => return b"\\x18\0" as *const u8 as *const c_char,
        25 => return b"\\x19\0" as *const u8 as *const c_char,
        26 => return b"\\x1A\0" as *const u8 as *const c_char,
        27 => return b"\\x1B\0" as *const u8 as *const c_char,
        28 => return b"\\x1C\0" as *const u8 as *const c_char,
        29 => return b"\\x1D\0" as *const u8 as *const c_char,
        30 => return b"\\x1E\0" as *const u8 as *const c_char,
        31 => return b"\\x1F\0" as *const u8 as *const c_char,
        32 => return b" \0" as *const u8 as *const c_char,
        33 => return b"!\0" as *const u8 as *const c_char,
        34 => return b"\\\"\0" as *const u8 as *const c_char,
        35 => return b"#\0" as *const u8 as *const c_char,
        36 => return b"$\0" as *const u8 as *const c_char,
        37 => return b"%\0" as *const u8 as *const c_char,
        38 => return b"&\0" as *const u8 as *const c_char,
        39 => return b"'\0" as *const u8 as *const c_char,
        40 => return b"(\0" as *const u8 as *const c_char,
        41 => return b")\0" as *const u8 as *const c_char,
        42 => return b"*\0" as *const u8 as *const c_char,
        43 => return b"+\0" as *const u8 as *const c_char,
        44 => return b",\0" as *const u8 as *const c_char,
        45 => return b"-\0" as *const u8 as *const c_char,
        46 => return b".\0" as *const u8 as *const c_char,
        47 => return b"/\0" as *const u8 as *const c_char,
        48 => return b"0\0" as *const u8 as *const c_char,
        49 => return b"1\0" as *const u8 as *const c_char,
        50 => return b"2\0" as *const u8 as *const c_char,
        51 => return b"3\0" as *const u8 as *const c_char,
        52 => return b"4\0" as *const u8 as *const c_char,
        53 => return b"5\0" as *const u8 as *const c_char,
        54 => return b"6\0" as *const u8 as *const c_char,
        55 => return b"7\0" as *const u8 as *const c_char,
        56 => return b"8\0" as *const u8 as *const c_char,
        57 => return b"9\0" as *const u8 as *const c_char,
        58 => return b":\0" as *const u8 as *const c_char,
        59 => return b";\0" as *const u8 as *const c_char,
        60 => return b"<\0" as *const u8 as *const c_char,
        61 => return b"=\0" as *const u8 as *const c_char,
        62 => return b">\0" as *const u8 as *const c_char,
        63 => return b"?\0" as *const u8 as *const c_char,
        64 => return b"@\0" as *const u8 as *const c_char,
        65 => return b"A\0" as *const u8 as *const c_char,
        66 => return b"B\0" as *const u8 as *const c_char,
        67 => return b"C\0" as *const u8 as *const c_char,
        68 => return b"D\0" as *const u8 as *const c_char,
        69 => return b"E\0" as *const u8 as *const c_char,
        70 => return b"F\0" as *const u8 as *const c_char,
        71 => return b"G\0" as *const u8 as *const c_char,
        72 => return b"H\0" as *const u8 as *const c_char,
        73 => return b"I\0" as *const u8 as *const c_char,
        74 => return b"J\0" as *const u8 as *const c_char,
        75 => return b"K\0" as *const u8 as *const c_char,
        76 => return b"L\0" as *const u8 as *const c_char,
        77 => return b"M\0" as *const u8 as *const c_char,
        78 => return b"N\0" as *const u8 as *const c_char,
        79 => return b"O\0" as *const u8 as *const c_char,
        80 => return b"P\0" as *const u8 as *const c_char,
        81 => return b"Q\0" as *const u8 as *const c_char,
        82 => return b"R\0" as *const u8 as *const c_char,
        83 => return b"S\0" as *const u8 as *const c_char,
        84 => return b"T\0" as *const u8 as *const c_char,
        85 => return b"U\0" as *const u8 as *const c_char,
        86 => return b"V\0" as *const u8 as *const c_char,
        87 => return b"W\0" as *const u8 as *const c_char,
        88 => return b"X\0" as *const u8 as *const c_char,
        89 => return b"Y\0" as *const u8 as *const c_char,
        90 => return b"Z\0" as *const u8 as *const c_char,
        91 => return b"[\0" as *const u8 as *const c_char,
        92 => return b"\\\\\0" as *const u8 as *const c_char,
        93 => return b"]\0" as *const u8 as *const c_char,
        94 => return b"^\0" as *const u8 as *const c_char,
        95 => return b"_\0" as *const u8 as *const c_char,
        96 => return b"`\0" as *const u8 as *const c_char,
        97 => return b"a\0" as *const u8 as *const c_char,
        98 => return b"b\0" as *const u8 as *const c_char,
        99 => return b"c\0" as *const u8 as *const c_char,
        100 => return b"d\0" as *const u8 as *const c_char,
        101 => return b"e\0" as *const u8 as *const c_char,
        102 => return b"f\0" as *const u8 as *const c_char,
        103 => return b"g\0" as *const u8 as *const c_char,
        104 => return b"h\0" as *const u8 as *const c_char,
        105 => return b"i\0" as *const u8 as *const c_char,
        106 => return b"j\0" as *const u8 as *const c_char,
        107 => return b"k\0" as *const u8 as *const c_char,
        108 => return b"l\0" as *const u8 as *const c_char,
        109 => return b"m\0" as *const u8 as *const c_char,
        110 => return b"n\0" as *const u8 as *const c_char,
        111 => return b"o\0" as *const u8 as *const c_char,
        112 => return b"p\0" as *const u8 as *const c_char,
        113 => return b"q\0" as *const u8 as *const c_char,
        114 => return b"r\0" as *const u8 as *const c_char,
        115 => return b"s\0" as *const u8 as *const c_char,
        116 => return b"t\0" as *const u8 as *const c_char,
        117 => return b"u\0" as *const u8 as *const c_char,
        118 => return b"v\0" as *const u8 as *const c_char,
        119 => return b"w\0" as *const u8 as *const c_char,
        120 => return b"x\0" as *const u8 as *const c_char,
        121 => return b"y\0" as *const u8 as *const c_char,
        122 => return b"z\0" as *const u8 as *const c_char,
        123 => return b"{\0" as *const u8 as *const c_char,
        124 => return b"|\0" as *const u8 as *const c_char,
        125 => return b"}\0" as *const u8 as *const c_char,
        126 => return b"~\0" as *const u8 as *const c_char,
        127 => return b"\\x7F\0" as *const u8 as *const c_char,
        128 => return b"\\x80\0" as *const u8 as *const c_char,
        129 => return b"\\x81\0" as *const u8 as *const c_char,
        130 => return b"\\x82\0" as *const u8 as *const c_char,
        131 => return b"\\x83\0" as *const u8 as *const c_char,
        132 => return b"\\x84\0" as *const u8 as *const c_char,
        133 => return b"\\x85\0" as *const u8 as *const c_char,
        134 => return b"\\x86\0" as *const u8 as *const c_char,
        135 => return b"\\x87\0" as *const u8 as *const c_char,
        136 => return b"\\x88\0" as *const u8 as *const c_char,
        137 => return b"\\x89\0" as *const u8 as *const c_char,
        138 => return b"\\x8A\0" as *const u8 as *const c_char,
        139 => return b"\\x8B\0" as *const u8 as *const c_char,
        140 => return b"\\x8C\0" as *const u8 as *const c_char,
        141 => return b"\\x8D\0" as *const u8 as *const c_char,
        142 => return b"\\x8E\0" as *const u8 as *const c_char,
        143 => return b"\\x8F\0" as *const u8 as *const c_char,
        144 => return b"\\x90\0" as *const u8 as *const c_char,
        145 => return b"\\x91\0" as *const u8 as *const c_char,
        146 => return b"\\x92\0" as *const u8 as *const c_char,
        147 => return b"\\x93\0" as *const u8 as *const c_char,
        148 => return b"\\x94\0" as *const u8 as *const c_char,
        149 => return b"\\x95\0" as *const u8 as *const c_char,
        150 => return b"\\x96\0" as *const u8 as *const c_char,
        151 => return b"\\x97\0" as *const u8 as *const c_char,
        152 => return b"\\x98\0" as *const u8 as *const c_char,
        153 => return b"\\x99\0" as *const u8 as *const c_char,
        154 => return b"\\x9A\0" as *const u8 as *const c_char,
        155 => return b"\\x9B\0" as *const u8 as *const c_char,
        156 => return b"\\x9C\0" as *const u8 as *const c_char,
        157 => return b"\\x9D\0" as *const u8 as *const c_char,
        158 => return b"\\x9E\0" as *const u8 as *const c_char,
        159 => return b"\\x9F\0" as *const u8 as *const c_char,
        160 => return b"\\xA0\0" as *const u8 as *const c_char,
        161 => return b"\\xA1\0" as *const u8 as *const c_char,
        162 => return b"\\xA2\0" as *const u8 as *const c_char,
        163 => return b"\\xA3\0" as *const u8 as *const c_char,
        164 => return b"\\xA4\0" as *const u8 as *const c_char,
        165 => return b"\\xA5\0" as *const u8 as *const c_char,
        166 => return b"\\xA6\0" as *const u8 as *const c_char,
        167 => return b"\\xA7\0" as *const u8 as *const c_char,
        168 => return b"\\xA8\0" as *const u8 as *const c_char,
        169 => return b"\\xA9\0" as *const u8 as *const c_char,
        170 => return b"\\xAA\0" as *const u8 as *const c_char,
        171 => return b"\\xAB\0" as *const u8 as *const c_char,
        172 => return b"\\xAC\0" as *const u8 as *const c_char,
        173 => return b"\\xAD\0" as *const u8 as *const c_char,
        174 => return b"\\xAE\0" as *const u8 as *const c_char,
        175 => return b"\\xAF\0" as *const u8 as *const c_char,
        176 => return b"\\xB0\0" as *const u8 as *const c_char,
        177 => return b"\\xB1\0" as *const u8 as *const c_char,
        178 => return b"\\xB2\0" as *const u8 as *const c_char,
        179 => return b"\\xB3\0" as *const u8 as *const c_char,
        180 => return b"\\xB4\0" as *const u8 as *const c_char,
        181 => return b"\\xB5\0" as *const u8 as *const c_char,
        182 => return b"\\xB6\0" as *const u8 as *const c_char,
        183 => return b"\\xB7\0" as *const u8 as *const c_char,
        184 => return b"\\xB8\0" as *const u8 as *const c_char,
        185 => return b"\\xB9\0" as *const u8 as *const c_char,
        186 => return b"\\xBA\0" as *const u8 as *const c_char,
        187 => return b"\\xBB\0" as *const u8 as *const c_char,
        188 => return b"\\xBC\0" as *const u8 as *const c_char,
        189 => return b"\\xBD\0" as *const u8 as *const c_char,
        190 => return b"\\xBE\0" as *const u8 as *const c_char,
        191 => return b"\\xBF\0" as *const u8 as *const c_char,
        192 => return b"\\xC0\0" as *const u8 as *const c_char,
        193 => return b"\\xC1\0" as *const u8 as *const c_char,
        194 => return b"\\xC2\0" as *const u8 as *const c_char,
        195 => return b"\\xC3\0" as *const u8 as *const c_char,
        196 => return b"\\xC4\0" as *const u8 as *const c_char,
        197 => return b"\\xC5\0" as *const u8 as *const c_char,
        198 => return b"\\xC6\0" as *const u8 as *const c_char,
        199 => return b"\\xC7\0" as *const u8 as *const c_char,
        200 => return b"\\xC8\0" as *const u8 as *const c_char,
        201 => return b"\\xC9\0" as *const u8 as *const c_char,
        202 => return b"\\xCA\0" as *const u8 as *const c_char,
        203 => return b"\\xCB\0" as *const u8 as *const c_char,
        204 => return b"\\xCC\0" as *const u8 as *const c_char,
        205 => return b"\\xCD\0" as *const u8 as *const c_char,
        206 => return b"\\xCE\0" as *const u8 as *const c_char,
        207 => return b"\\xCF\0" as *const u8 as *const c_char,
        208 => return b"\\xD0\0" as *const u8 as *const c_char,
        209 => return b"\\xD1\0" as *const u8 as *const c_char,
        210 => return b"\\xD2\0" as *const u8 as *const c_char,
        211 => return b"\\xD3\0" as *const u8 as *const c_char,
        212 => return b"\\xD4\0" as *const u8 as *const c_char,
        213 => return b"\\xD5\0" as *const u8 as *const c_char,
        214 => return b"\\xD6\0" as *const u8 as *const c_char,
        215 => return b"\\xD7\0" as *const u8 as *const c_char,
        216 => return b"\\xD8\0" as *const u8 as *const c_char,
        217 => return b"\\xD9\0" as *const u8 as *const c_char,
        218 => return b"\\xDA\0" as *const u8 as *const c_char,
        219 => return b"\\xDB\0" as *const u8 as *const c_char,
        220 => return b"\\xDC\0" as *const u8 as *const c_char,
        221 => return b"\\xDD\0" as *const u8 as *const c_char,
        222 => return b"\\xDE\0" as *const u8 as *const c_char,
        223 => return b"\\xDF\0" as *const u8 as *const c_char,
        224 => return b"\\xE0\0" as *const u8 as *const c_char,
        225 => return b"\\xE1\0" as *const u8 as *const c_char,
        226 => return b"\\xE2\0" as *const u8 as *const c_char,
        227 => return b"\\xE3\0" as *const u8 as *const c_char,
        228 => return b"\\xE4\0" as *const u8 as *const c_char,
        229 => return b"\\xE5\0" as *const u8 as *const c_char,
        230 => return b"\\xE6\0" as *const u8 as *const c_char,
        231 => return b"\\xE7\0" as *const u8 as *const c_char,
        232 => return b"\\xE8\0" as *const u8 as *const c_char,
        233 => return b"\\xE9\0" as *const u8 as *const c_char,
        234 => return b"\\xEA\0" as *const u8 as *const c_char,
        235 => return b"\\xEB\0" as *const u8 as *const c_char,
        236 => return b"\\xEC\0" as *const u8 as *const c_char,
        237 => return b"\\xED\0" as *const u8 as *const c_char,
        238 => return b"\\xEE\0" as *const u8 as *const c_char,
        239 => return b"\\xEF\0" as *const u8 as *const c_char,
        240 => return b"\\xF0\0" as *const u8 as *const c_char,
        241 => return b"\\xF1\0" as *const u8 as *const c_char,
        242 => return b"\\xF2\0" as *const u8 as *const c_char,
        243 => return b"\\xF3\0" as *const u8 as *const c_char,
        244 => return b"\\xF4\0" as *const u8 as *const c_char,
        245 => return b"\\xF5\0" as *const u8 as *const c_char,
        246 => return b"\\xF6\0" as *const u8 as *const c_char,
        247 => return b"\\xF7\0" as *const u8 as *const c_char,
        248 => return b"\\xF8\0" as *const u8 as *const c_char,
        249 => return b"\\xF9\0" as *const u8 as *const c_char,
        250 => return b"\\xFA\0" as *const u8 as *const c_char,
        251 => return b"\\xFB\0" as *const u8 as *const c_char,
        252 => return b"\\xFC\0" as *const u8 as *const c_char,
        253 => return b"\\xFD\0" as *const u8 as *const c_char,
        254 => return b"\\xFE\0" as *const u8 as *const c_char,
        255 => return b"\\xFF\0" as *const u8 as *const c_char,
        _ => {
            assert!(false, "0");
            return b"dead code\0" as *const u8 as *const c_char;
        }
    };
}

unsafe extern "C" fn getDebugLevel(
    mut variableName: *const c_char,
    mut defaultDebugLevel: c_ulong,
) -> c_ulong {
    let valueOrNull: *const c_char = crate::stdlib::getenv(variableName);
    if valueOrNull.is_null() {
        return defaultDebugLevel;
    }
    let value: *const c_char = valueOrNull;
    *__errno_location() = 0;
    let mut afterValue: *mut c_char = null_mut::<c_char>();
    let mut debugLevel: c_ulong = crate::stdlib::strtoul(value, &raw mut afterValue, 10);
    if *__errno_location() != 0
        || afterValue == value as *mut c_char
        || *afterValue.offset(0) as c_int != '\0' as i32
    {
        *__errno_location() = 0;
        return defaultDebugLevel;
    }
    return debugLevel;
}
unsafe extern "C" fn run_static_initializers() {
    xmlLen = (size_of::<[XML_Char; 37]>() as c_int as usize)
        .wrapping_div(size_of::<XML_Char>())
        .wrapping_sub(1usize) as c_int;
    xmlnsLen = (size_of::<[XML_Char; 30]>() as c_int as usize)
        .wrapping_div(size_of::<XML_Char>())
        .wrapping_sub(1usize) as c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
