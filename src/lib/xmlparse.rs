pub mod siphash_h {
    use crate::__stddef_size_t_h::size_t;
    use crate::siphash_h::siphash;
    use crate::siphash_h::sipkey;
    use crate::stdlib::uint64_t;
    use core::ffi::c_char;
    use core::ffi::c_int;
    use core::ffi::c_uchar;
    use core::ffi::c_void;
    use core::mem::size_of;

    pub(crate) fn sip_tokey(mut key: *mut sipkey, mut src: *const c_void) -> *mut sipkey {
        let src = src as *const c_uchar;
        let src8 = src.wrapping_add(8);
        let k0 = (unsafe { *src.wrapping_add(0) } as uint64_t)
            | (unsafe { *src.wrapping_add(1) } as uint64_t) << 8
            | (unsafe { *src.wrapping_add(2) } as uint64_t) << 16
            | (unsafe { *src.wrapping_add(3) } as uint64_t) << 24
            | (unsafe { *src.wrapping_add(4) } as uint64_t) << 32
            | (unsafe { *src.wrapping_add(5) } as uint64_t) << 40
            | (unsafe { *src.wrapping_add(6) } as uint64_t) << 48
            | (unsafe { *src.wrapping_add(7) } as uint64_t) << 56;
        let k1 = (unsafe { *src8.wrapping_add(0) } as uint64_t)
            | (unsafe { *src8.wrapping_add(1) } as uint64_t) << 8
            | (unsafe { *src8.wrapping_add(2) } as uint64_t) << 16
            | (unsafe { *src8.wrapping_add(3) } as uint64_t) << 24
            | (unsafe { *src8.wrapping_add(4) } as uint64_t) << 32
            | (unsafe { *src8.wrapping_add(5) } as uint64_t) << 40
            | (unsafe { *src8.wrapping_add(6) } as uint64_t) << 48
            | (unsafe { *src8.wrapping_add(7) } as uint64_t) << 56;
        unsafe { (*key).k[0] = k0 };
        unsafe { (*key).k[1] = k1 };
        key
    }

    pub(crate) fn sip_round(mut H: *mut siphash, rounds: c_int) {
        let mut i: c_int = 0;
        while i < rounds {
            let mut v0 = unsafe { (*H).v0 };
            let mut v1 = unsafe { (*H).v1 };
            let mut v2 = unsafe { (*H).v2 };
            let mut v3 = unsafe { (*H).v3 };
            v0 = v0.wrapping_add(v1);
            v1 = v1.rotate_left(13);
            v1 ^= v0;
            v0 = v0.rotate_right(64 - 32);
            v2 = v2.wrapping_add(v3);
            v3 = v3.rotate_left(16);
            v3 ^= v2;
            v0 = v0.wrapping_add(v3);
            v3 = v3.rotate_left(21);
            v3 ^= v0;
            v2 = v2.wrapping_add(v1);
            v1 = v1.rotate_left(17);
            v1 ^= v2;
            v2 = v2.rotate_right(64 - 32);
            unsafe { (*H).v0 = v0 };
            unsafe { (*H).v1 = v1 };
            unsafe { (*H).v2 = v2 };
            unsafe { (*H).v3 = v3 };
            i += 1;
        }
    }

    pub(crate) fn sip24_init(mut H: *mut siphash, mut key: *const sipkey) -> *mut siphash {
        let k0 = unsafe { (*key).k[0] };
        let k1 = unsafe { (*key).k[1] };
        unsafe { (*H).v0 = ((0x736f6d65u64) << 32 | 0x70736575) ^ k0 };
        unsafe { (*H).v1 = ((0x646f7261u64) << 32 | 0x6e646f6d) ^ k1 };
        unsafe { (*H).v2 = ((0x6c796765u64) << 32 | 0x6e657261) ^ k0 };
        unsafe { (*H).v3 = ((0x74656462u64) << 32 | 0x79746573) ^ k1 };
        unsafe { (*H).p = &raw mut (*H).buf as *mut c_uchar };
        unsafe { (*H).c = 0u64 };
        H
    }

    pub(crate) fn sip24_update(
        mut H: *mut siphash,
        mut src: *const c_void,
        mut len: size_t,
    ) -> *mut siphash {
        let mut p: *const c_uchar = src as *const c_uchar;
        let pe: *const c_uchar = p.wrapping_add(len);
        let buf_start = unsafe { &raw mut (*H).buf as *mut c_uchar };
        let buf_end =
            buf_start.wrapping_add((size_of::<[c_uchar; 8]>()).wrapping_div(size_of::<c_uchar>()));
        loop {
            while p < pe && unsafe { (*H).p < buf_end } {
                let fresh20 = p;
                p = p.wrapping_add(1);
                let fresh21 = unsafe { (*H).p };
                unsafe { (*H).p = (*H).p.wrapping_add(1) };
                unsafe { *fresh21 = *fresh20 };
            }
            if unsafe { (*H).p < buf_end } {
                break;
            }
            let m = (unsafe { (*H).buf[0] } as uint64_t)
                | (unsafe { (*H).buf[1] } as uint64_t) << 8
                | (unsafe { (*H).buf[2] } as uint64_t) << 16
                | (unsafe { (*H).buf[3] } as uint64_t) << 24
                | (unsafe { (*H).buf[4] } as uint64_t) << 32
                | (unsafe { (*H).buf[5] } as uint64_t) << 40
                | (unsafe { (*H).buf[6] } as uint64_t) << 48
                | (unsafe { (*H).buf[7] } as uint64_t) << 56;
            unsafe { (*H).v3 ^= m };
            sip_round(H, 2);
            unsafe { (*H).v0 ^= m };
            unsafe { (*H).p = buf_start };
            let c = unsafe { (*H).c };
            unsafe { (*H).c = c.wrapping_add(8u64) };
            if p >= pe {
                break;
            }
        }
        H
    }

    pub(crate) fn sip24_final(mut H: *mut siphash) -> uint64_t {
        let buf_start = unsafe { &raw mut (*H).buf as *mut c_uchar };
        let left: c_char = unsafe { (*H).p.offset_from(buf_start) as c_char };
        let mut b: uint64_t = unsafe { (*H).c }.wrapping_add(left as uint64_t) << 56;
        let mut current_block_6: u64;
        match left as c_int {
            7 => {
                b |= (unsafe { (*H).buf[6] } as uint64_t) << 48;
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
        if current_block_6 == 4021588137456158946 {
            b |= (unsafe { (*H).buf[5] } as uint64_t) << 40;
            current_block_6 = 12485585037491154495;
        }
        if current_block_6 == 12485585037491154495 {
            b |= (unsafe { (*H).buf[4] } as uint64_t) << 32;
            current_block_6 = 435354115069985819;
        }
        if current_block_6 == 435354115069985819 {
            b |= (unsafe { (*H).buf[3] } as uint64_t) << 24;
            current_block_6 = 1199690694990637288;
        }
        if current_block_6 == 1199690694990637288 {
            b |= (unsafe { (*H).buf[2] } as uint64_t) << 16;
            current_block_6 = 2615438511104190163;
        }
        if current_block_6 == 2615438511104190163 {
            b |= (unsafe { (*H).buf[1] } as uint64_t) << 8;
            current_block_6 = 4681268752173749360;
        }
        if current_block_6 == 4681268752173749360 {
            b |= unsafe { (*H).buf[0usize] } as uint64_t;
        }
        unsafe { (*H).v3 ^= b };
        sip_round(H, 2);
        unsafe { (*H).v0 ^= b };
        unsafe { (*H).v2 ^= 0xffu64 };
        sip_round(H, 4);
        let v0 = unsafe { (*H).v0 };
        let v1 = unsafe { (*H).v1 };
        let v2 = unsafe { (*H).v2 };
        let v3 = unsafe { (*H).v3 };
        v0 ^ v1 ^ v2 ^ v3
    }

    pub(crate) fn siphash24(
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
            p: core::ptr::null_mut::<c_uchar>(),
            c: 0u64,
        };
        sip24_final(sip24_update(sip24_init(&raw mut state, key), src, len))
    }

    pub(crate) fn sip24_valid() -> c_int {
        pub static vectors: [[c_uchar; 8]; 64] = [
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
                != (vectors[i][0] as uint64_t)
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
        1
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
pub(crate) use crate::src::lib::xmlparse::siphash_h::sip24_final;
pub(crate) use crate::src::lib::xmlparse::siphash_h::sip24_init;
pub(crate) use crate::src::lib::xmlparse::siphash_h::sip24_update;
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
use crate::src::lib::xmlrole::XmlPrologStateInit;
use crate::src::lib::xmlrole::XmlPrologStateInitExternalEntity;
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
use crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncoding;
use crate::src::lib::xmltok::xmltok_ns_c::XmlGetUtf8InternalEncodingNS;
use crate::src::lib::xmltok::xmltok_ns_c::XmlInitEncoding;
use crate::src::lib::xmltok::xmltok_ns_c::XmlInitEncodingNS;
use crate::src::lib::xmltok::xmltok_ns_c::XmlParseXmlDecl;
use crate::src::lib::xmltok::xmltok_ns_c::XmlParseXmlDeclNS;
pub use crate::src::lib::xmltok::XML_Convert_Result;
use crate::src::lib::xmltok::XmlInitUnknownEncoding;
use crate::src::lib::xmltok::XmlInitUnknownEncodingNS;
use crate::src::lib::xmltok::XmlSizeOfUnknownEncoding;
use crate::src::lib::xmltok::XmlUtf8Encode;
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

use crate::stdlib::__errno_location;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_long;
use core::ffi::c_longlong;
use core::ffi::c_uchar;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_ulonglong;
use core::ffi::c_void;
use core::mem::size_of;
use core::ptr::null;
use core::ptr::null_mut;
use core::ptr::write;

#[inline]
fn c_char_slice_from_ptr_end<'a>(ptr: *const c_char, end: *const c_char) -> &'a [c_char] {
    unsafe { core::slice::from_raw_parts(ptr, end.offset_from(ptr) as usize) }
}

#[derive(Clone)]
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
    pub m_unknownEncodingRelease: Option<extern "C" fn(*mut c_void) -> ()>,
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
    pub m_tagStack: Option<Box<tag>>,
    pub m_freeTagList: Option<Box<tag>>,
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
#[derive(Clone)]
#[repr(C)]

pub struct tag {
    pub parent: Option<Box<tag>>,
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
    extern "C" fn(XML_Parser, *const c_char, *const c_char, *mut *const c_char) -> XML_Error;
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
static xmlLen: c_int = (size_of::<[XML_Char; 37]>() / size_of::<XML_Char>() - 1) as c_int;
static xmlnsLen: c_int = (size_of::<[XML_Char; 30]>() / size_of::<XML_Char>() - 1) as c_int;

pub const INIT_TAG_BUF_SIZE: c_int = 32;

pub const INIT_DATA_BUF_SIZE: c_int = 1024;

pub const INIT_ATTS_SIZE: c_int = 16;

pub const INIT_ATTS_VERSION: c_uint = 0xffffffff;

pub const INIT_BLOCK_SIZE: c_int = 1024;

pub const INIT_BUFFER_SIZE: c_int = 1024;

pub const EXPAND_SPARE: c_int = 24;

pub const INIT_SCAFFOLD_ELEMENTS: c_int = 32;
#[no_mangle]

pub static mut g_reparseDeferralEnabledDefault: XML_Bool = XML_TRUE;
#[no_mangle]

pub static mut g_bytesScanned: c_uint = 0;

extern "C" fn expat_heap_stat(
    mut rootParser: XML_Parser,
    mut operator: c_char,
    mut absDiff: XmlBigCount,
    mut newTotal: XmlBigCount,
    mut peakTotal: XmlBigCount,
    mut sourceLine: c_int,
) {
    let count_bytes_direct = unsafe { (*rootParser).m_accounting.countBytesDirect };
    let amplification: c_float = newTotal as c_float / count_bytes_direct as c_float;
    let heap_stat_fmt =
        b"expat: Allocations(%p): Direct %10llu, allocated %c%10llu to %10llu (%10llu peak), amplification %8.2f (xmlparse.c:%d)\n\0"
            as *const u8 as *const c_char;
    unsafe {
        fprintf(
            stderr,
            heap_stat_fmt,
            rootParser as *mut c_void,
            count_bytes_direct,
            operator as c_int,
            absDiff,
            newTotal,
            peakTotal,
            amplification as core::ffi::c_double,
            sourceLine,
        );
    }
}

extern "C" fn expat_heap_increase_tolerable(
    mut rootParser: XML_Parser,
    mut increase: XmlBigCount,
    mut sourceLine: c_int,
) -> bool {
    assert!(!rootParser.is_null());
    assert!(increase > 0u64);
    let mut newTotal: XmlBigCount = 0;
    let mut tolerable: bool = true_0 != 0;
    let bytes_allocated = unsafe { (*rootParser).m_alloc_tracker.bytesAllocated };
    if (-(1i32) as XmlBigCount).wrapping_sub(bytes_allocated) < increase {
        tolerable = false_0 != 0;
    } else {
        newTotal = bytes_allocated.wrapping_add(increase);
        let activation_threshold =
            unsafe { (*rootParser).m_alloc_tracker.activationThresholdBytes };
        if newTotal >= activation_threshold {
            assert!(newTotal > 0);
            let count_bytes_direct = unsafe { (*rootParser).m_accounting.countBytesDirect };
            let amplification: c_float = newTotal as c_float / count_bytes_direct as c_float;
            let maximum_amplification =
                unsafe { (*rootParser).m_alloc_tracker.maximumAmplificationFactor };
            if amplification > maximum_amplification {
                tolerable = false_0 != 0;
            }
        }
    }
    let debug_level = unsafe { (*rootParser).m_alloc_tracker.debugLevel };
    if !tolerable && debug_level >= 1u64 {
        expat_heap_stat(
            rootParser,
            '+' as c_char,
            increase,
            newTotal,
            newTotal,
            sourceLine,
        );
    }
    tolerable
}
fn expat_malloc(mut parser: XML_Parser, mut size: size_t, mut sourceLine: c_int) -> *mut c_void {
    if (SIZE_MAX as size_t).wrapping_sub(size)
        < (size_of::<size_t>()).wrapping_add(EXPAT_MALLOC_PADDING)
    {
        return NULL;
    }
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    let bytesToAllocate: size_t = (size_of::<size_t>())
        .wrapping_add(EXPAT_MALLOC_PADDING)
        .wrapping_add(size);
    let bytes_allocated = unsafe { (*rootParser).m_alloc_tracker.bytesAllocated };
    if (-(1i32) as XmlBigCount).wrapping_sub(bytes_allocated) < bytesToAllocate as XmlBigCount {
        return NULL;
    }
    if !expat_heap_increase_tolerable(rootParser, bytesToAllocate as XmlBigCount, sourceLine) {
        return NULL;
    }
    let malloc_fcn = unsafe {
        (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")
    };
    let mallocedPtr: *mut c_void = malloc_fcn(bytesToAllocate);
    if mallocedPtr.is_null() {
        return NULL;
    }
    unsafe { *(mallocedPtr as *mut size_t) = size };
    let new_bytes_allocated = bytes_allocated.wrapping_add(bytesToAllocate as XmlBigCount);
    unsafe { (*rootParser).m_alloc_tracker.bytesAllocated = new_bytes_allocated };
    let debug_level = unsafe { (*rootParser).m_alloc_tracker.debugLevel };
    if debug_level >= 2 {
        let mut peak_bytes_allocated = unsafe { (*rootParser).m_alloc_tracker.peakBytesAllocated };
        if new_bytes_allocated > peak_bytes_allocated {
            peak_bytes_allocated = new_bytes_allocated;
            unsafe { (*rootParser).m_alloc_tracker.peakBytesAllocated = peak_bytes_allocated };
        }
        expat_heap_stat(
            rootParser,
            '+' as c_char,
            bytesToAllocate as XmlBigCount,
            new_bytes_allocated,
            peak_bytes_allocated,
            sourceLine,
        );
    }
    (mallocedPtr as *mut c_char)
        .wrapping_add(size_of::<size_t>())
        .wrapping_add(EXPAT_MALLOC_PADDING) as *mut c_void
}

#[cfg(feature = "expat_test_shims")]
#[export_name = "expat_malloc"]
extern "C" fn expat_malloc_test_shim(
    parser: XML_Parser,
    size: size_t,
    sourceLine: c_int,
) -> *mut c_void {
    expat_malloc(parser, size, sourceLine)
}

fn expat_free(mut parser: XML_Parser, mut ptr: *mut c_void, mut sourceLine: c_int) {
    assert!(!parser.is_null());
    if ptr.is_null() {
        return;
    }
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    let mallocedPtr: *mut c_void = (ptr as *mut c_char)
        .wrapping_offset(-(EXPAT_MALLOC_PADDING as isize))
        .wrapping_offset(-(size_of::<size_t>() as isize))
        as *mut c_void;
    let bytesAllocated: size_t = (size_of::<size_t>())
        .wrapping_add(EXPAT_MALLOC_PADDING)
        .wrapping_add(unsafe { *(mallocedPtr as *mut size_t) });
    let current_bytes_allocated = unsafe { (*rootParser).m_alloc_tracker.bytesAllocated };
    assert!(current_bytes_allocated >= bytesAllocated as XmlBigCount);
    let new_bytes_allocated = current_bytes_allocated.wrapping_sub(bytesAllocated as XmlBigCount);
    unsafe { (*rootParser).m_alloc_tracker.bytesAllocated = new_bytes_allocated };
    let debug_level = unsafe { (*rootParser).m_alloc_tracker.debugLevel };
    if debug_level >= 2 {
        let peak_bytes_allocated = unsafe { (*rootParser).m_alloc_tracker.peakBytesAllocated };
        expat_heap_stat(
            rootParser,
            '-' as c_char,
            bytesAllocated as XmlBigCount,
            new_bytes_allocated,
            peak_bytes_allocated,
            sourceLine,
        );
    }
    let free_fcn = unsafe { (*parser).m_mem.free_fcn.expect("non-null function pointer") };
    free_fcn(mallocedPtr);
}

#[cfg(feature = "expat_test_shims")]
#[export_name = "expat_free"]
extern "C" fn expat_free_test_shim(parser: XML_Parser, ptr: *mut c_void, sourceLine: c_int) {
    expat_free(parser, ptr, sourceLine);
}

fn expat_realloc(
    mut parser: XML_Parser,
    mut ptr: *mut c_void,
    mut size: size_t,
    mut sourceLine: c_int,
) -> *mut c_void {
    assert!(!parser.is_null());
    if ptr.is_null() {
        return expat_malloc(parser, size, sourceLine);
    }
    if size == 0usize {
        expat_free(parser, ptr, sourceLine);
        return NULL;
    }
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    let mut mallocedPtr: *mut c_void = (ptr as *mut c_char)
        .wrapping_offset(-(EXPAT_MALLOC_PADDING as isize))
        .wrapping_offset(-(size_of::<size_t>() as isize))
        as *mut c_void;
    let prevSize: size_t = unsafe { *(mallocedPtr as *mut size_t) };
    let isIncrease: bool = size > prevSize;
    let absDiff: size_t = if size > prevSize {
        size.wrapping_sub(prevSize)
    } else {
        prevSize.wrapping_sub(size)
    };
    if isIncrease && !expat_heap_increase_tolerable(rootParser, absDiff as XmlBigCount, sourceLine)
    {
        return NULL;
    }
    assert!(
        18446744073709551615_usize
            .wrapping_sub(size_of::<size_t>())
            .wrapping_sub((size_of::<c_longlong>()).wrapping_sub(size_of::<size_t>()))
            >= size
    );
    let realloc_fcn = unsafe {
        (*parser)
            .m_mem
            .realloc_fcn
            .expect("non-null function pointer")
    };
    let new_allocation_size = (size_of::<size_t>())
        .wrapping_add(EXPAT_MALLOC_PADDING)
        .wrapping_add(size);
    mallocedPtr = realloc_fcn(mallocedPtr, new_allocation_size);
    if mallocedPtr.is_null() {
        return NULL;
    }
    let current_bytes_allocated = unsafe { (*rootParser).m_alloc_tracker.bytesAllocated };
    let new_bytes_allocated = if isIncrease {
        assert!(
            (-(1i32) as XmlBigCount).wrapping_sub(current_bytes_allocated)
                >= absDiff as XmlBigCount
        );
        current_bytes_allocated.wrapping_add(absDiff as XmlBigCount)
    } else {
        assert!(current_bytes_allocated >= absDiff as XmlBigCount);
        current_bytes_allocated.wrapping_sub(absDiff as XmlBigCount)
    };
    unsafe { (*rootParser).m_alloc_tracker.bytesAllocated = new_bytes_allocated };
    let debug_level = unsafe { (*rootParser).m_alloc_tracker.debugLevel };
    if debug_level >= 2 {
        let mut peak_bytes_allocated = unsafe { (*rootParser).m_alloc_tracker.peakBytesAllocated };
        if new_bytes_allocated > peak_bytes_allocated {
            peak_bytes_allocated = new_bytes_allocated;
            unsafe { (*rootParser).m_alloc_tracker.peakBytesAllocated = peak_bytes_allocated };
        }
        expat_heap_stat(
            rootParser,
            if isIncrease {
                '+' as c_char
            } else {
                '-' as c_char
            },
            absDiff as XmlBigCount,
            new_bytes_allocated,
            peak_bytes_allocated,
            sourceLine,
        );
    }
    unsafe { *(mallocedPtr as *mut size_t) = size };
    (mallocedPtr as *mut c_char)
        .wrapping_add(size_of::<size_t>())
        .wrapping_add(EXPAT_MALLOC_PADDING) as *mut c_void
}

#[cfg(feature = "expat_test_shims")]
#[export_name = "expat_realloc"]
extern "C" fn expat_realloc_test_shim(
    parser: XML_Parser,
    ptr: *mut c_void,
    size: size_t,
    sourceLine: c_int,
) -> *mut c_void {
    expat_realloc(parser, ptr, size, sourceLine)
}

#[no_mangle]

pub extern "C" fn XML_ParserCreate(mut encodingName: *const XML_Char) -> XML_Parser {
    XML_ParserCreate_MM(
        encodingName,
        null::<XML_Memory_Handling_Suite>(),
        null::<XML_Char>(),
    )
}
#[no_mangle]

pub extern "C" fn XML_ParserCreateNS(
    mut encodingName: *const XML_Char,
    mut nsSep: XML_Char,
) -> XML_Parser {
    let mut tmp: [XML_Char; 2] = [nsSep, 0];
    XML_ParserCreate_MM(
        encodingName,
        null::<XML_Memory_Handling_Suite>(),
        &raw mut tmp as *mut XML_Char,
    )
}

static implicitContext: [XML_Char; 41] = [
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

#[inline]
fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

#[inline]
fn getrandom_call(target: *mut c_void, count: size_t, flags: c_uint) -> c_int {
    unsafe { getrandom(target, count, flags) as c_int }
}

#[inline]
fn open_dev_urandom_readonly() -> c_int {
    unsafe { crate::stdlib::open(b"/dev/urandom\0" as *const u8 as *const c_char, O_RDONLY) }
}

#[inline]
fn read_fd(fd: c_int, target: *mut c_void, count: size_t) -> ssize_t {
    unsafe { crate::stdlib::read(fd, target, count) }
}

#[inline]
fn close_fd(fd: c_int) {
    unsafe { crate::stdlib::close(fd) };
}

#[inline]
fn gettimeofday_call(tv: *mut timeval) -> c_int {
    unsafe { crate::stdlib::gettimeofday(tv, NULL) }
}

#[inline]
fn getpid_call() -> c_int {
    unsafe { crate::stdlib::getpid() }
}

extern "C" fn writeRandomBytes_getrandom_nonblock(
    mut target: *mut c_void,
    mut count: size_t,
) -> c_int {
    let mut success: c_int = 0;
    let mut bytesWrittenTotal: size_t = 0;
    let getrandomFlags: c_uint = GRND_NONBLOCK as c_uint;
    loop {
        let currentTarget: *mut c_void =
            (target as *mut c_char).wrapping_add(bytesWrittenTotal) as *mut c_void;
        let bytesToWrite: size_t = count.wrapping_sub(bytesWrittenTotal);
        assert!(bytesToWrite <= 2147483647i32 as size_t);
        let bytesWrittenMore: c_int = getrandom_call(currentTarget, bytesToWrite, getrandomFlags);
        if bytesWrittenMore > 0 {
            bytesWrittenTotal = bytesWrittenTotal.wrapping_add(bytesWrittenMore as size_t);
            if bytesWrittenTotal >= count {
                success = 1i32;
            }
        }
        if !(success == 0 && errno_value() == EINTR) {
            break;
        }
    }
    success
}

extern "C" fn writeRandomBytes_dev_urandom(mut target: *mut c_void, mut count: size_t) -> c_int {
    let mut success: c_int = 0;
    let mut bytesWrittenTotal: size_t = 0;
    let fd: c_int = open_dev_urandom_readonly();
    if fd < 0 {
        return 0i32;
    }
    loop {
        let currentTarget: *mut c_void =
            (target as *mut c_char).wrapping_add(bytesWrittenTotal) as *mut c_void;
        let bytesToWrite: size_t = count.wrapping_sub(bytesWrittenTotal);
        let bytesWrittenMore: ssize_t = read_fd(fd, currentTarget, bytesToWrite);
        if bytesWrittenMore > 0 {
            bytesWrittenTotal = bytesWrittenTotal.wrapping_add(bytesWrittenMore as size_t);
            if bytesWrittenTotal >= count {
                success = 1i32;
            }
        }
        if !(success == 0 && errno_value() == EINTR) {
            break;
        }
    }
    close_fd(fd);
    success
}

extern "C" fn gather_time_entropy() -> c_ulong {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let gettimeofday_res: c_int = gettimeofday_call(&raw mut tv);
    assert!(gettimeofday_res == 0);
    tv.tv_usec as c_ulong
}

extern "C" fn ENTROPY_DEBUG(mut label: *const c_char, mut entropy: c_ulong) -> c_ulong {
    if getDebugLevel(b"EXPAT_ENTROPY_DEBUG\0" as *const u8 as *const c_char, 0) >= 1 {
        unsafe {
            fprintf(
                stderr,
                b"expat: Entropy: %s --> 0x%0*lx (%lu bytes)\n\0" as *const u8 as *const c_char,
                label,
                size_of::<c_ulong>() as c_int * 2i32,
                entropy,
                size_of::<c_ulong>() as c_ulong,
            );
        }
    }
    entropy
}

extern "C" fn generate_hash_secret_salt(mut _parser: XML_Parser) -> c_ulong {
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
    entropy ^= getpid_call() as c_ulong;
    if size_of::<c_ulong>() == 4 {
        ENTROPY_DEBUG(
            b"fallback(4)\0" as *const u8 as *const c_char,
            entropy.wrapping_mul(2147483647u64),
        )
    } else {
        ENTROPY_DEBUG(
            b"fallback(8)\0" as *const u8 as *const c_char,
            entropy.wrapping_mul(2305843009213693951u64),
        )
    }
}

extern "C" fn get_hash_secret_salt(mut parser: XML_Parser) -> c_ulong {
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    unsafe { (*rootParser).m_hash_secret_salt }
}

extern "C" fn callProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let have_now: size_t = (if !end.is_null() && !start.is_null() {
        unsafe { end.offset_from(start) as c_long }
    } else {
        0
    }) as size_t;
    if unsafe {
        (*parser).m_reparseDeferralEnabled as c_int != 0
            && (*parser).m_parsingStatus.finalBuffer == 0
    } {
        let had_before: size_t = unsafe { (*parser).m_partialTokenBytesBefore };
        let mut available_buffer: size_t =
            (if unsafe { !(*parser).m_bufferPtr.is_null() && !(*parser).m_buffer.is_null() } {
                unsafe { (*parser).m_bufferPtr.offset_from((*parser).m_buffer) as c_long }
            } else {
                0
            }) as size_t;
        available_buffer = available_buffer.wrapping_sub(if available_buffer < 1024 {
            available_buffer
        } else {
            1024usize
        });
        available_buffer = available_buffer.wrapping_add(
            (if unsafe { !(*parser).m_bufferLim.is_null() && !(*parser).m_bufferEnd.is_null() } {
                unsafe { (*parser).m_bufferLim.offset_from((*parser).m_bufferEnd) as c_long }
            } else {
                0
            }) as size_t,
        );
        let enough: bool = have_now >= (2usize).wrapping_mul(had_before)
            || unsafe { (*parser).m_lastBufferRequestSize as size_t > available_buffer };
        if !enough {
            unsafe { *endPtr = start };
            return XML_ERROR_NONE;
        }
    }
    unsafe { g_bytesScanned = g_bytesScanned.wrapping_add(have_now as c_uint) };
    let mut ret: XML_Error = XML_ERROR_NONE;
    unsafe { *endPtr = start };
    loop {
        let processor = unsafe { (*parser).m_processor.expect("non-null function pointer") };
        ret = processor(parser, unsafe { *endPtr }, end, endPtr);
        if unsafe { (*parser).m_parsingStatus.parsing != XML_PARSING } {
            unsafe { (*parser).m_reenter = XML_FALSE };
        }
        if unsafe { (*parser).m_reenter == 0 } {
            break;
        }
        unsafe { (*parser).m_reenter = XML_FALSE };
        if ret != XML_ERROR_NONE {
            return ret;
        }
    }
    if ret == XML_ERROR_NONE {
        if unsafe { *endPtr == start } {
            unsafe { (*parser).m_partialTokenBytesBefore = have_now };
        } else {
            unsafe { (*parser).m_partialTokenBytesBefore = 0usize };
        }
    }
    ret
}

extern "C" fn startParsing(mut parser: XML_Parser) -> XML_Bool {
    if unsafe { (*parser).m_hash_secret_salt == 0u64 } {
        let hash_salt = generate_hash_secret_salt(parser);
        unsafe { (*parser).m_hash_secret_salt = hash_salt };
    }
    if unsafe { (*parser).m_ns != 0 } {
        return setContext(parser, &raw const implicitContext as *const XML_Char);
    }
    XML_TRUE
}
#[no_mangle]

pub extern "C" fn XML_ParserCreate_MM(
    mut encodingName: *const XML_Char,
    mut memsuite: *const XML_Memory_Handling_Suite,
    mut nameSep: *const XML_Char,
) -> XML_Parser {
    parserCreate(
        encodingName,
        memsuite,
        nameSep,
        null_mut::<DTD>(),
        null_mut::<XML_ParserStruct>(),
    )
}

extern "C" fn stdlib_malloc(size: size_t) -> *mut c_void {
    unsafe { crate::stdlib::malloc(size) }
}

extern "C" fn stdlib_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    unsafe { crate::stdlib::realloc(ptr, size) }
}

extern "C" fn stdlib_free(ptr: *mut c_void) {
    unsafe { crate::stdlib::free(ptr) }
}

#[inline]
fn memset_zero<T>(target: *mut T) {
    unsafe { memset(target as *mut c_void, 0, size_of::<T>()) };
}

extern "C" fn parserCreate(
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
        let memsuite_ref = unsafe { &*memsuite };
        let sizeAndParser: *mut c_void =
            memsuite_ref.malloc_fcn.expect("non-null function pointer")(
                (size_of::<size_t>())
                    .wrapping_add(EXPAT_MALLOC_PADDING)
                    .wrapping_add(size_of::<XML_ParserStruct>()),
            );
        if !sizeAndParser.is_null() {
            unsafe { *(sizeAndParser as *mut size_t) = size_of::<XML_ParserStruct>() };
            parser = (sizeAndParser as *mut c_char)
                .wrapping_add(size_of::<size_t>())
                .wrapping_add(EXPAT_MALLOC_PADDING) as XML_Parser;
            let mtemp = unsafe { &mut (*parser).m_mem };
            mtemp.malloc_fcn = memsuite_ref.malloc_fcn;
            mtemp.realloc_fcn = memsuite_ref.realloc_fcn;
            mtemp.free_fcn = memsuite_ref.free_fcn;
        }
    } else {
        let sizeAndParser_0: *mut c_void = stdlib_malloc(
            (size_of::<size_t>())
                .wrapping_add(EXPAT_MALLOC_PADDING)
                .wrapping_add(size_of::<XML_ParserStruct>()),
        );
        if !sizeAndParser_0.is_null() {
            unsafe { *(sizeAndParser_0 as *mut size_t) = size_of::<XML_ParserStruct>() };
            parser = (sizeAndParser_0 as *mut c_char)
                .wrapping_add(size_of::<size_t>())
                .wrapping_add(EXPAT_MALLOC_PADDING) as XML_Parser;
            let mtemp_0 = unsafe { &mut (*parser).m_mem };
            mtemp_0.malloc_fcn = Some(stdlib_malloc);
            mtemp_0.realloc_fcn = Some(stdlib_realloc);
            mtemp_0.free_fcn = Some(stdlib_free);
        }
    }
    if parser.is_null() {
        return parser;
    }

    let parser_ref = unsafe { &mut *parser };
    memset_zero(&raw mut parser_ref.m_alloc_tracker);
    if parentParser.is_null() {
        parser_ref.m_alloc_tracker.debugLevel =
            getDebugLevel(b"EXPAT_MALLOC_DEBUG\0" as *const u8 as *const c_char, 0);
        parser_ref.m_alloc_tracker.maximumAmplificationFactor =
            EXPAT_ALLOC_TRACKER_MAXIMUM_AMPLIFICATION_DEFAULT;
        parser_ref.m_alloc_tracker.activationThresholdBytes =
            EXPAT_ALLOC_TRACKER_ACTIVATION_THRESHOLD_DEFAULT as XmlBigCount;
        parser_ref.m_parentParser = null_mut::<XML_ParserStruct>();
        parser_ref.m_accounting.countBytesDirect = 0u64;
    } else {
        parser_ref.m_parentParser = parentParser;
    }

    let rootParser_0: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    let root_parser_ref = unsafe { &mut *rootParser_0 };
    assert!(root_parser_ref.m_parentParser.is_null());
    assert!(
        (18446744073709551615u64).wrapping_sub(root_parser_ref.m_alloc_tracker.bytesAllocated)
            >= increase as XmlBigCount
    );
    root_parser_ref.m_alloc_tracker.bytesAllocated = root_parser_ref
        .m_alloc_tracker
        .bytesAllocated
        .wrapping_add(increase as XmlBigCount);
    if root_parser_ref.m_alloc_tracker.debugLevel >= 2 {
        if root_parser_ref.m_alloc_tracker.bytesAllocated
            > root_parser_ref.m_alloc_tracker.peakBytesAllocated
        {
            root_parser_ref.m_alloc_tracker.peakBytesAllocated =
                root_parser_ref.m_alloc_tracker.bytesAllocated;
        }
        expat_heap_stat(
            rootParser_0,
            '+' as c_char,
            increase as XmlBigCount,
            root_parser_ref.m_alloc_tracker.bytesAllocated,
            root_parser_ref.m_alloc_tracker.peakBytesAllocated,
            1439i32,
        );
    }
    parser_ref.m_buffer = null_mut::<c_char>();
    parser_ref.m_bufferLim = null::<c_char>();
    parser_ref.m_attsSize = INIT_ATTS_SIZE;
    parser_ref.m_atts = expat_malloc(
        parser,
        (parser_ref.m_attsSize as size_t).wrapping_mul(size_of::<ATTRIBUTE>()),
        1449,
    ) as *mut ATTRIBUTE;
    if parser_ref.m_atts.is_null() {
        expat_free(parser, parser as *mut c_void, 1451);
        return null_mut::<XML_ParserStruct>();
    }
    parser_ref.m_dataBuf = expat_malloc(
        parser,
        (1024usize).wrapping_mul(size_of::<XML_Char>()),
        1462,
    ) as *mut XML_Char;
    if parser_ref.m_dataBuf.is_null() {
        expat_free(parser, parser_ref.m_atts as *mut c_void, 1464);
        expat_free(parser, parser as *mut c_void, 1468);
        return null_mut::<XML_ParserStruct>();
    }
    parser_ref.m_dataBufEnd = unsafe { parser_ref.m_dataBuf.offset(INIT_DATA_BUF_SIZE as isize) };
    if !dtd.is_null() {
        parser_ref.m_dtd = dtd;
    } else {
        parser_ref.m_dtd = dtdCreate(parser);
        if parser_ref.m_dtd.is_null() {
            expat_free(parser, parser_ref.m_dataBuf as *mut c_void, 1478);
            expat_free(parser, parser_ref.m_atts as *mut c_void, 1479);
            expat_free(parser, parser as *mut c_void, 1483);
            return null_mut::<XML_ParserStruct>();
        }
    }
    parser_ref.m_freeBindingList = null_mut::<BINDING>();
    unsafe { write(&raw mut parser_ref.m_tagStack, None) };
    unsafe { write(&raw mut parser_ref.m_freeTagList, None) };
    parser_ref.m_freeInternalEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    parser_ref.m_freeAttributeEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    parser_ref.m_freeValueEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    parser_ref.m_groupSize = 0;
    parser_ref.m_groupConnector = null_mut::<c_char>();
    parser_ref.m_unknownEncodingHandler = None;
    parser_ref.m_unknownEncodingHandlerData = NULL;
    parser_ref.m_namespaceSeparator = ASCII_EXCL as XML_Char;
    parser_ref.m_ns = XML_FALSE;
    parser_ref.m_ns_triplets = XML_FALSE;
    parser_ref.m_nsAtts = null_mut::<NS_ATT>();
    parser_ref.m_nsAttsVersion = 0;
    parser_ref.m_nsAttsPower = 0;
    parser_ref.m_protocolEncodingName = null::<XML_Char>();
    poolInit(&raw mut parser_ref.m_tempPool, parser);
    poolInit(&raw mut parser_ref.m_temp2Pool, parser);
    parserInit(parser, encodingName);
    if !encodingName.is_null() && parser_ref.m_protocolEncodingName.is_null() {
        if !dtd.is_null() {
            parser_ref.m_dtd = null_mut::<DTD>();
        }
        XML_ParserFree(parser);
        return null_mut::<XML_ParserStruct>();
    }
    if !nameSep.is_null() {
        parser_ref.m_ns = XML_TRUE;
        parser_ref.m_internalEncoding = XmlGetUtf8InternalEncodingNS();
        parser_ref.m_namespaceSeparator = unsafe { *nameSep };
    } else {
        parser_ref.m_internalEncoding = XmlGetUtf8InternalEncoding();
    }
    parser
}

extern "C" fn parserInit(mut parser: XML_Parser, mut encodingName: *const XML_Char) {
    let parser_ref = unsafe { &mut *parser };
    parser_ref.m_processor = Some(
        prologInitProcessor
            as extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    XmlPrologStateInit(&raw mut parser_ref.m_prologState);
    if !encodingName.is_null() {
        parser_ref.m_protocolEncodingName = copyString(encodingName, parser);
    }
    parser_ref.m_curBase = null::<XML_Char>();
    parser_ref.m_encoding = XmlInitEncoding(
        &raw mut parser_ref.m_initEncoding,
        &raw mut parser_ref.m_encoding,
        null::<c_char>(),
    )
    .1;
    parser_ref.m_userData = NULL;
    parser_ref.m_handlerArg = NULL;
    parser_ref.m_startElementHandler = None;
    parser_ref.m_endElementHandler = None;
    parser_ref.m_characterDataHandler = None;
    parser_ref.m_processingInstructionHandler = None;
    parser_ref.m_commentHandler = None;
    parser_ref.m_startCdataSectionHandler = None;
    parser_ref.m_endCdataSectionHandler = None;
    parser_ref.m_defaultHandler = None;
    parser_ref.m_startDoctypeDeclHandler = None;
    parser_ref.m_endDoctypeDeclHandler = None;
    parser_ref.m_unparsedEntityDeclHandler = None;
    parser_ref.m_notationDeclHandler = None;
    parser_ref.m_startNamespaceDeclHandler = None;
    parser_ref.m_endNamespaceDeclHandler = None;
    parser_ref.m_notStandaloneHandler = None;
    parser_ref.m_externalEntityRefHandler = None;
    parser_ref.m_externalEntityRefHandlerArg = parser;
    parser_ref.m_skippedEntityHandler = None;
    parser_ref.m_elementDeclHandler = None;
    parser_ref.m_attlistDeclHandler = None;
    parser_ref.m_entityDeclHandler = None;
    parser_ref.m_xmlDeclHandler = None;
    parser_ref.m_bufferPtr = parser_ref.m_buffer;
    parser_ref.m_bufferEnd = parser_ref.m_buffer;
    parser_ref.m_parseEndByteIndex = 0i64;
    parser_ref.m_parseEndPtr = null::<c_char>();
    parser_ref.m_partialTokenBytesBefore = 0usize;
    parser_ref.m_reparseDeferralEnabled = unsafe { g_reparseDeferralEnabledDefault };
    parser_ref.m_lastBufferRequestSize = 0;
    parser_ref.m_declElementType = null_mut::<ELEMENT_TYPE>();
    parser_ref.m_declAttributeId = null_mut::<ATTRIBUTE_ID>();
    parser_ref.m_declEntity = null_mut::<ENTITY>();
    parser_ref.m_doctypeName = null::<XML_Char>();
    parser_ref.m_doctypeSysid = null::<XML_Char>();
    parser_ref.m_doctypePubid = null::<XML_Char>();
    parser_ref.m_declAttributeType = null::<XML_Char>();
    parser_ref.m_declNotationName = null::<XML_Char>();
    parser_ref.m_declNotationPublicId = null::<XML_Char>();
    parser_ref.m_declAttributeIsCdata = XML_FALSE;
    parser_ref.m_declAttributeIsId = XML_FALSE;
    unsafe {
        memset(
            &raw mut parser_ref.m_position as *mut c_void,
            0,
            size_of::<POSITION>(),
        )
    };
    parser_ref.m_errorCode = XML_ERROR_NONE;
    parser_ref.m_eventPtr = null::<c_char>();
    parser_ref.m_eventEndPtr = null::<c_char>();
    parser_ref.m_positionPtr = null::<c_char>();
    parser_ref.m_openInternalEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    parser_ref.m_openAttributeEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    parser_ref.m_openValueEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
    parser_ref.m_defaultExpandInternalEntities = XML_TRUE;
    parser_ref.m_tagLevel = 0;
    parser_ref.m_tagStack = None;
    parser_ref.m_inheritedBindings = null_mut::<BINDING>();
    parser_ref.m_nSpecifiedAtts = 0;
    parser_ref.m_unknownEncodingMem = NULL;
    parser_ref.m_unknownEncodingRelease = None;
    parser_ref.m_unknownEncodingData = NULL;
    parser_ref.m_parsingStatus.parsing = XML_INITIALIZED;
    parser_ref.m_reenter = XML_FALSE;
    parser_ref.m_isParamEntity = XML_FALSE;
    parser_ref.m_useForeignDTD = XML_FALSE;
    parser_ref.m_paramEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER;
    parser_ref.m_hash_secret_salt = 0u64;
    unsafe {
        memset(
            &raw mut parser_ref.m_accounting as *mut c_void,
            0,
            size_of::<ACCOUNTING>(),
        )
    };
    parser_ref.m_accounting.debugLevel =
        getDebugLevel(b"EXPAT_ACCOUNTING_DEBUG\0" as *const u8 as *const c_char, 0);
    parser_ref.m_accounting.maximumAmplificationFactor =
        EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_MAXIMUM_AMPLIFICATION_DEFAULT;
    parser_ref.m_accounting.activationThresholdBytes =
        EXPAT_BILLION_LAUGHS_ATTACK_PROTECTION_ACTIVATION_THRESHOLD_DEFAULT as c_ulonglong;
    unsafe {
        memset(
            &raw mut parser_ref.m_entity_stats as *mut c_void,
            0,
            size_of::<ENTITY_STATS>(),
        )
    };
    parser_ref.m_entity_stats.debugLevel =
        getDebugLevel(b"EXPAT_ENTITY_DEBUG\0" as *const u8 as *const c_char, 0);
}

extern "C" fn moveToFreeBindingList(mut parser: XML_Parser, mut bindings: *mut BINDING) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        bindings = unsafe { (*bindings).nextTagBinding };
        let free_binding_list = unsafe { (*parser).m_freeBindingList };
        unsafe { (*b).nextTagBinding = free_binding_list };
        unsafe { (*parser).m_freeBindingList = b };
    }
}
#[no_mangle]

pub extern "C" fn XML_ParserReset(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Bool {
    let mut tStk: Option<Box<TAG>>;
    let mut openEntityList: *mut OPEN_INTERNAL_ENTITY = null_mut::<OPEN_INTERNAL_ENTITY>();
    if parser.is_null() {
        return XML_FALSE;
    }
    let dtd = {
        let parser_ref = unsafe { &mut *parser };
        if !parser_ref.m_parentParser.is_null() {
            return XML_FALSE;
        }
        tStk = core::mem::take(&mut parser_ref.m_tagStack);
        while let Some(mut tag) = tStk {
            tStk = tag.parent.take();
            moveToFreeBindingList(parser, tag.bindings);
            tag.bindings = null_mut::<BINDING>();
            tag.parent = parser_ref.m_freeTagList.take();
            parser_ref.m_freeTagList = Some(tag);
        }
        openEntityList = parser_ref.m_openInternalEntities;
        while !openEntityList.is_null() {
            let openEntity: *mut OPEN_INTERNAL_ENTITY = openEntityList;
            openEntityList = unsafe { (*openEntity).next };
            unsafe { (*openEntity).next = parser_ref.m_freeInternalEntities };
            parser_ref.m_freeInternalEntities = openEntity;
        }
        openEntityList = parser_ref.m_openAttributeEntities;
        while !openEntityList.is_null() {
            let openEntity_0: *mut OPEN_INTERNAL_ENTITY = openEntityList;
            openEntityList = unsafe { (*openEntity_0).next };
            unsafe { (*openEntity_0).next = parser_ref.m_freeAttributeEntities };
            parser_ref.m_freeAttributeEntities = openEntity_0;
        }
        openEntityList = parser_ref.m_openValueEntities;
        while !openEntityList.is_null() {
            let openEntity_1: *mut OPEN_INTERNAL_ENTITY = openEntityList;
            openEntityList = unsafe { (*openEntity_1).next };
            unsafe { (*openEntity_1).next = parser_ref.m_freeValueEntities };
            parser_ref.m_freeValueEntities = openEntity_1;
        }
        moveToFreeBindingList(parser, parser_ref.m_inheritedBindings);
        expat_free(parser, parser_ref.m_unknownEncodingMem, 1686);
        if parser_ref.m_unknownEncodingRelease.is_some() {
            parser_ref
                .m_unknownEncodingRelease
                .expect("non-null function pointer")(parser_ref.m_unknownEncodingData);
        }
        poolClear(&raw mut parser_ref.m_tempPool);
        poolClear(&raw mut parser_ref.m_temp2Pool);
        expat_free(
            parser,
            parser_ref.m_protocolEncodingName as *mut c_void,
            1691,
        );
        parser_ref.m_protocolEncodingName = null::<XML_Char>();
        parser_ref.m_dtd
    };
    parserInit(parser, encodingName);
    dtdReset(dtd, parser);
    XML_TRUE
}

extern "C" fn parserBusy(mut parser: XML_Parser) -> XML_Bool {
    match unsafe { (*parser).m_parsingStatus.parsing } {
        1 | 3 => XML_TRUE,
        0 | 2 | _ => XML_FALSE,
    }
}
#[no_mangle]

pub extern "C" fn XML_SetEncoding(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if parserBusy(parser) != 0 {
        return XML_STATUS_ERROR;
    }
    let protocol_encoding_name = unsafe { (*parser).m_protocolEncodingName };
    expat_free(parser, protocol_encoding_name as *mut c_void, 1723);
    if encodingName.is_null() {
        unsafe { (*parser).m_protocolEncodingName = null::<XML_Char>() };
    } else {
        let copied = copyString(encodingName, parser);
        unsafe { (*parser).m_protocolEncodingName = copied };
        if unsafe { (*parser).m_protocolEncodingName.is_null() } {
            return XML_STATUS_ERROR;
        }
    }
    XML_STATUS_OK
}
#[no_mangle]

pub extern "C" fn XML_ExternalEntityParserCreate(
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
    {
        let old_parser_ref = unsafe { &mut *parser };
        oldDtd = old_parser_ref.m_dtd;
        oldStartElementHandler = old_parser_ref.m_startElementHandler;
        oldEndElementHandler = old_parser_ref.m_endElementHandler;
        oldCharacterDataHandler = old_parser_ref.m_characterDataHandler;
        oldProcessingInstructionHandler = old_parser_ref.m_processingInstructionHandler;
        oldCommentHandler = old_parser_ref.m_commentHandler;
        oldStartCdataSectionHandler = old_parser_ref.m_startCdataSectionHandler;
        oldEndCdataSectionHandler = old_parser_ref.m_endCdataSectionHandler;
        oldDefaultHandler = old_parser_ref.m_defaultHandler;
        oldUnparsedEntityDeclHandler = old_parser_ref.m_unparsedEntityDeclHandler;
        oldNotationDeclHandler = old_parser_ref.m_notationDeclHandler;
        oldStartNamespaceDeclHandler = old_parser_ref.m_startNamespaceDeclHandler;
        oldEndNamespaceDeclHandler = old_parser_ref.m_endNamespaceDeclHandler;
        oldNotStandaloneHandler = old_parser_ref.m_notStandaloneHandler;
        oldExternalEntityRefHandler = old_parser_ref.m_externalEntityRefHandler;
        oldSkippedEntityHandler = old_parser_ref.m_skippedEntityHandler;
        oldUnknownEncodingHandler = old_parser_ref.m_unknownEncodingHandler;
        oldUnknownEncodingHandlerData = old_parser_ref.m_unknownEncodingHandlerData;
        oldElementDeclHandler = old_parser_ref.m_elementDeclHandler;
        oldAttlistDeclHandler = old_parser_ref.m_attlistDeclHandler;
        oldEntityDeclHandler = old_parser_ref.m_entityDeclHandler;
        oldXmlDeclHandler = old_parser_ref.m_xmlDeclHandler;
        oldDeclElementType = old_parser_ref.m_declElementType;
        oldUserData = old_parser_ref.m_userData;
        oldHandlerArg = old_parser_ref.m_handlerArg;
        oldDefaultExpandInternalEntities = old_parser_ref.m_defaultExpandInternalEntities;
        oldExternalEntityRefHandlerArg = old_parser_ref.m_externalEntityRefHandlerArg;
        oldParamEntityParsing = old_parser_ref.m_paramEntityParsing;
        oldInEntityValue = old_parser_ref.m_prologState.inEntityValue;
        oldns_triplets = old_parser_ref.m_ns_triplets;
        oldhash_secret_salt = old_parser_ref.m_hash_secret_salt;
        oldReparseDeferralEnabled = old_parser_ref.m_reparseDeferralEnabled;
        if context.is_null() {
            newDtd = oldDtd;
        }
        if old_parser_ref.m_ns != 0 {
            let mut tmp: [XML_Char; 2] = [old_parser_ref.m_namespaceSeparator, 0];
            parser = parserCreate(
                encodingName,
                &raw const old_parser_ref.m_mem,
                &raw mut tmp as *mut XML_Char,
                newDtd,
                oldParser,
            );
        } else {
            parser = parserCreate(
                encodingName,
                &raw const old_parser_ref.m_mem,
                null::<XML_Char>(),
                newDtd,
                oldParser,
            );
        }
    }
    if parser.is_null() {
        return null_mut::<XML_ParserStruct>();
    }
    let parser_ref = unsafe { &mut *parser };
    parser_ref.m_startElementHandler = oldStartElementHandler;
    parser_ref.m_endElementHandler = oldEndElementHandler;
    parser_ref.m_characterDataHandler = oldCharacterDataHandler;
    parser_ref.m_processingInstructionHandler = oldProcessingInstructionHandler;
    parser_ref.m_commentHandler = oldCommentHandler;
    parser_ref.m_startCdataSectionHandler = oldStartCdataSectionHandler;
    parser_ref.m_endCdataSectionHandler = oldEndCdataSectionHandler;
    parser_ref.m_defaultHandler = oldDefaultHandler;
    parser_ref.m_unparsedEntityDeclHandler = oldUnparsedEntityDeclHandler;
    parser_ref.m_notationDeclHandler = oldNotationDeclHandler;
    parser_ref.m_startNamespaceDeclHandler = oldStartNamespaceDeclHandler;
    parser_ref.m_endNamespaceDeclHandler = oldEndNamespaceDeclHandler;
    parser_ref.m_notStandaloneHandler = oldNotStandaloneHandler;
    parser_ref.m_externalEntityRefHandler = oldExternalEntityRefHandler;
    parser_ref.m_skippedEntityHandler = oldSkippedEntityHandler;
    parser_ref.m_unknownEncodingHandler = oldUnknownEncodingHandler;
    parser_ref.m_unknownEncodingHandlerData = oldUnknownEncodingHandlerData;
    parser_ref.m_elementDeclHandler = oldElementDeclHandler;
    parser_ref.m_attlistDeclHandler = oldAttlistDeclHandler;
    parser_ref.m_entityDeclHandler = oldEntityDeclHandler;
    parser_ref.m_xmlDeclHandler = oldXmlDeclHandler;
    parser_ref.m_declElementType = oldDeclElementType;
    parser_ref.m_userData = oldUserData;
    if oldUserData == oldHandlerArg {
        parser_ref.m_handlerArg = parser_ref.m_userData;
    } else {
        parser_ref.m_handlerArg = parser as *mut c_void;
    }
    if oldExternalEntityRefHandlerArg != oldParser {
        parser_ref.m_externalEntityRefHandlerArg = oldExternalEntityRefHandlerArg;
    }
    parser_ref.m_defaultExpandInternalEntities = oldDefaultExpandInternalEntities;
    parser_ref.m_ns_triplets = oldns_triplets;
    parser_ref.m_hash_secret_salt = oldhash_secret_salt;
    parser_ref.m_reparseDeferralEnabled = oldReparseDeferralEnabled;
    parser_ref.m_parentParser = oldParser;
    parser_ref.m_paramEntityParsing = oldParamEntityParsing;
    parser_ref.m_prologState.inEntityValue = oldInEntityValue;
    if !context.is_null() {
        if dtdCopy(oldParser, parser_ref.m_dtd, oldDtd, parser) == 0
            || setContext(parser, context) == 0
        {
            XML_ParserFree(parser);
            return null_mut::<XML_ParserStruct>();
        }
        parser_ref.m_processor = Some(
            externalEntityInitProcessor
                as extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
    } else {
        parser_ref.m_isParamEntity = XML_TRUE;
        XmlPrologStateInitExternalEntity(&raw mut parser_ref.m_prologState);
        parser_ref.m_processor = Some(
            externalParEntInitProcessor
                as extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
    }
    parser
}

extern "C" fn destroyBindings(mut bindings: *mut BINDING, mut parser: XML_Parser) {
    loop {
        let mut b: *mut BINDING = bindings;
        if b.is_null() {
            break;
        }
        bindings = unsafe { (*b).nextTagBinding };
        let uri = unsafe { (*b).uri };
        expat_free(parser, uri as *mut c_void, 1919);
        expat_free(parser, b as *mut c_void, 1920);
    }
}
#[no_mangle]

pub extern "C" fn XML_ParserFree(mut parser: XML_Parser) {
    if parser.is_null() {
        return;
    }
    let parser_ref = unsafe { &mut *parser };
    let mut tagList: Option<Box<TAG>> = core::mem::take(&mut parser_ref.m_tagStack);
    let mut entityList: *mut OPEN_INTERNAL_ENTITY = parser_ref.m_openInternalEntities;
    loop {
        let p: Box<TAG> = if let Some(p) = tagList {
            p
        } else if let Some(p) = parser_ref.m_freeTagList.take() {
            p
        } else {
            break;
        };
        tagList = p.parent;
        expat_free(parser, unsafe { p.buf.raw as *mut c_void }, 1942);
        destroyBindings(p.bindings, parser);
    }
    loop {
        if entityList.is_null() {
            if parser_ref.m_freeInternalEntities.is_null() {
                break;
            }
            entityList = parser_ref.m_freeInternalEntities;
            parser_ref.m_freeInternalEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        let openEntity: *mut OPEN_INTERNAL_ENTITY = entityList;
        entityList = unsafe { (*entityList).next };
        expat_free(parser, openEntity as *mut c_void, 1958);
    }
    entityList = parser_ref.m_openAttributeEntities;
    loop {
        if entityList.is_null() {
            if parser_ref.m_freeAttributeEntities.is_null() {
                break;
            }
            entityList = parser_ref.m_freeAttributeEntities;
            parser_ref.m_freeAttributeEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        let openEntity_0: *mut OPEN_INTERNAL_ENTITY = entityList;
        entityList = unsafe { (*entityList).next };
        expat_free(parser, openEntity_0 as *mut c_void, 1972);
    }
    entityList = parser_ref.m_openValueEntities;
    loop {
        if entityList.is_null() {
            if parser_ref.m_freeValueEntities.is_null() {
                break;
            }
            entityList = parser_ref.m_freeValueEntities;
            parser_ref.m_freeValueEntities = null_mut::<OPEN_INTERNAL_ENTITY>();
        }
        let openEntity_1: *mut OPEN_INTERNAL_ENTITY = entityList;
        entityList = unsafe { (*entityList).next };
        expat_free(parser, openEntity_1 as *mut c_void, 1986);
    }
    destroyBindings(parser_ref.m_freeBindingList, parser);
    destroyBindings(parser_ref.m_inheritedBindings, parser);
    poolDestroy(&raw mut parser_ref.m_tempPool);
    poolDestroy(&raw mut parser_ref.m_temp2Pool);
    expat_free(
        parser,
        parser_ref.m_protocolEncodingName as *mut c_void,
        1992,
    );
    if parser_ref.m_isParamEntity == 0 && !parser_ref.m_dtd.is_null() {
        dtdDestroy(
            parser_ref.m_dtd,
            parser_ref.m_parentParser.is_null() as XML_Bool,
            parser,
        );
    }
    expat_free(parser, parser_ref.m_atts as *mut c_void, 2002);
    expat_free(parser, parser_ref.m_groupConnector as *mut c_void, 2006);
    parser_ref
        .m_mem
        .free_fcn
        .expect("non-null function pointer")(parser_ref.m_buffer as *mut c_void);
    expat_free(parser, parser_ref.m_dataBuf as *mut c_void, 2011);
    expat_free(parser, parser_ref.m_nsAtts as *mut c_void, 2012);
    expat_free(parser, parser_ref.m_unknownEncodingMem, 2013);
    if parser_ref.m_unknownEncodingRelease.is_some() {
        parser_ref
            .m_unknownEncodingRelease
            .expect("non-null function pointer")(parser_ref.m_unknownEncodingData);
    }
    expat_free(parser, parser as *mut c_void, 2016);
}
#[no_mangle]

pub extern "C" fn XML_UseParserAsHandlerArg(mut parser: XML_Parser) {
    if !parser.is_null() {
        unsafe { (*parser).m_handlerArg = parser as *mut c_void };
    }
}
#[no_mangle]

pub extern "C" fn XML_UseForeignDTD(mut parser: XML_Parser, mut useDTD: XML_Bool) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    if parserBusy(parser) != 0 {
        return XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING;
    }
    unsafe { (*parser).m_useForeignDTD = useDTD };
    XML_ERROR_NONE
}
#[no_mangle]

pub extern "C" fn XML_SetReturnNSTriplet(mut parser: XML_Parser, mut do_nst: c_int) {
    if parser.is_null() {
        return;
    }
    if parserBusy(parser) != 0 {
        return;
    }
    let ns_triplets = (if do_nst != 0 {
        XML_TRUE as c_int
    } else {
        XML_FALSE as c_int
    }) as XML_Bool;
    unsafe { (*parser).m_ns_triplets = ns_triplets };
}
#[no_mangle]

pub extern "C" fn XML_SetUserData(mut parser: XML_Parser, mut p: *mut c_void) {
    if parser.is_null() {
        return;
    }
    let keep_in_sync = unsafe { (*parser).m_handlerArg == (*parser).m_userData };
    unsafe { (*parser).m_userData = p };
    if keep_in_sync {
        unsafe { (*parser).m_handlerArg = p };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetBase(mut parser: XML_Parser, mut p: *const XML_Char) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if !p.is_null() {
        let dtd = unsafe { (*parser).m_dtd };
        p = poolCopyString(unsafe { &raw mut (*dtd).pool }, p);
        if p.is_null() {
            return XML_STATUS_ERROR;
        }
        unsafe { (*parser).m_curBase = p };
    } else {
        unsafe { (*parser).m_curBase = null::<XML_Char>() };
    }
    XML_STATUS_OK
}
#[no_mangle]

pub extern "C" fn XML_GetBase(mut parser: XML_Parser) -> *const XML_Char {
    if parser.is_null() {
        return null::<XML_Char>();
    }
    unsafe { (*parser).m_curBase }
}
#[no_mangle]

pub extern "C" fn XML_GetSpecifiedAttributeCount(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return -(1i32);
    }
    unsafe { (*parser).m_nSpecifiedAtts }
}
#[no_mangle]

pub extern "C" fn XML_GetIdAttributeIndex(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return -(1i32);
    }
    unsafe { (*parser).m_idAttIndex }
}
#[no_mangle]

pub extern "C" fn XML_SetElementHandler(
    mut parser: XML_Parser,
    mut start: XML_StartElementHandler,
    mut end: XML_EndElementHandler,
) {
    if parser.is_null() {
        return;
    }
    unsafe { (*parser).m_startElementHandler = start };
    unsafe { (*parser).m_endElementHandler = end };
}
#[no_mangle]

pub extern "C" fn XML_SetStartElementHandler(
    mut parser: XML_Parser,
    mut start: XML_StartElementHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_startElementHandler = start };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetEndElementHandler(mut parser: XML_Parser, mut end: XML_EndElementHandler) {
    if !parser.is_null() {
        unsafe { (*parser).m_endElementHandler = end };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetCharacterDataHandler(
    mut parser: XML_Parser,
    mut handler: XML_CharacterDataHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_characterDataHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetProcessingInstructionHandler(
    mut parser: XML_Parser,
    mut handler: XML_ProcessingInstructionHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_processingInstructionHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetCommentHandler(mut parser: XML_Parser, mut handler: XML_CommentHandler) {
    if !parser.is_null() {
        unsafe { (*parser).m_commentHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetCdataSectionHandler(
    mut parser: XML_Parser,
    mut start: XML_StartCdataSectionHandler,
    mut end: XML_EndCdataSectionHandler,
) {
    if parser.is_null() {
        return;
    }
    unsafe { (*parser).m_startCdataSectionHandler = start };
    unsafe { (*parser).m_endCdataSectionHandler = end };
}
#[no_mangle]

pub extern "C" fn XML_SetStartCdataSectionHandler(
    mut parser: XML_Parser,
    mut start: XML_StartCdataSectionHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_startCdataSectionHandler = start };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetEndCdataSectionHandler(
    mut parser: XML_Parser,
    mut end: XML_EndCdataSectionHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_endCdataSectionHandler = end };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetDefaultHandler(mut parser: XML_Parser, mut handler: XML_DefaultHandler) {
    if parser.is_null() {
        return;
    }
    unsafe { (*parser).m_defaultHandler = handler };
    unsafe { (*parser).m_defaultExpandInternalEntities = XML_FALSE };
}
#[no_mangle]

pub extern "C" fn XML_SetDefaultHandlerExpand(
    mut parser: XML_Parser,
    mut handler: XML_DefaultHandler,
) {
    if parser.is_null() {
        return;
    }
    unsafe { (*parser).m_defaultHandler = handler };
    unsafe { (*parser).m_defaultExpandInternalEntities = XML_TRUE };
}
#[no_mangle]

pub extern "C" fn XML_SetDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartDoctypeDeclHandler,
    mut end: XML_EndDoctypeDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    unsafe { (*parser).m_startDoctypeDeclHandler = start };
    unsafe { (*parser).m_endDoctypeDeclHandler = end };
}
#[no_mangle]

pub extern "C" fn XML_SetStartDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartDoctypeDeclHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_startDoctypeDeclHandler = start };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetEndDoctypeDeclHandler(
    mut parser: XML_Parser,
    mut end: XML_EndDoctypeDeclHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_endDoctypeDeclHandler = end };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetUnparsedEntityDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_UnparsedEntityDeclHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_unparsedEntityDeclHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetNotationDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_NotationDeclHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_notationDeclHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartNamespaceDeclHandler,
    mut end: XML_EndNamespaceDeclHandler,
) {
    if parser.is_null() {
        return;
    }
    unsafe { (*parser).m_startNamespaceDeclHandler = start };
    unsafe { (*parser).m_endNamespaceDeclHandler = end };
}
#[no_mangle]

pub extern "C" fn XML_SetStartNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut start: XML_StartNamespaceDeclHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_startNamespaceDeclHandler = start };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetEndNamespaceDeclHandler(
    mut parser: XML_Parser,
    mut end: XML_EndNamespaceDeclHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_endNamespaceDeclHandler = end };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetNotStandaloneHandler(
    mut parser: XML_Parser,
    mut handler: XML_NotStandaloneHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_notStandaloneHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetExternalEntityRefHandler(
    mut parser: XML_Parser,
    mut handler: XML_ExternalEntityRefHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_externalEntityRefHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetExternalEntityRefHandlerArg(mut parser: XML_Parser, mut arg: *mut c_void) {
    if parser.is_null() {
        return;
    }
    if !arg.is_null() {
        unsafe { (*parser).m_externalEntityRefHandlerArg = arg as XML_Parser };
    } else {
        unsafe { (*parser).m_externalEntityRefHandlerArg = parser };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetSkippedEntityHandler(
    mut parser: XML_Parser,
    mut handler: XML_SkippedEntityHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_skippedEntityHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetUnknownEncodingHandler(
    mut parser: XML_Parser,
    mut handler: XML_UnknownEncodingHandler,
    mut data: *mut c_void,
) {
    if parser.is_null() {
        return;
    }
    unsafe { (*parser).m_unknownEncodingHandler = handler };
    unsafe { (*parser).m_unknownEncodingHandlerData = data };
}
#[no_mangle]

pub extern "C" fn XML_SetElementDeclHandler(
    mut parser: XML_Parser,
    mut eldecl: XML_ElementDeclHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_elementDeclHandler = eldecl };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetAttlistDeclHandler(
    mut parser: XML_Parser,
    mut attdecl: XML_AttlistDeclHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_attlistDeclHandler = attdecl };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetEntityDeclHandler(
    mut parser: XML_Parser,
    mut handler: XML_EntityDeclHandler,
) {
    if !parser.is_null() {
        unsafe { (*parser).m_entityDeclHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetXmlDeclHandler(mut parser: XML_Parser, mut handler: XML_XmlDeclHandler) {
    if !parser.is_null() {
        unsafe { (*parser).m_xmlDeclHandler = handler };
    }
}
#[no_mangle]

pub extern "C" fn XML_SetParamEntityParsing(
    mut parser: XML_Parser,
    mut peParsing: XML_ParamEntityParsing,
) -> c_int {
    if parser.is_null() {
        return 0i32;
    }
    if parserBusy(parser) != 0 {
        return 0i32;
    }
    unsafe { (*parser).m_paramEntityParsing = peParsing };
    1
}
#[no_mangle]

pub extern "C" fn XML_SetHashSalt(mut parser: XML_Parser, mut hash_salt: c_ulong) -> c_int {
    if parser.is_null() {
        return 0i32;
    }
    let rootParser: XML_Parser = getRootParserOf(parser, null_mut::<c_uint>());
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    if parserBusy(rootParser) != 0 {
        return 0i32;
    }
    unsafe { (*rootParser).m_hash_secret_salt = hash_salt };
    1
}
#[no_mangle]

pub extern "C" fn XML_Parse(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut len: c_int,
    mut isFinal: c_int,
) -> XML_Status {
    if parser.is_null() || len < 0 || s.is_null() && len != 0 {
        if !parser.is_null() {
            unsafe { (*parser).m_errorCode = XML_ERROR_INVALID_ARGUMENT };
        }
        return XML_STATUS_ERROR;
    }
    match unsafe { (*parser).m_parsingStatus.parsing } {
        3 => {
            unsafe { (*parser).m_errorCode = XML_ERROR_SUSPENDED };
            return XML_STATUS_ERROR;
        }
        2 => {
            unsafe { (*parser).m_errorCode = XML_ERROR_FINISHED };
            return XML_STATUS_ERROR;
        }
        0 => {
            if unsafe { (*parser).m_parentParser.is_null() } && startParsing(parser) == 0 {
                unsafe { (*parser).m_errorCode = XML_ERROR_NO_MEMORY };
                return XML_STATUS_ERROR;
            }
        }
        _ => {}
    }
    unsafe { (*parser).m_parsingStatus.parsing = XML_PARSING };
    let mut buff: *mut c_void = XML_GetBuffer(parser, len);
    if buff.is_null() {
        return XML_STATUS_ERROR;
    }
    if len > 0 {
        assert!(!s.is_null());
        unsafe { memcpy(buff, s as *const c_void, len as size_t) };
    }
    XML_ParseBuffer(parser, len, isFinal)
}
#[no_mangle]

pub extern "C" fn XML_ParseBuffer(
    mut parser: XML_Parser,
    mut len: c_int,
    mut isFinal: c_int,
) -> XML_Status {
    let mut result: XML_Status = XML_STATUS_OK;
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    let parser_ref = unsafe { &mut *parser };
    if len < 0 {
        parser_ref.m_errorCode = XML_ERROR_INVALID_ARGUMENT;
        return XML_STATUS_ERROR;
    }
    match parser_ref.m_parsingStatus.parsing {
        3 => {
            parser_ref.m_errorCode = XML_ERROR_SUSPENDED;
            return XML_STATUS_ERROR;
        }
        2 => {
            parser_ref.m_errorCode = XML_ERROR_FINISHED;
            return XML_STATUS_ERROR;
        }
        0 => {
            if parser_ref.m_bufferPtr.is_null() {
                parser_ref.m_errorCode = XML_ERROR_NO_BUFFER;
                return XML_STATUS_ERROR;
            }
            if parser_ref.m_parentParser.is_null() && startParsing(parser) == 0 {
                parser_ref.m_errorCode = XML_ERROR_NO_MEMORY;
                return XML_STATUS_ERROR;
            }
        }
        _ => {}
    }
    parser_ref.m_parsingStatus.parsing = XML_PARSING;
    let start: *const c_char = parser_ref.m_bufferPtr;
    parser_ref.m_positionPtr = start;
    parser_ref.m_bufferEnd = unsafe { parser_ref.m_bufferEnd.offset(len as isize) };
    parser_ref.m_parseEndPtr = parser_ref.m_bufferEnd;
    parser_ref.m_parseEndByteIndex += len as XML_Index;
    parser_ref.m_parsingStatus.finalBuffer = isFinal as XML_Bool;
    parser_ref.m_errorCode = callProcessor(
        parser,
        start,
        parser_ref.m_parseEndPtr,
        &raw mut parser_ref.m_bufferPtr,
    );
    if parser_ref.m_errorCode != XML_ERROR_NONE {
        parser_ref.m_eventEndPtr = parser_ref.m_eventPtr;
        parser_ref.m_processor = Some(
            errorProcessor
                as extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
        return XML_STATUS_ERROR;
    } else {
        match parser_ref.m_parsingStatus.parsing {
            3 => {
                result = XML_STATUS_SUSPENDED;
            }
            0 | 1 => {
                if isFinal != 0 {
                    parser_ref.m_parsingStatus.parsing = XML_FINISHED;
                    return result;
                }
            }
            _ => {}
        }
    }
    let encoding = parser_ref.m_encoding;
    unsafe {
        (*encoding).updatePosition(
            &*encoding,
            c_char_slice_from_ptr_end(parser_ref.m_positionPtr, parser_ref.m_bufferPtr),
            &raw mut parser_ref.m_position,
        )
    };
    parser_ref.m_positionPtr = parser_ref.m_bufferPtr;
    result
}
#[inline]
fn memmove_bytes(dst: *mut c_void, src: *const c_void, bytes: size_t) {
    unsafe { crate::stdlib::memmove(dst, src, bytes) };
}

#[inline]
fn memcpy_bytes(dst: *mut c_void, src: *const c_void, bytes: size_t) {
    unsafe { memcpy(dst, src, bytes) };
}

#[no_mangle]
pub extern "C" fn XML_GetBuffer(mut parser: XML_Parser, mut len: c_int) -> *mut c_void {
    if parser.is_null() {
        return NULL;
    }
    let parser_ref = unsafe { &mut *parser };
    if len < 0 {
        parser_ref.m_errorCode = XML_ERROR_NO_MEMORY;
        return NULL;
    }
    match parser_ref.m_parsingStatus.parsing {
        3 => {
            parser_ref.m_errorCode = XML_ERROR_SUSPENDED;
            return NULL;
        }
        2 => {
            parser_ref.m_errorCode = XML_ERROR_FINISHED;
            return NULL;
        }
        _ => {}
    }
    parser_ref.m_lastBufferRequestSize = len;
    if len as c_long
        > (if !parser_ref.m_bufferLim.is_null() && !parser_ref.m_bufferEnd.is_null() {
            unsafe { parser_ref.m_bufferLim.offset_from(parser_ref.m_bufferEnd) as c_long }
        } else {
            0
        })
        || parser_ref.m_buffer.is_null()
    {
        let mut keep: c_int = 0;
        let mut neededSize: c_int = (len as c_uint).wrapping_add(
            (if !parser_ref.m_bufferEnd.is_null() && !parser_ref.m_bufferPtr.is_null() {
                unsafe { parser_ref.m_bufferEnd.offset_from(parser_ref.m_bufferPtr) as c_long }
            } else {
                0
            }) as c_uint,
        ) as c_int;
        if neededSize < 0 {
            parser_ref.m_errorCode = XML_ERROR_NO_MEMORY;
            return NULL;
        }
        keep = (if !parser_ref.m_bufferPtr.is_null() && !parser_ref.m_buffer.is_null() {
            unsafe { parser_ref.m_bufferPtr.offset_from(parser_ref.m_buffer) as c_long }
        } else {
            0
        }) as c_int;
        if keep > XML_CONTEXT_BYTES {
            keep = XML_CONTEXT_BYTES;
        }
        if keep > INT_MAX - neededSize {
            parser_ref.m_errorCode = XML_ERROR_NO_MEMORY;
            return NULL;
        }
        neededSize += keep;
        if !parser_ref.m_buffer.is_null()
            && !parser_ref.m_bufferPtr.is_null()
            && neededSize as c_long
                <= (if !parser_ref.m_bufferLim.is_null() && !parser_ref.m_buffer.is_null() {
                    unsafe { parser_ref.m_bufferLim.offset_from(parser_ref.m_buffer) as c_long }
                } else {
                    0
                })
        {
            if (keep as c_long)
                < (if !parser_ref.m_bufferPtr.is_null() && !parser_ref.m_buffer.is_null() {
                    unsafe { parser_ref.m_bufferPtr.offset_from(parser_ref.m_buffer) as c_long }
                } else {
                    0
                })
            {
                let offset: c_int =
                    (if !parser_ref.m_bufferPtr.is_null() && !parser_ref.m_buffer.is_null() {
                        unsafe { parser_ref.m_bufferPtr.offset_from(parser_ref.m_buffer) as c_long }
                    } else {
                        0
                    }) as c_int
                        - keep;
                memmove_bytes(
                    parser_ref.m_buffer as *mut c_void,
                    unsafe { parser_ref.m_buffer.offset(offset as isize) } as *const c_void,
                    unsafe {
                        parser_ref.m_bufferEnd.offset_from(parser_ref.m_bufferPtr) as c_long
                            + keep as c_long
                    } as size_t,
                );
                parser_ref.m_bufferEnd =
                    unsafe { parser_ref.m_bufferEnd.offset(-(offset as isize)) };
                parser_ref.m_bufferPtr =
                    unsafe { parser_ref.m_bufferPtr.offset(-(offset as isize)) };
            }
        } else {
            let mut bufferSize: c_int =
                (if !parser_ref.m_bufferLim.is_null() && !parser_ref.m_buffer.is_null() {
                    unsafe { parser_ref.m_bufferLim.offset_from(parser_ref.m_buffer) as c_long }
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
                parser_ref.m_errorCode = XML_ERROR_NO_MEMORY;
                return NULL;
            }
            let malloc_fcn = parser_ref
                .m_mem
                .malloc_fcn
                .expect("non-null function pointer");
            let newBuf: *mut c_char = malloc_fcn(bufferSize as size_t) as *mut c_char;
            if newBuf.is_null() {
                parser_ref.m_errorCode = XML_ERROR_NO_MEMORY;
                return NULL;
            }
            parser_ref.m_bufferLim = unsafe { newBuf.offset(bufferSize as isize) };
            if !parser_ref.m_bufferPtr.is_null() {
                memcpy_bytes(
                    newBuf as *mut c_void,
                    unsafe { parser_ref.m_bufferPtr.offset(-keep as isize) } as *const c_void,
                    ((if !parser_ref.m_bufferEnd.is_null() && !parser_ref.m_bufferPtr.is_null() {
                        unsafe {
                            parser_ref.m_bufferEnd.offset_from(parser_ref.m_bufferPtr) as c_long
                        }
                    } else {
                        0
                    }) + keep as c_long) as size_t,
                );
                parser_ref
                    .m_mem
                    .free_fcn
                    .expect("non-null function pointer")(
                    parser_ref.m_buffer as *mut c_void
                );
                parser_ref.m_buffer = newBuf;
                let end_offset = if !parser_ref.m_bufferEnd.is_null()
                    && !parser_ref.m_bufferPtr.is_null()
                {
                    unsafe { parser_ref.m_bufferEnd.offset_from(parser_ref.m_bufferPtr) as c_long }
                } else {
                    0
                };
                parser_ref.m_bufferEnd = unsafe {
                    parser_ref
                        .m_buffer
                        .offset(end_offset as isize)
                        .offset(keep as isize)
                };
                parser_ref.m_bufferPtr = unsafe { parser_ref.m_buffer.offset(keep as isize) };
            } else {
                parser_ref.m_bufferEnd = newBuf;
                parser_ref.m_buffer = newBuf;
                parser_ref.m_bufferPtr = parser_ref.m_buffer;
            }
        }
        parser_ref.m_eventEndPtr = null::<c_char>();
        parser_ref.m_eventPtr = parser_ref.m_eventEndPtr;
        parser_ref.m_positionPtr = null::<c_char>();
    }
    parser_ref.m_bufferEnd as *mut c_void
}

extern "C" fn triggerReenter(mut parser: XML_Parser) {
    unsafe { (*parser).m_reenter = XML_TRUE };
}
#[no_mangle]

pub extern "C" fn XML_StopParser(mut parser: XML_Parser, mut resumable: XML_Bool) -> XML_Status {
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    match unsafe { (*parser).m_parsingStatus.parsing } {
        0 => {
            unsafe { (*parser).m_errorCode = XML_ERROR_NOT_STARTED };
            return XML_STATUS_ERROR;
        }
        3 => {
            if resumable != 0 {
                unsafe { (*parser).m_errorCode = XML_ERROR_SUSPENDED };
                return XML_STATUS_ERROR;
            }
            unsafe { (*parser).m_parsingStatus.parsing = XML_FINISHED };
        }
        2 => {
            unsafe { (*parser).m_errorCode = XML_ERROR_FINISHED };
            return XML_STATUS_ERROR;
        }
        1 => {
            if resumable != 0 {
                if unsafe { (*parser).m_isParamEntity != 0 } {
                    unsafe { (*parser).m_errorCode = XML_ERROR_SUSPEND_PE };
                    return XML_STATUS_ERROR;
                }
                unsafe { (*parser).m_parsingStatus.parsing = XML_SUSPENDED };
            } else {
                unsafe { (*parser).m_parsingStatus.parsing = XML_FINISHED };
            }
        }
        _ => {
            assert!(0i32 != 0);
        }
    }
    XML_STATUS_OK
}
#[no_mangle]

pub extern "C" fn XML_ResumeParser(mut parser: XML_Parser) -> XML_Status {
    let mut result: XML_Status = XML_STATUS_OK;
    if parser.is_null() {
        return XML_STATUS_ERROR;
    }
    if unsafe { (*parser).m_parsingStatus.parsing != XML_SUSPENDED } {
        unsafe { (*parser).m_errorCode = XML_ERROR_NOT_SUSPENDED };
        return XML_STATUS_ERROR;
    }
    unsafe { (*parser).m_parsingStatus.parsing = XML_PARSING };
    let buffer_ptr = unsafe { (*parser).m_bufferPtr };
    let parse_end_ptr = unsafe { (*parser).m_parseEndPtr };
    let buffer_ptr_ref = unsafe { &raw mut (*parser).m_bufferPtr };
    let error = callProcessor(parser, buffer_ptr, parse_end_ptr, buffer_ptr_ref);
    unsafe { (*parser).m_errorCode = error };
    if unsafe { (*parser).m_errorCode != XML_ERROR_NONE } {
        let event_ptr = unsafe { (*parser).m_eventPtr };
        unsafe { (*parser).m_eventEndPtr = event_ptr };
        unsafe {
            (*parser).m_processor = Some(
                errorProcessor
                    as extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            )
        };
        return XML_STATUS_ERROR;
    } else {
        match unsafe { (*parser).m_parsingStatus.parsing } {
            3 => {
                result = XML_STATUS_SUSPENDED;
            }
            0 | 1 => {
                if unsafe { (*parser).m_parsingStatus.finalBuffer != 0 } {
                    unsafe { (*parser).m_parsingStatus.parsing = XML_FINISHED };
                    return result;
                }
            }
            _ => {}
        }
    }
    let encoding = unsafe { (*parser).m_encoding };
    let position_ptr = unsafe { (*parser).m_positionPtr };
    let buffer_ptr = unsafe { (*parser).m_bufferPtr };
    let position = unsafe { &raw mut (*parser).m_position };
    unsafe {
        (*encoding).updatePosition(
            &*encoding,
            c_char_slice_from_ptr_end(position_ptr, buffer_ptr),
            position,
        )
    };
    unsafe { (*parser).m_positionPtr = buffer_ptr };
    result
}
#[no_mangle]

pub extern "C" fn XML_GetParsingStatus(mut parser: XML_Parser, mut status: *mut XML_ParsingStatus) {
    if parser.is_null() {
        return;
    }
    assert!(!status.is_null());
    unsafe { *status = (*parser).m_parsingStatus };
}
#[no_mangle]

pub extern "C" fn XML_GetErrorCode(mut parser: XML_Parser) -> XML_Error {
    if parser.is_null() {
        return XML_ERROR_INVALID_ARGUMENT;
    }
    unsafe { (*parser).m_errorCode }
}
#[no_mangle]

pub extern "C" fn XML_GetCurrentByteIndex(mut parser: XML_Parser) -> XML_Index {
    if parser.is_null() {
        return -1i64;
    }
    let event_ptr = unsafe { (*parser).m_eventPtr };
    if !event_ptr.is_null() {
        let parse_end = unsafe { (*parser).m_parseEndPtr };
        let parse_end_byte_index = unsafe { (*parser).m_parseEndByteIndex };
        return parse_end_byte_index - unsafe { parse_end.offset_from(event_ptr) as c_long };
    }
    -1i64
}
#[no_mangle]

pub extern "C" fn XML_GetCurrentByteCount(mut parser: XML_Parser) -> c_int {
    if parser.is_null() {
        return 0i32;
    }
    let event_end_ptr = unsafe { (*parser).m_eventEndPtr };
    let event_ptr = unsafe { (*parser).m_eventPtr };
    if !event_end_ptr.is_null() && !event_ptr.is_null() {
        return unsafe { event_end_ptr.offset_from(event_ptr) as c_int };
    }
    0
}
#[no_mangle]

pub extern "C" fn XML_GetInputContext(
    mut parser: XML_Parser,
    mut offset: *mut c_int,
    mut size: *mut c_int,
) -> *const c_char {
    if parser.is_null() {
        return null::<c_char>();
    }
    let event_ptr = unsafe { (*parser).m_eventPtr };
    let buffer = unsafe { (*parser).m_buffer };
    if !event_ptr.is_null() && !buffer.is_null() {
        if !offset.is_null() {
            unsafe { *offset = event_ptr.offset_from(buffer) as c_int };
        }
        if !size.is_null() {
            let buffer_end = unsafe { (*parser).m_bufferEnd };
            unsafe { *size = buffer_end.offset_from(buffer) as c_int };
        }
        return buffer;
    }
    null::<c_char>()
}
#[no_mangle]

pub extern "C" fn XML_GetCurrentLineNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0u64;
    }
    let event_ptr = unsafe { (*parser).m_eventPtr };
    let position_ptr = unsafe { (*parser).m_positionPtr };
    if !event_ptr.is_null() && event_ptr >= position_ptr {
        let encoding = unsafe { (*parser).m_encoding };
        let position = unsafe { &raw mut (*parser).m_position };
        unsafe {
            (*encoding).updatePosition(
                &*encoding,
                c_char_slice_from_ptr_end(position_ptr, event_ptr),
                position,
            )
        };
        unsafe { (*parser).m_positionPtr = event_ptr };
    }
    let line = unsafe { (*parser).m_position.lineNumber };
    line.wrapping_add(1u64)
}
#[no_mangle]

pub extern "C" fn XML_GetCurrentColumnNumber(mut parser: XML_Parser) -> XML_Size {
    if parser.is_null() {
        return 0u64;
    }
    let event_ptr = unsafe { (*parser).m_eventPtr };
    let position_ptr = unsafe { (*parser).m_positionPtr };
    if !event_ptr.is_null() && event_ptr >= position_ptr {
        let encoding = unsafe { (*parser).m_encoding };
        let position = unsafe { &raw mut (*parser).m_position };
        unsafe {
            (*encoding).updatePosition(
                &*encoding,
                c_char_slice_from_ptr_end(position_ptr, event_ptr),
                position,
            )
        };
        unsafe { (*parser).m_positionPtr = event_ptr };
    }
    unsafe { (*parser).m_position.columnNumber }
}
#[no_mangle]

pub extern "C" fn XML_FreeContentModel(mut parser: XML_Parser, mut model: *mut XML_Content) {
    if parser.is_null() {
        return;
    }
    let free_fcn = unsafe { (*parser).m_mem.free_fcn.expect("non-null function pointer") };
    free_fcn(model as *mut c_void);
}
#[no_mangle]

pub extern "C" fn XML_MemMalloc(mut parser: XML_Parser, mut size: size_t) -> *mut c_void {
    if parser.is_null() {
        return NULL;
    }
    let malloc_fcn = unsafe {
        (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")
    };
    malloc_fcn(size)
}
#[no_mangle]

pub extern "C" fn XML_MemRealloc(
    mut parser: XML_Parser,
    mut ptr: *mut c_void,
    mut size: size_t,
) -> *mut c_void {
    if parser.is_null() {
        return NULL;
    }
    let realloc_fcn = unsafe {
        (*parser)
            .m_mem
            .realloc_fcn
            .expect("non-null function pointer")
    };
    realloc_fcn(ptr, size)
}
#[no_mangle]

pub extern "C" fn XML_MemFree(mut parser: XML_Parser, mut ptr: *mut c_void) {
    if parser.is_null() {
        return;
    }
    let free_fcn = unsafe { (*parser).m_mem.free_fcn.expect("non-null function pointer") };
    free_fcn(ptr);
}
#[no_mangle]

pub extern "C" fn XML_DefaultCurrent(mut parser: XML_Parser) {
    if parser.is_null() {
        return;
    }
    if unsafe { (*parser).m_defaultHandler.is_some() } {
        let open_internal_entities = unsafe { (*parser).m_openInternalEntities };
        if !open_internal_entities.is_null() {
            let internal_encoding = unsafe { (*parser).m_internalEncoding };
            let internal_event_ptr = unsafe { (*open_internal_entities).internalEventPtr };
            let internal_event_end_ptr = unsafe { (*open_internal_entities).internalEventEndPtr };
            reportDefault(
                parser,
                unsafe { &*internal_encoding },
                internal_event_ptr,
                internal_event_end_ptr,
            );
        } else {
            let encoding = unsafe { (*parser).m_encoding };
            let event_ptr = unsafe { (*parser).m_eventPtr };
            let event_end_ptr = unsafe { (*parser).m_eventEndPtr };
            reportDefault(parser, unsafe { &*encoding }, event_ptr, event_end_ptr);
        }
    }
}
#[no_mangle]

pub extern "C" fn XML_ErrorString(mut code: XML_Error) -> *const XML_LChar {
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
    null::<XML_LChar>()
}
#[no_mangle]

pub extern "C" fn XML_ExpatVersion() -> *const XML_LChar {
    b"expat_2.7.4\0" as *const u8 as *const XML_LChar
}
#[no_mangle]

pub extern "C" fn XML_ExpatVersionInfo() -> XML_Expat_Version {
    let mut version: XML_Expat_Version = XML_Expat_Version {
        major: 0,
        minor: 0,
        micro: 0,
    };
    version.major = XML_MAJOR_VERSION;
    version.minor = XML_MINOR_VERSION;
    version.micro = XML_MICRO_VERSION;
    version
}
#[no_mangle]

pub extern "C" fn XML_GetFeatureList() -> *const XML_Feature {
    const FEATURES: [XML_Feature; 11] = [
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
    FEATURES.as_ptr()
}
#[no_mangle]

pub extern "C" fn XML_SetBillionLaughsAttackProtectionMaximumAmplification(
    mut parser: XML_Parser,
    mut maximumAmplificationFactor: c_float,
) -> XML_Bool {
    if parser.is_null()
        || unsafe { !(*parser).m_parentParser.is_null() }
        || maximumAmplificationFactor.is_nan() as i32 != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return XML_FALSE;
    }
    unsafe { (*parser).m_accounting.maximumAmplificationFactor = maximumAmplificationFactor };
    XML_TRUE
}
#[no_mangle]

pub extern "C" fn XML_SetBillionLaughsAttackProtectionActivationThreshold(
    mut parser: XML_Parser,
    mut activationThresholdBytes: c_ulonglong,
) -> XML_Bool {
    if parser.is_null() || unsafe { !(*parser).m_parentParser.is_null() } {
        return XML_FALSE;
    }
    unsafe { (*parser).m_accounting.activationThresholdBytes = activationThresholdBytes };
    XML_TRUE
}
#[no_mangle]

pub extern "C" fn XML_SetAllocTrackerMaximumAmplification(
    mut parser: XML_Parser,
    mut maximumAmplificationFactor: c_float,
) -> XML_Bool {
    if parser.is_null()
        || unsafe { !(*parser).m_parentParser.is_null() }
        || maximumAmplificationFactor.is_nan() as i32 != 0
        || maximumAmplificationFactor < 1.0f32
    {
        return XML_FALSE;
    }
    unsafe { (*parser).m_alloc_tracker.maximumAmplificationFactor = maximumAmplificationFactor };
    XML_TRUE
}
#[no_mangle]

pub extern "C" fn XML_SetAllocTrackerActivationThreshold(
    mut parser: XML_Parser,
    mut activationThresholdBytes: c_ulonglong,
) -> XML_Bool {
    if parser.is_null() || unsafe { !(*parser).m_parentParser.is_null() } {
        return XML_FALSE;
    }
    unsafe { (*parser).m_alloc_tracker.activationThresholdBytes = activationThresholdBytes };
    XML_TRUE
}
#[no_mangle]

pub extern "C" fn XML_SetReparseDeferralEnabled(
    mut parser: XML_Parser,
    mut enabled: XML_Bool,
) -> XML_Bool {
    if !parser.is_null()
        && (enabled as c_int == XML_TRUE as c_int || enabled as c_int == XML_FALSE as c_int)
    {
        unsafe { (*parser).m_reparseDeferralEnabled = enabled };
        return XML_TRUE;
    }
    XML_FALSE
}

extern "C" fn storeRawNames(mut parser: XML_Parser) -> XML_Bool {
    let parser_ref = unsafe { &mut *parser };
    let mut tag: *mut TAG = if let Some(tag) = parser_ref.m_tagStack.as_mut() {
        &mut **tag
    } else {
        null_mut::<TAG>()
    };
    while !tag.is_null() {
        let mut bufSize: size_t = 0;
        let name_len_chars = unsafe { (*tag).name.strLen + 1 };
        let nameLen: size_t = (size_of::<XML_Char>()).wrapping_mul(name_len_chars as size_t);
        let raw_name_len = unsafe { (*tag).rawNameLength as usize };
        let mut rawNameLen: size_t = raw_name_len
            .wrapping_add((size_of::<XML_Char>()).wrapping_sub(1usize))
            & !(size_of::<XML_Char>()).wrapping_sub(1usize);
        let tag_buf_raw = unsafe { (*tag).buf.raw };
        let mut rawNameBuf: *mut c_char = unsafe { tag_buf_raw.add(nameLen) };
        if std::ptr::eq(unsafe { (*tag).rawName }, rawNameBuf) {
            break;
        }
        if rawNameLen > (INT_MAX as size_t).wrapping_sub(nameLen) {
            return XML_FALSE;
        }
        bufSize = nameLen.wrapping_add(rawNameLen);
        if bufSize > unsafe { (*tag).bufEnd.offset_from((*tag).buf.raw) as size_t } {
            let mut temp: *mut c_char = expat_realloc(
                parser,
                unsafe { (*tag).buf.raw as *mut c_void },
                bufSize,
                3151,
            ) as *mut c_char;
            if temp.is_null() {
                return XML_FALSE;
            }
            if std::ptr::eq(unsafe { (*tag).name.str_0 }, unsafe { (*tag).buf.str_0 }) {
                unsafe { (*tag).name.str_0 = temp };
            }
            if !unsafe { (*tag).name.localPart.is_null() } {
                let local_part_offset =
                    unsafe { (*tag).name.localPart.offset_from((*tag).buf.str_0) };
                unsafe { (*tag).name.localPart = temp.offset(local_part_offset) };
            }
            unsafe {
                (*tag).buf.raw = temp;
                (*tag).bufEnd = temp.add(bufSize);
            }
            rawNameBuf = unsafe { temp.add(nameLen) };
        }
        unsafe {
            memcpy(
                rawNameBuf as *mut c_void,
                (*tag).rawName as *const c_void,
                (*tag).rawNameLength as size_t,
            )
        };
        unsafe { (*tag).rawName = rawNameBuf };
        tag = if let Some(parent) = unsafe { (*tag).parent.as_mut() } {
            &mut **parent
        } else {
            null_mut::<TAG>()
        };
    }
    XML_TRUE
}

extern "C" fn contentProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let has_parent = unsafe { !(*parser).m_parentParser.is_null() };
    let encoding = unsafe { (*parser).m_encoding };
    let is_final_buffer = unsafe { (*parser).m_parsingStatus.finalBuffer != 0 };
    let mut result: XML_Error = unsafe {
        doContent(
            parser,
            if has_parent { 1 } else { 0 },
            &*encoding,
            start,
            end,
            endPtr,
            (!is_final_buffer) as XML_Bool,
            XML_ACCOUNT_DIRECT,
        )
    };
    if result == XML_ERROR_NONE && storeRawNames(parser) == 0 {
        return XML_ERROR_NO_MEMORY;
    }
    result
}

extern "C" fn externalEntityInitProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let mut result: XML_Error = initializeEncoding(parser);
    if result != XML_ERROR_NONE {
        return result;
    }
    let parser_ref = unsafe { &mut *parser };
    parser_ref.m_processor = Some(
        externalEntityInitProcessor2
            as extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    externalEntityInitProcessor2(parser, start, end, endPtr)
}

extern "C" fn externalEntityInitProcessor2(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let parser_ref = unsafe { &mut *parser };
    let encoding = parser_ref.m_encoding;
    let mut next: *const c_char = start;
    let tok: c_int = {
        let (tok_value, next_tok_value) =
            unsafe { (*encoding).scanners[1](&*encoding, c_char_slice_from_ptr_end(start, end)) };
        next = next_tok_value;
        tok_value
    };
    match tok {
        XML_TOK_BOM => {
            if accountingDiffTolerated(parser, tok, start, next, 3208, XML_ACCOUNT_DIRECT) == 0 {
                accountingOnAbort(parser);
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            if next == end && parser_ref.m_parsingStatus.finalBuffer == 0 {
                unsafe { *endPtr = next };
                return XML_ERROR_NONE;
            }
            start = next;
        }
        XML_TOK_PARTIAL => {
            if parser_ref.m_parsingStatus.finalBuffer == 0 {
                unsafe { *endPtr = start };
                return XML_ERROR_NONE;
            }
            parser_ref.m_eventPtr = start;
            return XML_ERROR_UNCLOSED_TOKEN;
        }
        XML_TOK_PARTIAL_CHAR => {
            if parser_ref.m_parsingStatus.finalBuffer == 0 {
                unsafe { *endPtr = start };
                return XML_ERROR_NONE;
            }
            parser_ref.m_eventPtr = start;
            return XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    parser_ref.m_processor = Some(
        externalEntityInitProcessor3
            as extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    externalEntityInitProcessor3(parser, start, end, endPtr)
}

extern "C" fn externalEntityInitProcessor3(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let parser_ref = unsafe { &mut *parser };
    let encoding = parser_ref.m_encoding;
    parser_ref.m_eventPtr = start;
    let mut next: *const c_char = start;
    let tok: c_int = {
        let (tok_value, next_tok_value) =
            unsafe { (*encoding).scanners[1](&*encoding, c_char_slice_from_ptr_end(start, end)) };
        next = next_tok_value;
        tok_value
    };
    parser_ref.m_eventEndPtr = next;
    match tok {
        XML_TOK_XML_DECL => {
            let result: XML_Error = processXmlDecl(parser, 1, start, next);
            if result != XML_ERROR_NONE {
                return result;
            }
            match parser_ref.m_parsingStatus.parsing {
                3 => {
                    unsafe { *endPtr = next };
                    return XML_ERROR_NONE;
                }
                2 => return XML_ERROR_ABORTED,
                1 => {
                    if parser_ref.m_reenter != 0 {
                        return XML_ERROR_UNEXPECTED_STATE;
                    }
                }
                _ => {}
            }
            start = next;
        }
        XML_TOK_PARTIAL => {
            if parser_ref.m_parsingStatus.finalBuffer == 0 {
                unsafe { *endPtr = start };
                return XML_ERROR_NONE;
            }
            return XML_ERROR_UNCLOSED_TOKEN;
        }
        XML_TOK_PARTIAL_CHAR => {
            if parser_ref.m_parsingStatus.finalBuffer == 0 {
                unsafe { *endPtr = start };
                return XML_ERROR_NONE;
            }
            return XML_ERROR_PARTIAL_CHAR;
        }
        _ => {}
    }
    parser_ref.m_processor = Some(
        externalEntityContentProcessor
            as extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    parser_ref.m_tagLevel = 1;
    externalEntityContentProcessor(parser, start, end, endPtr)
}

extern "C" fn externalEntityContentProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let parser_ref = unsafe { &mut *parser };
    let encoding = parser_ref.m_encoding;
    let mut result: XML_Error = unsafe {
        doContent(
            parser,
            1,
            &*encoding,
            start,
            end,
            endPtr,
            (parser_ref.m_parsingStatus.finalBuffer == 0) as XML_Bool,
            XML_ACCOUNT_ENTITY_EXPANSION,
        )
    };
    if result == XML_ERROR_NONE && storeRawNames(parser) == 0 {
        return XML_ERROR_NO_MEMORY;
    }
    result
}

unsafe extern "C" fn doContent(
    mut parser: XML_Parser,
    mut startTagLevel: c_int,
    enc: &ENCODING,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
    mut account: XML_Account,
) -> XML_Error {
    let parser_ref = &mut *parser;
    let dtd: *mut DTD = parser_ref.m_dtd;
    let mut eventPP: *mut *const c_char = null_mut::<*const c_char>();
    let mut eventEndPP: *mut *const c_char = null_mut::<*const c_char>();
    if core::ptr::eq(enc, &*parser_ref.m_encoding) {
        eventPP = &raw mut parser_ref.m_eventPtr;
        eventEndPP = &raw mut parser_ref.m_eventEndPtr;
    } else {
        eventPP = &raw mut (*parser_ref.m_openInternalEntities).internalEventPtr;
        eventEndPP = &raw mut (*parser_ref.m_openInternalEntities).internalEventEndPtr;
    }
    *eventPP = s;
    loop {
        let mut next: *const c_char = s;
        let mut tok: c_int = {
            let (tok_value, next_tok_value) =
                enc.scanners[1](enc, c_char_slice_from_ptr_end(s, end));
            next = next_tok_value;
            tok_value
        };
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
                if parser_ref.m_characterDataHandler.is_some() {
                    let mut c: XML_Char = 0xa;
                    parser_ref
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        parser_ref.m_handlerArg,
                        &raw mut c,
                        1i32,
                    );
                } else if parser_ref.m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                if startTagLevel == 0 {
                    return XML_ERROR_NO_ELEMENTS;
                }
                if parser_ref.m_tagLevel != startTagLevel {
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
                    if parser_ref.m_tagLevel != startTagLevel {
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
                let mut ch: XML_Char = (*enc).predefinedEntityName(
                    enc,
                    c_char_slice_from_ptr_end(
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
                    ),
                ) as XML_Char;
                if ch != 0 {
                    accountingDiffTolerated(
                        parser,
                        tok,
                        &raw mut ch,
                        (&raw mut ch).add(size_of::<XML_Char>()),
                        3403,
                        XML_ACCOUNT_ENTITY_EXPANSION,
                    );
                    if parser_ref.m_characterDataHandler.is_some() {
                        parser_ref
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            parser_ref.m_handlerArg,
                            &raw mut ch,
                            1i32,
                        );
                    } else if parser_ref.m_defaultHandler.is_some() {
                        reportDefault(parser, enc, s, next);
                    }
                } else {
                    name = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
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
                        if parser_ref.m_skippedEntityHandler.is_some() {
                            parser_ref
                                .m_skippedEntityHandler
                                .expect("non-null function pointer")(
                                parser_ref.m_handlerArg,
                                name,
                                0i32,
                            );
                        } else if parser_ref.m_defaultHandler.is_some() {
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
                                if parser_ref.m_defaultExpandInternalEntities == 0 {
                                    if parser_ref.m_skippedEntityHandler.is_some() {
                                        parser_ref
                                            .m_skippedEntityHandler
                                            .expect("non-null function pointer")(
                                            parser_ref.m_handlerArg,
                                            (*entity).name,
                                            0i32,
                                        );
                                    } else if parser_ref.m_defaultHandler.is_some() {
                                        reportDefault(parser, enc, s, next);
                                    }
                                } else {
                                    result =
                                        processEntity(parser, entity, XML_FALSE, ENTITY_INTERNAL);
                                    if result != XML_ERROR_NONE {
                                        return result;
                                    }
                                }
                            } else if parser_ref.m_externalEntityRefHandler.is_some() {
                                let mut context: *const XML_Char = null::<XML_Char>();
                                (*entity).open = XML_TRUE;
                                context = getContext(parser);
                                (*entity).open = XML_FALSE;
                                if context.is_null() {
                                    return XML_ERROR_NO_MEMORY;
                                }
                                if parser_ref
                                    .m_externalEntityRefHandler
                                    .expect("non-null function pointer")(
                                    parser_ref.m_externalEntityRefHandlerArg,
                                    context,
                                    (*entity).base,
                                    (*entity).systemId,
                                    (*entity).publicId,
                                ) == 0
                                {
                                    return XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                }
                                parser_ref.m_tempPool.ptr = parser_ref.m_tempPool.start;
                            } else if parser_ref.m_defaultHandler.is_some() {
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
                if let Some(mut free_tag) = parser_ref.m_freeTagList.take() {
                    parser_ref.m_freeTagList = free_tag.parent.take();
                    free_tag.parent = parser_ref.m_tagStack.take();
                    parser_ref.m_tagStack = Some(free_tag);
                } else {
                    let mut tag_buf: *mut c_char = expat_malloc(parser, 32, 3480) as *mut c_char;
                    if tag_buf.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    let mut new_tag: Box<TAG> = Box::new(TAG {
                        parent: parser_ref.m_tagStack.take(),
                        rawName: null::<c_char>(),
                        rawNameLength: 0,
                        name: TAG_NAME {
                            str_0: null::<XML_Char>(),
                            localPart: null::<XML_Char>(),
                            prefix: null::<XML_Char>(),
                            strLen: 0,
                            uriLen: 0,
                            prefixLen: 0,
                        },
                        buf: C2RustUnnamed_1 { raw: tag_buf },
                        bufEnd: tag_buf.offset(INIT_TAG_BUF_SIZE as isize),
                        bindings: null_mut::<BINDING>(),
                    });
                    parser_ref.m_tagStack = Some(new_tag);
                }
                tag = if let Some(tag_on_stack) = parser_ref.m_tagStack.as_mut() {
                    &mut **tag_on_stack
                } else {
                    return XML_ERROR_UNEXPECTED_STATE;
                };
                (*tag).bindings = null_mut::<BINDING>();
                (*tag).name.localPart = null::<XML_Char>();
                (*tag).name.prefix = null::<XML_Char>();
                (*tag).rawName = s.offset(enc.minBytesPerChar as isize);
                (*tag).rawNameLength = (*enc).nameLength(enc, (*tag).rawName);
                parser_ref.m_tagLevel += 1;
                let mut rawNameEnd: *const c_char =
                    (*tag).rawName.offset((*tag).rawNameLength as isize);
                let mut fromPtr: *const c_char = (*tag).rawName;
                toPtr = (*tag).buf.str_0;
                loop {
                    let mut convLen: c_int = 0;
                    let convert_res: XML_Convert_Result;
                    (convert_res, fromPtr, toPtr) = (*enc).utf8Convert(
                        enc,
                        fromPtr,
                        rawNameEnd,
                        toPtr,
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
                        (*tag).bufEnd = temp.add(bufSize);
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
                if parser_ref.m_startElementHandler.is_some() {
                    parser_ref
                        .m_startElementHandler
                        .expect("non-null function pointer")(
                        parser_ref.m_handlerArg,
                        (*tag).name.str_0,
                        parser_ref.m_atts as *mut *const XML_Char,
                    );
                } else if parser_ref.m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&raw mut parser_ref.m_tempPool);
            }
            XML_TOK_EMPTY_ELEMENT_NO_ATTS | XML_TOK_EMPTY_ELEMENT_WITH_ATTS => {
                let mut rawName: *const c_char = s.offset(enc.minBytesPerChar as isize);
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
                    &raw mut parser_ref.m_tempPool,
                    enc,
                    rawName,
                    rawName.offset((*enc).nameLength(enc, rawName) as isize),
                );
                if name_0.str_0.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
                parser_ref.m_tempPool.start = parser_ref.m_tempPool.ptr;
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
                parser_ref.m_tempPool.start = parser_ref.m_tempPool.ptr;
                if parser_ref.m_startElementHandler.is_some() {
                    parser_ref
                        .m_startElementHandler
                        .expect("non-null function pointer")(
                        parser_ref.m_handlerArg,
                        name_0.str_0,
                        parser_ref.m_atts as *mut *const XML_Char,
                    );
                    noElmHandlers = XML_FALSE;
                }
                if parser_ref.m_endElementHandler.is_some() {
                    if parser_ref.m_startElementHandler.is_some() {
                        *eventPP = *eventEndPP;
                    }
                    parser_ref
                        .m_endElementHandler
                        .expect("non-null function pointer")(
                        parser_ref.m_handlerArg, name_0.str_0
                    );
                    noElmHandlers = XML_FALSE;
                }
                if noElmHandlers as c_int != 0 && parser_ref.m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                poolClear(&raw mut parser_ref.m_tempPool);
                freeBindings(parser, bindings);
                if parser_ref.m_tagLevel == 0 && parser_ref.m_parsingStatus.parsing != XML_FINISHED
                {
                    if parser_ref.m_parsingStatus.parsing == XML_SUSPENDED
                        || parser_ref.m_parsingStatus.parsing == XML_PARSING
                            && parser_ref.m_reenter as c_int != 0
                    {
                        parser_ref.m_processor = Some(
                            epilogProcessor
                                as extern "C" fn(
                                    XML_Parser,
                                    *const c_char,
                                    *const c_char,
                                    *mut *const c_char,
                                ) -> XML_Error,
                        );
                    } else {
                        return epilogProcessor(parser, next, end, nextPtr);
                    }
                }
            }
            XML_TOK_END_TAG => {
                if parser_ref.m_tagLevel == startTagLevel {
                    return XML_ERROR_ASYNC_ENTITY;
                } else {
                    let mut len: c_int = 0;
                    let mut rawName_0: *const c_char = null::<c_char>();
                    let mut tag_0: *mut TAG = if let Some(tag) = parser_ref.m_tagStack.as_mut() {
                        &mut **tag
                    } else {
                        return XML_ERROR_UNEXPECTED_STATE;
                    };
                    rawName_0 = s.offset((enc.minBytesPerChar * 2i32) as isize);
                    len = (*enc).nameLength(enc, rawName_0);
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
                    let mut closed_tag: Box<TAG> = parser_ref
                        .m_tagStack
                        .take()
                        .expect("tag stack cannot be empty here");
                    tag_0 = &mut *closed_tag;
                    parser_ref.m_tagStack = closed_tag.parent.take();
                    closed_tag.parent = parser_ref.m_freeTagList.take();
                    parser_ref.m_freeTagList = Some(closed_tag);
                    parser_ref.m_tagLevel -= 1;
                    if parser_ref.m_endElementHandler.is_some() {
                        let mut localPart: *const XML_Char = null::<XML_Char>();
                        let mut prefix: *const XML_Char = null::<XML_Char>();
                        let mut uri: *mut XML_Char = null_mut::<XML_Char>();
                        localPart = (*tag_0).name.localPart;
                        if parser_ref.m_ns as c_int != 0 && !localPart.is_null() {
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
                            if parser_ref.m_ns_triplets as c_int != 0 && !prefix.is_null() {
                                let fresh24 = uri;
                                uri = uri.offset(1);
                                *fresh24 = parser_ref.m_namespaceSeparator;
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
                        parser_ref
                            .m_endElementHandler
                            .expect("non-null function pointer")(
                            parser_ref.m_handlerArg,
                            (*tag_0).name.str_0,
                        );
                    } else if parser_ref.m_defaultHandler.is_some() {
                        reportDefault(parser, enc, s, next);
                    }
                    while !(*tag_0).bindings.is_null() {
                        let mut b: *mut BINDING = (*tag_0).bindings;
                        if parser_ref.m_endNamespaceDeclHandler.is_some() {
                            parser_ref
                                .m_endNamespaceDeclHandler
                                .expect("non-null function pointer")(
                                parser_ref.m_handlerArg,
                                (*(*b).prefix).name,
                            );
                        }
                        (*tag_0).bindings = (*(*tag_0).bindings).nextTagBinding;
                        (*b).nextTagBinding = parser_ref.m_freeBindingList;
                        parser_ref.m_freeBindingList = b;
                        (*(*b).prefix).binding = (*b).prevPrefixBinding;
                    }
                    if parser_ref.m_tagLevel == 0
                        && parser_ref.m_parsingStatus.parsing != XML_FINISHED
                    {
                        if parser_ref.m_parsingStatus.parsing == XML_SUSPENDED
                            || parser_ref.m_parsingStatus.parsing == XML_PARSING
                                && parser_ref.m_reenter as c_int != 0
                        {
                            parser_ref.m_processor = Some(
                                epilogProcessor
                                    as extern "C" fn(
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
                let mut n: c_int = (*enc).charRefNumber(enc, s);
                if n < 0 {
                    return XML_ERROR_BAD_CHAR_REF;
                }
                if parser_ref.m_characterDataHandler.is_some() {
                    let mut buf: [XML_Char; 4] = [0; 4];
                    parser_ref
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        parser_ref.m_handlerArg,
                        &raw mut buf as *mut XML_Char,
                        XmlUtf8Encode(n, &raw mut buf as *mut c_char),
                    );
                } else if parser_ref.m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_XML_DECL => return XML_ERROR_MISPLACED_XML_PI,
            XML_TOK_DATA_NEWLINE => {
                if parser_ref.m_characterDataHandler.is_some() {
                    let mut c_0: XML_Char = 0xa;
                    parser_ref
                        .m_characterDataHandler
                        .expect("non-null function pointer")(
                        parser_ref.m_handlerArg,
                        &raw mut c_0,
                        1i32,
                    );
                } else if parser_ref.m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_CDATA_SECT_OPEN => {
                let mut result_2: XML_Error = XML_ERROR_NONE;
                if parser_ref.m_startCdataSectionHandler.is_some() {
                    parser_ref
                        .m_startCdataSectionHandler
                        .expect("non-null function pointer")(
                        parser_ref.m_handlerArg
                    );
                } else if parser_ref.m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
                result_2 =
                    doCdataSection(parser, enc, &raw mut next, end, nextPtr, haveMore, account);
                if result_2 != XML_ERROR_NONE {
                    return result_2;
                } else if next.is_null() {
                    parser_ref.m_processor = Some(
                        cdataSectionProcessor
                            as extern "C" fn(
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
                if parser_ref.m_characterDataHandler.is_some() {
                    if enc.isUtf8 == 0 {
                        let mut dataPtr: *mut ICHAR = parser_ref.m_dataBuf;
                        (_, s, dataPtr) =
                            (*enc).utf8Convert(enc, s, end, dataPtr, parser_ref.m_dataBufEnd);
                        parser_ref
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            parser_ref.m_handlerArg,
                            parser_ref.m_dataBuf,
                            dataPtr.offset_from(parser_ref.m_dataBuf) as c_int,
                        );
                    } else {
                        parser_ref
                            .m_characterDataHandler
                            .expect("non-null function pointer")(
                            parser_ref.m_handlerArg,
                            s,
                            (end).offset_from(s) as c_int,
                        );
                    }
                } else if parser_ref.m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, end);
                }
                if startTagLevel == 0 {
                    *eventPP = end;
                    return XML_ERROR_NO_ELEMENTS;
                }
                if parser_ref.m_tagLevel != startTagLevel {
                    *eventPP = end;
                    return XML_ERROR_ASYNC_ENTITY;
                }
                *nextPtr = end;
                return XML_ERROR_NONE;
            }
            XML_TOK_DATA_CHARS => {
                let mut charDataHandler: XML_CharacterDataHandler =
                    parser_ref.m_characterDataHandler;
                if charDataHandler.is_some() {
                    if enc.isUtf8 == 0 {
                        loop {
                            let mut dataPtr_0: *mut ICHAR = parser_ref.m_dataBuf;
                            let convert_res_0: XML_Convert_Result;
                            (convert_res_0, s, dataPtr_0) = (*enc).utf8Convert(
                                enc,
                                s,
                                next,
                                dataPtr_0,
                                parser_ref.m_dataBufEnd,
                            );
                            *eventEndPP = s;
                            charDataHandler.expect("non-null function pointer")(
                                parser_ref.m_handlerArg,
                                parser_ref.m_dataBuf,
                                dataPtr_0.offset_from(parser_ref.m_dataBuf) as c_int,
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
                            parser_ref.m_handlerArg,
                            s,
                            (next).offset_from(s) as c_int,
                        );
                    }
                } else if parser_ref.m_defaultHandler.is_some() {
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
                if parser_ref.m_defaultHandler.is_some() {
                    reportDefault(parser, enc, s, next);
                }
            }
        }
        match parser_ref.m_parsingStatus.parsing {
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
                if parser_ref.m_reenter != 0 {
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

extern "C" fn freeBindings(mut parser: XML_Parser, mut bindings: *mut BINDING) {
    while !bindings.is_null() {
        let mut b: *mut BINDING = bindings;
        if unsafe { (*parser).m_endNamespaceDeclHandler.is_some() } {
            let end_namespace_decl_handler = unsafe {
                (*parser)
                    .m_endNamespaceDeclHandler
                    .expect("non-null function pointer")
            };
            let handler_arg = unsafe { (*parser).m_handlerArg };
            let prefix = unsafe { (*b).prefix };
            end_namespace_decl_handler(handler_arg, unsafe { (*prefix).name });
        }
        bindings = unsafe { (*bindings).nextTagBinding };
        let free_binding_list = unsafe { (*parser).m_freeBindingList };
        unsafe { (*b).nextTagBinding = free_binding_list };
        unsafe { (*parser).m_freeBindingList = b };
        let prefix = unsafe { (*b).prefix };
        let prev_prefix_binding = unsafe { (*b).prevPrefixBinding };
        unsafe { (*prefix).binding = prev_prefix_binding };
    }
}

unsafe extern "C" fn storeAtts(
    mut parser: XML_Parser,
    enc: &ENCODING,
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
    n = (*enc).getAtts(enc, attStr, (*parser).m_attsSize, (*parser).m_atts);
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
            (*enc).getAtts(enc, attStr, n, (*parser).m_atts);
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
                .offset((*enc).nameLength(enc, (*currAtt).name) as isize),
        );
        if attId.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        if *(*attId).name.offset(-1) != 0 {
            if core::ptr::eq(enc, &*(*parser).m_encoding) {
                (*parser).m_eventPtr = (*(*parser).m_atts.offset(i as isize)).name;
            }
            return XML_ERROR_DUPLICATE_ATTRIBUTE;
        }
        *(*attId).name.offset(-1) = 1i8;
        let fresh27 = attIndex;
        attIndex += 1;
        let fresh28 = &mut *appAtts.offset(fresh27 as isize);
        *fresh28 = (*attId).name;
        if (*(*parser).m_atts.offset(i as isize)).normalized == 0 {
            let mut result: XML_Error = XML_ERROR_NONE;
            let mut isCdata: XML_Bool = XML_TRUE;
            if (*attId).maybeTokenized != 0 {
                let mut j: c_int = 0;
                j = 0;
                while j < nDefaultAtts {
                    if std::ptr::eq(attId, (*(*elementType).defaultAtts.offset(j as isize)).id) {
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
            let fresh29 = &mut *appAtts.offset(attIndex as isize);
            *fresh29 = (*parser).m_tempPool.start;
            (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
        } else {
            let fresh30 = &mut *appAtts.offset(attIndex as isize);
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
            if std::ptr::eq(*appAtts.offset(i as isize), (*(*elementType).idAtt).name) {
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
                    attIndex += 1;
                    let fresh32 = &mut *appAtts.offset(fresh31 as isize);
                    *fresh32 = (*(*da).id).name;
                    let fresh33 = attIndex;
                    attIndex += 1;
                    let fresh34 = &mut *appAtts.offset(fresh33 as isize);
                    *fresh34 = (*da).value;
                }
            } else {
                *(*(*da).id).name.offset(-1) = 1i8;
                let fresh35 = attIndex;
                attIndex += 1;
                let fresh36 = &mut *appAtts.offset(fresh35 as isize);
                *fresh36 = (*(*da).id).name;
                let fresh37 = attIndex;
                attIndex += 1;
                let fresh38 = &mut *appAtts.offset(fresh37 as isize);
                *fresh38 = (*da).value;
            }
        }
        i += 1;
    }
    let fresh39 = &mut *appAtts.offset(attIndex as isize);
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
                if nPrefixes >> fresh40 as c_int == 0 {
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
                    if if std::ptr::eq((*parser).m_tempPool.ptr, (*parser).m_tempPool.end)
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
                    if *fresh42 as c_int == 0x3a {
                        break;
                    }
                }
                sip24_update(
                    &raw mut sip_state,
                    s as *const c_void,
                    keylen(s).wrapping_mul(size_of::<XML_Char>()),
                );
                loop {
                    if if std::ptr::eq((*parser).m_tempPool.ptr, (*parser).m_tempPool.end)
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
                    if *fresh44 == 0 {
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
                        step = ((uriHash & !mask) >> ((*parser).m_nsAttsPower as c_int - 1i32)
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
                        if if std::ptr::eq((*parser).m_tempPool.ptr, (*parser).m_tempPool.end)
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
                        if *fresh46 == 0 {
                            break;
                        }
                    }
                }
                s = (*parser).m_tempPool.start;
                (*parser).m_tempPool.start = (*parser).m_tempPool.ptr;
                let fresh47 = &mut *appAtts.offset(i as isize);
                *fresh47 = s;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).version = version;
                (*(*parser).m_nsAtts.offset(j_0 as isize)).hash = uriHash;
                let fresh48 = &mut (*(*parser).m_nsAtts.offset(j_0 as isize)).uriName;
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
            if *fresh49 as c_int == 0x3a {
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
            prefixLen += 1;
            if *(*(*binding).prefix).name.offset(fresh50 as isize) == 0 {
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
        i += 1;
        if *localPart.offset(fresh51 as isize) == 0 {
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
        p = if let Some(tag) = (*parser).m_tagStack.as_mut() {
            &mut **tag
        } else {
            null_mut::<TAG>()
        };
        while !p.is_null() {
            if std::ptr::eq((*p).name.str_0, (*binding).uri) {
                (*p).name.str_0 = uri;
            }
            p = if let Some(parent) = (*p).parent.as_mut() {
                &mut **parent
            } else {
                null_mut::<TAG>()
            };
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
    XML_ERROR_NONE
}

extern "C" fn is_rfc3986_uri_char(mut candidate: XML_Char) -> XML_Bool {
    match candidate as c_int {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104
        | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118
        | 119 | 120 | 121 | 122 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 37 | 45
        | 46 | 95 | 126 | 58 | 47 | 63 | 35 | 91 | 93 | 64 | 33 | 36 | 38 | 39 | 40 | 41 | 42
        | 43 | 44 | 59 | 61 => XML_TRUE,
        _ => XML_FALSE,
    }
}

extern "C" fn addBinding(
    mut parser: XML_Parser,
    mut prefix: *mut PREFIX,
    mut attId: *const ATTRIBUTE_ID,
    mut uri: *const XML_Char,
    mut bindingsPtr: *mut *mut BINDING,
) -> XML_Error {
    static xmlNamespace: [XML_Char; 37] = [
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
    static xmlnsNamespace: [XML_Char; 30] = [
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
    if unsafe { *uri as c_int == '\0' as i32 } && unsafe { !(*prefix).name.is_null() } {
        return XML_ERROR_UNDECLARING_PREFIX;
    }
    if unsafe { !(*prefix).name.is_null() }
        && unsafe { *(*prefix).name.offset(0) as c_int == 0x78 }
        && unsafe { *(*prefix).name.offset(1) as c_int == 0x6d }
        && unsafe { *(*prefix).name.offset(2) as c_int == 0x6c }
    {
        if unsafe { *(*prefix).name.offset(3) as c_int == 0x6e }
            && unsafe { *(*prefix).name.offset(4) as c_int == 0x73 }
            && unsafe { *(*prefix).name.offset(5) as c_int == '\0' as i32 }
        {
            return XML_ERROR_RESERVED_PREFIX_XMLNS;
        }
        if unsafe { *(*prefix).name.offset(3) as c_int == '\0' as i32 } {
            mustBeXML = XML_TRUE;
        }
    }
    len = 0;
    while unsafe { *uri.offset(len as isize) != 0 } {
        if isXML as c_int != 0
            && (len > xmlLen
                || unsafe {
                    *uri.offset(len as isize) as c_int != xmlNamespace[len as usize] as c_int
                })
        {
            isXML = XML_FALSE;
        }
        if mustBeXML == 0
            && isXMLNS as c_int != 0
            && (len > xmlnsLen
                || unsafe {
                    *uri.offset(len as isize) as c_int != xmlnsNamespace[len as usize] as c_int
                })
        {
            isXMLNS = XML_FALSE;
        }
        if unsafe { (*parser).m_ns as c_int != 0 }
            && unsafe {
                *uri.offset(len as isize) as c_int == (*parser).m_namespaceSeparator as c_int
            }
            && is_rfc3986_uri_char(unsafe { *uri.offset(len as isize) }) == 0
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
    if unsafe { (*parser).m_namespaceSeparator != 0 } {
        len += 1;
    }
    if unsafe { !(*parser).m_freeBindingList.is_null() } {
        b = unsafe { (*parser).m_freeBindingList };
        if len > unsafe { (*b).uriAlloc } {
            if len > INT_MAX - EXPAND_SPARE {
                return XML_ERROR_NO_MEMORY;
            }
            let temp = expat_realloc(
                parser,
                unsafe { (*b).uri as *mut c_void },
                (size_of::<XML_Char>()).wrapping_mul((len + 24) as size_t),
                4517,
            ) as *mut XML_Char;
            if temp.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            unsafe { (*b).uri = temp };
            unsafe { (*b).uriAlloc = len + EXPAND_SPARE };
        }
        unsafe { (*parser).m_freeBindingList = (*b).nextTagBinding };
    } else {
        b = expat_malloc(parser, size_of::<BINDING>(), 4525) as *mut BINDING;
        if b.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
        if len > INT_MAX - EXPAND_SPARE {
            return XML_ERROR_NO_MEMORY;
        }
        let b_uri = expat_malloc(
            parser,
            (size_of::<XML_Char>()).wrapping_mul((len + 24) as size_t),
            4543,
        ) as *mut XML_Char;
        unsafe { (*b).uri = b_uri };
        if b_uri.is_null() {
            expat_free(parser, b as *mut c_void, 4545);
            return XML_ERROR_NO_MEMORY;
        }
        unsafe { (*b).uriAlloc = len + EXPAND_SPARE };
    }
    unsafe { (*b).uriLen = len };
    unsafe {
        memcpy(
            (*b).uri as *mut c_void,
            uri as *const c_void,
            (len as size_t).wrapping_mul(size_of::<XML_Char>()),
        )
    };
    if unsafe { (*parser).m_namespaceSeparator != 0 } {
        unsafe { *(*b).uri.offset((len - 1i32) as isize) = (*parser).m_namespaceSeparator };
    }
    unsafe { (*b).prefix = prefix };
    unsafe { (*b).attId = attId };
    unsafe { (*b).prevPrefixBinding = (*prefix).binding };
    if unsafe { *uri as c_int == '\0' as i32 }
        && prefix == unsafe { &raw mut (*(*parser).m_dtd).defaultPrefix }
    {
        unsafe { (*prefix).binding = null_mut::<BINDING>() };
    } else {
        unsafe { (*prefix).binding = b };
    }
    unsafe { (*b).nextTagBinding = *bindingsPtr };
    unsafe { *bindingsPtr = b };
    if !attId.is_null() && unsafe { (*parser).m_startNamespaceDeclHandler.is_some() } {
        let start_namespace_decl_handler = unsafe {
            (*parser)
                .m_startNamespaceDeclHandler
                .expect("non-null function pointer")
        };
        start_namespace_decl_handler(
            unsafe { (*parser).m_handlerArg },
            unsafe { (*prefix).name },
            if unsafe { !(*prefix).binding.is_null() } {
                uri
            } else {
                null::<XML_Char>()
            },
        );
    }
    XML_ERROR_NONE
}

extern "C" fn cdataSectionProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let encoding = unsafe { (*parser).m_encoding };
    let final_buffer = unsafe { (*parser).m_parsingStatus.finalBuffer };
    let mut result: XML_Error = doCdataSection(
        parser,
        unsafe { &*encoding },
        &raw mut start,
        end,
        endPtr,
        (final_buffer == 0) as XML_Bool,
        XML_ACCOUNT_DIRECT,
    );
    if result != XML_ERROR_NONE {
        return result;
    }
    if !start.is_null() {
        if unsafe { !(*parser).m_parentParser.is_null() } {
            let parser_ref = unsafe { &mut *parser };
            parser_ref.m_processor = Some(
                externalEntityContentProcessor
                    as extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            );
            return externalEntityContentProcessor(parser, start, end, endPtr);
        } else {
            let parser_ref = unsafe { &mut *parser };
            parser_ref.m_processor = Some(
                contentProcessor
                    as extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            );
            return contentProcessor(parser, start, end, endPtr);
        }
    }
    result
}

extern "C" fn doCdataSection(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut startPtr: *mut *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
    mut account: XML_Account,
) -> XML_Error {
    let mut s: *const c_char = unsafe { *startPtr };
    let eventPP: *mut *const c_char;
    let eventEndPP: *mut *const c_char;
    if core::ptr::eq(enc, unsafe { &*(*parser).m_encoding }) {
        eventPP = unsafe { &raw mut (*parser).m_eventPtr };
        unsafe { *eventPP = s };
        eventEndPP = unsafe { &raw mut (*parser).m_eventEndPtr };
    } else {
        eventPP = unsafe { &raw mut (*(*parser).m_openInternalEntities).internalEventPtr };
        eventEndPP = unsafe { &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr };
    }
    unsafe { *eventPP = s };
    unsafe { *startPtr = null::<c_char>() };
    loop {
        let mut next: *const c_char = s;
        let tok: c_int = {
            let (tok_value, next_tok_value) =
                enc.scanners[2](enc, c_char_slice_from_ptr_end(s, end));
            next = next_tok_value;
            tok_value
        };
        if accountingDiffTolerated(parser, tok, s, next, 4619, account) == 0 {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        unsafe { *eventEndPP = next };
        match tok {
            XML_TOK_CDATA_SECT_CLOSE => {
                if unsafe { (*parser).m_endCdataSectionHandler.is_some() } {
                    let end_cdata_handler = unsafe {
                        (*parser)
                            .m_endCdataSectionHandler
                            .expect("non-null function pointer")
                    };
                    end_cdata_handler(unsafe { (*parser).m_handlerArg });
                } else if unsafe { (*parser).m_defaultHandler.is_some() } {
                    reportDefault(parser, enc, s, next);
                }
                unsafe { *startPtr = next };
                unsafe { *nextPtr = next };
                if unsafe { (*parser).m_parsingStatus.parsing == XML_FINISHED } {
                    return XML_ERROR_ABORTED;
                } else {
                    return XML_ERROR_NONE;
                }
            }
            XML_TOK_DATA_NEWLINE => {
                if unsafe { (*parser).m_characterDataHandler.is_some() } {
                    let mut c: XML_Char = 0xa;
                    let char_data_handler = unsafe {
                        (*parser)
                            .m_characterDataHandler
                            .expect("non-null function pointer")
                    };
                    char_data_handler(unsafe { (*parser).m_handlerArg }, &raw mut c, 1i32);
                } else if unsafe { (*parser).m_defaultHandler.is_some() } {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_DATA_CHARS => {
                let charDataHandler: XML_CharacterDataHandler =
                    unsafe { (*parser).m_characterDataHandler };
                if charDataHandler.is_some() {
                    if enc.isUtf8 == 0 {
                        loop {
                            let mut dataPtr: *mut ICHAR = unsafe { (*parser).m_dataBuf };
                            let convert_res: XML_Convert_Result;
                            (convert_res, s, dataPtr) =
                                (*enc).utf8Convert(enc, s, next, dataPtr, unsafe {
                                    (*parser).m_dataBufEnd
                                });
                            unsafe { *eventEndPP = next };
                            charDataHandler.expect("non-null function pointer")(
                                unsafe { (*parser).m_handlerArg },
                                unsafe { (*parser).m_dataBuf },
                                unsafe { dataPtr.offset_from((*parser).m_dataBuf) as c_int },
                            );
                            if convert_res == XML_CONVERT_COMPLETED
                                || convert_res == XML_CONVERT_INPUT_INCOMPLETE
                            {
                                break;
                            }
                            unsafe { *eventPP = s };
                        }
                    } else {
                        charDataHandler.expect("non-null function pointer")(
                            unsafe { (*parser).m_handlerArg },
                            s,
                            unsafe { next.offset_from(s) as c_int },
                        );
                    }
                } else if unsafe { (*parser).m_defaultHandler.is_some() } {
                    reportDefault(parser, enc, s, next);
                }
            }
            XML_TOK_INVALID => {
                unsafe { *eventPP = next };
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL_CHAR => {
                if haveMore != 0 {
                    unsafe { *nextPtr = s };
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            XML_TOK_PARTIAL | XML_TOK_NONE => {
                if haveMore != 0 {
                    unsafe { *nextPtr = s };
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_CDATA_SECTION;
            }
            _ => {
                unsafe { *eventPP = next };
                return XML_ERROR_UNEXPECTED_STATE;
            }
        }
        match unsafe { (*parser).m_parsingStatus.parsing } {
            3 => {
                unsafe { *eventPP = next };
                unsafe { *nextPtr = next };
                return XML_ERROR_NONE;
            }
            2 => {
                unsafe { *eventPP = next };
                return XML_ERROR_ABORTED;
            }
            1 => {
                if unsafe { (*parser).m_reenter != 0 } {
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }
            _ => {}
        }
        s = next;
        unsafe { *eventPP = s };
    }
}

extern "C" fn ignoreSectionProcessor(
    mut parser: XML_Parser,
    mut start: *const c_char,
    mut end: *const c_char,
    mut endPtr: *mut *const c_char,
) -> XML_Error {
    let encoding = unsafe { (*parser).m_encoding };
    let final_buffer = unsafe { (*parser).m_parsingStatus.finalBuffer };
    let mut result: XML_Error = doIgnoreSection(
        parser,
        unsafe { &*encoding },
        &raw mut start,
        end,
        endPtr,
        (final_buffer == 0) as XML_Bool,
    );
    if result != XML_ERROR_NONE {
        return result;
    }
    if !start.is_null() {
        let parser_ref = unsafe { &mut *parser };
        parser_ref.m_processor = Some(
            prologProcessor
                as extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        );
        return prologProcessor(parser, start, end, endPtr);
    }
    result
}

extern "C" fn doIgnoreSection(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut startPtr: *mut *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
    mut haveMore: XML_Bool,
) -> XML_Error {
    let mut next: *const c_char = unsafe { *startPtr };
    let mut tok: c_int = 0;
    let s: *const c_char = unsafe { *startPtr };
    let eventPP: *mut *const c_char;
    let eventEndPP: *mut *const c_char;
    if core::ptr::eq(enc, unsafe { &*(*parser).m_encoding }) {
        eventPP = unsafe { &raw mut (*parser).m_eventPtr };
        unsafe { *eventPP = s };
        eventEndPP = unsafe { &raw mut (*parser).m_eventEndPtr };
    } else {
        eventPP = unsafe { &raw mut (*(*parser).m_openInternalEntities).internalEventPtr };
        eventEndPP = unsafe { &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr };
    }
    unsafe { *eventPP = s };
    unsafe { *startPtr = null::<c_char>() };
    tok = {
        let (tok_value, next_tok_value) = enc.scanners[3](enc, c_char_slice_from_ptr_end(s, end));
        next = next_tok_value;
        tok_value
    };
    if accountingDiffTolerated(parser, tok, s, next, 4778, XML_ACCOUNT_DIRECT) == 0 {
        accountingOnAbort(parser);
        return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    unsafe { *eventEndPP = next };
    match tok {
        XML_TOK_IGNORE_SECT => {
            if unsafe { (*parser).m_defaultHandler.is_some() } {
                reportDefault(parser, enc, s, next);
            }
            unsafe { *startPtr = next };
            unsafe { *nextPtr = next };
            if unsafe { (*parser).m_parsingStatus.parsing == XML_FINISHED } {
                XML_ERROR_ABORTED
            } else {
                XML_ERROR_NONE
            }
        }
        XML_TOK_INVALID => {
            unsafe { *eventPP = next };
            XML_ERROR_INVALID_TOKEN
        }
        XML_TOK_PARTIAL_CHAR => {
            if haveMore != 0 {
                unsafe { *nextPtr = s };
                return XML_ERROR_NONE;
            }
            XML_ERROR_PARTIAL_CHAR
        }
        XML_TOK_PARTIAL | XML_TOK_NONE => {
            if haveMore != 0 {
                unsafe { *nextPtr = s };
                return XML_ERROR_NONE;
            }
            XML_ERROR_SYNTAX
        }
        _ => {
            unsafe { *eventPP = next };
            XML_ERROR_UNEXPECTED_STATE
        }
    }
}

extern "C" fn initializeEncoding(mut parser: XML_Parser) -> XML_Error {
    let s: *const c_char = unsafe { (*parser).m_protocolEncodingName };
    let (initStatus, initEncoding) = if unsafe { (*parser).m_ns as c_int != 0 } {
        unsafe {
            XmlInitEncodingNS(
                &raw mut (*parser).m_initEncoding,
                &raw mut (*parser).m_encoding,
                s,
            )
        }
    } else {
        unsafe {
            XmlInitEncoding(
                &raw mut (*parser).m_initEncoding,
                &raw mut (*parser).m_encoding,
                s,
            )
        }
    };
    if initStatus != 0 {
        unsafe { (*parser).m_encoding = initEncoding };
        return XML_ERROR_NONE;
    }
    handleUnknownEncoding(parser, unsafe { (*parser).m_protocolEncodingName })
}

extern "C" fn processXmlDecl(
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
    let mut xmlDeclParseStatus: c_int = 0;
    let mut badPtr: *const c_char = null::<c_char>();
    if accountingDiffTolerated(parser, XML_TOK_XML_DECL, s, next, 4870, XML_ACCOUNT_DIRECT) == 0 {
        accountingOnAbort(parser);
        return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
    }
    let parser_encoding = unsafe { (*parser).m_encoding };
    (
        xmlDeclParseStatus,
        badPtr,
        version,
        versionend,
        encodingName,
        newEncoding,
        standalone,
    ) = if unsafe { (*parser).m_ns as c_int != 0 } {
        XmlParseXmlDeclNS(
            isGeneralTextEntity,
            unsafe { &*parser_encoding },
            c_char_slice_from_ptr_end(s, next),
        )
    } else {
        XmlParseXmlDecl(
            isGeneralTextEntity,
            unsafe { &*parser_encoding },
            c_char_slice_from_ptr_end(s, next),
        )
    };
    if xmlDeclParseStatus == 0 {
        unsafe { (*parser).m_eventPtr = badPtr };
        if isGeneralTextEntity != 0 {
            return XML_ERROR_TEXT_DECL;
        } else {
            return XML_ERROR_XML_DECL;
        }
    }
    if isGeneralTextEntity == 0 && standalone == 1 {
        unsafe { (*(*parser).m_dtd).standalone = XML_TRUE };
        if unsafe { (*parser).m_paramEntityParsing == XML_PARAM_ENTITY_PARSING_UNLESS_STANDALONE } {
            unsafe { (*parser).m_paramEntityParsing = XML_PARAM_ENTITY_PARSING_NEVER };
        }
    }
    if unsafe { (*parser).m_xmlDeclHandler.is_some() } {
        if !encodingName.is_null() {
            storedEncName = poolStoreString(
                unsafe { &raw mut (*parser).m_temp2Pool },
                unsafe { &*(*parser).m_encoding },
                encodingName,
                unsafe {
                    encodingName.offset(
                        (*(*parser).m_encoding).nameLength(&*(*parser).m_encoding, encodingName)
                            as isize,
                    )
                },
            );
            if storedEncName.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            unsafe { (*parser).m_temp2Pool.start = (*parser).m_temp2Pool.ptr };
        }
        if !version.is_null() {
            storedversion = poolStoreString(
                unsafe { &raw mut (*parser).m_temp2Pool },
                unsafe { &*(*parser).m_encoding },
                version,
                unsafe { versionend.offset(-((*(*parser).m_encoding).minBytesPerChar as isize)) },
            );
            if storedversion.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
        }
        let xml_decl_handler = unsafe {
            (*parser)
                .m_xmlDeclHandler
                .expect("non-null function pointer")
        };
        xml_decl_handler(
            unsafe { (*parser).m_handlerArg },
            storedversion,
            storedEncName,
            standalone,
        );
    } else if unsafe { (*parser).m_defaultHandler.is_some() } {
        reportDefault(parser, unsafe { &*(*parser).m_encoding }, s, next);
    }
    if unsafe { (*parser).m_protocolEncodingName.is_null() } {
        if !newEncoding.is_null() {
            if unsafe { (*newEncoding).minBytesPerChar != (*(*parser).m_encoding).minBytesPerChar }
                || (unsafe { (*newEncoding).minBytesPerChar == 2 }
                    && newEncoding != unsafe { (*parser).m_encoding })
            {
                unsafe { (*parser).m_eventPtr = encodingName };
                return XML_ERROR_INCORRECT_ENCODING;
            }
            unsafe { (*parser).m_encoding = newEncoding };
        } else if !encodingName.is_null() {
            if storedEncName.is_null() {
                storedEncName = poolStoreString(
                    unsafe { &raw mut (*parser).m_temp2Pool },
                    unsafe { &*(*parser).m_encoding },
                    encodingName,
                    unsafe {
                        encodingName.offset(
                            (*(*parser).m_encoding).nameLength(&*(*parser).m_encoding, encodingName)
                                as isize,
                        )
                    },
                );
                if storedEncName.is_null() {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            let result = handleUnknownEncoding(parser, storedEncName);
            poolClear(unsafe { &raw mut (*parser).m_temp2Pool });
            if result == XML_ERROR_UNKNOWN_ENCODING {
                unsafe { (*parser).m_eventPtr = encodingName };
            }
            return result;
        }
    }
    if !storedEncName.is_null() || !storedversion.is_null() {
        poolClear(unsafe { &raw mut (*parser).m_temp2Pool });
    }
    XML_ERROR_NONE
}

extern "C" fn handleUnknownEncoding(
    mut parser: XML_Parser,
    mut encodingName: *const XML_Char,
) -> XML_Error {
    let parser_ref = unsafe { &mut *parser };
    if parser_ref.m_unknownEncodingHandler.is_some() {
        let mut info: XML_Encoding = XML_Encoding {
            map: [0; 256],
            data: null_mut::<c_void>(),
            convert: None,
            release: None,
        };
        let mut i: c_int = 0;
        while i < 256 {
            info.map[i as usize] = -(1);
            i += 1;
        }
        info.convert = None;
        info.data = NULL;
        info.release = None;
        if parser_ref
            .m_unknownEncodingHandler
            .expect("non-null function pointer")(
            parser_ref.m_unknownEncodingHandlerData,
            encodingName,
            &raw mut info,
        ) != 0
        {
            parser_ref.m_unknownEncodingMem =
                expat_malloc(parser, XmlSizeOfUnknownEncoding() as size_t, 4963);
            if parser_ref.m_unknownEncodingMem.is_null() {
                if info.release.is_some() {
                    info.release.expect("non-null function pointer")(info.data);
                }
                return XML_ERROR_NO_MEMORY;
            }
            let enc = if parser_ref.m_ns as c_int != 0 {
                Some(
                    XmlInitUnknownEncodingNS
                        as fn(*mut c_void, *const c_int, CONVERTER, *mut c_void) -> *mut ENCODING,
                )
            } else {
                Some(
                    XmlInitUnknownEncoding
                        as fn(*mut c_void, *const c_int, CONVERTER, *mut c_void) -> *mut ENCODING,
                )
            }
            .expect("non-null function pointer")(
                parser_ref.m_unknownEncodingMem,
                &raw mut info.map as *mut c_int,
                info.convert,
                info.data,
            );
            if !enc.is_null() {
                parser_ref.m_unknownEncodingData = info.data;
                parser_ref.m_unknownEncodingRelease = info.release;
                parser_ref.m_encoding = enc;
                return XML_ERROR_NONE;
            }
        }
        if info.release.is_some() {
            info.release.expect("non-null function pointer")(info.data);
        }
    }
    XML_ERROR_UNKNOWN_ENCODING
}

extern "C" fn prologInitProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let result: XML_Error = initializeEncoding(parser);
    if result != XML_ERROR_NONE {
        return result;
    }
    unsafe {
        (*parser).m_processor = Some(
            prologProcessor
                as extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        )
    };
    prologProcessor(parser, s, end, nextPtr)
}

extern "C" fn externalParEntInitProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let result: XML_Error = initializeEncoding(parser);
    if result != XML_ERROR_NONE {
        return result;
    }
    unsafe { (*(*parser).m_dtd).paramEntityRead = XML_TRUE };
    if unsafe { (*parser).m_prologState.inEntityValue != 0 } {
        unsafe {
            (*parser).m_processor = Some(
                entityValueInitProcessor
                    as extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            )
        };
        entityValueInitProcessor(parser, s, end, nextPtr)
    } else {
        unsafe {
            (*parser).m_processor = Some(
                externalParEntProcessor
                    as extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            )
        };
        externalParEntProcessor(parser, s, end, nextPtr)
    }
}

extern "C" fn entityValueInitProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut start: *const c_char = s;
    let mut next: *const c_char = start;
    unsafe { (*parser).m_eventPtr = start };
    loop {
        let tok: c_int = {
            let encoding = unsafe { (*parser).m_encoding };
            let (tok_value, next_tok_value) = unsafe {
                (*encoding).scanners[0](&*encoding, c_char_slice_from_ptr_end(start, end))
            };
            next = next_tok_value;
            tok_value
        };
        unsafe { (*parser).m_eventEndPtr = next };
        if tok <= 0 {
            if unsafe { (*parser).m_parsingStatus.finalBuffer == 0 } && tok != XML_TOK_INVALID {
                unsafe { *nextPtr = s };
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
                unsafe { &*(*parser).m_encoding },
                s,
                end,
                XML_ACCOUNT_DIRECT,
                null_mut::<*const c_char>(),
            );
        } else if tok == XML_TOK_XML_DECL {
            let result: XML_Error = processXmlDecl(parser, 0, start, next);
            if result != XML_ERROR_NONE {
                return result;
            }
            if unsafe { (*parser).m_parsingStatus.parsing == XML_FINISHED } {
                return XML_ERROR_ABORTED;
            }
            unsafe {
                *nextPtr = next;
                (*parser).m_processor = Some(
                    entityValueProcessor
                        as extern "C" fn(
                            XML_Parser,
                            *const c_char,
                            *const c_char,
                            *mut *const c_char,
                        ) -> XML_Error,
                )
            };
            return entityValueProcessor(parser, next, end, nextPtr);
        } else if tok == XML_TOK_BOM {
            if accountingDiffTolerated(parser, tok, s, next, 5077, XML_ACCOUNT_DIRECT) == 0 {
                accountingOnAbort(parser);
                return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            }
            unsafe { *nextPtr = next };
            s = next;
        } else if tok == XML_TOK_INSTANCE_START {
            unsafe { *nextPtr = next };
            return XML_ERROR_SYNTAX;
        }
        start = next;
        unsafe { (*parser).m_eventPtr = start };
    }
}

extern "C" fn externalParEntProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = s;
    let mut tok: c_int = {
        let encoding = unsafe { (*parser).m_encoding };
        let (tok_value, next_tok_value) =
            unsafe { (*encoding).scanners[0](&*encoding, c_char_slice_from_ptr_end(s, end)) };
        next = next_tok_value;
        tok_value
    };
    if tok <= 0 {
        if unsafe { (*parser).m_parsingStatus.finalBuffer == 0 } && tok != XML_TOK_INVALID {
            unsafe { *nextPtr = s };
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
        tok = {
            let encoding = unsafe { (*parser).m_encoding };
            let (tok_value, next_tok_value) = unsafe {
                (*encoding).scanners[0usize](&*encoding, c_char_slice_from_ptr_end(s, end))
            };
            next = next_tok_value;
            tok_value
        };
    }
    unsafe {
        (*parser).m_processor = Some(
            prologProcessor
                as extern "C" fn(
                    XML_Parser,
                    *const c_char,
                    *const c_char,
                    *mut *const c_char,
                ) -> XML_Error,
        )
    };
    let current_encoding = unsafe { (*parser).m_encoding };
    unsafe {
        doProlog(
            parser,
            &*current_encoding,
            s,
            end,
            tok,
            next,
            nextPtr,
            ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
            XML_TRUE,
            XML_ACCOUNT_DIRECT,
        )
    }
}

extern "C" fn entityValueProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut start: *const c_char = s;
    let mut next: *const c_char = s;
    let encoding = unsafe { (*parser).m_encoding };
    let enc: &ENCODING = unsafe { &*encoding };
    loop {
        let tok: c_int = {
            let (tok_value, next_tok_value) =
                enc.scanners[0](enc, c_char_slice_from_ptr_end(start, end));
            next = next_tok_value;
            tok_value
        };
        if tok <= 0 {
            if unsafe { (*parser).m_parsingStatus.finalBuffer == 0 } && tok != XML_TOK_INVALID {
                unsafe { *nextPtr = s };
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

extern "C" fn prologProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let encoding = unsafe { (*parser).m_encoding };
    let mut next: *const c_char = s;
    let tok: c_int = {
        let (tok_value, next_tok_value) =
            unsafe { (*encoding).scanners[0](&*encoding, c_char_slice_from_ptr_end(s, end)) };
        next = next_tok_value;
        tok_value
    };
    let current_encoding = unsafe { (*parser).m_encoding };
    unsafe {
        doProlog(
            parser,
            &*current_encoding,
            s,
            end,
            tok,
            next,
            nextPtr,
            ((*parser).m_parsingStatus.finalBuffer == 0) as XML_Bool,
            XML_TRUE,
            XML_ACCOUNT_DIRECT,
        )
    }
}

unsafe extern "C" fn doProlog(
    mut parser: XML_Parser,
    mut enc: &ENCODING,
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
    static externalSubsetName: [XML_Char; 2] = [ASCII_HASH as XML_Char, '\0' as XML_Char];
    static atypeCDATA: [XML_Char; 6] = [
        ASCII_C as XML_Char,
        ASCII_D as XML_Char,
        ASCII_A as XML_Char,
        ASCII_T as XML_Char,
        ASCII_A as XML_Char,
        '\0' as XML_Char,
    ];
    static atypeID: [XML_Char; 3] = [ASCII_I as XML_Char, ASCII_D as XML_Char, '\0' as XML_Char];
    static atypeIDREF: [XML_Char; 6] = [
        ASCII_I as XML_Char,
        ASCII_D as XML_Char,
        ASCII_R as XML_Char,
        ASCII_E as XML_Char,
        ASCII_F as XML_Char,
        '\0' as XML_Char,
    ];
    static atypeIDREFS: [XML_Char; 7] = [
        ASCII_I as XML_Char,
        ASCII_D as XML_Char,
        ASCII_R as XML_Char,
        ASCII_E as XML_Char,
        ASCII_F as XML_Char,
        ASCII_S as XML_Char,
        '\0' as XML_Char,
    ];
    static atypeENTITY: [XML_Char; 7] = [
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        ASCII_T as XML_Char,
        ASCII_I as XML_Char,
        ASCII_T as XML_Char,
        ASCII_Y as XML_Char,
        '\0' as XML_Char,
    ];
    static atypeENTITIES: [XML_Char; 9] = [
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
    static atypeNMTOKEN: [XML_Char; 8] = [
        ASCII_N as XML_Char,
        ASCII_M as XML_Char,
        ASCII_T as XML_Char,
        ASCII_O as XML_Char,
        ASCII_K as XML_Char,
        ASCII_E as XML_Char,
        ASCII_N as XML_Char,
        '\0' as XML_Char,
    ];
    static atypeNMTOKENS: [XML_Char; 9] = [
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
    static notationPrefix: [XML_Char; 10] = [
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
    static enumValueSep: [XML_Char; 2] = [ASCII_PIPE as XML_Char, '\0' as XML_Char];
    static enumValueStart: [XML_Char; 2] = [ASCII_LPAREN as XML_Char, '\0' as XML_Char];
    let dtd: *mut DTD = (*parser).m_dtd;
    let mut eventPP: *mut *const c_char = null_mut::<*const c_char>();
    let mut eventEndPP: *mut *const c_char = null_mut::<*const c_char>();
    let mut quant: XML_Content_Quant = XML_CQUANT_NONE;
    if core::ptr::eq(enc, &*(*parser).m_encoding) {
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
                    if !core::ptr::eq(enc, &*(*parser).m_encoding)
                        && (*(*parser).m_openInternalEntities).betweenDecl == 0
                    {
                        *nextPtr = s;
                        return XML_ERROR_NONE;
                    }
                    if (*parser).m_isParamEntity as c_int != 0
                        || !core::ptr::eq(enc, &*(*parser).m_encoding)
                    {
                        if (*parser)
                            .m_prologState
                            .handler
                            .expect("non-null function pointer")(
                            &raw mut (*parser).m_prologState,
                            -(4),
                            c_char_slice_from_ptr_end(end, end),
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
            c_char_slice_from_ptr_end(s, next),
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
                enc = &*(*parser).m_encoding;
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
                enc = &*(*parser).m_encoding;
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
                    let mut is_public_id: c_int = 0;
                    let mut bad_ptr: *const c_char = null::<c_char>();
                    (is_public_id, bad_ptr) =
                        (*enc).isPublicId(enc, c_char_slice_from_ptr_end(s, next));
                    if is_public_id == 0 {
                        *eventPP = bad_ptr;
                        return XML_ERROR_PUBLICID;
                    }
                    pubId = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
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
                        as extern "C" fn(
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
                            if (if std::ptr::eq((*parser).m_tempPool.ptr, (*parser).m_tempPool.end)
                                && poolGrow(&raw mut (*parser).m_tempPool) == 0
                            {
                                0
                            } else {
                                let fresh1 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *fresh1 = 0x29i8;
                                1
                            }) == 0
                                || (if std::ptr::eq(
                                    (*parser).m_tempPool.ptr,
                                    (*parser).m_tempPool.end,
                                ) && poolGrow(&raw mut (*parser).m_tempPool) == 0
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
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
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
                            if (if std::ptr::eq((*parser).m_tempPool.ptr, (*parser).m_tempPool.end)
                                && poolGrow(&raw mut (*parser).m_tempPool) == 0
                            {
                                0
                            } else {
                                let fresh3 = (*parser).m_tempPool.ptr;
                                (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1);
                                *fresh3 = 0x29i8;
                                1
                            }) == 0
                                || (if std::ptr::eq(
                                    (*parser).m_tempPool.ptr,
                                    (*parser).m_tempPool.end,
                                ) && poolGrow(&raw mut (*parser).m_tempPool) == 0
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
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
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
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
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
                if (*enc).predefinedEntityName(enc, c_char_slice_from_ptr_end(s, next)) != 0 {
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
                let mut is_public_id: c_int = 0;
                let mut bad_ptr: *const c_char = null::<c_char>();
                (is_public_id, bad_ptr) =
                    (*enc).isPublicId(enc, c_char_slice_from_ptr_end(s, next));
                if is_public_id == 0 {
                    *eventPP = bad_ptr;
                    return XML_ERROR_PUBLICID;
                }
                if !(*parser).m_declNotationName.is_null() {
                    let mut tem_0: *mut XML_Char = poolStoreString(
                        &raw mut (*parser).m_tempPool,
                        enc,
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
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
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
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
                            as extern "C" fn(
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
                    assert!(!(*dtd).scaffIndex.is_null());
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
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
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
                if tok == XML_TOK_BOM {
                    handleDefault = XML_FALSE;
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
                let mut is_public_id: c_int = 0;
                let mut bad_ptr: *const c_char = null::<c_char>();
                (is_public_id, bad_ptr) =
                    (*enc).isPublicId(enc, c_char_slice_from_ptr_end(s, next));
                if is_public_id == 0 {
                    *eventPP = bad_ptr;
                    return XML_ERROR_PUBLICID;
                }
                current_block = 13941306361429013238;
            }
            14343490084333691418 => {
                if (*dtd).keepProcessing as c_int != 0 && !(*parser).m_declEntity.is_null() {
                    (*(*parser).m_declEntity).systemId = poolStoreString(
                        &raw mut (*dtd).pool,
                        enc,
                        s.offset(enc.minBytesPerChar as isize),
                        next.offset(-(enc.minBytesPerChar as isize)),
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
                        next.offset(-(enc.minBytesPerChar as isize))
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
                    let fresh5 = &mut (*(*dtd).scaffold.offset(myindex_0 as isize)).name;
                    *fresh5 = name_2;
                    nameLen = 0;
                    loop {
                        let fresh6 = nameLen;
                        nameLen = nameLen.wrapping_add(1);
                        if *name_2.add(fresh6) == 0 {
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
        if current_block == 13941306361429013238
            && (*dtd).keepProcessing as c_int != 0
            && !(*parser).m_declEntity.is_null()
        {
            let mut tem: *mut XML_Char = poolStoreString(
                &raw mut (*dtd).pool,
                enc,
                s.offset(enc.minBytesPerChar as isize),
                next.offset(-(enc.minBytesPerChar as isize)),
            );
            if tem.is_null() {
                return XML_ERROR_NO_MEMORY;
            }
            normalizePublicId(tem);
            (*(*parser).m_declEntity).publicId = tem;
            (*dtd).pool.start = (*dtd).pool.ptr;
            if (*parser).m_entityDeclHandler.is_some() && role == XML_ROLE_ENTITY_PUBLIC_ID {
                handleDefault = XML_FALSE;
            }
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
        tok = {
            let (tok_value, next_tok_value) =
                enc.scanners[0](enc, c_char_slice_from_ptr_end(s, end));
            next = next_tok_value;
            tok_value
        };
    }
}

extern "C" fn epilogProcessor(
    mut parser: XML_Parser,
    mut s: *const c_char,
    mut end: *const c_char,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let epilog_proc = Some(
        epilogProcessor
            as extern "C" fn(
                XML_Parser,
                *const c_char,
                *const c_char,
                *mut *const c_char,
            ) -> XML_Error,
    );
    unsafe { (*parser).m_processor = epilog_proc };
    unsafe { (*parser).m_eventPtr = s };
    loop {
        let mut next: *const c_char = null::<c_char>();
        let encoding = unsafe { (*parser).m_encoding };
        let tok: c_int = {
            let (tok_value, next_tok_value) =
                unsafe { (*encoding).scanners[0](&*encoding, c_char_slice_from_ptr_end(s, end)) };
            next = next_tok_value;
            tok_value
        };
        if accountingDiffTolerated(parser, tok, s, next, 6279, XML_ACCOUNT_DIRECT) == 0 {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        unsafe { (*parser).m_eventEndPtr = next };
        match tok {
            -15 => {
                if unsafe { (*parser).m_defaultHandler.is_some() } {
                    reportDefault(parser, unsafe { &*encoding }, s, next);
                    if unsafe { (*parser).m_parsingStatus.parsing == XML_FINISHED } {
                        return XML_ERROR_ABORTED;
                    }
                }
                unsafe { *nextPtr = next };
                return XML_ERROR_NONE;
            }
            XML_TOK_NONE => {
                unsafe { *nextPtr = s };
                return XML_ERROR_NONE;
            }
            XML_TOK_PROLOG_S => {
                if unsafe { (*parser).m_defaultHandler.is_some() } {
                    reportDefault(parser, unsafe { &*encoding }, s, next);
                }
            }
            XML_TOK_PI => {
                if reportProcessingInstruction(parser, unsafe { &*encoding }, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            XML_TOK_COMMENT => {
                if reportComment(parser, unsafe { &*encoding }, s, next) == 0 {
                    return XML_ERROR_NO_MEMORY;
                }
            }
            XML_TOK_INVALID => {
                unsafe { (*parser).m_eventPtr = next };
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL => {
                if unsafe { (*parser).m_parsingStatus.finalBuffer == 0 } {
                    unsafe { *nextPtr = s };
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_UNCLOSED_TOKEN;
            }
            XML_TOK_PARTIAL_CHAR => {
                if unsafe { (*parser).m_parsingStatus.finalBuffer == 0 } {
                    unsafe { *nextPtr = s };
                    return XML_ERROR_NONE;
                }
                return XML_ERROR_PARTIAL_CHAR;
            }
            _ => return XML_ERROR_JUNK_AFTER_DOC_ELEMENT,
        }
        match unsafe { (*parser).m_parsingStatus.parsing } {
            3 => {
                unsafe { (*parser).m_eventPtr = next };
                unsafe { *nextPtr = next };
                return XML_ERROR_NONE;
            }
            2 => {
                unsafe { (*parser).m_eventPtr = next };
                return XML_ERROR_ABORTED;
            }
            1 => {
                if unsafe { (*parser).m_reenter != 0 } {
                    return XML_ERROR_UNEXPECTED_STATE;
                }
            }
            _ => {}
        }
        s = next;
        unsafe { (*parser).m_eventPtr = s };
    }
}

extern "C" fn processEntity(
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
            let internal_entity_proc = Some(
                internalEntityProcessor
                    as extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            );
            unsafe { (*parser).m_processor = internal_entity_proc };
            openEntityList = unsafe { &raw mut (*parser).m_openInternalEntities };
            freeEntityList = unsafe { &raw mut (*parser).m_freeInternalEntities };
        }
        1 => {
            openEntityList = unsafe { &raw mut (*parser).m_openAttributeEntities };
            freeEntityList = unsafe { &raw mut (*parser).m_freeAttributeEntities };
        }
        2 => {
            openEntityList = unsafe { &raw mut (*parser).m_openValueEntities };
            freeEntityList = unsafe { &raw mut (*parser).m_freeValueEntities };
        }
        _ => {
            assert!(0i32 != 0);
        }
    }
    if unsafe { !(*freeEntityList).is_null() } {
        openEntity = unsafe { *freeEntityList };
        unsafe { *freeEntityList = (*openEntity).next };
    } else {
        openEntity = expat_malloc(parser, size_of::<OPEN_INTERNAL_ENTITY>(), 6382)
            as *mut OPEN_INTERNAL_ENTITY;
        if openEntity.is_null() {
            return XML_ERROR_NO_MEMORY;
        }
    }
    unsafe { (*entity).open = XML_TRUE };
    unsafe { (*entity).hasMore = XML_TRUE };
    entityTrackingOnOpen(parser, entity, 6389);
    unsafe { (*entity).processed = 0 };
    unsafe { (*openEntity).next = *openEntityList };
    unsafe { *openEntityList = openEntity };
    unsafe { (*openEntity).entity = entity };
    unsafe { (*openEntity).type_0 = type_0 };
    unsafe { (*openEntity).startTagLevel = (*parser).m_tagLevel };
    unsafe { (*openEntity).betweenDecl = betweenDecl };
    unsafe { (*openEntity).internalEventPtr = null::<c_char>() };
    unsafe { (*openEntity).internalEventEndPtr = null::<c_char>() };
    if type_0 == ENTITY_INTERNAL {
        triggerReenter(parser);
    }
    XML_ERROR_NONE
}

extern "C" fn internalEntityProcessor(
    mut parser: XML_Parser,
    mut _s: *const c_char,
    mut _end: *const c_char,
    mut _nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut next: *const c_char = null::<c_char>();
    let mut result: XML_Error = XML_ERROR_NONE;
    let openEntity: *mut OPEN_INTERNAL_ENTITY = unsafe { (*parser).m_openInternalEntities };
    if openEntity.is_null() {
        return XML_ERROR_UNEXPECTED_STATE;
    }
    let entity: *mut ENTITY = unsafe { (*openEntity).entity };
    if unsafe { (*entity).hasMore != 0 } {
        let textStart: *const c_char =
            unsafe { ((*entity).textPtr).offset((*entity).processed as isize) };
        let textEnd: *const c_char =
            unsafe { (*entity).textPtr.offset((*entity).textLen as isize) };
        next = textStart;
        if unsafe { (*entity).is_param != 0 } {
            let internal_encoding = unsafe { (*parser).m_internalEncoding };
            let tok: c_int = {
                let (tok_value, next_tok_value) = unsafe {
                    (*internal_encoding).scanners[0](
                        &*internal_encoding,
                        c_char_slice_from_ptr_end(textStart, textEnd),
                    )
                };
                next = next_tok_value;
                tok_value
            };
            result = unsafe {
                doProlog(
                    parser,
                    &*internal_encoding,
                    textStart,
                    textEnd,
                    tok,
                    next,
                    &raw mut next,
                    XML_FALSE,
                    XML_FALSE,
                    XML_ACCOUNT_ENTITY_EXPANSION,
                )
            };
        } else {
            let internal_encoding = unsafe { (*parser).m_internalEncoding };
            result = unsafe {
                doContent(
                    parser,
                    (*openEntity).startTagLevel,
                    &*internal_encoding,
                    textStart,
                    textEnd,
                    &raw mut next,
                    XML_FALSE,
                    XML_ACCOUNT_ENTITY_EXPANSION,
                )
            };
        }
        if result != XML_ERROR_NONE {
            return result;
        }
        let parsing_suspended = unsafe { (*parser).m_parsingStatus.parsing == XML_SUSPENDED };
        let parsing_reentered = unsafe {
            (*parser).m_parsingStatus.parsing == XML_PARSING && (*parser).m_reenter as c_int != 0
        };
        if textEnd != next && (parsing_suspended || parsing_reentered) {
            unsafe { (*entity).processed = next.offset_from((*entity).textPtr) as c_int };
            return result;
        }
        unsafe { (*entity).hasMore = XML_FALSE };
        if unsafe { (*entity).is_param == 0 && (*openEntity).startTagLevel != (*parser).m_tagLevel }
        {
            return XML_ERROR_ASYNC_ENTITY;
        }
        triggerReenter(parser);
        return result;
    }
    entityTrackingOnClose(parser, entity, 6470);
    assert!(unsafe { (*parser).m_openInternalEntities == openEntity });
    unsafe { (*entity).open = XML_FALSE };
    unsafe { (*parser).m_openInternalEntities = (*(*parser).m_openInternalEntities).next };
    unsafe { (*openEntity).next = (*parser).m_freeInternalEntities };
    unsafe { (*parser).m_freeInternalEntities = openEntity };
    if unsafe { (*parser).m_openInternalEntities.is_null() } {
        let processor = if unsafe { (*entity).is_param as c_int != 0 } {
            Some(
                prologProcessor
                    as extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            )
        } else {
            Some(
                contentProcessor
                    as extern "C" fn(
                        XML_Parser,
                        *const c_char,
                        *const c_char,
                        *mut *const c_char,
                    ) -> XML_Error,
            )
        };
        unsafe { (*parser).m_processor = processor };
    }
    triggerReenter(parser);
    XML_ERROR_NONE
}

extern "C" fn errorProcessor(
    mut parser: XML_Parser,
    mut _s: *const c_char,
    mut _end: *const c_char,
    mut _nextPtr: *mut *const c_char,
) -> XML_Error {
    unsafe { (*parser).m_errorCode }
}

extern "C" fn storeAttributeValue(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut isCdata: XML_Bool,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut pool: *mut STRING_POOL,
    mut account: XML_Account,
) -> XML_Error {
    let mut next: *const c_char = ptr;
    let mut result: XML_Error = XML_ERROR_NONE;
    loop {
        if unsafe { (*parser).m_openAttributeEntities.is_null() } {
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
            let openEntity: *mut OPEN_INTERNAL_ENTITY =
                unsafe { (*parser).m_openAttributeEntities };
            if openEntity.is_null() {
                return XML_ERROR_UNEXPECTED_STATE;
            }
            let entity: *mut ENTITY = unsafe { (*openEntity).entity };
            let textStart: *const c_char =
                unsafe { ((*entity).textPtr).offset((*entity).processed as isize) };
            let textEnd: *const c_char =
                unsafe { (*entity).textPtr.offset((*entity).textLen as isize) };
            let mut nextInEntity: *const c_char = textStart;
            if unsafe { (*entity).hasMore != 0 } {
                let internal_encoding = unsafe { (*parser).m_internalEncoding };
                result = appendAttributeValue(
                    parser,
                    unsafe { &*internal_encoding },
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
                    unsafe {
                        (*entity).processed = nextInEntity.offset_from((*entity).textPtr) as c_int
                    };
                    continue;
                } else {
                    unsafe { (*entity).hasMore = XML_FALSE };
                    continue;
                }
            } else {
                entityTrackingOnClose(parser, entity, 6547);
                assert!(unsafe { (*parser).m_openAttributeEntities == openEntity });
                unsafe { (*entity).open = XML_FALSE };
                unsafe {
                    (*parser).m_openAttributeEntities = (*(*parser).m_openAttributeEntities).next
                };
                unsafe { (*openEntity).next = (*parser).m_freeAttributeEntities };
                unsafe { (*parser).m_freeAttributeEntities = openEntity };
            }
        }
        if result != 0 || (unsafe { (*parser).m_openAttributeEntities.is_null() } && end == next) {
            break;
        }
    }
    if result as u64 != 0 {
        return result;
    }
    if isCdata == 0
        && unsafe { (*pool).ptr.offset_from((*pool).start) as c_long != 0 }
        && unsafe { *(*pool).ptr.offset(-1) as c_int == 0x20 }
    {
        unsafe { (*pool).ptr = (*pool).ptr.offset(-1) };
    }
    if if unsafe { std::ptr::eq((*pool).ptr, (*pool).end) } && poolGrow(pool) == 0 {
        0
    } else {
        let fresh55 = unsafe { (*pool).ptr };
        unsafe { (*pool).ptr = (*pool).ptr.offset(1) };
        unsafe { *fresh55 = '\0' as XML_Char };
        1
    } == 0
    {
        return XML_ERROR_NO_MEMORY;
    }
    XML_ERROR_NONE
}

extern "C" fn appendAttributeValue(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut isCdata: XML_Bool,
    mut ptr: *const c_char,
    mut end: *const c_char,
    mut pool: *mut STRING_POOL,
    mut account: XML_Account,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let parser_ref = unsafe { &mut *parser };
    let dtd: *mut DTD = parser_ref.m_dtd;
    let pool_ref = unsafe { &mut *pool };
    loop {
        let mut next: *const c_char = ptr;
        let mut tok: c_int = {
            let (tok_value, next_tok_value) =
                enc.literalScanners[0](enc, c_char_slice_from_ptr_end(ptr, end));
            next = next_tok_value;
            tok_value
        };
        if accountingDiffTolerated(parser, tok, ptr, next, 6591, account) == 0 {
            accountingOnAbort(parser);
            return XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
        }
        let mut current_block_70: u64;
        match tok {
            XML_TOK_NONE => {
                if !nextPtr.is_null() {
                    unsafe { *nextPtr = next };
                }
                return XML_ERROR_NONE;
            }
            XML_TOK_INVALID => {
                if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                    parser_ref.m_eventPtr = next;
                }
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_PARTIAL => {
                if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                    parser_ref.m_eventPtr = ptr;
                }
                return XML_ERROR_INVALID_TOKEN;
            }
            XML_TOK_CHAR_REF => {
                let mut buf: [XML_Char; 4] = [0; 4];
                let mut i: c_int = 0;
                let mut n: c_int = (*enc).charRefNumber(enc, ptr);
                if n < 0 {
                    if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                        parser_ref.m_eventPtr = ptr;
                    }
                    return XML_ERROR_BAD_CHAR_REF;
                }
                if isCdata == 0
                    && n == 0x20
                    && (unsafe { pool_ref.ptr.offset_from(pool_ref.start) as c_long == 0 }
                        || unsafe { *pool_ref.ptr.offset(-1) as c_int == 0x20 })
                {
                    current_block_70 = 18038362259723567392;
                } else {
                    n = XmlUtf8Encode(n, &raw mut buf as *mut c_char);
                    i = 0;
                    while i < n {
                        if if std::ptr::eq(pool_ref.ptr, pool_ref.end) && poolGrow(pool) == 0 {
                            0
                        } else {
                            let fresh56 = pool_ref.ptr;
                            pool_ref.ptr = unsafe { pool_ref.ptr.offset(1) };
                            unsafe { *fresh56 = buf[i as usize] };
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
                next = unsafe { ptr.offset(enc.minBytesPerChar as isize) };
                current_block_70 = 1987954931741999833;
            }
            XML_TOK_ATTRIBUTE_VALUE_S | XML_TOK_DATA_NEWLINE => {
                current_block_70 = 1987954931741999833;
            }
            XML_TOK_ENTITY_REF => {
                let mut name: *const XML_Char = null::<XML_Char>();
                let mut entity: *mut ENTITY = null_mut::<ENTITY>();
                let mut checkEntityDecl: bool = false;
                let mut ch: XML_Char = (*enc).predefinedEntityName(
                    enc,
                    c_char_slice_from_ptr_end(
                        unsafe { ptr.offset(enc.minBytesPerChar as isize) },
                        unsafe { next.offset(-(enc.minBytesPerChar as isize)) },
                    ),
                ) as XML_Char;
                if ch != 0 {
                    accountingDiffTolerated(
                        parser,
                        tok,
                        &raw mut ch,
                        unsafe { (&raw mut ch).add(size_of::<XML_Char>()) },
                        6663,
                        XML_ACCOUNT_ENTITY_EXPANSION,
                    );
                    if if std::ptr::eq(pool_ref.ptr, pool_ref.end) && poolGrow(pool) == 0 {
                        0
                    } else {
                        let fresh58 = pool_ref.ptr;
                        pool_ref.ptr = unsafe { pool_ref.ptr.offset(1) };
                        unsafe { *fresh58 = ch };
                        1
                    } == 0
                    {
                        return XML_ERROR_NO_MEMORY;
                    }
                } else {
                    name = poolStoreString(
                        &raw mut parser_ref.m_temp2Pool,
                        enc,
                        unsafe { ptr.offset(enc.minBytesPerChar as isize) },
                        unsafe { next.offset(-(enc.minBytesPerChar as isize)) },
                    );
                    if name.is_null() {
                        return XML_ERROR_NO_MEMORY;
                    }
                    entity = lookup(parser, unsafe { &raw mut (*dtd).generalEntities }, name, 0)
                        as *mut ENTITY;
                    parser_ref.m_temp2Pool.ptr = parser_ref.m_temp2Pool.start;
                    if std::ptr::eq(pool, unsafe { &raw mut (*dtd).pool }) {
                        checkEntityDecl = parser_ref.m_prologState.documentEntity != 0
                            && (if unsafe { (*dtd).standalone as c_int != 0 } {
                                parser_ref.m_openInternalEntities.is_null() as c_int
                            } else {
                                (unsafe { (*dtd).hasParamEntityRefs == 0 }) as c_int
                            }) != 0;
                    } else {
                        checkEntityDecl = unsafe {
                            (*dtd).hasParamEntityRefs == 0 || (*dtd).standalone as c_int != 0
                        };
                    }
                    if checkEntityDecl {
                        if entity.is_null() {
                            return XML_ERROR_UNDEFINED_ENTITY;
                        } else if unsafe { (*entity).is_internal == 0 } {
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
                            let entity_ref = unsafe { &mut *entity };
                            if entity_ref.open != 0 {
                                if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                                    parser_ref.m_eventPtr = ptr;
                                }
                                return XML_ERROR_RECURSIVE_ENTITY_REF;
                            }
                            if !entity_ref.notation.is_null() {
                                if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                                    parser_ref.m_eventPtr = ptr;
                                }
                                return XML_ERROR_BINARY_ENTITY_REF;
                            }
                            if entity_ref.textPtr.is_null() {
                                if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                                    parser_ref.m_eventPtr = ptr;
                                }
                                return XML_ERROR_ATTRIBUTE_EXTERNAL_ENTITY_REF;
                            } else {
                                let mut result: XML_Error = XML_ERROR_NONE;
                                result = processEntity(parser, entity, XML_FALSE, ENTITY_ATTRIBUTE);
                                if result == XML_ERROR_NONE && !nextPtr.is_null() {
                                    unsafe { *nextPtr = next };
                                }
                                return result;
                            }
                        }
                    }
                }
                current_block_70 = 18038362259723567392;
            }
            _ => {
                if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                    parser_ref.m_eventPtr = ptr;
                }
                return XML_ERROR_UNEXPECTED_STATE;
            }
        }
        if current_block_70 == 1987954931741999833
            && !(isCdata == 0
                && (unsafe { pool_ref.ptr.offset_from(pool_ref.start) as c_long == 0 }
                    || unsafe { *pool_ref.ptr.offset(-1) as c_int == 0x20 }))
            && if std::ptr::eq(pool_ref.ptr, pool_ref.end) && poolGrow(pool) == 0 {
                0
            } else {
                let fresh57 = pool_ref.ptr;
                pool_ref.ptr = unsafe { pool_ref.ptr.offset(1) };
                unsafe { *fresh57 = 0x20i8 };
                1
            } == 0
        {
            return XML_ERROR_NO_MEMORY;
        }
        ptr = next;
    }
}

extern "C" fn storeEntityValue(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut entityTextPtr: *const c_char,
    mut entityTextEnd: *const c_char,
    mut account: XML_Account,
    mut nextPtr: *mut *const c_char,
) -> XML_Error {
    let mut current_block: u64;
    let parser_ref = unsafe { &mut *parser };
    let dtd_ref = unsafe { &mut *parser_ref.m_dtd };
    let pool = &mut dtd_ref.entityValuePool;
    let mut result: XML_Error = XML_ERROR_NONE;
    let oldInEntityValue: c_int = parser_ref.m_prologState.inEntityValue;
    parser_ref.m_prologState.inEntityValue = 1;
    if pool.blocks.is_null() && poolGrow(pool) == 0 {
        return XML_ERROR_NO_MEMORY;
    }
    let mut next: *const c_char = null::<c_char>();
    's_35: loop {
        next = entityTextPtr;
        let mut tok: c_int = {
            let (tok_value, next_tok_value) = enc.literalScanners[1](
                enc,
                c_char_slice_from_ptr_end(entityTextPtr, entityTextEnd),
            );
            next = next_tok_value;
            tok_value
        };
        if accountingDiffTolerated(parser, tok, entityTextPtr, next, 6798, account) == 0 {
            accountingOnAbort(parser);
            result = XML_ERROR_AMPLIFICATION_LIMIT_BREACH;
            break;
        } else {
            match tok {
                XML_TOK_PARAM_ENTITY_REF => {
                    if parser_ref.m_isParamEntity as c_int != 0
                        || !core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding })
                    {
                        let name = poolStoreString(
                            &raw mut parser_ref.m_tempPool,
                            enc,
                            unsafe { entityTextPtr.offset(enc.minBytesPerChar as isize) },
                            unsafe { next.offset(-(enc.minBytesPerChar as isize)) },
                        );
                        if name.is_null() {
                            result = XML_ERROR_NO_MEMORY;
                            break;
                        } else {
                            let entity = lookup(parser, &raw mut dtd_ref.paramEntities, name, 0)
                                as *mut ENTITY;
                            parser_ref.m_tempPool.ptr = parser_ref.m_tempPool.start;
                            if entity.is_null() {
                                dtd_ref.keepProcessing = dtd_ref.standalone;
                                break;
                            } else {
                                let entity_ref = unsafe { &mut *entity };
                                if entity_ref.open as c_int != 0
                                    || entity == parser_ref.m_declEntity
                                {
                                    if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                                        parser_ref.m_eventPtr = entityTextPtr;
                                    }
                                    result = XML_ERROR_RECURSIVE_ENTITY_REF;
                                    break;
                                } else if !entity_ref.systemId.is_null() {
                                    if parser_ref.m_externalEntityRefHandler.is_some() {
                                        dtd_ref.paramEntityRead = XML_FALSE;
                                        entity_ref.open = XML_TRUE;
                                        entityTrackingOnOpen(parser, entity, 6840);
                                        if parser_ref
                                            .m_externalEntityRefHandler
                                            .expect("non-null function pointer")(
                                            parser_ref.m_externalEntityRefHandlerArg,
                                            null::<XML_Char>(),
                                            entity_ref.base,
                                            entity_ref.systemId,
                                            entity_ref.publicId,
                                        ) == 0
                                        {
                                            entityTrackingOnClose(parser, entity, 6844);
                                            entity_ref.open = XML_FALSE;
                                            result = XML_ERROR_EXTERNAL_ENTITY_HANDLING;
                                            break;
                                        } else {
                                            entityTrackingOnClose(parser, entity, 6849);
                                            entity_ref.open = XML_FALSE;
                                            if dtd_ref.paramEntityRead == 0 {
                                                dtd_ref.keepProcessing = dtd_ref.standalone;
                                            }
                                        }
                                    } else {
                                        dtd_ref.keepProcessing = dtd_ref.standalone;
                                    }
                                } else {
                                    result = processEntity(parser, entity, XML_FALSE, ENTITY_VALUE);
                                    break;
                                }
                            }
                        }
                    } else {
                        parser_ref.m_eventPtr = entityTextPtr;
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
                    next = unsafe { entityTextPtr.offset(enc.minBytesPerChar as isize) };
                    current_block = 14913579936405700701;
                }
                XML_TOK_DATA_NEWLINE => {
                    current_block = 14913579936405700701;
                }
                XML_TOK_CHAR_REF => {
                    let mut buf: [XML_Char; 4] = [0; 4];
                    let mut i: c_int = 0;
                    let mut n: c_int = (*enc).charRefNumber(enc, entityTextPtr);
                    if n < 0 {
                        if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                            parser_ref.m_eventPtr = entityTextPtr;
                        }
                        result = XML_ERROR_BAD_CHAR_REF;
                        break;
                    } else {
                        n = XmlUtf8Encode(n, &raw mut buf as *mut c_char);
                        i = 0;
                        while i < n {
                            if std::ptr::eq(pool.end, pool.ptr) && poolGrow(pool) == 0 {
                                result = XML_ERROR_NO_MEMORY;
                                break 's_35;
                            } else {
                                let fresh73 = pool.ptr;
                                pool.ptr = unsafe { pool.ptr.offset(1) };
                                unsafe { *fresh73 = buf[i as usize] };
                                i += 1;
                            }
                        }
                    }
                    current_block = 5028470053297453708;
                }
                XML_TOK_PARTIAL => {
                    if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                        parser_ref.m_eventPtr = entityTextPtr;
                    }
                    result = XML_ERROR_INVALID_TOKEN;
                    break;
                }
                XML_TOK_INVALID => {
                    if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                        parser_ref.m_eventPtr = next;
                    }
                    result = XML_ERROR_INVALID_TOKEN;
                    break;
                }
                _ => {
                    if core::ptr::eq(enc, unsafe { &*parser_ref.m_encoding }) {
                        parser_ref.m_eventPtr = entityTextPtr;
                    }
                    result = XML_ERROR_UNEXPECTED_STATE;
                    break;
                }
            }
            if current_block == 14913579936405700701 {
                if std::ptr::eq(pool.end, pool.ptr) && poolGrow(pool) == 0 {
                    result = XML_ERROR_NO_MEMORY;
                    break;
                } else {
                    let fresh72 = pool.ptr;
                    pool.ptr = unsafe { pool.ptr.offset(1) };
                    unsafe { *fresh72 = 0xai8 };
                }
            }
            entityTextPtr = next;
        }
    }
    parser_ref.m_prologState.inEntityValue = oldInEntityValue;
    if !nextPtr.is_null() {
        unsafe { *nextPtr = next };
    }
    result
}

extern "C" fn callStoreEntityValue(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut entityTextPtr: *const c_char,
    mut entityTextEnd: *const c_char,
    mut account: XML_Account,
) -> XML_Error {
    let mut next: *const c_char = entityTextPtr;
    let mut result: XML_Error = XML_ERROR_NONE;
    loop {
        let open_value_entities = unsafe { (*parser).m_openValueEntities };
        if open_value_entities.is_null() {
            result = storeEntityValue(parser, enc, next, entityTextEnd, account, &raw mut next);
        } else {
            let openEntity: *mut OPEN_INTERNAL_ENTITY = open_value_entities;
            if openEntity.is_null() {
                return XML_ERROR_UNEXPECTED_STATE;
            }
            let entity: *mut ENTITY = unsafe { (*openEntity).entity };
            let text_ptr = unsafe { (*entity).textPtr };
            let textStart: *const c_char = unsafe { text_ptr.offset((*entity).processed as isize) };
            let textEnd: *const c_char = unsafe { text_ptr.offset((*entity).textLen as isize) };
            let mut nextInEntity: *const c_char = textStart;
            if unsafe { (*entity).hasMore != 0 } {
                result = storeEntityValue(
                    parser,
                    unsafe { &*(*parser).m_internalEncoding },
                    textStart,
                    textEnd,
                    XML_ACCOUNT_ENTITY_EXPANSION,
                    &raw mut nextInEntity,
                );
                if result != XML_ERROR_NONE {
                    break;
                }
                if textEnd != nextInEntity {
                    unsafe { (*entity).processed = nextInEntity.offset_from(text_ptr) as c_int };
                    continue;
                } else {
                    unsafe { (*entity).hasMore = XML_FALSE };
                    continue;
                }
            } else {
                entityTrackingOnClose(parser, entity, 6998);
                assert!(unsafe { (*parser).m_openValueEntities == openEntity });
                unsafe { (*entity).open = XML_FALSE };
                let next_open = unsafe { (*(*parser).m_openValueEntities).next };
                unsafe { (*parser).m_openValueEntities = next_open };
                let free_value_entities = unsafe { (*parser).m_freeValueEntities };
                unsafe { (*openEntity).next = free_value_entities };
                unsafe { (*parser).m_freeValueEntities = openEntity };
            }
        }
        if result != 0
            || (unsafe { (*parser).m_openValueEntities.is_null() } && entityTextEnd == next)
        {
            break;
        }
    }
    result
}

extern "C" fn normalizeLines(mut s: *mut XML_Char) {
    let mut p: *mut XML_Char = null_mut::<XML_Char>();
    loop {
        if unsafe { *s as c_int == '\0' as i32 } {
            return;
        }
        if unsafe { *s as c_int == 0xd } {
            break;
        }
        s = unsafe { s.offset(1) };
    }
    p = s;
    loop {
        if unsafe { *s as c_int == 0xd } {
            let fresh7 = p;
            p = unsafe { p.offset(1) };
            unsafe { *fresh7 = 0xai8 };
            s = unsafe { s.offset(1) };
            if unsafe { *s as c_int == 0xa } {
                s = unsafe { s.offset(1) };
            }
        } else {
            let fresh8 = s;
            s = unsafe { s.offset(1) };
            let fresh9 = p;
            p = unsafe { p.offset(1) };
            unsafe { *fresh9 = *fresh8 };
        }
        if unsafe { *s == 0 } {
            break;
        }
    }
    unsafe { *p = '\0' as XML_Char };
}

extern "C" fn reportProcessingInstruction(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut start: *const c_char,
    mut end: *const c_char,
) -> c_int {
    let mut target: *const XML_Char = null::<XML_Char>();
    let mut data: *mut XML_Char = null_mut::<XML_Char>();
    let mut tem: *const c_char = null::<c_char>();
    if unsafe { (*parser).m_processingInstructionHandler.is_none() } {
        if unsafe { (*parser).m_defaultHandler.is_some() } {
            reportDefault(parser, enc, start, end);
        }
        return 1i32;
    }
    start = unsafe { start.offset((enc.minBytesPerChar * 2i32) as isize) };
    tem = unsafe { start.offset((*enc).nameLength(enc, start) as isize) };
    target = poolStoreString(unsafe { &raw mut (*parser).m_tempPool }, enc, start, tem);
    if target.is_null() {
        return 0i32;
    }
    let pool_ptr = unsafe { (*parser).m_tempPool.ptr };
    unsafe { (*parser).m_tempPool.start = pool_ptr };
    data = poolStoreString(
        unsafe { &raw mut (*parser).m_tempPool },
        enc,
        (*enc).skipS(enc, tem),
        unsafe { end.offset(-((enc.minBytesPerChar * 2i32) as isize)) },
    );
    if data.is_null() {
        return 0i32;
    }
    normalizeLines(data);
    let processing_instruction_handler = unsafe {
        (*parser)
            .m_processingInstructionHandler
            .expect("non-null function pointer")
    };
    let handler_arg = unsafe { (*parser).m_handlerArg };
    processing_instruction_handler(handler_arg, target, data);
    poolClear(unsafe { &raw mut (*parser).m_tempPool });
    1
}

extern "C" fn reportComment(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut start: *const c_char,
    mut end: *const c_char,
) -> c_int {
    let mut data: *mut XML_Char = null_mut::<XML_Char>();
    if unsafe { (*parser).m_commentHandler.is_none() } {
        if unsafe { (*parser).m_defaultHandler.is_some() } {
            reportDefault(parser, enc, start, end);
        }
        return 1i32;
    }
    data = poolStoreString(
        unsafe { &raw mut (*parser).m_tempPool },
        enc,
        unsafe { start.offset((enc.minBytesPerChar * 4i32) as isize) },
        unsafe { end.offset(-((enc.minBytesPerChar * 3i32) as isize)) },
    );
    if data.is_null() {
        return 0i32;
    }
    normalizeLines(data);
    let comment_handler = unsafe {
        (*parser)
            .m_commentHandler
            .expect("non-null function pointer")
    };
    let handler_arg = unsafe { (*parser).m_handlerArg };
    comment_handler(handler_arg, data);
    poolClear(unsafe { &raw mut (*parser).m_tempPool });
    1
}

extern "C" fn reportDefault(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut s: *const c_char,
    mut end: *const c_char,
) {
    if enc.isUtf8 == 0 {
        let mut convert_res: XML_Convert_Result = XML_CONVERT_COMPLETED;
        let mut eventPP: *mut *const c_char = null_mut::<*const c_char>();
        let mut eventEndPP: *mut *const c_char = null_mut::<*const c_char>();
        if core::ptr::eq(enc, unsafe { &*(*parser).m_encoding }) {
            eventPP = unsafe { &raw mut (*parser).m_eventPtr };
            eventEndPP = unsafe { &raw mut (*parser).m_eventEndPtr };
        } else {
            eventPP = unsafe { &raw mut (*(*parser).m_openInternalEntities).internalEventPtr };
            eventEndPP =
                unsafe { &raw mut (*(*parser).m_openInternalEntities).internalEventEndPtr };
        }
        loop {
            let mut dataPtr: *mut ICHAR = unsafe { (*parser).m_dataBuf };
            (convert_res, s, dataPtr) =
                (*enc).utf8Convert(enc, s, end, dataPtr, unsafe { (*parser).m_dataBufEnd });
            unsafe { *eventEndPP = s };
            let default_handler = unsafe {
                (*parser)
                    .m_defaultHandler
                    .expect("non-null function pointer")
            };
            let handler_arg = unsafe { (*parser).m_handlerArg };
            let data_buf = unsafe { (*parser).m_dataBuf };
            default_handler(handler_arg, data_buf, unsafe {
                dataPtr.offset_from(data_buf) as c_int
            });
            unsafe { *eventPP = s };
            if !(convert_res != XML_CONVERT_COMPLETED
                && convert_res != XML_CONVERT_INPUT_INCOMPLETE)
            {
                break;
            }
        }
    } else {
        let default_handler = unsafe {
            (*parser)
                .m_defaultHandler
                .expect("non-null function pointer")
        };
        let handler_arg = unsafe { (*parser).m_handlerArg };
        default_handler(handler_arg, s, unsafe { end.offset_from(s) as c_int });
    };
}

extern "C" fn defineAttribute(
    mut type_0: *mut ELEMENT_TYPE,
    mut attId: *mut ATTRIBUTE_ID,
    mut isCdata: XML_Bool,
    mut isId: XML_Bool,
    mut value: *const XML_Char,
    mut parser: XML_Parser,
) -> c_int {
    let att: *mut DEFAULT_ATTRIBUTE;
    if !value.is_null() || isId as c_int != 0 {
        let mut i: c_int = 0;
        i = 0;
        while i < unsafe { (*type_0).nDefaultAtts } {
            if std::ptr::eq(attId, unsafe {
                (*(*type_0).defaultAtts.offset(i as isize)).id
            }) {
                return 1i32;
            }
            i += 1;
        }
        if isId as c_int != 0
            && unsafe { (*type_0).idAtt.is_null() }
            && unsafe { (*attId).xmlns == 0 }
        {
            unsafe { (*type_0).idAtt = attId };
        }
    }
    if unsafe { (*type_0).nDefaultAtts == (*type_0).allocDefaultAtts } {
        if unsafe { (*type_0).allocDefaultAtts == 0 } {
            unsafe { (*type_0).allocDefaultAtts = 8 };
            let alloc_default_atts = unsafe { (*type_0).allocDefaultAtts as size_t };
            let default_atts = expat_malloc(
                parser,
                alloc_default_atts.wrapping_mul(size_of::<DEFAULT_ATTRIBUTE>()),
                7182,
            ) as *mut DEFAULT_ATTRIBUTE;
            unsafe { (*type_0).defaultAtts = default_atts };
            if default_atts.is_null() {
                unsafe { (*type_0).allocDefaultAtts = 0 };
                return 0i32;
            }
        } else {
            if unsafe { (*type_0).allocDefaultAtts > INT_MAX / 2 } {
                return 0i32;
            }
            let count: c_int = unsafe { (*type_0).allocDefaultAtts * 2 };
            let old_default_atts = unsafe { (*type_0).defaultAtts as *mut c_void };
            let temp = expat_realloc(
                parser,
                old_default_atts,
                (count as size_t).wrapping_mul(size_of::<DEFAULT_ATTRIBUTE>()),
                7208,
            ) as *mut DEFAULT_ATTRIBUTE;
            if temp.is_null() {
                return 0i32;
            }
            unsafe { (*type_0).allocDefaultAtts = count };
            unsafe { (*type_0).defaultAtts = temp };
        }
    }
    att = unsafe {
        (*type_0)
            .defaultAtts
            .offset((*type_0).nDefaultAtts as isize)
    };
    unsafe { (*att).id = attId };
    unsafe { (*att).value = value };
    unsafe { (*att).isCdata = isCdata };
    if isCdata == 0 {
        unsafe { (*attId).maybeTokenized = XML_TRUE };
    }
    unsafe { (*type_0).nDefaultAtts += 1 };
    1
}

extern "C" fn setElementTypePrefix(
    mut parser: XML_Parser,
    mut elementType: *mut ELEMENT_TYPE,
) -> c_int {
    let dtd: *mut DTD = unsafe { (*parser).m_dtd };
    let mut name: *const XML_Char = unsafe { (*elementType).name };
    while unsafe { *name != 0 } {
        if unsafe { *name as c_int == 0x3a } {
            let mut s: *const XML_Char = unsafe { (*elementType).name };
            while s != name {
                let exhausted = unsafe { std::ptr::eq((*dtd).pool.ptr, (*dtd).pool.end) };
                if exhausted && poolGrow(unsafe { &raw mut (*dtd).pool }) == 0 {
                    return 0i32;
                }
                let fresh = unsafe { (*dtd).pool.ptr };
                unsafe { (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1) };
                unsafe { *fresh = *s };
                s = unsafe { s.offset(1) };
            }
            let exhausted = unsafe { std::ptr::eq((*dtd).pool.ptr, (*dtd).pool.end) };
            if exhausted && poolGrow(unsafe { &raw mut (*dtd).pool }) == 0 {
                return 0i32;
            }
            let fresh = unsafe { (*dtd).pool.ptr };
            unsafe { (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1) };
            unsafe { *fresh = '\0' as XML_Char };
            let prefix = lookup(
                parser,
                unsafe { &raw mut (*dtd).prefixes },
                unsafe { (*dtd).pool.start as KEY },
                size_of::<PREFIX>(),
            ) as *mut PREFIX;
            if prefix.is_null() {
                return 0i32;
            }
            if unsafe { std::ptr::eq((*prefix).name, (*dtd).pool.start) } {
                unsafe { (*dtd).pool.start = (*dtd).pool.ptr };
            } else {
                unsafe { (*dtd).pool.ptr = (*dtd).pool.start };
            }
            unsafe { (*elementType).prefix = prefix };
            break;
        } else {
            name = unsafe { name.offset(1) };
        }
    }
    1
}

#[inline]
fn dtdPoolAppendChar(mut dtd: *mut DTD, ch: XML_Char) -> XML_Bool {
    if unsafe { std::ptr::eq((*dtd).pool.ptr, (*dtd).pool.end) }
        && poolGrow(unsafe { &raw mut (*dtd).pool }) == 0
    {
        return XML_FALSE;
    }
    let fresh = unsafe { (*dtd).pool.ptr };
    unsafe { (*dtd).pool.ptr = (*dtd).pool.ptr.offset(1) };
    unsafe { *fresh = ch };
    XML_TRUE
}

#[inline]
fn parserTempPoolAppendChar(mut parser: XML_Parser, ch: XML_Char) -> XML_Bool {
    if unsafe { std::ptr::eq((*parser).m_tempPool.ptr, (*parser).m_tempPool.end) }
        && poolGrow(unsafe { &raw mut (*parser).m_tempPool }) == 0
    {
        return XML_FALSE;
    }
    let fresh = unsafe { (*parser).m_tempPool.ptr };
    unsafe { (*parser).m_tempPool.ptr = (*parser).m_tempPool.ptr.offset(1) };
    unsafe { *fresh = ch };
    XML_TRUE
}

extern "C" fn getAttributeId(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut start: *const c_char,
    mut end: *const c_char,
) -> *mut ATTRIBUTE_ID {
    let dtd: *mut DTD = unsafe { (*parser).m_dtd };
    if dtdPoolAppendChar(dtd, '\0' as XML_Char) == 0 {
        return null_mut::<ATTRIBUTE_ID>();
    }
    let mut name: *const XML_Char =
        poolStoreString(unsafe { &raw mut (*dtd).pool }, enc, start, end);
    if name.is_null() {
        return null_mut::<ATTRIBUTE_ID>();
    }
    name = unsafe { name.offset(1) };
    let id = lookup(
        parser,
        unsafe { &raw mut (*dtd).attributeIds },
        name,
        size_of::<ATTRIBUTE_ID>(),
    ) as *mut ATTRIBUTE_ID;
    if id.is_null() {
        return null_mut::<ATTRIBUTE_ID>();
    }
    if !unsafe { std::ptr::eq((*id).name, name) } {
        unsafe { (*dtd).pool.ptr = (*dtd).pool.start };
    } else {
        unsafe { (*dtd).pool.start = (*dtd).pool.ptr };
        if unsafe { (*parser).m_ns != 0 } {
            if unsafe { *name.offset(0) as c_int == 0x78 }
                && unsafe { *name.offset(1) as c_int == 0x6d }
                && unsafe { *name.offset(2) as c_int == 0x6c }
                && unsafe { *name.offset(3) as c_int == 0x6e }
                && unsafe { *name.offset(4) as c_int == 0x73 }
                && (unsafe { *name.offset(5) as c_int == '\0' as i32 }
                    || unsafe { *name.offset(5) as c_int == 0x3a })
            {
                if unsafe { *name.offset(5) as c_int == '\0' as i32 } {
                    unsafe { (*id).prefix = &raw mut (*dtd).defaultPrefix };
                } else {
                    let prefix = lookup(
                        parser,
                        unsafe { &raw mut (*dtd).prefixes },
                        unsafe { name.offset(6isize) },
                        size_of::<PREFIX>(),
                    ) as *mut PREFIX;
                    unsafe { (*id).prefix = prefix };
                }
                unsafe { (*id).xmlns = XML_TRUE };
            } else {
                let mut i: c_int = 0;
                i = 0;
                while unsafe { *name.offset(i as isize) != 0 } {
                    if unsafe { *name.offset(i as isize) as c_int == 0x3a } {
                        let mut j: c_int = 0;
                        j = 0;
                        while j < i {
                            if dtdPoolAppendChar(dtd, unsafe { *name.offset(j as isize) }) == 0 {
                                return null_mut::<ATTRIBUTE_ID>();
                            }
                            j += 1;
                        }
                        if dtdPoolAppendChar(dtd, '\0' as XML_Char) == 0 {
                            return null_mut::<ATTRIBUTE_ID>();
                        }
                        let prefix = lookup(
                            parser,
                            unsafe { &raw mut (*dtd).prefixes },
                            unsafe { (*dtd).pool.start as KEY },
                            size_of::<PREFIX>(),
                        ) as *mut PREFIX;
                        unsafe { (*id).prefix = prefix };
                        if unsafe { (*id).prefix.is_null() } {
                            return null_mut::<ATTRIBUTE_ID>();
                        }
                        if unsafe { std::ptr::eq((*(*id).prefix).name, (*dtd).pool.start) } {
                            unsafe { (*dtd).pool.start = (*dtd).pool.ptr };
                        } else {
                            unsafe { (*dtd).pool.ptr = (*dtd).pool.start };
                        }
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
        }
    }
    id
}

extern "C" fn getContext(mut parser: XML_Parser) -> *const XML_Char {
    let dtd: *mut DTD = unsafe { (*parser).m_dtd };
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: null_mut::<*mut NAMED>(),
        end: null_mut::<*mut NAMED>(),
        table: null::<HASH_TABLE>(),
        index: 0 as size_t,
    };
    let mut needSep: XML_Bool = XML_FALSE;
    if unsafe { !(*dtd).defaultPrefix.binding.is_null() } {
        if parserTempPoolAppendChar(parser, 0x3di8) == 0 {
            return null::<XML_Char>();
        }
        let mut len: c_int = unsafe { (*(*dtd).defaultPrefix.binding).uriLen };
        if unsafe { (*parser).m_namespaceSeparator != 0 } {
            len -= 1;
        }
        let mut i: c_int = 0;
        while i < len {
            if parserTempPoolAppendChar(parser, unsafe {
                *(*(*dtd).defaultPrefix.binding).uri.offset(i as isize)
            }) == 0
            {
                return null::<XML_Char>();
            }
            i += 1;
        }
        needSep = XML_TRUE;
    }
    hashTableIterInit(&raw mut iter, unsafe { &raw mut (*dtd).prefixes });
    loop {
        let prefix: *mut PREFIX = hashTableIterNext(&raw mut iter) as *mut PREFIX;
        if prefix.is_null() {
            break;
        }
        if unsafe { (*prefix).binding.is_null() } {
            continue;
        }
        if needSep as c_int != 0 && parserTempPoolAppendChar(parser, 0xci8) == 0 {
            return null::<XML_Char>();
        }
        let mut s: *const XML_Char = unsafe { (*prefix).name };
        while unsafe { *s != 0 } {
            if parserTempPoolAppendChar(parser, unsafe { *s }) == 0 {
                return null::<XML_Char>();
            }
            s = unsafe { s.offset(1) };
        }
        if parserTempPoolAppendChar(parser, 0x3di8) == 0 {
            return null::<XML_Char>();
        }
        let mut len_0: c_int = unsafe { (*(*prefix).binding).uriLen };
        if unsafe { (*parser).m_namespaceSeparator != 0 } {
            len_0 -= 1;
        }
        let mut i_0: c_int = 0;
        while i_0 < len_0 {
            if parserTempPoolAppendChar(parser, unsafe {
                *(*(*prefix).binding).uri.offset(i_0 as isize)
            }) == 0
            {
                return null::<XML_Char>();
            }
            i_0 += 1;
        }
        needSep = XML_TRUE;
    }
    hashTableIterInit(&raw mut iter, unsafe { &raw mut (*dtd).generalEntities });
    loop {
        let e: *mut ENTITY = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if e.is_null() {
            break;
        }
        if unsafe { (*e).open == 0 } {
            continue;
        }
        if needSep as c_int != 0 && parserTempPoolAppendChar(parser, 0xci8) == 0 {
            return null::<XML_Char>();
        }
        let mut s_0: *const XML_Char = unsafe { (*e).name };
        while unsafe { *s_0 != 0 } {
            if parserTempPoolAppendChar(parser, unsafe { *s_0 }) == 0 {
                return null::<XML_Char>();
            }
            s_0 = unsafe { s_0.offset(1) };
        }
        needSep = XML_TRUE;
    }
    if parserTempPoolAppendChar(parser, '\0' as XML_Char) == 0 {
        return null::<XML_Char>();
    }
    unsafe { (*parser).m_tempPool.start }
}

extern "C" fn setContext(mut parser: XML_Parser, mut context: *const XML_Char) -> XML_Bool {
    if context.is_null() {
        return XML_FALSE;
    }
    let dtd: *mut DTD = unsafe { (*parser).m_dtd };
    let mut s: *const XML_Char = context;
    while unsafe { *context as c_int != '\0' as i32 } {
        if unsafe { *s as c_int == 0xc || *s as c_int == '\0' as i32 } {
            if parserTempPoolAppendChar(parser, '\0' as XML_Char) == 0 {
                return XML_FALSE;
            }
            let e = lookup(
                parser,
                unsafe { &raw mut (*dtd).generalEntities },
                unsafe { (*parser).m_tempPool.start as KEY },
                0,
            ) as *mut ENTITY;
            if !e.is_null() {
                unsafe { (*e).open = XML_TRUE };
            }
            if unsafe { *s as c_int != '\0' as i32 } {
                s = unsafe { s.offset(1) };
            }
            context = s;
            unsafe { (*parser).m_tempPool.ptr = (*parser).m_tempPool.start };
        } else if unsafe { *s as c_int == 0x3d } {
            let prefix: *mut PREFIX;
            if unsafe {
                (*parser)
                    .m_tempPool
                    .ptr
                    .offset_from((*parser).m_tempPool.start) as c_long
                    == 0
            } {
                prefix = unsafe { &raw mut (*dtd).defaultPrefix };
            } else {
                if parserTempPoolAppendChar(parser, '\0' as XML_Char) == 0 {
                    return XML_FALSE;
                }
                prefix = lookup(
                    parser,
                    unsafe { &raw mut (*dtd).prefixes },
                    unsafe { (*parser).m_tempPool.start as KEY },
                    size_of::<PREFIX>(),
                ) as *mut PREFIX;
                if prefix.is_null() {
                    return XML_FALSE;
                }
                if unsafe { std::ptr::eq((*prefix).name, (*parser).m_tempPool.start) } {
                    let copied =
                        poolCopyString(unsafe { &raw mut (*dtd).pool }, unsafe { (*prefix).name });
                    if copied.is_null() {
                        return XML_FALSE;
                    }
                    unsafe { (*prefix).name = copied };
                }
                unsafe { (*parser).m_tempPool.ptr = (*parser).m_tempPool.start };
            }
            context = unsafe { s.offset(1) };
            while unsafe { *context as c_int != 0xc && *context as c_int != '\0' as i32 } {
                if parserTempPoolAppendChar(parser, unsafe { *context }) == 0 {
                    return XML_FALSE;
                }
                context = unsafe { context.offset(1) };
            }
            if parserTempPoolAppendChar(parser, '\0' as XML_Char) == 0 {
                return XML_FALSE;
            }
            if addBinding(
                parser,
                prefix,
                null::<ATTRIBUTE_ID>(),
                unsafe { (*parser).m_tempPool.start },
                unsafe { &raw mut (*parser).m_inheritedBindings },
            ) != XML_ERROR_NONE
            {
                return XML_FALSE;
            }
            unsafe { (*parser).m_tempPool.ptr = (*parser).m_tempPool.start };
            if unsafe { *context as c_int != '\0' as i32 } {
                context = unsafe { context.offset(1) };
            }
            s = context;
        } else {
            if parserTempPoolAppendChar(parser, unsafe { *s }) == 0 {
                return XML_FALSE;
            }
            s = unsafe { s.offset(1) };
        }
    }
    XML_TRUE
}

extern "C" fn normalizePublicId(mut publicId: *mut XML_Char) {
    let mut p: *mut XML_Char = publicId;
    let mut s: *mut XML_Char = publicId;
    while unsafe { *s != 0 } {
        match unsafe { *s as c_int } {
            32 | 13 | 10 => {
                if p != publicId && unsafe { *p.offset(-1) as c_int != 0x20 } {
                    let fresh70 = p;
                    p = unsafe { p.offset(1) };
                    unsafe { *fresh70 = 0x20i8 };
                }
            }
            _ => {
                let fresh71 = p;
                p = unsafe { p.offset(1) };
                unsafe { *fresh71 = *s };
            }
        }
        s = unsafe { s.offset(1) };
    }
    if p != publicId && unsafe { *p.offset(-1) as c_int == 0x20 } {
        p = unsafe { p.offset(-1) };
    }
    unsafe { *p = '\0' as XML_Char };
}

extern "C" fn dtdCreate(mut parser: XML_Parser) -> *mut DTD {
    let mut p: *mut DTD = expat_malloc(parser, size_of::<DTD>(), 7500) as *mut DTD;
    if p.is_null() {
        return p;
    }
    poolInit(unsafe { &raw mut (*p).pool }, parser);
    poolInit(unsafe { &raw mut (*p).entityValuePool }, parser);
    hashTableInit(unsafe { &raw mut (*p).generalEntities }, parser);
    hashTableInit(unsafe { &raw mut (*p).elementTypes }, parser);
    hashTableInit(unsafe { &raw mut (*p).attributeIds }, parser);
    hashTableInit(unsafe { &raw mut (*p).prefixes }, parser);
    unsafe { (*p).paramEntityRead = XML_FALSE };
    hashTableInit(unsafe { &raw mut (*p).paramEntities }, parser);
    unsafe { (*p).defaultPrefix.name = null::<XML_Char>() };
    unsafe { (*p).defaultPrefix.binding = null_mut::<BINDING>() };
    unsafe { (*p).in_eldecl = XML_FALSE };
    unsafe { (*p).scaffIndex = null_mut::<c_int>() };
    unsafe { (*p).scaffold = null_mut::<CONTENT_SCAFFOLD>() };
    unsafe { (*p).scaffLevel = 0 };
    unsafe { (*p).scaffSize = 0 };
    unsafe { (*p).scaffCount = 0 };
    unsafe { (*p).contentStringLen = 0 };
    unsafe { (*p).keepProcessing = XML_TRUE };
    unsafe { (*p).hasParamEntityRefs = XML_FALSE };
    unsafe { (*p).standalone = XML_FALSE };
    p
}

extern "C" fn dtdReset(mut p: *mut DTD, mut parser: XML_Parser) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: null_mut::<*mut NAMED>(),
        end: null_mut::<*mut NAMED>(),
        table: null::<HASH_TABLE>(),
        index: 0 as size_t,
    };
    hashTableIterInit(&raw mut iter, unsafe { &raw mut (*p).elementTypes });
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if unsafe { (*e).allocDefaultAtts != 0 } {
            let default_atts = unsafe { (*e).defaultAtts };
            expat_free(parser, default_atts as *mut c_void, 7539i32);
        }
    }
    hashTableClear(unsafe { &raw mut (*p).generalEntities });
    unsafe { (*p).paramEntityRead = XML_FALSE };
    hashTableClear(unsafe { &raw mut (*p).paramEntities });
    hashTableClear(unsafe { &raw mut (*p).elementTypes });
    hashTableClear(unsafe { &raw mut (*p).attributeIds });
    hashTableClear(unsafe { &raw mut (*p).prefixes });
    poolClear(unsafe { &raw mut (*p).pool });
    poolClear(unsafe { &raw mut (*p).entityValuePool });
    unsafe { (*p).defaultPrefix.name = null::<XML_Char>() };
    unsafe { (*p).defaultPrefix.binding = null_mut::<BINDING>() };
    unsafe { (*p).in_eldecl = XML_FALSE };
    let scaff_index = unsafe { (*p).scaffIndex };
    expat_free(parser, scaff_index as *mut c_void, 7556);
    unsafe { (*p).scaffIndex = null_mut::<c_int>() };
    let scaffold = unsafe { (*p).scaffold };
    expat_free(parser, scaffold as *mut c_void, 7558);
    unsafe { (*p).scaffold = null_mut::<CONTENT_SCAFFOLD>() };
    unsafe { (*p).scaffLevel = 0 };
    unsafe { (*p).scaffSize = 0u32 };
    unsafe { (*p).scaffCount = 0u32 };
    unsafe { (*p).contentStringLen = 0u32 };
    unsafe { (*p).keepProcessing = XML_TRUE };
    unsafe { (*p).hasParamEntityRefs = XML_FALSE };
    unsafe { (*p).standalone = XML_FALSE };
}

extern "C" fn dtdDestroy(mut p: *mut DTD, mut isDocEntity: XML_Bool, mut parser: XML_Parser) {
    let mut iter: HASH_TABLE_ITER = HASH_TABLE_ITER {
        p: null_mut::<*mut NAMED>(),
        end: null_mut::<*mut NAMED>(),
        table: null::<HASH_TABLE>(),
        index: 0 as size_t,
    };
    hashTableIterInit(&raw mut iter, unsafe { &raw mut (*p).elementTypes });
    loop {
        let mut e: *mut ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if e.is_null() {
            break;
        }
        if unsafe { (*e).allocDefaultAtts != 0 } {
            let default_atts = unsafe { (*e).defaultAtts };
            expat_free(parser, default_atts as *mut c_void, 7580i32);
        }
    }
    hashTableDestroy(unsafe { &raw mut (*p).generalEntities });
    hashTableDestroy(unsafe { &raw mut (*p).paramEntities });
    hashTableDestroy(unsafe { &raw mut (*p).elementTypes });
    hashTableDestroy(unsafe { &raw mut (*p).attributeIds });
    hashTableDestroy(unsafe { &raw mut (*p).prefixes });
    poolDestroy(unsafe { &raw mut (*p).pool });
    poolDestroy(unsafe { &raw mut (*p).entityValuePool });
    if isDocEntity != 0 {
        let scaff_index = unsafe { (*p).scaffIndex };
        let scaffold = unsafe { (*p).scaffold };
        expat_free(parser, scaff_index as *mut c_void, 7592);
        expat_free(parser, scaffold as *mut c_void, 7593i32);
    }
    expat_free(parser, p as *mut c_void, 7595);
}

extern "C" fn dtdCopy(
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
    hashTableIterInit(&raw mut iter, unsafe { &raw const (*oldDtd).prefixes });
    loop {
        let oldP: *const PREFIX = hashTableIterNext(&raw mut iter) as *mut PREFIX;
        if oldP.is_null() {
            break;
        }
        let name = poolCopyString(unsafe { &raw mut (*newDtd).pool }, unsafe { (*oldP).name });
        if name.is_null() {
            return 0i32;
        }
        if lookup(
            oldParser,
            unsafe { &raw mut (*newDtd).prefixes },
            name,
            size_of::<PREFIX>(),
        )
        .is_null()
        {
            return 0i32;
        }
    }
    hashTableIterInit(&raw mut iter, unsafe { &raw const (*oldDtd).attributeIds });
    loop {
        let oldA: *const ATTRIBUTE_ID = hashTableIterNext(&raw mut iter) as *mut ATTRIBUTE_ID;
        if oldA.is_null() {
            break;
        }
        if dtdPoolAppendChar(newDtd, '\0' as XML_Char) == 0 {
            return 0i32;
        }
        let mut name_0 =
            poolCopyString(unsafe { &raw mut (*newDtd).pool }, unsafe { (*oldA).name });
        if name_0.is_null() {
            return 0i32;
        }
        name_0 = unsafe { name_0.offset(1) };
        let newA = lookup(
            oldParser,
            unsafe { &raw mut (*newDtd).attributeIds },
            name_0,
            size_of::<ATTRIBUTE_ID>(),
        ) as *mut ATTRIBUTE_ID;
        if newA.is_null() {
            return 0i32;
        }
        unsafe { (*newA).maybeTokenized = (*oldA).maybeTokenized };
        if unsafe { !(*oldA).prefix.is_null() } {
            unsafe { (*newA).xmlns = (*oldA).xmlns };
            if unsafe { std::ptr::eq((*oldA).prefix, &raw const (*oldDtd).defaultPrefix) } {
                unsafe { (*newA).prefix = &raw mut (*newDtd).defaultPrefix };
            } else {
                let prefix = lookup(
                    oldParser,
                    unsafe { &raw mut (*newDtd).prefixes },
                    unsafe { (*(*oldA).prefix).name },
                    0usize,
                ) as *mut PREFIX;
                unsafe { (*newA).prefix = prefix };
            }
        }
    }
    hashTableIterInit(&raw mut iter, unsafe { &raw const (*oldDtd).elementTypes });
    loop {
        let oldE: *const ELEMENT_TYPE = hashTableIterNext(&raw mut iter) as *mut ELEMENT_TYPE;
        if oldE.is_null() {
            break;
        }
        let name_1 = poolCopyString(unsafe { &raw mut (*newDtd).pool }, unsafe { (*oldE).name });
        if name_1.is_null() {
            return 0i32;
        }
        let newE = lookup(
            oldParser,
            unsafe { &raw mut (*newDtd).elementTypes },
            name_1,
            size_of::<ELEMENT_TYPE>(),
        ) as *mut ELEMENT_TYPE;
        if newE.is_null() {
            return 0i32;
        }
        if unsafe { (*oldE).nDefaultAtts != 0 } {
            let default_atts = expat_malloc(
                parser,
                (unsafe { (*oldE).nDefaultAtts as size_t })
                    .wrapping_mul(size_of::<DEFAULT_ATTRIBUTE>()),
                7683,
            ) as *mut DEFAULT_ATTRIBUTE;
            unsafe { (*newE).defaultAtts = default_atts };
            if default_atts.is_null() {
                return 0i32;
            }
        }
        if unsafe { !(*oldE).idAtt.is_null() } {
            let id_att = lookup(
                oldParser,
                unsafe { &raw mut (*newDtd).attributeIds },
                unsafe { (*(*oldE).idAtt).name as KEY },
                0usize,
            ) as *mut ATTRIBUTE_ID;
            unsafe { (*newE).idAtt = id_att };
        }
        unsafe { (*newE).nDefaultAtts = (*oldE).nDefaultAtts };
        unsafe { (*newE).allocDefaultAtts = (*newE).nDefaultAtts };
        if unsafe { !(*oldE).prefix.is_null() } {
            let prefix = lookup(
                oldParser,
                unsafe { &raw mut (*newDtd).prefixes },
                unsafe { (*(*oldE).prefix).name },
                0usize,
            ) as *mut PREFIX;
            unsafe { (*newE).prefix = prefix };
        }
        let mut i: c_int = 0;
        while i < unsafe { (*newE).nDefaultAtts } {
            let old_default_att = unsafe { (*oldE).defaultAtts.offset(i as isize) };
            let new_default_att = unsafe { (*newE).defaultAtts.offset(i as isize) };
            let id = lookup(
                oldParser,
                unsafe { &raw mut (*newDtd).attributeIds },
                unsafe { (*(*old_default_att).id).name as KEY },
                0,
            ) as *mut ATTRIBUTE_ID;
            unsafe { (*new_default_att).id = id };
            unsafe { (*new_default_att).isCdata = (*old_default_att).isCdata };
            if unsafe { !(*old_default_att).value.is_null() } {
                let copied = poolCopyString(unsafe { &raw mut (*newDtd).pool }, unsafe {
                    (*old_default_att).value
                });
                if copied.is_null() {
                    return 0i32;
                }
                unsafe { (*new_default_att).value = copied };
            } else {
                unsafe { (*new_default_att).value = null::<XML_Char>() };
            }
            i += 1;
        }
    }
    if copyEntityTable(
        oldParser,
        unsafe { &raw mut (*newDtd).generalEntities },
        unsafe { &raw mut (*newDtd).pool },
        unsafe { &raw const (*oldDtd).generalEntities },
    ) == 0
    {
        return 0i32;
    }
    if copyEntityTable(
        oldParser,
        unsafe { &raw mut (*newDtd).paramEntities },
        unsafe { &raw mut (*newDtd).pool },
        unsafe { &raw const (*oldDtd).paramEntities },
    ) == 0
    {
        return 0i32;
    }
    unsafe { (*newDtd).paramEntityRead = (*oldDtd).paramEntityRead };
    unsafe { (*newDtd).keepProcessing = (*oldDtd).keepProcessing };
    unsafe { (*newDtd).hasParamEntityRefs = (*oldDtd).hasParamEntityRefs };
    unsafe { (*newDtd).standalone = (*oldDtd).standalone };
    unsafe { (*newDtd).in_eldecl = (*oldDtd).in_eldecl };
    unsafe { (*newDtd).scaffold = (*oldDtd).scaffold };
    unsafe { (*newDtd).contentStringLen = (*oldDtd).contentStringLen };
    unsafe { (*newDtd).scaffSize = (*oldDtd).scaffSize };
    unsafe { (*newDtd).scaffLevel = (*oldDtd).scaffLevel };
    unsafe { (*newDtd).scaffIndex = (*oldDtd).scaffIndex };
    1
}

extern "C" fn copyEntityTable(
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
        let oldE: *const ENTITY = hashTableIterNext(&raw mut iter) as *mut ENTITY;
        if oldE.is_null() {
            break;
        }
        let name = poolCopyString(newPool, unsafe { (*oldE).name });
        if name.is_null() {
            return 0i32;
        }
        let newE = lookup(oldParser, newTable, name, size_of::<ENTITY>()) as *mut ENTITY;
        if newE.is_null() {
            return 0i32;
        }
        if unsafe { !(*oldE).systemId.is_null() } {
            let mut tem: *const XML_Char = poolCopyString(newPool, unsafe { (*oldE).systemId });
            if tem.is_null() {
                return 0i32;
            }
            unsafe { (*newE).systemId = tem };
            if unsafe { !(*oldE).base.is_null() } {
                if unsafe { (*oldE).base == cachedOldBase } {
                    unsafe { (*newE).base = cachedNewBase };
                } else {
                    cachedOldBase = unsafe { (*oldE).base };
                    tem = poolCopyString(newPool, cachedOldBase);
                    if tem.is_null() {
                        return 0i32;
                    }
                    unsafe { (*newE).base = tem };
                    cachedNewBase = unsafe { (*newE).base };
                }
            }
            if unsafe { !(*oldE).publicId.is_null() } {
                tem = poolCopyString(newPool, unsafe { (*oldE).publicId });
                if tem.is_null() {
                    return 0i32;
                }
                unsafe { (*newE).publicId = tem };
            }
        } else {
            let tem_0: *const XML_Char =
                poolCopyStringN(newPool, unsafe { (*oldE).textPtr }, unsafe {
                    (*oldE).textLen
                });
            if tem_0.is_null() {
                return 0i32;
            }
            unsafe { (*newE).textPtr = tem_0 };
            unsafe { (*newE).textLen = (*oldE).textLen };
        }
        if unsafe { !(*oldE).notation.is_null() } {
            let tem_1: *const XML_Char = poolCopyString(newPool, unsafe { (*oldE).notation });
            if tem_1.is_null() {
                return 0i32;
            }
            unsafe { (*newE).notation = tem_1 };
        }
        unsafe { (*newE).is_param = (*oldE).is_param };
        unsafe { (*newE).is_internal = (*oldE).is_internal };
    }
    1
}

pub const INIT_POWER: c_int = 6;

extern "C" fn keyeq(mut s1: KEY, mut s2: KEY) -> XML_Bool {
    while unsafe { *s1 as c_int == *s2 as c_int } {
        if unsafe { *s1 as c_int == 0 } {
            return XML_TRUE;
        }
        s1 = unsafe { s1.offset(1) };
        s2 = unsafe { s2.offset(1) };
    }
    XML_FALSE
}

extern "C" fn keylen(mut s: KEY) -> size_t {
    let mut len: size_t = 0;
    while unsafe { *s != 0 } {
        s = unsafe { s.offset(1) };
        len = len.wrapping_add(1);
    }
    len
}

extern "C" fn copy_salt_to_sipkey(mut parser: XML_Parser, mut key: *mut sipkey) {
    unsafe { (*key).k[0] = 0u64 };
    unsafe { (*key).k[1] = get_hash_secret_salt(parser) };
}

extern "C" fn hash(mut parser: XML_Parser, mut s: KEY) -> c_ulong {
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
    sip24_final(&raw mut state)
}

extern "C" fn lookup(
    mut parser: XML_Parser,
    mut table: *mut HASH_TABLE,
    mut name: KEY,
    mut createSize: size_t,
) -> *mut NAMED {
    let _ = parser;
    let mut key: Vec<XML_Char> = Vec::new();
    let mut key_cursor: KEY = name;
    while unsafe { *key_cursor != 0 } {
        key.push(unsafe { *key_cursor });
        key_cursor = unsafe { key_cursor.offset(1) };
    }
    if let Some(&entry) = unsafe { (*table).entries.get(&key) } {
        return entry;
    }
    if createSize == 0 {
        return null_mut::<NAMED>();
    }
    let table_parser = unsafe { (*table).parser };
    let mut entry: *mut NAMED = expat_malloc(table_parser, createSize, 7914 as c_int) as *mut NAMED;
    if entry.is_null() {
        return null_mut::<NAMED>();
    }
    unsafe { memset(entry as *mut c_void, 0 as c_int, createSize) };
    unsafe { (*entry).name = name };
    unsafe { (*table).entries.insert(key, entry) };
    entry
}

extern "C" fn hashTableClear(mut table: *mut HASH_TABLE) {
    let parser = unsafe { (*table).parser };
    let entries = unsafe { &mut (*table).entries };
    for (_, entry) in entries.drain() {
        expat_free(parser, entry as *mut c_void, 7927 as c_int);
    }
}

extern "C" fn hashTableDestroy(mut table: *mut HASH_TABLE) {
    hashTableClear(table);
    unsafe { core::ptr::drop_in_place(table) };
}

extern "C" fn hashTableInit(mut p: *mut HASH_TABLE, mut parser: XML_Parser) {
    unsafe {
        write(
            p,
            HASH_TABLE {
                entries: std::collections::HashMap::new(),
                parser,
            },
        );
    }
}

extern "C" fn hashTableIterInit(mut iter: *mut HASH_TABLE_ITER, mut table: *const HASH_TABLE) {
    unsafe { (*iter).p = null_mut::<*mut NAMED>() };
    unsafe { (*iter).end = null_mut::<*mut NAMED>() };
    unsafe { (*iter).table = table };
    unsafe { (*iter).index = 0 as size_t };
}

extern "C" fn hashTableIterNext(mut iter: *mut HASH_TABLE_ITER) -> *mut NAMED {
    let table_ptr = unsafe { (*iter).table };
    if table_ptr.is_null() {
        return null_mut::<NAMED>();
    }
    let mut table: &HASH_TABLE = unsafe { &*table_ptr };
    let index = unsafe { (*iter).index };
    if index >= table.entries.len() {
        return null_mut::<NAMED>();
    }
    unsafe { (*iter).index = index.wrapping_add(1) };
    if let Some(&entry) = table.entries.values().nth(index) {
        return entry;
    }
    null_mut::<NAMED>()
}

extern "C" fn poolInit(mut pool: *mut STRING_POOL, mut parser: XML_Parser) {
    unsafe { (*pool).blocks = null_mut::<BLOCK>() };
    unsafe { (*pool).freeBlocks = null_mut::<BLOCK>() };
    unsafe { (*pool).start = null_mut::<XML_Char>() };
    unsafe { (*pool).ptr = null_mut::<XML_Char>() };
    unsafe { (*pool).end = null::<XML_Char>() };
    unsafe { (*pool).parser = parser };
}

extern "C" fn poolClear(mut pool: *mut STRING_POOL) {
    if unsafe { (*pool).freeBlocks.is_null() } {
        let blocks = unsafe { (*pool).blocks };
        unsafe { (*pool).freeBlocks = blocks };
    } else {
        let mut p: *mut BLOCK = unsafe { (*pool).blocks };
        while !p.is_null() {
            let mut tem: *mut BLOCK = unsafe { (*p).next };
            let free_blocks = unsafe { (*pool).freeBlocks };
            unsafe { (*p).next = free_blocks };
            unsafe { (*pool).freeBlocks = p };
            p = tem;
        }
    }
    unsafe { (*pool).blocks = null_mut::<BLOCK>() };
    unsafe { (*pool).start = null_mut::<XML_Char>() };
    unsafe { (*pool).ptr = null_mut::<XML_Char>() };
    unsafe { (*pool).end = null::<XML_Char>() };
}

extern "C" fn poolDestroy(mut pool: *mut STRING_POOL) {
    let parser = unsafe { (*pool).parser };
    let mut p: *mut BLOCK = unsafe { (*pool).blocks };
    while !p.is_null() {
        let mut tem: *mut BLOCK = unsafe { (*p).next };
        expat_free(parser, p as *mut c_void, 8000);
        p = tem;
    }
    p = unsafe { (*pool).freeBlocks };
    while !p.is_null() {
        let mut tem_0: *mut BLOCK = unsafe { (*p).next };
        expat_free(parser, p as *mut c_void, 8006);
        p = tem_0;
    }
}

extern "C" fn poolAppend(
    mut pool: *mut STRING_POOL,
    enc: &ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
) -> *mut XML_Char {
    if unsafe { (*pool).ptr.is_null() } && poolGrow(pool) == 0 {
        return null_mut::<XML_Char>();
    }
    loop {
        let convert_res: XML_Convert_Result;
        let pool_ptr = unsafe { (*pool).ptr };
        let pool_end = unsafe { (*pool).end };
        let new_pool_ptr: *mut XML_Char;
        (convert_res, ptr, new_pool_ptr) = (*enc).utf8Convert(enc, ptr, end, pool_ptr, pool_end);
        unsafe { (*pool).ptr = new_pool_ptr };
        if convert_res == XML_CONVERT_COMPLETED || convert_res == XML_CONVERT_INPUT_INCOMPLETE {
            break;
        }
        if poolGrow(pool) == 0 {
            return null_mut::<XML_Char>();
        }
    }
    unsafe { (*pool).start }
}

extern "C" fn poolCopyString(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
) -> *const XML_Char {
    loop {
        let ptr_at_end = unsafe { std::ptr::eq((*pool).ptr, (*pool).end) };
        if (if ptr_at_end && poolGrow(pool) == 0 {
            0
        } else {
            let fresh59 = unsafe { (*pool).ptr };
            unsafe { (*pool).ptr = (*pool).ptr.offset(1) };
            unsafe { *fresh59 = *s };
            1
        }) == 0
        {
            return null::<XML_Char>();
        }
        let fresh60 = s;
        s = unsafe { s.offset(1) };
        if unsafe { *fresh60 == 0 } {
            break;
        }
    }
    let start = unsafe { (*pool).start };
    let ptr = unsafe { (*pool).ptr };
    unsafe { (*pool).start = ptr };
    start
}

extern "C" fn poolCopyStringN(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
    mut n: c_int,
) -> *const XML_Char {
    if unsafe { (*pool).ptr.is_null() } && poolGrow(pool) == 0 {
        return null::<XML_Char>();
    }
    while n > 0 {
        let ptr_at_end = unsafe { std::ptr::eq((*pool).ptr, (*pool).end) };
        if (if ptr_at_end && poolGrow(pool) == 0 {
            0
        } else {
            let fresh85 = unsafe { (*pool).ptr };
            unsafe { (*pool).ptr = (*pool).ptr.offset(1) };
            unsafe { *fresh85 = *s };
            1
        }) == 0
        {
            return null::<XML_Char>();
        }
        n -= 1;
        s = unsafe { s.offset(1) };
    }
    let start = unsafe { (*pool).start };
    let ptr = unsafe { (*pool).ptr };
    unsafe { (*pool).start = ptr };
    start
}

extern "C" fn poolAppendString(
    mut pool: *mut STRING_POOL,
    mut s: *const XML_Char,
) -> *const XML_Char {
    while unsafe { *s != 0 } {
        let ptr_at_end = unsafe { std::ptr::eq((*pool).ptr, (*pool).end) };
        if (if ptr_at_end && poolGrow(pool) == 0 {
            0
        } else {
            let fresh74 = unsafe { (*pool).ptr };
            unsafe { (*pool).ptr = (*pool).ptr.offset(1) };
            unsafe { *fresh74 = *s };
            1
        }) == 0
        {
            return null::<XML_Char>();
        }
        s = unsafe { s.offset(1) };
    }
    unsafe { (*pool).start }
}

extern "C" fn poolStoreString(
    mut pool: *mut STRING_POOL,
    enc: &ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
) -> *mut XML_Char {
    if poolAppend(pool, enc, ptr, end).is_null() {
        return null_mut::<XML_Char>();
    }
    if unsafe { std::ptr::eq((*pool).ptr, (*pool).end) } && poolGrow(pool) == 0 {
        return null_mut::<XML_Char>();
    }
    let fresh10 = unsafe { (*pool).ptr };
    unsafe { (*pool).ptr = (*pool).ptr.offset(1) };
    unsafe { *fresh10 = 0i8 };
    unsafe { (*pool).start }
}

extern "C" fn poolBytesToAllocateFor(mut blockSize: c_int) -> size_t {
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
    bytesToAllocate as size_t
}

extern "C" fn poolGrow(mut pool: *mut STRING_POOL) -> XML_Bool {
    if unsafe { !(*pool).freeBlocks.is_null() } {
        if unsafe { (*pool).start.is_null() } {
            unsafe { (*pool).blocks = (*pool).freeBlocks };
            unsafe { (*pool).freeBlocks = (*(*pool).freeBlocks).next };
            unsafe { (*(*pool).blocks).next = null_mut::<block>() };
            unsafe { (*pool).start = &raw mut (*(*pool).blocks).s as *mut XML_Char };
            unsafe { (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize) };
            unsafe { (*pool).ptr = (*pool).start };
            return XML_TRUE;
        }
        if unsafe {
            ((*pool).end.offset_from((*pool).start) as c_long)
                < (*(*pool).freeBlocks).size as c_long
        } {
            let tem: *mut BLOCK = unsafe { (*(*pool).freeBlocks).next };
            unsafe { (*(*pool).freeBlocks).next = (*pool).blocks };
            unsafe { (*pool).blocks = (*pool).freeBlocks };
            unsafe { (*pool).freeBlocks = tem };
            let dst = unsafe { &raw mut (*(*pool).blocks).s as *mut c_void };
            let src = unsafe { (*pool).start as *const c_void };
            let copied = unsafe {
                ((*pool).end.offset_from((*pool).start) as size_t)
                    .wrapping_mul(size_of::<XML_Char>())
            };
            unsafe { memcpy(dst, src, copied) };
            let offset = unsafe { (*pool).ptr.offset_from((*pool).start) };
            unsafe { (*pool).ptr = (&raw mut (*(*pool).blocks).s as *mut XML_Char).offset(offset) };
            unsafe { (*pool).start = &raw mut (*(*pool).blocks).s as *mut XML_Char };
            unsafe { (*pool).end = (*pool).start.offset((*(*pool).blocks).size as isize) };
            return XML_TRUE;
        }
    }
    if unsafe {
        !(*pool).blocks.is_null() && (*pool).start == &raw mut (*(*pool).blocks).s as *mut XML_Char
    } {
        let block_size: c_int = unsafe {
            ((*pool).end.offset_from((*pool).start) as c_uint).wrapping_mul(2u32) as c_int
        };
        if block_size < 0 {
            return XML_FALSE;
        }
        let bytes_to_allocate: size_t = poolBytesToAllocateFor(block_size);
        if bytes_to_allocate == 0 {
            return XML_FALSE;
        }
        let parser_ref = unsafe { (*pool).parser };
        let blocks_ref = unsafe { (*pool).blocks as *mut c_void };
        let temp = expat_realloc(parser_ref, blocks_ref, bytes_to_allocate, 8161) as *mut BLOCK;
        if temp.is_null() {
            return XML_FALSE;
        }
        let offset_inside_block: ptrdiff_t = unsafe { (*pool).ptr.offset_from((*pool).start) };
        unsafe { (*pool).blocks = temp };
        unsafe { (*(*pool).blocks).size = block_size };
        unsafe {
            (*pool).ptr =
                (&raw mut (*(*pool).blocks).s as *mut XML_Char).offset(offset_inside_block)
        };
        unsafe { (*pool).start = &raw mut (*(*pool).blocks).s as *mut XML_Char };
        unsafe { (*pool).end = (*pool).start.offset(block_size as isize) };
    } else {
        let mut block_size_0: c_int = unsafe { (*pool).end.offset_from((*pool).start) as c_int };
        if block_size_0 < 0 {
            return XML_FALSE;
        }
        if block_size_0 < INIT_BLOCK_SIZE {
            block_size_0 = INIT_BLOCK_SIZE;
        } else {
            if ((block_size_0 as c_uint).wrapping_mul(2u32) as c_int) < 0 {
                return XML_FALSE;
            }
            block_size_0 *= 2i32;
        }
        let bytes_to_allocate_0: size_t = poolBytesToAllocateFor(block_size_0);
        if bytes_to_allocate_0 == 0 {
            return XML_FALSE;
        }
        let parser_ref = unsafe { (*pool).parser };
        let tem_0 = expat_malloc(parser_ref, bytes_to_allocate_0, 8201) as *mut BLOCK;
        if tem_0.is_null() {
            return XML_FALSE;
        }
        unsafe { (*tem_0).size = block_size_0 };
        unsafe { (*tem_0).next = (*pool).blocks };
        unsafe { (*pool).blocks = tem_0 };
        if unsafe { (*pool).ptr != (*pool).start } {
            let dst = unsafe { &raw mut (*tem_0).s as *mut c_void };
            let src = unsafe { (*pool).start as *const c_void };
            let copied = unsafe {
                ((*pool).ptr.offset_from((*pool).start) as size_t)
                    .wrapping_mul(size_of::<XML_Char>())
            };
            unsafe { memcpy(dst, src, copied) };
        }
        let offset = unsafe { (*pool).ptr.offset_from((*pool).start) };
        unsafe { (*pool).ptr = (&raw mut (*tem_0).s as *mut XML_Char).offset(offset) };
        unsafe { (*pool).start = &raw mut (*tem_0).s as *mut XML_Char };
        unsafe {
            (*pool).end = (&raw mut (*tem_0).s as *mut XML_Char).offset(block_size_0 as isize)
        };
    }
    XML_TRUE
}

extern "C" fn nextScaffoldPart(mut parser: XML_Parser) -> c_int {
    let dtd: *mut DTD = unsafe { (*parser).m_dtd };
    let me: *mut CONTENT_SCAFFOLD;
    let next: c_int;
    if unsafe { (*dtd).scaffIndex.is_null() } {
        unsafe {
            (*dtd).scaffIndex = expat_malloc(
                parser,
                ((*parser).m_groupSize as size_t).wrapping_mul(size_of::<c_int>()),
                8232,
            ) as *mut c_int
        };
        if unsafe { (*dtd).scaffIndex.is_null() } {
            return -(1i32);
        }
        unsafe { *(*dtd).scaffIndex.offset(0isize) = 0i32 };
    }
    if unsafe { (*dtd).scaffCount > INT_MAX as c_uint } {
        return -(1i32);
    }
    if unsafe { (*dtd).scaffCount >= (*dtd).scaffSize } {
        let temp: *mut CONTENT_SCAFFOLD = if unsafe { !(*dtd).scaffold.is_null() } {
            if unsafe { (*dtd).scaffSize > UINT_MAX.wrapping_div(2u32) } {
                return -(1i32);
            }
            let temp = unsafe {
                expat_realloc(
                    parser,
                    (*dtd).scaffold as *mut c_void,
                    ((*dtd).scaffSize.wrapping_mul(2u32) as size_t)
                        .wrapping_mul(size_of::<CONTENT_SCAFFOLD>()),
                    8261,
                ) as *mut CONTENT_SCAFFOLD
            };
            if temp.is_null() {
                return -(1i32);
            }
            unsafe { (*dtd).scaffSize = (*dtd).scaffSize.wrapping_mul(2u32) };
            temp
        } else {
            let temp = expat_malloc(
                parser,
                (32usize).wrapping_mul(size_of::<CONTENT_SCAFFOLD>()),
                8266,
            ) as *mut CONTENT_SCAFFOLD;
            if temp.is_null() {
                return -(1i32);
            }
            unsafe { (*dtd).scaffSize = INIT_SCAFFOLD_ELEMENTS as c_uint };
            temp
        };
        unsafe { (*dtd).scaffold = temp };
    }
    next = unsafe { (*dtd).scaffCount as c_int };
    unsafe { (*dtd).scaffCount = (*dtd).scaffCount.wrapping_add(1) };
    me = unsafe { (*dtd).scaffold.offset(next as isize) };
    if unsafe { (*dtd).scaffLevel != 0 } {
        let parent: *mut CONTENT_SCAFFOLD = unsafe {
            (*dtd)
                .scaffold
                .offset(*(*dtd).scaffIndex.offset(((*dtd).scaffLevel - 1) as isize) as isize)
        };
        if unsafe { (*parent).lastchild != 0 } {
            unsafe { (*(*dtd).scaffold.offset((*parent).lastchild as isize)).nextsib = next };
        }
        if unsafe { (*parent).childcnt == 0 } {
            unsafe { (*parent).firstchild = next };
        }
        unsafe { (*parent).lastchild = next };
        unsafe { (*parent).childcnt += 1 };
    }
    unsafe { (*me).nextsib = 0 };
    unsafe { (*me).childcnt = (*me).nextsib };
    unsafe { (*me).lastchild = (*me).childcnt };
    unsafe { (*me).firstchild = (*me).lastchild };
    next
}

extern "C" fn build_model(mut parser: XML_Parser) -> *mut XML_Content {
    let dtd: *mut DTD = unsafe { (*parser).m_dtd };
    let mut str: *mut XML_Char = null_mut::<XML_Char>();
    if unsafe {
        ((*dtd).scaffCount as usize).wrapping_mul(size_of::<XML_Content>())
            > (SIZE_MAX as usize).wrapping_sub(
                ((*dtd).contentStringLen as usize).wrapping_mul(size_of::<XML_Char>()),
            )
    } {
        return null_mut::<XML_Content>();
    }
    let allocsize: size_t = unsafe { (*dtd).scaffCount as size_t }
        .wrapping_mul(size_of::<XML_Content>())
        .wrapping_add(unsafe {
            ((*dtd).contentStringLen as size_t).wrapping_mul(size_of::<XML_Char>())
        });
    let ret = unsafe {
        (*parser)
            .m_mem
            .malloc_fcn
            .expect("non-null function pointer")(allocsize) as *mut XML_Content
    };
    if ret.is_null() {
        return null_mut::<XML_Content>();
    }
    let mut dest: *mut XML_Content = ret;
    let destLimit: *mut XML_Content = unsafe { ret.offset((*dtd).scaffCount as isize) };
    let mut jobDest: *mut XML_Content = ret;
    str = unsafe { ret.offset((*dtd).scaffCount as isize) as *mut XML_Char };
    let fresh11 = jobDest;
    jobDest = unsafe { jobDest.offset(1) };
    unsafe { (*fresh11).numchildren = 0u32 };
    while dest < destLimit {
        let src_node: c_int = unsafe { (*dest).numchildren as c_int };
        unsafe { (*dest).type_0 = (*(*dtd).scaffold.offset(src_node as isize)).type_0 };
        unsafe { (*dest).quant = (*(*dtd).scaffold.offset(src_node as isize)).quant };
        if unsafe { (*dest).type_0 == XML_CTYPE_NAME } {
            let mut src: *const XML_Char = null::<XML_Char>();
            unsafe { (*dest).name = str };
            src = unsafe { (*(*dtd).scaffold.offset(src_node as isize)).name };
            loop {
                let fresh12 = str;
                str = unsafe { str.offset(1) };
                unsafe { *fresh12 = *src };
                if unsafe { *src == 0 } {
                    break;
                }
                src = unsafe { src.offset(1) };
            }
            unsafe { (*dest).numchildren = 0 };
            unsafe { (*dest).children = null_mut::<XML_Content>() };
        } else {
            let mut i: c_uint = 0;
            let mut cn: c_int = 0;
            unsafe { (*dest).name = null_mut::<XML_Char>() };
            unsafe {
                (*dest).numchildren =
                    (*(*dtd).scaffold.offset(src_node as isize)).childcnt as c_uint
            };
            unsafe { (*dest).children = jobDest };
            i = 0;
            cn = unsafe { (*(*dtd).scaffold.offset(src_node as isize)).firstchild };
            while i < unsafe { (*dest).numchildren } {
                let fresh13 = jobDest;
                jobDest = unsafe { jobDest.offset(1) };
                unsafe { (*fresh13).numchildren = cn as c_uint };
                i = i.wrapping_add(1);
                cn = unsafe { (*(*dtd).scaffold.offset(cn as isize)).nextsib };
            }
        }
        dest = unsafe { dest.offset(1) };
    }
    ret
}

extern "C" fn getElementType(
    mut parser: XML_Parser,
    enc: &ENCODING,
    mut ptr: *const c_char,
    mut end: *const c_char,
) -> *mut ELEMENT_TYPE {
    let dtd: *mut DTD = unsafe { (*parser).m_dtd };
    let pool = unsafe { &raw mut (*dtd).pool };
    let mut name: *const XML_Char = poolStoreString(pool, enc, ptr, end);
    let mut ret: *mut ELEMENT_TYPE = null_mut::<ELEMENT_TYPE>();
    if name.is_null() {
        return null_mut::<ELEMENT_TYPE>();
    }
    let element_types = unsafe { &raw mut (*dtd).elementTypes };
    ret = lookup(parser, element_types, name, size_of::<ELEMENT_TYPE>()) as *mut ELEMENT_TYPE;
    if ret.is_null() {
        return null_mut::<ELEMENT_TYPE>();
    }
    if unsafe { (*ret).name != name } {
        let pool_start = unsafe { (*dtd).pool.start };
        unsafe { (*dtd).pool.ptr = pool_start };
    } else {
        let pool_ptr = unsafe { (*dtd).pool.ptr };
        unsafe { (*dtd).pool.start = pool_ptr };
        if setElementTypePrefix(parser, ret) == 0 {
            return null_mut::<ELEMENT_TYPE>();
        }
    }
    ret
}

extern "C" fn copyString(mut s: *const XML_Char, mut parser: XML_Parser) -> *mut XML_Char {
    let mut charsRequired: size_t = 0;
    let mut result: *mut XML_Char = null_mut::<XML_Char>();
    while unsafe { *s.add(charsRequired) as c_int != 0 } {
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
    unsafe {
        memcpy(
            result as *mut c_void,
            s as *const c_void,
            charsRequired.wrapping_mul(size_of::<XML_Char>()),
        );
    }
    result
}

extern "C" fn accountingGetCurrentAmplification(mut rootParser: XML_Parser) -> c_float {
    let lenOfShortestInclude: size_t = (size_of::<[c_char; 23]>()).wrapping_sub(1usize);
    let count_bytes_direct = unsafe { (*rootParser).m_accounting.countBytesDirect };
    let count_bytes_indirect = unsafe { (*rootParser).m_accounting.countBytesIndirect };
    let countBytesOutput: XmlBigCount = count_bytes_direct.wrapping_add(count_bytes_indirect);
    let amplificationFactor: c_float = if count_bytes_direct != 0 {
        countBytesOutput as c_float / count_bytes_direct as c_float
    } else {
        (lenOfShortestInclude as XmlBigCount).wrapping_add(count_bytes_indirect) as c_float
            / lenOfShortestInclude as c_float
    };
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    amplificationFactor
}

extern "C" fn accountingReportStats(mut originParser: XML_Parser, mut epilog: *const c_char) {
    let rootParser: XML_Parser = getRootParserOf(originParser, null_mut::<c_uint>());
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    let debug_level = unsafe { (*rootParser).m_accounting.debugLevel };
    if debug_level == 0 {
        return;
    }
    let amplificationFactor: c_float = accountingGetCurrentAmplification(rootParser);
    let count_direct = unsafe { (*rootParser).m_accounting.countBytesDirect };
    let count_indirect = unsafe { (*rootParser).m_accounting.countBytesIndirect };
    unsafe {
        fprintf(
            stderr,
            b"expat: Accounting(%p): Direct %10llu, indirect %10llu, amplification %8.2f%s\0"
                as *const u8 as *const c_char,
            rootParser as *mut c_void,
            count_direct,
            count_indirect,
            amplificationFactor as core::ffi::c_double,
            epilog,
        );
    }
}

extern "C" fn accountingOnAbort(mut originParser: XML_Parser) {
    accountingReportStats(originParser, b" ABORTING\n\0" as *const u8 as *const c_char);
}

extern "C" fn accountingReportDiff(
    mut rootParser: XML_Parser,
    mut levelsAwayFromRootParser: c_uint,
    mut before: *const c_char,
    mut after: *const c_char,
    mut bytesMore: ptrdiff_t,
    mut source_line: c_int,
    mut account: XML_Account,
) {
    let root_parser_ref = unsafe { &mut *rootParser };
    assert!(root_parser_ref.m_parentParser.is_null());
    let account_label = if account == XML_ACCOUNT_DIRECT {
        b"DIR\0" as *const u8 as *const c_char
    } else {
        b"EXP\0" as *const u8 as *const c_char
    };
    unsafe {
        fprintf(
            stderr,
            b" (+%6ld bytes %s|%u, xmlparse.c:%d) %*s\"\0" as *const u8 as *const c_char,
            bytesMore,
            account_label,
            levelsAwayFromRootParser,
            source_line,
            10i32,
            b"\0" as *const u8 as *const c_char,
        )
    };
    let ellipis: [c_char; 5] = unsafe { core::mem::transmute::<[u8; 5], [c_char; 5]>(*b"[..]\0") };
    let ellipsisLength: size_t = (size_of::<[c_char; 5]>()).wrapping_sub(1usize);
    let contextLength: c_uint = 10;
    let mut walker: *const c_char = before;
    if root_parser_ref.m_accounting.debugLevel >= 3u64
        || unsafe { after.offset_from(before) }
            <= (contextLength as size_t)
                .wrapping_add(ellipsisLength)
                .wrapping_add(contextLength as size_t) as ptrdiff_t
    {
        while walker < after {
            unsafe {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const c_char,
                    unsignedCharToPrintable(*walker.offset(0isize) as c_uchar),
                )
            };
            walker = unsafe { walker.offset(1) };
        }
    } else {
        while walker < unsafe { before.offset(contextLength as isize) } {
            unsafe {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const c_char,
                    unsignedCharToPrintable(*walker.offset(0isize) as c_uchar),
                )
            };
            walker = unsafe { walker.offset(1) };
        }
        unsafe { fprintf(stderr, &raw const ellipis as *const c_char) };
        walker = unsafe { after.offset(-(contextLength as isize)) };
        while walker < after {
            unsafe {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const c_char,
                    unsignedCharToPrintable(*walker.offset(0isize) as c_uchar),
                )
            };
            walker = unsafe { walker.offset(1) };
        }
    }
    unsafe { fprintf(stderr, b"\"\n\0" as *const u8 as *const c_char) };
}

extern "C" fn accountingDiffTolerated(
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
    let root_parser_ref = unsafe { &mut *rootParser };
    assert!(root_parser_ref.m_parentParser.is_null());
    let isDirect: c_int = (account == XML_ACCOUNT_DIRECT && originParser == rootParser) as c_int;
    let bytesMore: ptrdiff_t = unsafe { after.offset_from(before) };
    let additionTarget: *mut XmlBigCount = if isDirect != 0 {
        &raw mut root_parser_ref.m_accounting.countBytesDirect
    } else {
        &raw mut root_parser_ref.m_accounting.countBytesIndirect
    };
    if unsafe { *additionTarget } > (-(1i32) as XmlBigCount).wrapping_sub(bytesMore as XmlBigCount)
    {
        return XML_FALSE;
    }
    unsafe { *additionTarget = (*additionTarget).wrapping_add(bytesMore as XmlBigCount) };
    let countBytesOutput: XmlBigCount = root_parser_ref
        .m_accounting
        .countBytesDirect
        .wrapping_add(root_parser_ref.m_accounting.countBytesIndirect);
    let amplificationFactor: c_float = accountingGetCurrentAmplification(rootParser);
    let tolerated: XML_Bool = (countBytesOutput
        < root_parser_ref.m_accounting.activationThresholdBytes
        || amplificationFactor <= root_parser_ref.m_accounting.maximumAmplificationFactor)
        as XML_Bool;
    if root_parser_ref.m_accounting.debugLevel >= 2 {
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
    tolerated
}
fn testingAccountingGetCountBytesDirect(mut parser: XML_Parser) -> c_ulonglong {
    if parser.is_null() {
        return 0u64;
    }
    unsafe { (*parser).m_accounting.countBytesDirect }
}

#[cfg(feature = "expat_test_shims")]
#[export_name = "testingAccountingGetCountBytesDirect"]
extern "C" fn testing_accounting_get_count_bytes_direct_test_shim(
    parser: XML_Parser,
) -> c_ulonglong {
    testingAccountingGetCountBytesDirect(parser)
}

fn testingAccountingGetCountBytesIndirect(mut parser: XML_Parser) -> c_ulonglong {
    if parser.is_null() {
        return 0u64;
    }
    unsafe { (*parser).m_accounting.countBytesIndirect }
}

#[cfg(feature = "expat_test_shims")]
#[export_name = "testingAccountingGetCountBytesIndirect"]
extern "C" fn testing_accounting_get_count_bytes_indirect_test_shim(
    parser: XML_Parser,
) -> c_ulonglong {
    testingAccountingGetCountBytesIndirect(parser)
}

extern "C" fn entityTrackingReportStats(
    mut rootParser: XML_Parser,
    mut entity: *mut ENTITY,
    mut action: *const c_char,
    mut sourceLine: c_int,
) {
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    if unsafe { (*rootParser).m_entity_stats.debugLevel == 0u64 } {
        return;
    }
    let entityName: *const c_char = unsafe { (*entity).name };
    let count_ever_opened = unsafe { (*rootParser).m_entity_stats.countEverOpened };
    let current_depth = unsafe { (*rootParser).m_entity_stats.currentDepth };
    let max_depth = unsafe { (*rootParser).m_entity_stats.maximumDepthSeen };
    let indent = (current_depth as c_int - 1i32) * 2i32;
    let entity_prefix = if unsafe { (*entity).is_param as c_int != 0 } {
        b"%\0" as *const u8 as *const c_char
    } else {
        b"&\0" as *const u8 as *const c_char
    };
    let text_len = unsafe { (*entity).textLen };
    unsafe {
        fprintf(stderr, b"expat: Entities(%p): Count %9u, depth %2u/%2u %*s%s%s; %s length %d (xmlparse.c:%d)\n\0" as *const u8 as *const c_char, rootParser as *mut c_void, count_ever_opened, current_depth, max_depth, indent, b"\0" as *const u8 as *const c_char, entity_prefix, entityName, action, text_len, sourceLine);
    }
}

extern "C" fn entityTrackingOnOpen(
    mut originParser: XML_Parser,
    mut entity: *mut ENTITY,
    mut sourceLine: c_int,
) {
    let rootParser: XML_Parser = getRootParserOf(originParser, null_mut::<c_uint>());
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    let count_ever_opened = unsafe { (*rootParser).m_entity_stats.countEverOpened };
    unsafe { (*rootParser).m_entity_stats.countEverOpened = count_ever_opened.wrapping_add(1) };
    let current_depth = unsafe { (*rootParser).m_entity_stats.currentDepth };
    unsafe { (*rootParser).m_entity_stats.currentDepth = current_depth.wrapping_add(1) };
    let current_depth = unsafe { (*rootParser).m_entity_stats.currentDepth };
    let max_depth = unsafe { (*rootParser).m_entity_stats.maximumDepthSeen };
    if current_depth > max_depth {
        unsafe { (*rootParser).m_entity_stats.maximumDepthSeen = max_depth.wrapping_add(1) };
    }
    entityTrackingReportStats(
        rootParser,
        entity,
        b"OPEN \0" as *const u8 as *const c_char,
        sourceLine,
    );
}

extern "C" fn entityTrackingOnClose(
    mut originParser: XML_Parser,
    mut entity: *mut ENTITY,
    mut sourceLine: c_int,
) {
    let rootParser: XML_Parser = getRootParserOf(originParser, null_mut::<c_uint>());
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    entityTrackingReportStats(
        rootParser,
        entity,
        b"CLOSE\0" as *const u8 as *const c_char,
        sourceLine,
    );
    let current_depth = unsafe { (*rootParser).m_entity_stats.currentDepth };
    unsafe { (*rootParser).m_entity_stats.currentDepth = current_depth.wrapping_sub(1) };
}

extern "C" fn getRootParserOf(mut parser: XML_Parser, mut outLevelDiff: *mut c_uint) -> XML_Parser {
    let mut rootParser: XML_Parser = parser;
    let mut stepsTakenUpwards: c_uint = 0;
    while unsafe { !(*rootParser).m_parentParser.is_null() } {
        rootParser = unsafe { (*rootParser).m_parentParser };
        stepsTakenUpwards = stepsTakenUpwards.wrapping_add(1);
    }
    assert!(unsafe { (*rootParser).m_parentParser.is_null() });
    if !outLevelDiff.is_null() {
        unsafe { *outLevelDiff = stepsTakenUpwards };
    }
    rootParser
}
fn unsignedCharToPrintable(mut c: c_uchar) -> *const c_char {
    match c as c_int {
        0 => b"\\0\0" as *const u8 as *const c_char,
        1 => b"\\x1\0" as *const u8 as *const c_char,
        2 => b"\\x2\0" as *const u8 as *const c_char,
        3 => b"\\x3\0" as *const u8 as *const c_char,
        4 => b"\\x4\0" as *const u8 as *const c_char,
        5 => b"\\x5\0" as *const u8 as *const c_char,
        6 => b"\\x6\0" as *const u8 as *const c_char,
        7 => b"\\x7\0" as *const u8 as *const c_char,
        8 => b"\\x8\0" as *const u8 as *const c_char,
        9 => b"\\t\0" as *const u8 as *const c_char,
        10 => b"\\n\0" as *const u8 as *const c_char,
        11 => b"\\xB\0" as *const u8 as *const c_char,
        12 => b"\\xC\0" as *const u8 as *const c_char,
        13 => b"\\r\0" as *const u8 as *const c_char,
        14 => b"\\xE\0" as *const u8 as *const c_char,
        15 => b"\\xF\0" as *const u8 as *const c_char,
        16 => b"\\x10\0" as *const u8 as *const c_char,
        17 => b"\\x11\0" as *const u8 as *const c_char,
        18 => b"\\x12\0" as *const u8 as *const c_char,
        19 => b"\\x13\0" as *const u8 as *const c_char,
        20 => b"\\x14\0" as *const u8 as *const c_char,
        21 => b"\\x15\0" as *const u8 as *const c_char,
        22 => b"\\x16\0" as *const u8 as *const c_char,
        23 => b"\\x17\0" as *const u8 as *const c_char,
        24 => b"\\x18\0" as *const u8 as *const c_char,
        25 => b"\\x19\0" as *const u8 as *const c_char,
        26 => b"\\x1A\0" as *const u8 as *const c_char,
        27 => b"\\x1B\0" as *const u8 as *const c_char,
        28 => b"\\x1C\0" as *const u8 as *const c_char,
        29 => b"\\x1D\0" as *const u8 as *const c_char,
        30 => b"\\x1E\0" as *const u8 as *const c_char,
        31 => b"\\x1F\0" as *const u8 as *const c_char,
        32 => b" \0" as *const u8 as *const c_char,
        33 => b"!\0" as *const u8 as *const c_char,
        34 => b"\\\"\0" as *const u8 as *const c_char,
        35 => b"#\0" as *const u8 as *const c_char,
        36 => b"$\0" as *const u8 as *const c_char,
        37 => b"%\0" as *const u8 as *const c_char,
        38 => b"&\0" as *const u8 as *const c_char,
        39 => b"'\0" as *const u8 as *const c_char,
        40 => b"(\0" as *const u8 as *const c_char,
        41 => b")\0" as *const u8 as *const c_char,
        42 => b"*\0" as *const u8 as *const c_char,
        43 => b"+\0" as *const u8 as *const c_char,
        44 => b",\0" as *const u8 as *const c_char,
        45 => b"-\0" as *const u8 as *const c_char,
        46 => b".\0" as *const u8 as *const c_char,
        47 => b"/\0" as *const u8 as *const c_char,
        48 => b"0\0" as *const u8 as *const c_char,
        49 => b"1\0" as *const u8 as *const c_char,
        50 => b"2\0" as *const u8 as *const c_char,
        51 => b"3\0" as *const u8 as *const c_char,
        52 => b"4\0" as *const u8 as *const c_char,
        53 => b"5\0" as *const u8 as *const c_char,
        54 => b"6\0" as *const u8 as *const c_char,
        55 => b"7\0" as *const u8 as *const c_char,
        56 => b"8\0" as *const u8 as *const c_char,
        57 => b"9\0" as *const u8 as *const c_char,
        58 => b":\0" as *const u8 as *const c_char,
        59 => b";\0" as *const u8 as *const c_char,
        60 => b"<\0" as *const u8 as *const c_char,
        61 => b"=\0" as *const u8 as *const c_char,
        62 => b">\0" as *const u8 as *const c_char,
        63 => b"?\0" as *const u8 as *const c_char,
        64 => b"@\0" as *const u8 as *const c_char,
        65 => b"A\0" as *const u8 as *const c_char,
        66 => b"B\0" as *const u8 as *const c_char,
        67 => b"C\0" as *const u8 as *const c_char,
        68 => b"D\0" as *const u8 as *const c_char,
        69 => b"E\0" as *const u8 as *const c_char,
        70 => b"F\0" as *const u8 as *const c_char,
        71 => b"G\0" as *const u8 as *const c_char,
        72 => b"H\0" as *const u8 as *const c_char,
        73 => b"I\0" as *const u8 as *const c_char,
        74 => b"J\0" as *const u8 as *const c_char,
        75 => b"K\0" as *const u8 as *const c_char,
        76 => b"L\0" as *const u8 as *const c_char,
        77 => b"M\0" as *const u8 as *const c_char,
        78 => b"N\0" as *const u8 as *const c_char,
        79 => b"O\0" as *const u8 as *const c_char,
        80 => b"P\0" as *const u8 as *const c_char,
        81 => b"Q\0" as *const u8 as *const c_char,
        82 => b"R\0" as *const u8 as *const c_char,
        83 => b"S\0" as *const u8 as *const c_char,
        84 => b"T\0" as *const u8 as *const c_char,
        85 => b"U\0" as *const u8 as *const c_char,
        86 => b"V\0" as *const u8 as *const c_char,
        87 => b"W\0" as *const u8 as *const c_char,
        88 => b"X\0" as *const u8 as *const c_char,
        89 => b"Y\0" as *const u8 as *const c_char,
        90 => b"Z\0" as *const u8 as *const c_char,
        91 => b"[\0" as *const u8 as *const c_char,
        92 => b"\\\\\0" as *const u8 as *const c_char,
        93 => b"]\0" as *const u8 as *const c_char,
        94 => b"^\0" as *const u8 as *const c_char,
        95 => b"_\0" as *const u8 as *const c_char,
        96 => b"`\0" as *const u8 as *const c_char,
        97 => b"a\0" as *const u8 as *const c_char,
        98 => b"b\0" as *const u8 as *const c_char,
        99 => b"c\0" as *const u8 as *const c_char,
        100 => b"d\0" as *const u8 as *const c_char,
        101 => b"e\0" as *const u8 as *const c_char,
        102 => b"f\0" as *const u8 as *const c_char,
        103 => b"g\0" as *const u8 as *const c_char,
        104 => b"h\0" as *const u8 as *const c_char,
        105 => b"i\0" as *const u8 as *const c_char,
        106 => b"j\0" as *const u8 as *const c_char,
        107 => b"k\0" as *const u8 as *const c_char,
        108 => b"l\0" as *const u8 as *const c_char,
        109 => b"m\0" as *const u8 as *const c_char,
        110 => b"n\0" as *const u8 as *const c_char,
        111 => b"o\0" as *const u8 as *const c_char,
        112 => b"p\0" as *const u8 as *const c_char,
        113 => b"q\0" as *const u8 as *const c_char,
        114 => b"r\0" as *const u8 as *const c_char,
        115 => b"s\0" as *const u8 as *const c_char,
        116 => b"t\0" as *const u8 as *const c_char,
        117 => b"u\0" as *const u8 as *const c_char,
        118 => b"v\0" as *const u8 as *const c_char,
        119 => b"w\0" as *const u8 as *const c_char,
        120 => b"x\0" as *const u8 as *const c_char,
        121 => b"y\0" as *const u8 as *const c_char,
        122 => b"z\0" as *const u8 as *const c_char,
        123 => b"{\0" as *const u8 as *const c_char,
        124 => b"|\0" as *const u8 as *const c_char,
        125 => b"}\0" as *const u8 as *const c_char,
        126 => b"~\0" as *const u8 as *const c_char,
        127 => b"\\x7F\0" as *const u8 as *const c_char,
        128 => b"\\x80\0" as *const u8 as *const c_char,
        129 => b"\\x81\0" as *const u8 as *const c_char,
        130 => b"\\x82\0" as *const u8 as *const c_char,
        131 => b"\\x83\0" as *const u8 as *const c_char,
        132 => b"\\x84\0" as *const u8 as *const c_char,
        133 => b"\\x85\0" as *const u8 as *const c_char,
        134 => b"\\x86\0" as *const u8 as *const c_char,
        135 => b"\\x87\0" as *const u8 as *const c_char,
        136 => b"\\x88\0" as *const u8 as *const c_char,
        137 => b"\\x89\0" as *const u8 as *const c_char,
        138 => b"\\x8A\0" as *const u8 as *const c_char,
        139 => b"\\x8B\0" as *const u8 as *const c_char,
        140 => b"\\x8C\0" as *const u8 as *const c_char,
        141 => b"\\x8D\0" as *const u8 as *const c_char,
        142 => b"\\x8E\0" as *const u8 as *const c_char,
        143 => b"\\x8F\0" as *const u8 as *const c_char,
        144 => b"\\x90\0" as *const u8 as *const c_char,
        145 => b"\\x91\0" as *const u8 as *const c_char,
        146 => b"\\x92\0" as *const u8 as *const c_char,
        147 => b"\\x93\0" as *const u8 as *const c_char,
        148 => b"\\x94\0" as *const u8 as *const c_char,
        149 => b"\\x95\0" as *const u8 as *const c_char,
        150 => b"\\x96\0" as *const u8 as *const c_char,
        151 => b"\\x97\0" as *const u8 as *const c_char,
        152 => b"\\x98\0" as *const u8 as *const c_char,
        153 => b"\\x99\0" as *const u8 as *const c_char,
        154 => b"\\x9A\0" as *const u8 as *const c_char,
        155 => b"\\x9B\0" as *const u8 as *const c_char,
        156 => b"\\x9C\0" as *const u8 as *const c_char,
        157 => b"\\x9D\0" as *const u8 as *const c_char,
        158 => b"\\x9E\0" as *const u8 as *const c_char,
        159 => b"\\x9F\0" as *const u8 as *const c_char,
        160 => b"\\xA0\0" as *const u8 as *const c_char,
        161 => b"\\xA1\0" as *const u8 as *const c_char,
        162 => b"\\xA2\0" as *const u8 as *const c_char,
        163 => b"\\xA3\0" as *const u8 as *const c_char,
        164 => b"\\xA4\0" as *const u8 as *const c_char,
        165 => b"\\xA5\0" as *const u8 as *const c_char,
        166 => b"\\xA6\0" as *const u8 as *const c_char,
        167 => b"\\xA7\0" as *const u8 as *const c_char,
        168 => b"\\xA8\0" as *const u8 as *const c_char,
        169 => b"\\xA9\0" as *const u8 as *const c_char,
        170 => b"\\xAA\0" as *const u8 as *const c_char,
        171 => b"\\xAB\0" as *const u8 as *const c_char,
        172 => b"\\xAC\0" as *const u8 as *const c_char,
        173 => b"\\xAD\0" as *const u8 as *const c_char,
        174 => b"\\xAE\0" as *const u8 as *const c_char,
        175 => b"\\xAF\0" as *const u8 as *const c_char,
        176 => b"\\xB0\0" as *const u8 as *const c_char,
        177 => b"\\xB1\0" as *const u8 as *const c_char,
        178 => b"\\xB2\0" as *const u8 as *const c_char,
        179 => b"\\xB3\0" as *const u8 as *const c_char,
        180 => b"\\xB4\0" as *const u8 as *const c_char,
        181 => b"\\xB5\0" as *const u8 as *const c_char,
        182 => b"\\xB6\0" as *const u8 as *const c_char,
        183 => b"\\xB7\0" as *const u8 as *const c_char,
        184 => b"\\xB8\0" as *const u8 as *const c_char,
        185 => b"\\xB9\0" as *const u8 as *const c_char,
        186 => b"\\xBA\0" as *const u8 as *const c_char,
        187 => b"\\xBB\0" as *const u8 as *const c_char,
        188 => b"\\xBC\0" as *const u8 as *const c_char,
        189 => b"\\xBD\0" as *const u8 as *const c_char,
        190 => b"\\xBE\0" as *const u8 as *const c_char,
        191 => b"\\xBF\0" as *const u8 as *const c_char,
        192 => b"\\xC0\0" as *const u8 as *const c_char,
        193 => b"\\xC1\0" as *const u8 as *const c_char,
        194 => b"\\xC2\0" as *const u8 as *const c_char,
        195 => b"\\xC3\0" as *const u8 as *const c_char,
        196 => b"\\xC4\0" as *const u8 as *const c_char,
        197 => b"\\xC5\0" as *const u8 as *const c_char,
        198 => b"\\xC6\0" as *const u8 as *const c_char,
        199 => b"\\xC7\0" as *const u8 as *const c_char,
        200 => b"\\xC8\0" as *const u8 as *const c_char,
        201 => b"\\xC9\0" as *const u8 as *const c_char,
        202 => b"\\xCA\0" as *const u8 as *const c_char,
        203 => b"\\xCB\0" as *const u8 as *const c_char,
        204 => b"\\xCC\0" as *const u8 as *const c_char,
        205 => b"\\xCD\0" as *const u8 as *const c_char,
        206 => b"\\xCE\0" as *const u8 as *const c_char,
        207 => b"\\xCF\0" as *const u8 as *const c_char,
        208 => b"\\xD0\0" as *const u8 as *const c_char,
        209 => b"\\xD1\0" as *const u8 as *const c_char,
        210 => b"\\xD2\0" as *const u8 as *const c_char,
        211 => b"\\xD3\0" as *const u8 as *const c_char,
        212 => b"\\xD4\0" as *const u8 as *const c_char,
        213 => b"\\xD5\0" as *const u8 as *const c_char,
        214 => b"\\xD6\0" as *const u8 as *const c_char,
        215 => b"\\xD7\0" as *const u8 as *const c_char,
        216 => b"\\xD8\0" as *const u8 as *const c_char,
        217 => b"\\xD9\0" as *const u8 as *const c_char,
        218 => b"\\xDA\0" as *const u8 as *const c_char,
        219 => b"\\xDB\0" as *const u8 as *const c_char,
        220 => b"\\xDC\0" as *const u8 as *const c_char,
        221 => b"\\xDD\0" as *const u8 as *const c_char,
        222 => b"\\xDE\0" as *const u8 as *const c_char,
        223 => b"\\xDF\0" as *const u8 as *const c_char,
        224 => b"\\xE0\0" as *const u8 as *const c_char,
        225 => b"\\xE1\0" as *const u8 as *const c_char,
        226 => b"\\xE2\0" as *const u8 as *const c_char,
        227 => b"\\xE3\0" as *const u8 as *const c_char,
        228 => b"\\xE4\0" as *const u8 as *const c_char,
        229 => b"\\xE5\0" as *const u8 as *const c_char,
        230 => b"\\xE6\0" as *const u8 as *const c_char,
        231 => b"\\xE7\0" as *const u8 as *const c_char,
        232 => b"\\xE8\0" as *const u8 as *const c_char,
        233 => b"\\xE9\0" as *const u8 as *const c_char,
        234 => b"\\xEA\0" as *const u8 as *const c_char,
        235 => b"\\xEB\0" as *const u8 as *const c_char,
        236 => b"\\xEC\0" as *const u8 as *const c_char,
        237 => b"\\xED\0" as *const u8 as *const c_char,
        238 => b"\\xEE\0" as *const u8 as *const c_char,
        239 => b"\\xEF\0" as *const u8 as *const c_char,
        240 => b"\\xF0\0" as *const u8 as *const c_char,
        241 => b"\\xF1\0" as *const u8 as *const c_char,
        242 => b"\\xF2\0" as *const u8 as *const c_char,
        243 => b"\\xF3\0" as *const u8 as *const c_char,
        244 => b"\\xF4\0" as *const u8 as *const c_char,
        245 => b"\\xF5\0" as *const u8 as *const c_char,
        246 => b"\\xF6\0" as *const u8 as *const c_char,
        247 => b"\\xF7\0" as *const u8 as *const c_char,
        248 => b"\\xF8\0" as *const u8 as *const c_char,
        249 => b"\\xF9\0" as *const u8 as *const c_char,
        250 => b"\\xFA\0" as *const u8 as *const c_char,
        251 => b"\\xFB\0" as *const u8 as *const c_char,
        252 => b"\\xFC\0" as *const u8 as *const c_char,
        253 => b"\\xFD\0" as *const u8 as *const c_char,
        254 => b"\\xFE\0" as *const u8 as *const c_char,
        255 => b"\\xFF\0" as *const u8 as *const c_char,
        _ => {
            assert!(0 != 0);
            b"dead code\0" as *const u8 as *const c_char
        }
    }
}

#[cfg(feature = "expat_test_shims")]
#[export_name = "unsignedCharToPrintable"]
extern "C" fn unsigned_char_to_printable_test_shim(c: c_uchar) -> *const c_char {
    unsignedCharToPrintable(c)
}

extern "C" fn getDebugLevel(
    mut variableName: *const c_char,
    mut defaultDebugLevel: c_ulong,
) -> c_ulong {
    let valueOrNull: *const c_char = unsafe { crate::stdlib::getenv(variableName) };
    if valueOrNull.is_null() {
        return defaultDebugLevel;
    }
    let value: *const c_char = valueOrNull;
    let errno_ptr = unsafe { __errno_location() };
    unsafe { *errno_ptr = 0 };
    let mut afterValue: *mut c_char = null_mut::<c_char>();
    let mut debugLevel: c_ulong = unsafe { crate::stdlib::strtoul(value, &raw mut afterValue, 10) };
    let has_parse_error = unsafe { *errno_ptr != 0 }
        || std::ptr::eq(afterValue, value)
        || unsafe { *afterValue.offset(0) as c_int != '\0' as i32 };
    if has_parse_error {
        unsafe { *errno_ptr = 0 };
        return defaultDebugLevel;
    }
    debugLevel
}
